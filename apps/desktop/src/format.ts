import type { FieldDifference, ReportComparison } from "./types";

const safeLabel = (value: string) => value.replace(/[<>]/g, "");
export function countComparisonChanges(value: ReportComparison): number {
  return value.fields.filter((field: FieldDifference) => field.relationship === "changed" || field.relationship === "added" || field.relationship === "removed").length;
}
export function filenameDisplay(path: string): string {
  const parts = path.split(/[\\/]/).filter(Boolean);
  return safeLabel(parts.at(-1) || "No file selected");
}
export function durationLabel(ms: number): string {
  if (!Number.isFinite(ms) || ms < 0) return "Not recorded";
  return ms < 1000 ? `${ms} ms` : `${(ms / 1000).toFixed(2)} s`;
}
