import { invoke } from "@tauri-apps/api/core";
import type { DesktopStatus, LogAnalysis, Recipe, ReportComparison, ReportSummary, ReviewOptions, ReviewedReport } from "./types";

export function desktopAvailable(): boolean {
  return typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;
}

async function call<T>(command: string, args: Record<string, unknown> = {}): Promise<T> {
  if (!desktopAvailable()) {
    throw new Error("Native desktop functions require the installed Tauri application. Browser preview is read-only.");
  }
  return invoke<T>(command, args);
}

export const desktop = {
  status: () => call<DesktopStatus>("desktop_status"),
  inspect: (path: string) => call<ReportSummary>("inspect_report", { path }),
  compare: (baselinePath: string, candidatePath: string) => call<ReportComparison>("compare_report_files", { baselinePath, candidatePath }),
  analyze: (path: string) => call<LogAnalysis>("analyze_log_file", { path }),
  recipe: (path: string) => call<Recipe>("preview_recipe", { path }),
  previewReview: (path: string, options: ReviewOptions) => call<ReviewedReport>("preview_review", { path, ...options }),
  saveReview: (path: string, outputPath: string, options: ReviewOptions) => call<string>("save_review", { path, outputPath, ...options }),
};
