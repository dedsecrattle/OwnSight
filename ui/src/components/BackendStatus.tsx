interface BackendStatusProps {
  className?: string;
}

export function BackendStatus({ className = "" }: BackendStatusProps) {
  return (
    <div className={`flex items-center gap-2 ${className}`}>
      <div className="flex items-center gap-1.5 px-3 py-1.5 rounded-lg bg-gray-100 dark:bg-gray-800">
        <div className="w-2 h-2 rounded-full bg-blue-500" />
        <span className="text-sm font-medium text-gray-700 dark:text-gray-300">
          Simple
        </span>
      </div>
    </div>
  );
}
