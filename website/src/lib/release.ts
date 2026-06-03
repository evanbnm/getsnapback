import type { ReleaseAssets } from "@/app/components/DownloadButtons";

const REPO = "evanbnm/getsnapback";
const FALLBACK_URL = `https://github.com/${REPO}/releases/latest`;
// Used only when the GitHub API call fails (e.g. private repo, rate-limit).
// Once the repo is public this value is overridden by the live tag.
const DEFAULT_TAG = "v0.1.11";

type GhAsset = { name: string; browser_download_url: string };
type GhRelease = { tag_name: string; assets: GhAsset[] };

function pick(assets: GhAsset[], tests: RegExp[]): string | null {
  for (const test of tests) {
    const hit = assets.find((a) => test.test(a.name));
    if (hit) return hit.browser_download_url;
  }
  return null;
}

export async function getReleaseAssets(): Promise<ReleaseAssets> {
  try {
    const res = await fetch(
      `https://api.github.com/repos/${REPO}/releases/latest`,
      {
        headers: { Accept: "application/vnd.github+json" },
        next: { revalidate: 3600 },
      }
    );
    if (!res.ok) throw new Error(`GH api ${res.status}`);
    const data = (await res.json()) as GhRelease;
    const assets = data.assets ?? [];

    return {
      tag: data.tag_name || DEFAULT_TAG,
      mac:
        pick(assets, [/aarch64.*\.dmg$/i, /\.dmg$/i]) ?? FALLBACK_URL,
      windows:
        pick(assets, [/\.msi$/i, /-setup\.exe$/i, /\.exe$/i]) ?? FALLBACK_URL,
      linux:
        pick(assets, [/\.appimage$/i, /\.deb$/i]) ?? FALLBACK_URL,
      fallback: FALLBACK_URL,
    };
  } catch {
    return {
      tag: DEFAULT_TAG,
      mac: FALLBACK_URL,
      windows: FALLBACK_URL,
      linux: FALLBACK_URL,
      fallback: FALLBACK_URL,
    };
  }
}
