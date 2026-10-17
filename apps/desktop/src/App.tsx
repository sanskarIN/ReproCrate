import { useEffect, useState } from "react";
import { desktop, desktopAvailable } from "./bridge";
import HistoryPanel from "./HistoryPanel";
import { countComparisonChanges, durationLabel, filenameDisplay } from "./format";
import type { DesktopStatus, LogAnalysis, Recipe, ReportComparison, ReportSummary, ReviewedReport, ReviewOptions, View } from "./types";

const sections: {id:View; title:string; summary:string}[] = [
  {id:"overview",title:"Workspace",summary:"Your local diagnostics desk"},
  {id:"history",title:"Report history",summary:"Your opt-in private index"},
  {id:"inspect",title:"Inspect report",summary:"Read stored metadata"},
  {id:"compare",title:"Compare reports",summary:"Private structural changes"},
  {id:"logs",title:"Analyze logs",summary:"Heuristic failure signals"},
  {id:"recipe",title:"Manual recipe",summary:"A non-executable checklist"},
  {id:"privacy",title:"Privacy review",summary:"Control what gets exported"},
];
const reviewDefaults: ReviewOptions = {
  excludeStdout:false, excludeStderr:false, excludeArguments:false,
  excludeExecutable:false, excludeAnalysis:false, literals:"",
};

function FileField({id,label,value,change}:{id:string;label:string;value:string;change:(v:string)=>void}) {
  return <label className="file-field" htmlFor={id}><span>{label}</span><input id={id} value={value}
    type="text" autoComplete="off" spellCheck={false} placeholder="Enter the absolute local file path"
    onChange={(event)=>change(event.target.value)}/></label>;
}

function Empty({message}:{message:string}) {
  return <div className="empty"><span aria-hidden="true">◇</span><p>{message}</p></div>;
}

