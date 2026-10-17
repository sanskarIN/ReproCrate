import { describe, expect, it } from "vitest";
import { countComparisonChanges, durationLabel, filenameDisplay } from "./format";
import type { ReportComparison } from "./types";

describe("safe desktop presentations", () => {
  it("counts changed, added and removed while ignoring unknown and same", () => {
    const value = { fields: [
      { field: "a", relationship: "same" }, {field: "b", relationship: "changed"},
      { field: "c", relationship: "indeterminate" }, {field: "d", relationship: "added"},
      { field: "e", relationship: "removed" },
    ] } as ReportComparison;
    expect(countComparisonChanges(value)).toBe(3);
  });
  it("only displays a leaf name instead of a private path", () => {
    expect(filenameDisplay("C:\\private\\logs\\report.json")).toBe("report.json");
    expect(filenameDisplay("/home/user/project/log.txt")).toBe("log.txt");
  });
  it("does not misrepresent missing duration values", () => {
    expect(durationLabel(-2)).toBe("Not recorded");
    expect(durationLabel(1500)).toBe("1.50 s");
  });
});
