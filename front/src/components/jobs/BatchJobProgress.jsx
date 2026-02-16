import React from "react";
import {
  CheckCircle,
  XCircle,
  Clock,
  Loader,
  AlertCircle,
  BarChart3,
  Flame,
} from "lucide-react";

export default function BatchJobProgress({
  stats,
  jobs = [],
  isLoading = false,
  isComplete = false,
  onCancel,
  showDetails = false,
  jobType = "batch", // New prop to identify job type
}) {
  if (!stats) {
    return (
      <div className="flex items-center gap-2 text-sm text-white/60">
        <Loader className="w-4 h-4 animate-spin" />
        <span>Initializing batch...</span>
      </div>
    );
  }

  const progress =
    stats.total > 0
      ? ((stats.completed + stats.failed + stats.cancelled) / stats.total) * 100
      : 0;
  const successRate =
    stats.total > 0 ? (stats.completed / stats.total) * 100 : 0;

  // Get appropriate icon and colors based on job type
  const getJobTypeInfo = () => {
    if (jobType === "burn" || jobType === "burn_tokens") {
      return {
        icon: <Flame className="w-5 h-5 text-orange-400" />,
        title: "Burn Tokens Progress",
        colors: {
          bg: "bg-orange-500/10",
          border: "border-orange-400/20",
          text: "text-orange-400",
          progress: isComplete
            ? successRate > 90
              ? "bg-green-500"
              : successRate > 70
              ? "bg-yellow-500"
              : "bg-red-500"
            : "bg-orange-500",
        },
      };
    } else if (jobType === "close" || jobType === "close_accounts") {
      return {
        icon: <XCircle className="w-5 h-5 text-red-400" />,
        title: "Close Accounts Progress",
        colors: {
          bg: "bg-red-500/10",
          border: "border-red-400/20",
          text: "text-red-400",
          progress: isComplete
            ? successRate > 90
              ? "bg-green-500"
              : successRate > 70
              ? "bg-yellow-500"
              : "bg-red-500"
            : "bg-red-500",
        },
      };
    } else {
      return {
        icon: <BarChart3 className="w-5 h-5 text-blue-400" />,
        title: "Batch Progress",
        colors: {
          bg: "bg-blue-500/10",
          border: "border-blue-400/20",
          text: "text-blue-400",
          progress: isComplete
            ? successRate > 90
              ? "bg-green-500"
              : successRate > 70
              ? "bg-yellow-500"
              : "bg-red-500"
            : "bg-blue-500",
        },
      };
    }
  };

  const jobInfo = getJobTypeInfo();

  return (
    <div
      className={`space-y-4 p-3 rounded-lg ${jobInfo.colors.bg} border ${jobInfo.colors.border}`}
    >
      {/* En-tête avec statistiques principales */}
      <div className="flex items-center justify-between">
        <div className="flex items-center gap-3">
          {jobInfo.icon}
          <div>
            <h4 className={`text-sm font-medium text-white`}>
              {jobInfo.title}
            </h4>
            <p className="text-xs text-white/60">
              {stats.completed + stats.failed + stats.cancelled} / {stats.total}{" "}
              jobs processed
            </p>
          </div>
        </div>

        {isLoading && !isComplete && onCancel && (
          <button
            onClick={onCancel}
            className="px-3 py-1 text-xs rounded-lg bg-red-500/20 hover:bg-red-500/30 text-red-400 border border-red-400/30 transition"
          >
            Cancel Batch
          </button>
        )}
      </div>

      {/* Barre de progression */}
      <div className="space-y-2">
        <div className="flex items-center justify-between text-sm">
          <span className="text-white/80">Overall Progress</span>
          <span className="text-white/60">{Math.round(progress)}%</span>
        </div>
        <div className="w-full bg-white/10 rounded-full h-2">
          <div
            className={`h-2 rounded-full transition-all duration-500 ${jobInfo.colors.progress}`}
            style={{ width: `${progress}%` }}
          />
        </div>
      </div>

      {/* Grille des statistiques détaillées */}
      <div className="grid grid-cols-5 gap-2">
        <div className="flex items-center gap-2 p-2 rounded-lg bg-green-500/10 border border-green-400/20">
          <CheckCircle className="w-4 h-4 text-green-400" />
          <div className="text-center">
            <div className="text-sm font-medium text-green-400">
              {stats.completed}
            </div>
            <div className="text-xs text-green-400/70">Completed</div>
          </div>
        </div>

        <div className="flex items-center gap-2 p-2 rounded-lg bg-red-500/10 border border-red-400/20">
          <XCircle className="w-4 h-4 text-red-400" />
          <div className="text-center">
            <div className="text-sm font-medium text-red-400">
              {stats.failed}
            </div>
            <div className="text-xs text-red-400/70">Failed</div>
          </div>
        </div>

        <div className="flex items-center gap-2 p-2 rounded-lg bg-blue-500/10 border border-blue-400/20">
          <Loader className="w-4 h-4 text-blue-400 animate-spin" />
          <div className="text-center">
            <div className="text-sm font-medium text-blue-400">
              {stats.running}
            </div>
            <div className="text-xs text-blue-400/70">Running</div>
          </div>
        </div>

        <div className="flex items-center gap-2 p-2 rounded-lg bg-yellow-500/10 border border-yellow-400/20">
          <Clock className="w-4 h-4 text-yellow-400" />
          <div className="text-center">
            <div className="text-sm font-medium text-yellow-400">
              {stats.pending}
            </div>
            <div className="text-xs text-yellow-400/70">Pending</div>
          </div>
        </div>

        <div className="flex items-center gap-2 p-2 rounded-lg bg-gray-500/10 border border-gray-400/20">
          <AlertCircle className="w-4 h-4 text-gray-400" />
          <div className="text-center">
            <div className="text-sm font-medium text-gray-400">
              {stats.cancelled}
            </div>
            <div className="text-xs text-gray-400/70">Cancelled</div>
          </div>
        </div>
      </div>

      {/* Résumé des résultats si terminé */}
      {isComplete && (
        <div
          className={`p-3 rounded-lg border ${
            successRate > 90
              ? "bg-green-500/10 border-green-400/30 text-green-400"
              : successRate > 70
              ? "bg-yellow-500/10 border-yellow-400/30 text-yellow-400"
              : "bg-red-500/10 border-red-400/30 text-red-400"
          }`}
        >
          <div className="text-sm font-medium flex items-center gap-2">
            {jobInfo.icon}
            Batch Complete: {Math.round(successRate)}% success rate
          </div>
          <div className="text-xs mt-1 opacity-80">
            {stats.completed} successful, {stats.failed} failed,{" "}
            {stats.cancelled} cancelled
          </div>
        </div>
      )}

      {/* Détails des jobs individuels */}
      {showDetails && jobs.length > 0 && (
        <div className="space-y-2">
          <h5 className="text-sm font-medium text-white/80">Job Details:</h5>
          <div className="max-h-40 overflow-y-auto space-y-1">
            {jobs.map((job, index) => (
              <div
                key={job.id || index}
                className="flex items-center justify-between text-xs p-2 rounded bg-white/5"
              >
                <span className="text-white/70">Job {index + 1}</span>
                <div
                  className={`px-2 py-1 rounded ${
                    job.status === "completed"
                      ? "bg-green-500/20 text-green-400"
                      : job.status === "failed"
                      ? "bg-red-500/20 text-red-400"
                      : job.status === "running"
                      ? "bg-blue-500/20 text-blue-400"
                      : job.status === "cancelled"
                      ? "bg-gray-500/20 text-gray-400"
                      : "bg-yellow-500/20 text-yellow-400"
                  }`}
                >
                  {job.status}
                </div>
              </div>
            ))}
          </div>
        </div>
      )}
    </div>
  );
}
