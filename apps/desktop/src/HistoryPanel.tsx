import { useCallback, useEffect, useState } from "react";
import { desktop, desktopAvailable } from "./bridge";
import { filenameDisplay } from "./format";
import type { HistoryEntry } from "./types";

export default function HistoryPanel({ onOpen }: { onOpen: (path: string) => void }) {
  const native = desktopAvailable();
  const [path, setPath] = useState("");
  const [query, setQuery] = useState("");
  const [items, setItems] = useState<HistoryEntry[]>([]);
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState("");
  const [info, setInfo] = useState("");

  const load = useCallback(async (search: string) => {
    if (!native) return;
    setBusy(true); setError("");
    try {
      setItems(await desktop.history(search));
    } catch (cause) {
      setError(String(cause));
    } finally {
      setBusy(false);
    }
  }, [native]);

  useEffect(() => {
    void load("");
  }, [load]);

  async function remember() {
    if (!path.trim() || !native) return;
    setBusy(true); setError(""); setInfo("");
    try {
      const entry = await desktop.remember(path.trim());
      setInfo("Indexed " + entry.label + " locally. No report contents were copied.");
      setPath("");
      setItems(await desktop.history(query));
    } catch (cause) {
      setError(String(cause));
    } finally {
      setBusy(false);
    }
  }

  async function forget(entry: HistoryEntry) {
    if (!native) return;
    setBusy(true); setError(""); setInfo("");
    try {
      await desktop.forget(entry.id);
      setInfo("Removed index entry. The original report file was not deleted.");
      setItems(await desktop.history(query));
    } catch (cause) {
      setError(String(cause));
    } finally {
      setBusy(false);
    }
  }

  return <>
    <div className="page-intro">
      <span>02 / OPT-IN PRIVATE INDEX</span>
      <h1>Report history</h1>
      <p>Keep a searchable index of reports you explicitly choose. Metadata and local file paths are stored in a private SQLite database. No logs, arguments, attachments or full report JSON are copied.</p>
    </div>
    <section className="form-panel">
      <label className="file-field" htmlFor="index-file">
        <span>SELECT A SAVED REPORT TO REMEMBER</span>
        <input type="text" spellCheck={false} autoComplete="off" id="index-file"
          placeholder="Enter the absolute JSON report file path" value={path}
          onChange={(event) => setPath(event.target.value)} />
      </label>
      <button type="button" className="primary" disabled={!native || busy || !path.trim()}
        onClick={() => void remember()}>Index this report locally →</button>
      <p className="annotation">Indexing is manual, never automatic. The saved path, report ID and title may be sensitive; do not share the database.</p>
    </section>
    <section className="history-section" aria-label="Search stored report index">
      <div className="section-heading"><h2>Saved report index</h2><span>Up to 1,000 entries · first 50 matches shown</span></div>
      <form className="history-search" onSubmit={(event) => { event.preventDefault(); void load(query); }}>
        <label className="file-field" htmlFor="history-search">
          <span>SEARCH BY TITLE, OS OR ARCHITECTURE</span>
          <input type="search" id="history-search" value={query} maxLength={128} autoComplete="off"
            placeholder="Rust, Linux, Python..." onChange={(event) => setQuery(event.target.value)} />
        </label>
        <button type="submit" className="secondary" disabled={busy || !native}>Search →</button>
      </form>
      {error && <div role="alert" className="notice error">{error}</div>}
      {info && <div role="status" className="notice success">{info}</div>}
      {items.length > 0
        ? <div className="history-results">{items.map((entry) => <article className="history-item" key={entry.id}>
            <div className="history-description">
              <span className="history-identity">{entry.os} / {entry.architecture}</span>
              <strong>{entry.label}</strong>
              <small>{filenameDisplay(entry.path)} · Indexed {new Date(entry.indexed_unix_seconds * 1000).toLocaleDateString()}</small>
            </div>
            <div className="history-actions">
              <button type="button" className="secondary" disabled={busy} onClick={() => onOpen(entry.path)}>Inspect →</button>
              <button type="button" className="forget" disabled={busy} onClick={() => void forget(entry)}>Forget index</button>
            </div>
          </article>)}</div>
        : <div className="empty"><span aria-hidden="true">◇</span><p>{busy ? "Loading saved reports…" : "No indexed reports match yet. Only explicitly added reports appear here."}</p></div>}
    </section>
  </>;
}