export default function App() {
  const native = desktopAvailable();
  const [view,setView] = useState<View>("overview");
  const [status,setStatus] = useState<DesktopStatus|null>(null);
  const [busy,setBusy] = useState(false);
  const [failure,setFailure] = useState("");
  const [success,setSuccess] = useState("");
  const [reportPath,setReportPath] = useState("");
  const [candidatePath,setCandidatePath] = useState("");
  const [logPath,setLogPath] = useState("");
  const [exportPath,setExportPath] = useState("");
  const [options,setOptions] = useState<ReviewOptions>(reviewDefaults);
  const [summary,setSummary] = useState<ReportSummary|null>(null);
  const [comparison,setComparison] = useState<ReportComparison|null>(null);
  const [analysis,setAnalysis] = useState<LogAnalysis|null>(null);
  const [recipe,setRecipe] = useState<Recipe|null>(null);
  const [review,setReview] = useState<ReviewedReport|null>(null);

  useEffect(()=>{
    if(native) desktop.status().then(setStatus).catch((error:unknown)=>setFailure(String(error)));
  },[native]);

  function navigate(destination:View) {
    setView(destination); setFailure(""); setSuccess("");
  }
  async function perform<T>(job:()=>Promise<T>, receive:(value:T)=>void) {
    setBusy(true); setFailure(""); setSuccess("");
    try { receive(await job()); }
    catch(error) { setFailure(error instanceof Error ? error.message : String(error)); }
    finally { setBusy(false); }
  }
  function toggle(key:keyof ReviewOptions,value:boolean) {
    setOptions(previous=>({...previous,[key]:value})); setReview(null);
  }
  const disabled = busy || !native;
  const current = sections.find(item=>item.id===view) || sections[0];

  return <div className="shell">
    <aside className="sidebar">
      <div className="brand"><span className="brand-icon">R</span><div><strong>ReproCrate</strong><small>DIAGNOSTIC WORKSPACE</small></div></div>
      <p className="aside-label">TOOLS / LOCAL ONLY</p>
      <nav aria-label="Diagnostic tools">
        {sections.map((section,index)=><button type="button" key={section.id} aria-current={view===section.id?"page":undefined}
          className={"nav-link "+(view===section.id?"active":"")} onClick={()=>navigate(section.id)}>
          <span className="nav-index">{String(index+1).padStart(2,"0")}</span><span className="nav-title">{section.title}<small>{section.summary}</small></span><span className="nav-arrow">↗</span>
        </button>)}
      </nav>
      <div className="aside-bottom"><span className="connected-dot"/><div><strong>Local-first by design</strong><small>No automatic upload or telemetry</small></div></div>
    </aside>
    <div className="page">
      <header className="topbar"><div className="breadcrumbs">ReproCrate <span>/</span> <strong>{current.title}</strong></div>
        <div className="top-right"><span className="connection">● {native?"Desktop connected":"Browser preview"}</span><span>v{status?.version??"0.5.0"} · PRE-RELEASE</span></div>
      </header>
      <main className="content">
        {failure&&<div className="notice error" role="alert"><strong>Action could not be completed</strong><p>{failure}</p></div>}
        {success&&<div className="notice success" role="status">{success}</div>}
        {!native&&<div className="notice" role="note"><strong>Browser preview only.</strong> Launch the Tauri application to use files on your computer.</div>}

        {view==="overview"&&<>
          <section className="hero">
            <div className="hero-text"><p className="eyebrow">◈ YOUR PRIVATE DEBUGGING DESK</p>
              <h1>Diagnose locally.<br/><em>Understand precisely.</em></h1>
              <p>Inspect captured evidence, compare outcomes, investigate failure signals, and prepare a reviewed report without leaving your machine.</p>
              <button type="button" className="primary" onClick={()=>navigate("inspect")}>Open a report →</button>
            </div><div className="orbital" aria-hidden="true"><div className="ring"/><div className="inner-ring"/><span>RC</span></div>
          </section>
          <div className="section-heading"><h2>Choose a workflow</h2><span>Built on the same Rust core as the CLI</span></div>
          <div className="tool-grid">
            {sections.filter(s=>s.id!=="overview").map((section,index)=><button className="tool-card" type="button" key={section.id} onClick={()=>navigate(section.id)}>
              <small>0{index+2} / LOCAL</small><strong>{section.title}</strong><p>{section.summary}. Your data remains on the device.</p><span aria-hidden="true">↗</span>
            </button>)}
          </div>
          <div className="privacy-line"><span aria-hidden="true">◈</span><strong>Designed for trust</strong><p>Explicit file paths · no imported command execution · no background upload · manual review before sharing</p></div>
        </>}

        {view==="history"&&<HistoryPanel onOpen={(path)=>{setReportPath(path);setSummary(null);navigate("inspect");}}/>}

        {view==="inspect"&&<>
          <div className="page-intro"><span>02 / READ-ONLY</span><h1>Inspect a diagnostic report</h1><p>Load a bounded ReproCrate JSON report and examine recorded metadata. No command is executed.</p></div>
          <section className="form-panel"><FileField id="inspect-path" label="REPORT JSON" value={reportPath} change={setReportPath}/>
            <button className="primary" type="button" disabled={disabled||!reportPath.trim()} onClick={()=>perform(()=>desktop.inspect(reportPath),setSummary)}>{busy?"Reading…":"Inspect report →"}</button>
          </section>
          {summary?<section className="result"><div className="result-kicker"><span>REPORT LOADED</span><strong>{filenameDisplay(reportPath)}</strong></div>
            <h2>{summary.title}</h2><div className="stats">
              <div><small>Operating system</small><strong>{summary.os}</strong></div>
              <div><small>Architecture</small><strong>{summary.architecture}</strong></div>
              <div><small>Exit code</small><strong>{summary.exit_code??"Unobserved"}</strong></div>
              <div><small>Duration</small><strong>{durationLabel(summary.duration_ms)}</strong></div>
            </div>
            <p className="annotation">Schema {summary.schema_version} · v{summary.report_version} · {summary.warning_count} warning(s) · {summary.has_analysis?"Failure findings included":"No failure analysis"}</p>
            {(summary.timed_out||summary.stdout_truncated||summary.stderr_truncated)&&<p className="caution">Some execution evidence is timed out or truncated.</p>}
          </section>:<Empty message="Report details will appear here once you open a file."/>}
        </>}

        {view==="compare"&&<>
          <div className="page-intro"><span>03 / STRUCTURAL DIFF</span><h1>Compare two observations</h1><p>Find structural differences without exporting raw command arguments or captured log lines.</p></div>
          <section className="form-panel"><div className="two-fields"><FileField id="baseline" label="BASELINE REPORT" value={reportPath} change={setReportPath}/>
            <FileField id="candidate" label="CANDIDATE REPORT" value={candidatePath} change={setCandidatePath}/></div>
            <button className="primary" type="button" disabled={disabled||!reportPath.trim()||!candidatePath.trim()} onClick={()=>perform(()=>desktop.compare(reportPath,candidatePath),setComparison)}>{busy?"Comparing…":"Compare reports →"}</button>
          </section>
          {comparison?<section className="result"><div className="result-kicker"><span>COMPARISON</span><strong>{countComparisonChanges(comparison)} observed differences</strong></div>
            <div className="table-wrap"><table><thead><tr><th>Field</th><th>Baseline</th><th>Candidate</th><th>Relationship</th></tr></thead>
              <tbody>{comparison.fields.map(field=><tr key={field.field}><td>{field.field}</td><td>{field.baseline_state}</td><td>{field.candidate_state}</td><td><span className={"status-tag "+field.relationship}>{field.relationship}</span></td></tr>)}</tbody>
            </table></div><p className="annotation">{comparison.fingerprint_comparison_complete?String(comparison.fingerprint_changes.length)+" heuristic fingerprint groups compared.":"Fingerprint comparison omitted because one or both analyses are incomplete."} Similarity does not prove causality.</p>
          </section>:<Empty message="Missing, redacted, and truncated information will be marked inconclusive."/>}
        </>}

        {view==="logs"&&<>
          <div className="page-intro"><span>04 / LOCAL LOGS</span><h1>Analyze failure signals</h1><p>Examine a bounded 1 MiB preview of your selected log. Imported contents never run.</p></div>
          <section className="form-panel"><FileField id="log-path" label="LOG FILE" value={logPath} change={setLogPath}/>
            <button className="primary" type="button" disabled={disabled||!logPath.trim()} onClick={()=>perform(()=>desktop.analyze(logPath),setAnalysis)}>{busy?"Analyzing…":"Analyze log →"}</button>
          </section>
          {analysis?<section className="result"><div className="result-kicker"><span>HEURISTIC LOG ANALYSIS</span><strong>{analysis.detected_format} · {analysis.scanned_lines} lines</strong></div>
            <div className="findings">{analysis.findings.length?analysis.findings.map((finding,i)=><article className="finding" key={finding.fingerprint+":"+i}>
              <span className="severity">{finding.severity}</span><div><strong>{finding.category} · line {finding.line}</strong><p>{finding.message}</p><small>{finding.occurrences} occurrence(s) · non-cryptographic grouping</small></div>
            </article>):<p>No recognizable failure signals in the scanned portion.</p>}</div>
            {(analysis.truncated_input||analysis.truncated_findings)&&<p className="caution">Analysis is incomplete because a safety cap was reached.</p>}
            <p className="annotation">Review all findings before sharing: automatic masking may miss secrets.</p>
          </section>:<Empty message="Failure observations will appear here without executing the log."/>}
        </>}

        {view==="recipe"&&<>
          <div className="page-intro"><span>05 / HUMAN REVIEWED</span><h1>Prepare a manual recipe</h1><p>Generate a non-executable checklist. Command and argument details are omitted by default.</p></div>
          <section className="form-panel"><FileField id="recipe-path" label="REPORT JSON" value={reportPath} change={setReportPath}/>
            <button className="primary" type="button" disabled={disabled||!reportPath.trim()} onClick={()=>perform(()=>desktop.recipe(reportPath),setRecipe)}>{busy?"Preparing…":"Prepare recipe →"}</button>
          </section>
          {recipe?<section className="result"><div className="result-kicker"><span>MANUAL CHECKLIST</span><strong>{recipe.steps.length} steps</strong></div>
            <div className="steps">{recipe.steps.map(step=><div className="step" key={step.order}><span>{String(step.order).padStart(2,"0")}</span><div><strong>{step.kind.replaceAll("_"," ")}</strong><p>{step.instruction}</p></div></div>)}</div>
            <p className="annotation">Command omitted: {recipe.command===null?"yes":"no"} · Automatic execution: {recipe.automatic_execution?"yes":"no"}. This is not a verified reproduction.</p>
          </section>:<Empty message="A local, non-executable diagnostic guide will appear here."/>}
        </>}

        {view==="privacy"&&<>
          <div className="page-intro"><span>06 / EXPLICIT APPROVAL</span><h1>Review before sharing</h1><p>Exclude unwanted fields and mask literal strings in a new, reviewed report. The original stays untouched.</p></div>
          <section className="form-panel"><FileField id="private-report" label="SOURCE REPORT" value={reportPath} change={setReportPath}/>
            <div className="check-grid">{([
              ["excludeStdout","Exclude stdout"],["excludeStderr","Exclude stderr"],
              ["excludeArguments","Exclude arguments"],["excludeExecutable","Exclude executable"],
              ["excludeAnalysis","Exclude findings"],
            ] as const).map(([key,label])=><label key={key}><input type="checkbox" checked={options[key]}
              onChange={(e)=>toggle(key,e.target.checked)}/>{label}</label>)}</div>
            <label className="file-field" htmlFor="manual-masks"><span>ADDITIONAL LITERAL MASKS / ONE PER LINE</span>
              <textarea rows={3} id="manual-masks" placeholder="Optional confidential strings to remove" spellCheck={false}
                value={options.literals} onChange={(event)=>{setOptions(previous=>({...previous,literals:event.target.value}));setReview(null);}}/></label>
            <button className="primary" type="button" disabled={disabled||!reportPath.trim()} onClick={()=>perform(()=>desktop.previewReview(reportPath,options),setReview)}>{busy?"Preparing…":"Preview reviewed copy →"}</button>
          </section>
          {review?<section className="result"><div className="result-kicker"><span>LOCAL CANDIDATE</span><strong>{review.title}</strong></div>
            <p className="annotation">This preview can still contain private details. Examine each field before saving or sharing.</p>
            <div className="preview-block"><span>COMMAND</span><pre>{[review.capture.executable,...review.capture.arguments].join(" ")}</pre>
              <span>STDOUT · FIRST 4,000 CHARACTERS</span><pre>{review.capture.stdout.slice(0,4000)||"No output"}</pre>
              <span>STDERR · FIRST 4,000 CHARACTERS</span><pre>{review.capture.stderr.slice(0,4000)||"No output"}</pre></div>
            <FileField id="export-path" label="NEW EXPORT PATH · NO OVERWRITE" value={exportPath} change={setExportPath}/>
            <button className="secondary" type="button" disabled={disabled||!exportPath.trim()}
              onClick={()=>perform(()=>desktop.saveReview(reportPath,exportPath,options),setSuccess)}>{busy?"Saving…":"Save reviewed file locally →"}</button>
          </section>:<Empty message="A filtered report preview appears here before any export."/>}
        </>}
      </main>
      <footer className="footer"><span>ReproCrate · EPL-2.0 · Desktop foundation</span><span>LOCAL FIRST / NO AUTOMATIC UPLOAD</span></footer>
    </div>
  </div>;
}
