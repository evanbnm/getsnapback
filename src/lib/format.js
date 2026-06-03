/** Format seconds into a human-readable string: "1 min 23 s" */
export function formatDuration(seconds) {
  if (seconds < 0 || !isFinite(seconds)) return null;
  if (seconds < 60) return `${Math.round(seconds)} s`;
  const m = Math.floor(seconds / 60);
  const s = Math.round(seconds % 60);
  return s > 0 ? `${m} min ${s} s` : `${m} min`;
}

/** Shorten a full path for display: keep last 2 segments. */
export function shortPath(path) {
  if (!path) return '';
  const sep = path.includes('/') ? '/' : '\\';
  const parts = path.split(sep).filter(Boolean);
  if (parts.length <= 3) return path;
  return '…' + sep + parts.slice(-2).join(sep);
}

/** Derive a sensible default output path from the input path. */
export function defaultOutputPath(inputPath) {
  if (!inputPath) return '';
  const sep = inputPath.includes('/') ? '/' : '\\';
  const parts = inputPath.split(sep);
  parts.pop();
  return parts.join(sep) + sep + 'snapchat_final';
}
