import { getVersion } from '@tauri-apps/api/app';

const REPO = 'evanbnm/getsnapback';
const DISMISS_KEY = 'getsnapback-dismissed-update';

/** Strict numeric semver comparison. Returns true if `a` is strictly newer. */
function isNewer(a, b) {
  const pa = a.split('.').map(Number);
  const pb = b.split('.').map(Number);
  for (let i = 0; i < Math.max(pa.length, pb.length); i++) {
    const ai = pa[i] ?? 0;
    const bi = pb[i] ?? 0;
    if (ai > bi) return true;
    if (ai < bi) return false;
  }
  return false;
}

/**
 * Check GitHub for a newer release than the running app version. Returns
 * `{ tag, url }` on a real update, or `null` otherwise. Honors a per-version
 * dismissal stored in localStorage so the banner doesn't re-appear after
 * the user closes it.
 */
export async function checkForUpdate() {
  let current;
  try {
    current = await getVersion();
  } catch {
    return null;
  }

  let latestTag = null;
  try {
    const res = await fetch(
      `https://api.github.com/repos/${REPO}/releases/latest`,
      { headers: { Accept: 'application/vnd.github+json' } }
    );
    if (!res.ok) return null;
    const data = await res.json();
    latestTag = data.tag_name;
  } catch {
    return null;
  }
  if (!latestTag) return null;

  const latestVersion = latestTag.replace(/^v/, '');
  if (!isNewer(latestVersion, current)) return null;

  try {
    if (localStorage.getItem(DISMISS_KEY) === latestTag) return null;
  } catch {}

  return {
    tag: latestTag,
    url: `https://github.com/${REPO}/releases/tag/${latestTag}`,
  };
}

export function dismissUpdate(tag) {
  try {
    localStorage.setItem(DISMISS_KEY, tag);
  } catch {}
}
