import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { BackendAvailability } from "../types";

interface BackendStatusProps {
  currentBackend?: string;
  className?: string;
}

export function BackendStatus({
  currentBackend = "simple",
  className = "",
}: BackendStatusProps) {
  const [availability, setAvailability] = useState<BackendAvailability | null>(
    null,
  );

  useEffect(() => {
    checkBackendAvailability();
  }, []);

  const checkBackendAvailability = async () => {
    try {
      const result = await invoke<BackendAvailability>(
        "check_backend_availability",
      );
      setAvailability(result);
    } catch (error) {
      console.error("Failed to check backend availability:", error);
    }
  };

  if (!availability) {
    return null;
  }

  const isMirActive = currentBackend === "mir" && availability.mir;

  return (
    <div className={`flex items-center gap-2 ${className}`}>
      {/* Backend indicator badge */}
      <div className="flex items-center gap-1.5 px-3 py-1.5 rounded-lg bg-gray-100 dark:bg-gray-800">
        <div
          className={`w-2 h-2 rounded-full ${
            isMirActive ? "bg-green-500" : "bg-blue-500"
          }`}
        />
        <span className="text-sm font-medium text-gray-700 dark:text-gray-300">
          {isMirActive ? "MIR" : "Simple"}
        </span>
      </div>

      {/* Status tooltip */}
      <div className="group relative">
        <svg
          className="w-4 h-4 text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 cursor-help"
          fill="none"
          viewBox="0 0 24 24"
          stroke="currentColor"
        >
          <path
            strokeLinecap="round"
            strokeLinejoin="round"
            strokeWidth={2}
            d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
          />
        </svg>

        {/* Tooltip content */}
        <div className="absolute bottom-full left-1/2 -translate-x-1/2 mb-2 hidden group-hover:block z-50">
          <div className="bg-gray-900 dark:bg-gray-700 text-white text-xs rounded-lg py-2 px-3 whitespace-nowrap shadow-lg">
            <div className="font-semibold mb-1">Backend Status</div>
            <div className="space-y-1">
              <div className="flex items-center gap-2">
                <span
                  className={
                    availability.simple ? "text-green-400" : "text-red-400"
                  }
                >
                  {availability.simple ? "✓" : "✗"}
                </span>
                <span>Simple (syntax-based)</span>
              </div>
              <div className="flex items-center gap-2">
                <span
                  className={
                    availability.mir ? "text-green-400" : "text-yellow-400"
                  }
                >
                  {availability.mir ? "✓" : "⚠"}
                </span>
                <span>MIR (compiler-backed)</span>
              </div>
              {availability.mir_error && (
                <div className="mt-2 pt-2 border-t border-gray-600 text-yellow-300 text-xs">
                  {availability.mir_error}
                </div>
              )}
            </div>
            <div className="absolute top-full left-1/2 -translate-x-1/2 -mt-1">
              <div className="border-4 border-transparent border-t-gray-900 dark:border-t-gray-700" />
            </div>
          </div>
        </div>
      </div>

      {/* Warning if MIR requested but unavailable */}
      {currentBackend === "mir" && !availability.mir && (
        <div className="flex items-center gap-1.5 px-2 py-1 rounded bg-yellow-100 dark:bg-yellow-900/30">
          <svg
            className="w-4 h-4 text-yellow-600 dark:text-yellow-500"
            fill="none"
            viewBox="0 0 24 24"
            stroke="currentColor"
          >
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              strokeWidth={2}
              d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"
            />
          </svg>
          <span className="text-xs text-yellow-700 dark:text-yellow-400">
            Using fallback
          </span>
        </div>
      )}
    </div>
  );
}
