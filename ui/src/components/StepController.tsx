import { useState, useEffect } from "react";
import {
  Play,
  Pause,
  SkipBack,
  SkipForward,
  ChevronsLeft,
  ChevronsRight,
} from "lucide-react";

interface StepControllerProps {
  currentStep: number;
  maxSteps: number;
  onStepChange: (step: number) => void;
}

export default function StepController({
  currentStep,
  maxSteps,
  onStepChange,
}: StepControllerProps) {
  const [isPlaying, setIsPlaying] = useState(false);
  const [speed, setSpeed] = useState(1000);

  useEffect(() => {
    if (!isPlaying) return;

    const interval = setInterval(() => {
      onStepChange((prev) => {
        if (prev >= maxSteps - 1) {
          setIsPlaying(false);
          return prev;
        }
        return prev + 1;
      });
    }, speed);

    return () => clearInterval(interval);
  }, [isPlaying, speed, maxSteps, onStepChange]);

  const handlePlayPause = () => {
    if (currentStep >= maxSteps - 1) {
      onStepChange(0);
      setIsPlaying(true);
    } else {
      setIsPlaying(!isPlaying);
    }
  };

  const handlePrevious = () => {
    setIsPlaying(false);
    onStepChange(Math.max(0, currentStep - 1));
  };

  const handleNext = () => {
    setIsPlaying(false);
    onStepChange(Math.min(maxSteps - 1, currentStep + 1));
  };

  const handleFirst = () => {
    setIsPlaying(false);
    onStepChange(0);
  };

  const handleLast = () => {
    setIsPlaying(false);
    onStepChange(maxSteps - 1);
  };

  return (
    <div className="bg-gray-800 px-6 py-4">
      <div className="flex items-center gap-4">
        {/* Controls */}
        <div className="flex items-center gap-2">
          <button
            onClick={handleFirst}
            disabled={currentStep === 0}
            className="p-2 rounded hover:bg-gray-700 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
            title="First step"
          >
            <ChevronsLeft className="w-5 h-5" />
          </button>
          <button
            onClick={handlePrevious}
            disabled={currentStep === 0}
            className="p-2 rounded hover:bg-gray-700 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
            title="Previous step"
          >
            <SkipBack className="w-5 h-5" />
          </button>
          <button
            onClick={handlePlayPause}
            className="p-3 rounded-full bg-blue-600 hover:bg-blue-700 transition-colors"
            title={isPlaying ? "Pause" : "Play"}
          >
            {isPlaying ? (
              <Pause className="w-5 h-5" />
            ) : (
              <Play className="w-5 h-5" />
            )}
          </button>
          <button
            onClick={handleNext}
            disabled={currentStep >= maxSteps - 1}
            className="p-2 rounded hover:bg-gray-700 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
            title="Next step"
          >
            <SkipForward className="w-5 h-5" />
          </button>
          <button
            onClick={handleLast}
            disabled={currentStep >= maxSteps - 1}
            className="p-2 rounded hover:bg-gray-700 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
            title="Last step"
          >
            <ChevronsRight className="w-5 h-5" />
          </button>
        </div>

        {/* Progress Bar */}
        <div className="flex-1">
          <div className="flex items-center gap-3">
            <input
              type="range"
              min="0"
              max={maxSteps - 1}
              value={currentStep}
              onChange={(e) => {
                setIsPlaying(false);
                onStepChange(parseInt(e.target.value));
              }}
              className="flex-1 h-2 bg-gray-700 rounded-lg appearance-none cursor-pointer slider"
            />
            <span className="text-sm text-gray-400 min-w-[80px]">
              {currentStep + 1} / {maxSteps}
            </span>
          </div>
        </div>

        {/* Speed Control */}
        <div className="flex items-center gap-2">
          <label className="text-sm text-gray-400">Speed:</label>
          <select
            value={speed}
            onChange={(e) => setSpeed(parseInt(e.target.value))}
            className="bg-gray-700 border border-gray-600 rounded px-2 py-1 text-sm"
          >
            <option value={2000}>0.5x</option>
            <option value={1000}>1x</option>
            <option value={500}>2x</option>
            <option value={250}>4x</option>
          </select>
        </div>
      </div>

      <style>{`
        .slider::-webkit-slider-thumb {
          appearance: none;
          width: 16px;
          height: 16px;
          background: #3b82f6;
          cursor: pointer;
          border-radius: 50%;
        }
        .slider::-moz-range-thumb {
          width: 16px;
          height: 16px;
          background: #3b82f6;
          cursor: pointer;
          border-radius: 50%;
          border: none;
        }
      `}</style>
    </div>
  );
}
