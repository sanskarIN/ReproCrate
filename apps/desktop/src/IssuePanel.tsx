import { useState } from "react";
import { desktop, desktopAvailable } from "./bridge";
import type { IssueDraft, IssueOptions, IssueTarget } from "./types";

const defaults: IssueOptions = {
  includeTitle: false,
  includeEnvironment: false,
  includeCategories: false,
  acknowledgeReviewed: false,
};

export default function IssuePanel() {
  const native = desktopAvailable();
  const [source, setSource] = useState("");
  const [output, setOutput] = useState("");
  const [target, setTarget] = useState<IssueTarget>("github");
  const [options, setOptions] = useState<IssueOptions>(defaults);
  const [draft, setDraft] = useState<IssueDraft | null>(null);
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState("");
  const [success, setSuccess] = useState("");

  function updateOption(key: keyof IssueOptions, value: boolean) {
    setOptions(old => ({ ...old, [key]: value }));
    setDraft(null);
    setSuccess("");
  }
  const needsApproval = options.includeTitle || options.includeEnvironment || options.includeCategories;
  const canPreview = native && !busy && source.trim() && (!needsApproval || options.acknowledgeReviewed);

  async function preview() {
    setBusy(true); setError(""); setSuccess(""); setDraft(null);
    try {
      setDraft(await desktop.issuePreview(source.trim(), target, options));
    } catch (cause) {
      setError(String(cause));
    } finally { setBusy(false); }
  }

  async function save() {
    if (!draft || !output.trim()) return;
    setBusy(true); setError(""); setSuccess("");
    try {
      setSuccess(await desktop.issueSave(source.trim(), output.trim(), target, options));
    } catch (cause) {
      setError(String(cause));
    } finally { setBusy(false); }
  }

  return <>
    <div className="page-intro">
      <span>07 / OFFLINE ISSUE ADAPTERS</span>
      <h1>Prepare a reviewed issue draft</h1>
      <p>Build a local Markdown issue for GitHub, GitLab, Gitea or a generic tracker. No accounts, tokens, automatic posts or uploads.</p>
    </div>
    <section className="form-panel">
      <label className="file-field" htmlFor="issue-input">
        <span>REPORT JSON FILE</span>
        <input type="text" id="issue-input" autoComplete="off" spellCheck={false} value={source}
          placeholder="Absolute path to a saved report"
          onChange={event => { setSource(event.target.value); setDraft(null); }} />
      </label>
      <label className="file-field" htmlFor="issue-target">
        <span>ISSUE TRACKER / MARKDOWN FORMAT</span>
        <select id="issue-target" value={target} onChange={event => {setTarget(event.target.value as IssueTarget);setDraft(null);}}>
          <option value="github">GitHub</option>
          <option value="gitlab">GitLab</option>
          <option value="gitea">Gitea</option>
          <option value="generic">Generic Markdown</option>
        </select>
      </label>
      <div className="check-grid" aria-label="Optional report details">
        <label><input type="checkbox" checked={options.includeTitle} onChange={e => updateOption("includeTitle", e.target.checked)}/>Include report title</label>
        <label><input type="checkbox" checked={options.includeEnvironment} onChange={e => updateOption("includeEnvironment", e.target.checked)}/>Include OS and architecture</label>
        <label><input type="checkbox" checked={options.includeCategories} onChange={e => updateOption("includeCategories", e.target.checked)}/>Include heuristic categories</label>
      </div>
      <label>
        <input type="checkbox" checked={options.acknowledgeReviewed}
          onChange={e => updateOption("acknowledgeReviewed", e.target.checked)}/>
        I reviewed the selected report and approve optional metadata for this draft
      </label>
      <p className="annotation">Default exports omit title, raw log lines, commands, arguments, paths and finding messages. No output is shared automatically. Optional metadata can still be sensitive.</p>
      <button type="button" className="primary" disabled={!canPreview} onClick={() => void preview()}>
        {busy ? "Preparing…" : "Preview complete draft →"}
      </button>
    </section>
    {error && <div role="alert" className="notice error">{error}</div>}
    {success && <div role="status" className="notice success">{success}</div>}
    {draft && <section className="result">
      <div className="result-kicker"><span>MANUAL REVIEW REQUIRED</span><strong>{draft.markdown.length} characters</strong></div>
      <h2>{draft.suggested_title}</h2>
      <p className="annotation">Review the entire Markdown below and remove any private details. The app never posts it for you.</p>
      <div className="preview-block"><pre>{draft.markdown}</pre></div>
      <label className="file-field" htmlFor="issue-output">
        <span>NEW LOCAL MARKDOWN PATH · NO OVERWRITE</span>
        <input type="text" id="issue-output" autoComplete="off" spellCheck={false}
          value={output} placeholder="Absolute path to a new .md file"
          onChange={event => setOutput(event.target.value)} />
      </label>
      <button type="button" className="secondary" disabled={busy || !native || !output.trim()}
        onClick={() => void save()}>Save reviewed draft locally →</button>
    </section>}
  </>;
}
