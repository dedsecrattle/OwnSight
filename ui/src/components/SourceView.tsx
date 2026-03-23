import { useEffect, useRef } from "react";
import Editor from "@monaco-editor/react";
import { ProgramAnalysis } from "../types";

interface SourceViewProps {
  code: string;
  onChange: (code: string) => void;
  analysis: ProgramAnalysis | null;
  currentStep: number;
}

export default function SourceView({
  code,
  onChange,
  analysis,
  currentStep,
}: SourceViewProps) {
  const editorRef = useRef<any>(null);

  useEffect(() => {
    if (!editorRef.current || !analysis) return;

    const editor = editorRef.current;
    const model = editor.getModel();
    if (!model) return;

    const decorations: any[] = [];
    const currentEvent = analysis.events[currentStep];

    if (currentEvent) {
      decorations.push({
        range: {
          startLineNumber: currentEvent.line_number,
          startColumn: 1,
          endLineNumber: currentEvent.line_number,
          endColumn: model.getLineMaxColumn(currentEvent.line_number),
        },
        options: {
          isWholeLine: true,
          className: "current-line-highlight",
          glyphMarginClassName: "current-line-glyph",
        },
      });

      const eventColor = getEventColor(currentEvent.kind);
      decorations.push({
        range: {
          startLineNumber: currentEvent.line_number,
          startColumn: 1,
          endLineNumber: currentEvent.line_number,
          endColumn: model.getLineMaxColumn(currentEvent.line_number),
        },
        options: {
          isWholeLine: true,
          className: `event-highlight-${eventColor}`,
        },
      });
    }

    editor.deltaDecorations([], decorations);
  }, [analysis, currentStep]);

  const handleEditorDidMount = (editor: any) => {
    editorRef.current = editor;
  };

  return (
    <div className="flex-1 relative">
      <Editor
        height="100%"
        defaultLanguage="rust"
        value={code}
        onChange={(value) => onChange(value || "")}
        onMount={handleEditorDidMount}
        theme="vs-dark"
        options={{
          minimap: { enabled: false },
          fontSize: 14,
          lineNumbers: "on",
          scrollBeyondLastLine: false,
          automaticLayout: true,
        }}
      />
      <style>{`
        .current-line-highlight {
          background-color: rgba(59, 130, 246, 0.1);
          border-left: 3px solid #3b82f6;
        }
        .current-line-glyph {
          background-color: #3b82f6;
          width: 5px !important;
          margin-left: 3px;
        }
        .event-highlight-green {
          background-color: rgba(34, 197, 94, 0.15);
        }
        .event-highlight-red {
          background-color: rgba(239, 68, 68, 0.15);
        }
        .event-highlight-blue {
          background-color: rgba(59, 130, 246, 0.15);
        }
        .event-highlight-yellow {
          background-color: rgba(234, 179, 8, 0.15);
        }
        .event-highlight-purple {
          background-color: rgba(168, 85, 247, 0.15);
        }
      `}</style>
    </div>
  );
}

function getEventColor(kind: string): string {
  switch (kind) {
    case "Create":
    case "StorageLive":
      return "green";
    case "MoveOut":
    case "Conflict":
      return "red";
    case "MoveIn":
      return "blue";
    case "BorrowShared":
      return "blue";
    case "BorrowMut":
      return "yellow";
    case "Drop":
    case "StorageDead":
      return "purple";
    default:
      return "blue";
  }
}
