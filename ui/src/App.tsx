import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import SourceView from "./components/SourceView";
import TimelineView from "./components/TimelineView";
import GraphView from "./components/GraphView";
import StepController from "./components/StepController";
import QueryPanel from "./components/QueryPanel";
import { ProgramAnalysis } from "./types";
import { Play, FileCode, Info, ExternalLink } from "lucide-react";

interface BackendAvailability {
  simple: boolean;
  mir: boolean;
}

function App() {
  const [analysis, setAnalysis] = useState<ProgramAnalysis | null>(null);
  const [currentStep, setCurrentStep] = useState(0);
  const [code, setCode] = useState(EXAMPLE_CODE);
  const [mode, setMode] = useState<"teaching" | "debug">("teaching");
  const [backend, setBackend] = useState<"simple" | "mir">("simple");
  const [activeView, setActiveView] = useState<"timeline" | "graph">(
    "timeline",
  );
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [backendAvailability, setBackendAvailability] =
    useState<BackendAvailability>({
      simple: true,
      mir: false,
    });
  const [showMirGuide, setShowMirGuide] = useState(false);

  useEffect(() => {
    // Check backend availability on mount
    invoke<BackendAvailability>("check_backend_availability")
      .then(setBackendAvailability)
      .catch(console.error);
  }, []);

  const handleAnalyze = async () => {
    setLoading(true);
    setError(null);
    try {
      const result = await invoke<ProgramAnalysis>("analyze_snippet", {
        request: {
          code,
          filename: "snippet.rs",
          mode,
          backend,
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
            <span className="text-sm text-gray-400">
              Rust Ownership Visualizer
            </span>
            {backend === "mir" && (
              <span className="ml-2 px-2 py-1 text-xs bg-purple-600 text-white rounded-full">
                Layer 2
              </span>
            )}
          </div>
          <div className="flex items-center gap-4">
            <div className="flex items-center gap-2">
              <select
                value={backend}
                onChange={(e) => setBackend(e.target.value as "simple" | "mir")}
                className="bg-gray-700 border border-gray-600 rounded px-3 py-2 text-sm"
                title="Analysis backend: Simple (fast, syntax-based) or MIR (accurate, compiler-based)"
              >
                <option value="simple">Simple Backend</option>
                <option value="mir" disabled={!backendAvailability.mir}>
                  MIR Backend (Layer 2){" "}
                  {!backendAvailability.mir && "- Not Available"}
                </option>
              </select>
              {!backendAvailability.mir && (
                <button
                  onClick={() => setShowMirGuide(true)}
                  className="text-blue-400 hover:text-blue-300 transition-colors"
                  title="Learn how to enable MIR backend"
                >
                  <Info className="w-5 h-5" />
                </button>
              )}
            </div>
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
                  <p>
                    Enter Rust code and click "Analyze" to visualize ownership
                  </p>
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

      {/* MIR Setup Guide Modal */}
      {showMirGuide && (
        <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
          <div className="bg-gray-800 rounded-lg shadow-xl max-w-2xl w-full mx-4 max-h-[80vh] overflow-y-auto">
            <div className="p-6">
              <div className="flex items-center justify-between mb-4">
                <h2 className="text-2xl font-bold text-white flex items-center gap-2">
                  <Info className="w-6 h-6 text-blue-400" />
                  Enable MIR Backend (Layer 2)
                </h2>
                <button
                  onClick={() => setShowMirGuide(false)}
                  className="text-gray-400 hover:text-white transition-colors"
                >
                  ✕
                </button>
              </div>

              <div className="space-y-4 text-gray-300">
                <p className="text-sm">
                  The MIR (Mid-level Intermediate Representation) backend
                  provides compiler-accurate ownership analysis with advanced
                  features like partial moves, closure captures, and async/await
                  support.
                </p>

                <div className="bg-gray-900 rounded-lg p-4 border border-gray-700">
                  <h3 className="font-semibold text-white mb-2">
                    ✨ Layer 2 Features
                  </h3>
                  <ul className="text-sm space-y-1 list-disc list-inside">
                    <li>Partial move detection (struct fields)</li>
                    <li>Closure capture analysis (ByValue, ByRef, ByMutRef)</li>
                    <li>Async/await suspension points</li>
                    <li>Non-lexical lifetimes (NLL) foundation</li>
                    <li>Function ownership summaries</li>
                  </ul>
                </div>

                <div className="bg-blue-900/20 rounded-lg p-4 border border-blue-700">
                  <h3 className="font-semibold text-white mb-3 flex items-center gap-2">
                    <ExternalLink className="w-5 h-5" />
                    How to Enable MIR Backend
                  </h3>

                  <div className="space-y-3 text-sm">
                    <div>
                      <p className="font-medium text-blue-300 mb-1">
                        1. Install Rust Nightly
                      </p>
                      <code className="block bg-gray-900 p-2 rounded text-xs">
                        rustup toolchain install nightly
                      </code>
                    </div>

                    <div>
                      <p className="font-medium text-blue-300 mb-1">
                        2. Install rustc-dev Component
                      </p>
                      <code className="block bg-gray-900 p-2 rounded text-xs">
                        rustup component add rustc-dev llvm-tools-preview
                        --toolchain nightly
                      </code>
                    </div>

                    <div>
                      <p className="font-medium text-blue-300 mb-1">
                        3. Clone and Build from Source
                      </p>
                      <code className="block bg-gray-900 p-2 rounded text-xs whitespace-pre">
                        {`git clone https://github.com/dedsecrattle/ownsight
cd ownsight/ui
rustup run nightly cargo build --release --features mir`}
                      </code>
                    </div>

                    <div>
                      <p className="font-medium text-blue-300 mb-1">
                        4. Run the App
                      </p>
                      <code className="block bg-gray-900 p-2 rounded text-xs">
                        bun run tauri dev
                      </code>
                    </div>
                  </div>
                </div>

                <div className="bg-yellow-900/20 rounded-lg p-4 border border-yellow-700">
                  <h3 className="font-semibold text-yellow-300 mb-2">
                    ⚠️ Note
                  </h3>
                  <p className="text-sm">
                    The MIR backend requires nightly Rust and rustc internals.
                    It's designed for advanced users and developers who need
                    compiler-accurate analysis. The Simple backend works great
                    for learning and most use cases!
                  </p>
                </div>

                <div className="flex gap-3 pt-4">
                  <a
                    href="https://github.com/dedsecrattle/ownsight/blob/main/LAYER2.md"
                    target="_blank"
                    rel="noopener noreferrer"
                    className="flex-1 bg-blue-600 hover:bg-blue-700 text-white px-4 py-2 rounded flex items-center justify-center gap-2 transition-colors"
                  >
                    <ExternalLink className="w-4 h-4" />
                    Read Full Documentation
                  </a>
                  <button
                    onClick={() => setShowMirGuide(false)}
                    className="px-6 py-2 bg-gray-700 hover:bg-gray-600 text-white rounded transition-colors"
                  >
                    Close
                  </button>
                </div>
              </div>
            </div>
          </div>
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
