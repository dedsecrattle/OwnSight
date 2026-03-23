import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { ProgramAnalysis } from "../types";
import { Search, HelpCircle } from "lucide-react";

interface QueryPanelProps {
  analysis: ProgramAnalysis;
  currentStep: number;
}

export default function QueryPanel({ analysis, currentStep }: QueryPanelProps) {
  const [selectedVariable, setSelectedVariable] = useState<number | null>(null);
  const [queryResult, setQueryResult] = useState<string | null>(null);
  const [queryType, setQueryType] = useState<"why" | "where" | "what">("why");

  const handleQuery = async () => {
    if (selectedVariable === null) return;

    const analysisJson = JSON.stringify(analysis);
    const currentLine = analysis.events[currentStep]?.line_number || 1;

    try {
      if (queryType === "why") {
        const result = await invoke<string | null>("query_why_cant_use", {
          analysisJson,
          varId: selectedVariable,
          line: currentLine,
        });
        setQueryResult(result || "Variable is valid at this line");
      } else if (queryType === "where") {
        const result = await invoke<number[]>("query_where_moved", {
          analysisJson,
          varId: selectedVariable,
        });
        setQueryResult(
          result.length > 0
            ? `Moved at lines: ${result.join(", ")}`
            : "Variable was never moved"
        );
      } else if (queryType === "what") {
        const result = await invoke<string[]>("query_what_borrows", {
          analysisJson,
          varId: selectedVariable,
          line: currentLine,
        });
        setQueryResult(
          result.length > 0
            ? result.join("; ")
            : "No active borrows at this line"
        );
      }
    } catch (error) {
      setQueryResult(`Error: ${error}`);
    }
  };

  return (
    <div className="p-4">
      <div className="flex items-center gap-4">
        <HelpCircle className="w-5 h-5 text-blue-400" />
        <h3 className="font-semibold">Query Interface</h3>
      </div>

      <div className="mt-4 flex items-center gap-4">
        <select
          value={queryType}
          onChange={(e) => setQueryType(e.target.value as any)}
          className="bg-gray-700 border border-gray-600 rounded px-3 py-2 text-sm"
        >
          <option value="why">Why can't I use this?</option>
          <option value="where">Where was this moved?</option>
          <option value="what">What is borrowing this?</option>
        </select>

        <select
          value={selectedVariable ?? ""}
          onChange={(e) => setSelectedVariable(parseInt(e.target.value))}
          className="bg-gray-700 border border-gray-600 rounded px-3 py-2 text-sm flex-1"
        >
          <option value="">Select a variable...</option>
          {analysis.variables.map((variable) => (
            <option key={variable.id[0]} value={variable.id[0]}>
              {variable.name} ({variable.ty})
            </option>
          ))}
        </select>

        <button
          onClick={handleQuery}
          disabled={selectedVariable === null}
          className="bg-blue-600 hover:bg-blue-700 disabled:bg-gray-600 px-4 py-2 rounded flex items-center gap-2 transition-colors"
        >
          <Search className="w-4 h-4" />
          Query
        </button>
      </div>

      {queryResult && (
        <div className="mt-4 p-4 bg-gray-700 rounded border border-gray-600">
          <p className="text-sm">{queryResult}</p>
        </div>
      )}
    </div>
  );
}
