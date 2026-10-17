// SPDX-License-Identifier: EPL-2.0
//! Explicit, bounded, non-shell process capture. The process is NOT sandboxed.
use crate::report::{timestamp_millis, Capture};
use std::ffi::OsString;
use std::io::{self, Read};
use std::process::{Command, ExitStatus, Stdio};
use std::sync::mpsc::{self, RecvTimeoutError, SyncSender};
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct CaptureOptions {
    pub timeout: Duration,
    /// Maximum retained bytes in EACH output stream.
    pub max_output_bytes_per_stream: usize,
}

impl Default for CaptureOptions {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(30),
            max_output_bytes_per_stream: 65_536,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Stream {
    Stdout,
    Stderr,
}

enum Event {
    Data(Stream, Vec<u8>),
    Finished,
}

fn drain_reader<R: Read + Send + 'static>(mut reader: R, stream: Stream, tx: SyncSender<Event>) {
    std::thread::spawn(move || {
        let mut buf = [0_u8; 4096];
        loop {
            match reader.read(&mut buf) {
                Ok(0) => break,
                Ok(n) => {
                    if tx.send(Event::Data(stream, buf[..n].to_vec())).is_err() {
                        break;
                    }
                }
                Err(ref e) if e.kind() == io::ErrorKind::Interrupted => continue,
                Err(_) => break,
            }
        }
        let _ = tx.send(Event::Finished);
    });
}

fn push_bounded(destination: &mut Vec<u8>, incoming: &[u8], cap: usize, truncated: &mut bool) {
    let room = cap.saturating_sub(destination.len());
    destination.extend_from_slice(&incoming[..incoming.len().min(room)]);
    if incoming.len() > room {
        *truncated = true;
    }
}

#[cfg(unix)]
fn status_signal(status: &ExitStatus) -> Option<i32> {
    use std::os::unix::process::ExitStatusExt;
    status.signal()
}

#[cfg(not(unix))]
fn status_signal(_status: &ExitStatus) -> Option<i32> {
    None
}

/// Captures only a user-selected command and its bounded output.
///
/// Errors launching a process become a reportable capture record. Invalid CLI
/// options or failures supervising an already-running process return an I/O error.
pub fn capture_command(args: &[OsString], options: &CaptureOptions) -> io::Result<Capture> {
    if args.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "missing executable",
        ));
    }
    if options.timeout.is_zero() || options.timeout > Duration::from_secs(86_400) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "timeout must be between 1 second and 24 hours",
        ));
    }
    if !(1..=10_485_760).contains(&options.max_output_bytes_per_stream) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "stream limit must be between 1 and 10485760 bytes",
        ));
    }

    let started_unix_ms = timestamp_millis();
    let started = Instant::now();
    let program = args[0].to_string_lossy().into_owned();
    let arguments: Vec<String> = args[1..]
        .iter()
        .map(|s| s.to_string_lossy().into_owned())
        .collect();
    let empty = || Capture {
        executable: program.clone(),
        arguments: arguments.clone(),
        started_unix_ms,
        duration_ms: u64::try_from(started.elapsed().as_millis()).unwrap_or(u64::MAX),
        exit_code: None,
        termination_signal: None,
        timed_out: false,
        launch_error: None,
        stdout: String::new(),
        stderr: String::new(),
        stdout_truncated: false,
        stderr_truncated: false,
        max_output_bytes_per_stream: options.max_output_bytes_per_stream,
    };

    let mut child = match Command::new(&args[0])
        .args(&args[1..])
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
    {
        Ok(child) => child,
        Err(error) => {
            return Ok(Capture {
                launch_error: Some(error.to_string()),
                ..empty()
            })
        }
    };

    let (tx, rx) = mpsc::sync_channel(16);
    if let Some(stdout) = child.stdout.take() {
        drain_reader(stdout, Stream::Stdout, tx.clone());
    }
    if let Some(stderr) = child.stderr.take() {
        drain_reader(stderr, Stream::Stderr, tx.clone());
    }
    drop(tx);

    let mut out = Vec::new();
    let mut err = Vec::new();
    let mut out_truncated = false;
    let mut err_truncated = false;
    let mut streams_finished = 0_u8;
    let mut timed_out = false;
    let mut status: Option<ExitStatus> = None;
    let mut exited_at: Option<Instant> = None;

    loop {
        if status.is_none() {
            status = child.try_wait()?;
            if status.is_some() {
                exited_at = Some(Instant::now());
            } else if started.elapsed() >= options.timeout {
                // Killing and reaping the direct child is best-effort; grandchild
                // processes are not reliably terminated on every OS.
                let _ = child.kill();
                timed_out = true;
                status = Some(child.wait()?);
                exited_at = Some(Instant::now());
            }
        }
        if status.is_some() && streams_finished >= 2 {
            break;
        }
        // Do not hang indefinitely when a grandchild inherits the pipe handles.
        if exited_at.is_some_and(|when| when.elapsed() > Duration::from_millis(750)) {
            if streams_finished < 2 {
                out_truncated = true;
                err_truncated = true;
            }
            break;
        }
        match rx.recv_timeout(Duration::from_millis(20)) {
            Ok(Event::Data(Stream::Stdout, bytes)) => push_bounded(
                &mut out,
                &bytes,
                options.max_output_bytes_per_stream,
                &mut out_truncated,
            ),
            Ok(Event::Data(Stream::Stderr, bytes)) => push_bounded(
                &mut err,
                &bytes,
                options.max_output_bytes_per_stream,
                &mut err_truncated,
            ),
            Ok(Event::Finished) => streams_finished = streams_finished.saturating_add(1),
            Err(RecvTimeoutError::Timeout) => {}
            Err(RecvTimeoutError::Disconnected) => {
                // Handles that could not be drained are marked incomplete.
                if streams_finished < 2 {
                    out_truncated = true;
                    err_truncated = true;
                }
                break;
            }
        }
    }

    let status = match status {
        Some(status) => status,
        None => child.wait()?,
    };
    Ok(Capture {
        executable: program,
        arguments,
        started_unix_ms,
        duration_ms: u64::try_from(started.elapsed().as_millis()).unwrap_or(u64::MAX),
        exit_code: status.code(),
        termination_signal: status_signal(&status),
        timed_out,
        launch_error: None,
        stdout: String::from_utf8_lossy(&out).into_owned(),
        stderr: String::from_utf8_lossy(&err).into_owned(),
        stdout_truncated: out_truncated,
        stderr_truncated: err_truncated,
        max_output_bytes_per_stream: options.max_output_bytes_per_stream,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn missing_executable_generates_reportable_error() {
        let args = vec![OsString::from("reprocrate_nonexistent_program_x9y8z7")];
        let captured =
            capture_command(&args, &CaptureOptions::default()).expect("capture response");
        assert!(captured.launch_error.is_some());
        assert_eq!(captured.exit_code, None);
    }
    #[test]
    fn verifies_bounded_buffer() {
        let mut v = Vec::new();
        let mut truncated = false;
        push_bounded(&mut v, b"abcdefgh", 3, &mut truncated);
        assert_eq!(&v, b"abc");
        assert!(truncated);
    }
    #[test]
    fn rejects_unbounded_memory_requests() {
        let options = CaptureOptions {
            timeout: Duration::from_secs(1),
            max_output_bytes_per_stream: usize::MAX,
        };
        let result = capture_command(&[OsString::from("ignored")], &options);
        assert_eq!(
            result.expect_err("invalid option").kind(),
            io::ErrorKind::InvalidInput
        );
    }
}
