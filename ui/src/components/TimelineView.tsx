import { ProgramAnalysis } from "../types";
import {
  Sparkles,
  Package,
  Eye,
  Edit3,
  Trash2,
  AlertTriangle,
  ArrowDownCircle,
  ArrowUpCircle,
} from "lucide-react";

interface TimelineViewProps {
  analysis: ProgramAnalysis;
  currentStep: number;
}

export default function TimelineView({
  analysis,
  currentStep,
}: TimelineViewProps) {
  const getVariable = (varId: { 0: number }) => {
    return analysis.variables.find((v) => v.id[0] === varId[0]);
  };

  const getEventIcon = (kind: string) => {
    switch (kind) {
      case "Create":
      case "StorageLive":
        return <Sparkles className="w-5 h-5 text-green-400" />;
      case "MoveOut":
        return <Package className="w-5 h-5 text-red-400" />;
      case "MoveIn":
        return <ArrowDownCircle className="w-5 h-5 text-blue-400" />;
      case "BorrowShared":
        return <Eye className="w-5 h-5 text-cyan-400" />;
      case "BorrowMut":
        return <Edit3 className="w-5 h-5 text-yellow-400" />;
      case "Drop":
      case "StorageDead":
        return <Trash2 className="w-5 h-5 text-purple-400" />;
      case "Conflict":
        return <AlertTriangle className="w-5 h-5 text-red-400" />;
      default:
        return <ArrowUpCircle className="w-5 h-5 text-gray-400" />;
    }
  };

  const getEventColor = (kind: string) => {
    switch (kind) {
      case "Create":
      case "StorageLive":
        return "border-green-500 bg-green-900/20";
      case "MoveOut":
      case "Conflict":
        return "border-red-500 bg-red-900/20";
      case "MoveIn":
        return "border-blue-500 bg-blue-900/20";
      case "BorrowShared":
        return "border-cyan-500 bg-cyan-900/20";
      case "BorrowMut":
        return "border-yellow-500 bg-yellow-900/20";
      case "Drop":
      case "StorageDead":
        return "border-purple-500 bg-purple-900/20";
      default:
        return "border-gray-500 bg-gray-900/20";
    }
  };

  return (
    <div className="p-6 space-y-4">
      <div className="mb-6">
        <h3 className="text-lg font-semibold mb-2">Ownership Timeline</h3>
        <p className="text-sm text-gray-400">
          Step {currentStep + 1} of {analysis.events.length}
        </p>
      </div>

      <div className="space-y-3">
        {analysis.events.map((event, index) => {
          const variable = getVariable(event.variable_id);
          const isActive = index === currentStep;
          const isPast = index < currentStep;

          return (
            <div
              key={index}
              className={`border-l-4 p-4 rounded-r transition-all ${
                isActive
                  ? `${getEventColor(event.kind)} scale-105 shadow-lg`
                  : isPast
                    ? "border-gray-700 bg-gray-800/50 opacity-60"
                    : "border-gray-700 bg-gray-800/30 opacity-40"
              }`}
            >
              <div className="flex items-start gap-3">
                <div className="mt-1">{getEventIcon(event.kind)}</div>
                <div className="flex-1">
                  <div className="flex items-center gap-2 mb-1">
                    <span className="text-xs text-gray-500">
                      Line {event.line_number}
                    </span>
                    <span className="text-xs font-mono bg-gray-700 px-2 py-0.5 rounded">
                      {event.kind}
                    </span>
                  </div>
                  <div className="font-medium mb-1">
                    <code className="text-blue-300">
                      {variable?.name || "?"}
                    </code>
                  </div>
                  <p className="text-sm text-gray-300">{event.explanation}</p>
                  {event.related_variable_id && (
                    <div className="mt-2 text-xs text-gray-400">
                      Related to:{" "}
                      <code className="text-cyan-300">
                        {getVariable(event.related_variable_id)?.name || "?"}
                      </code>
                    </div>
                  )}
                </div>
              </div>
            </div>
          );
        })}
      </div>
    </div>
  );
}
