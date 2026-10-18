export type View = "overview" | "history" | "inspect" | "compare" | "logs" | "recipe" | "privacy" | "issue";
export type Relationship = "same" | "changed" | "added" | "removed" | "indeterminate";
export interface DesktopStatus { name: string; version: string; operating_system: string; architecture: string; mode: string }
export interface ReportSummary {
  report_id: string; title: string; report_version: string; schema_version: string;
  os: string; architecture: string; exit_code: number | null; duration_ms: number;
  timed_out: boolean; has_analysis: boolean; warning_count: number; stdout_truncated: boolean;
  stderr_truncated: boolean;
}
export interface FieldDifference { field: string; baseline_state: string; candidate_state: string; relationship: Relationship }
export interface FingerprintDifference { fingerprint: string; baseline_count: number; candidate_count: number; relationship: Relationship }
export interface ReportComparison { schema_version: string; baseline_report_id: string; candidate_report_id: string; fields: FieldDifference[]; fingerprint_changes: FingerprintDifference[]; fingerprint_comparison_complete: boolean; warnings: string[] }
export interface Finding { line: number; message: string; fingerprint: string; occurrences: number; severity: string; category: string }
export interface LogAnalysis { detected_format: string; scanned_lines: number; truncated_input: boolean; truncated_findings: boolean; findings: Finding[]; warnings: string[] }
export interface Recipe { source_report_id: string; schema_version: string; command: {executable:string; arguments:string[]} | null; steps: {order:number; kind:string; instruction:string; requires_user_action:boolean}[]; warnings:string[]; automatic_execution: boolean }
export interface ReviewOptions { excludeStdout:boolean; excludeStderr:boolean; excludeArguments:boolean; excludeExecutable:boolean; excludeAnalysis:boolean; literals:string }
export interface ReviewedReport { title:string; report_id:string; capture:{executable:string; arguments:string[]; stdout:string; stderr:string}; warnings:string[] }

export interface HistoryEntry {
  id: number;
  path: string;
  label: string;
  report_id: string;
  os: string;
  architecture: string;
  indexed_unix_seconds: number;
}

export type IssueTarget = "github" | "gitlab" | "gitea" | "generic";
export interface IssueOptions {
  includeTitle: boolean;
  includeEnvironment: boolean;
  includeCategories: boolean;
  acknowledgeReviewed: boolean;
}
export interface IssueDraft {
  target: IssueTarget;
  suggested_title: string;
  markdown: string;
  warnings: string[];
}
