import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import SourceView from "./components/SourceView";
import TimelineView from "./components/TimelineView";
import GraphView from "./components/GraphView";
import StepController from "./components/StepController";
import QueryPanel from "./components/QueryPanel";
import { ProgramAnalysis } from "./types";
import { Play, FileCode, Info } from "lucide-react";

function App() {
  const [analysis, setAnalysis] = useState<ProgramAnalysis | null>(null);
  const [currentStep, setCurrentStep] = useState(0);
  const [code, setCode] = useState(EXAMPLE_CODE);
  const [mode, setMode] = useState<"teaching" | "debug">("teaching");
  const [activeView, setActiveView] = useState<"timeline" | "graph">("timeline");
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const handleAnalyze = async () => {
    setLoading(true);
    setError(null);
    try {
      const result = await invoke<ProgramAnalysis>("analyze_snippet", {
        request: {
          code,
          filename: "snippet.rs",
          mode,
        },
      });
      setAnalysis(result);
      setCurrentStep(0);
    } catch (err) {
      setError(err as string);
    } finally {
      setLoading(false);
    }
  };

  const maxSteps = analysis?.events.length || 0;

  return (
    <div className="h-screen flex flex-col bg-gray-900 text-gray-100">
      {/* Header */}
      <header className="bg-gray-800 border-b border-gray-700 px-6 py-4">
        <div className="flex items-center justify-between">
          <div className="flex items-center gap-3">
            <FileCode className="w-8 h-8 text-blue-500" />
            <h1 className="text-2xl font-bold">Ownsight</h1>
            <span className="text-sm text-gray-400">Rust Ownership Visualizer</span>
          </div>
          <div className="flex items-center gap-4">
            <select
              value={mode}
              onChange={(e) => setMode(e.target.value as "teaching" | "debug")}
              className="bg-gray-700 border border-gray-600 rounded px-3 py-2 text-sm"
            >
              <option value="teaching">Teaching Mode</option>
              <option value="debug">Debug Mode</option>
            </select>
            <button
              onClick={handleAnalyze}
              disabled={loading}
              className="bg-blue-600 hover:bg-blue-700 disabled:bg-gray-600 px-4 py-2 rounded flex items-center gap-2 transition-colors"
            >
              <Play className="w-4 h-4" />
              {loading ? "Analyzing..." : "Analyze"}
            </button>
          </div>
        </div>
      </header>

      {/* Main Content */}
      <div className="flex-1 flex overflow-hidden">
        {/* Left Panel - Code Editor */}
        <div className="w-1/2 border-r border-gray-700 flex flex-col">
          <div className="bg-gray-800 px-4 py-2 border-b border-gray-700">
            <h2 className="font-semibold">Source Code</h2>
          </div>
          <SourceView
            code={code}
            onChange={setCode}
            analysis={analysis}
            currentStep={currentStep}
          />
        </div>

        {/* Right Panel - Visualization */}
        <div className="w-1/2 flex flex-col">
          {/* View Tabs */}
          <div className="bg-gray-800 border-b border-gray-700 flex">
            <button
              onClick={() => setActiveView("timeline")}
              className={`px-6 py-3 font-medium transition-colors ${
                activeView === "timeline"
                  ? "bg-gray-700 text-blue-400 border-b-2 border-blue-400"
                  : "text-gray-400 hover:text-gray-200"
              }`}
            >
              Timeline View
            </button>
            <button
              onClick={() => setActiveView("graph")}
              className={`px-6 py-3 font-medium transition-colors ${
                activeView === "graph"
                  ? "bg-gray-700 text-blue-400 border-b-2 border-blue-400"
                  : "text-gray-400 hover:text-gray-200"
              }`}
            >
              Graph View
            </button>
          </div>

          {/* View Content */}
          <div className="flex-1 overflow-auto">
            {error && (
              <div className="m-4 p-4 bg-red-900/50 border border-red-700 rounded">
                <div className="flex items-center gap-2 text-red-400">
                  <Info className="w-5 h-5" />
                  <span className="font-semibold">Error</span>
                </div>
                <p className="mt-2 text-sm">{error}</p>
              </div>
            )}

            {!analysis && !error && (
              <div className="flex items-center justify-center h-full text-gray-500">
                <div className="text-center">
                  <FileCode className="w-16 h-16 mx-auto mb-4 opacity-50" />
                  <p>Enter Rust code and click "Analyze" to visualize ownership</p>
                </div>
              </div>
            )}

            {analysis && activeView === "timeline" && (
              <TimelineView analysis={analysis} currentStep={currentStep} />
            )}

            {analysis && activeView === "graph" && (
              <GraphView analysis={analysis} currentStep={currentStep} />
            )}
          </div>

          {/* Step Controller */}
          {analysis && (
            <div className="border-t border-gray-700">
              <StepController
                currentStep={currentStep}
                maxSteps={maxSteps}
                onStepChange={setCurrentStep}
              />
            </div>
          )}
        </div>
      </div>

      {/* Bottom Panel - Query Interface */}
      {analysis && (
        <div className="border-t border-gray-700 bg-gray-800">
          <QueryPanel analysis={analysis} currentStep={currentStep} />
        </div>
      )}
    </div>
  );
}

const EXAMPLE_CODE = `fn main() {
    let s = String::from("hello");
    let r1 = &s;
    println!("{}", r1);
    let r2 = &mut s;
    println!("{}", r2);
}`;

export default App;
