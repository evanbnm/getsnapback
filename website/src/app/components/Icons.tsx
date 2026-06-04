import type { SVGProps } from "react";

type P = SVGProps<SVGSVGElement>;

export const CalendarIcon = (p: P) => (
  <svg width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" {...p}>
    <rect x="3" y="4.5" width="18" height="16" rx="3" />
    <path d="M3 9h18M8 2.5v4M16 2.5v4" />
    <path d="M15.5 14.5l-3.5 2.5v-5z" fill="currentColor" stroke="none" />
  </svg>
);

export const BrushIcon = (p: P) => (
  <svg width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" {...p}>
    <path d="M15.5 3.5l5 5L11 18l-5 1 1-5z" />
    <path d="M13 6l5 5" />
    <path d="M6 19c-1.5 1.5-3 1-3 1s.5-2 2-3" />
  </svg>
);

export const LockIcon = (p: P) => (
  <svg width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" {...p}>
    <rect x="4.5" y="10.5" width="15" height="10" rx="2.5" />
    <path d="M8 10.5V8a4 4 0 0 1 8 0v2.5" />
    <circle cx="12" cy="15.5" r="1.3" fill="currentColor" stroke="none" />
  </svg>
);

export const LockMiniIcon = (p: P) => (
  <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2.4" strokeLinecap="round" strokeLinejoin="round" {...p}>
    <rect x="4.5" y="10.5" width="15" height="10" rx="2.5" />
    <path d="M8 10.5V8a4 4 0 0 1 8 0v2.5" />
  </svg>
);

export const FolderIcon = (p: P) => (
  <svg width="30" height="30" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.8" strokeLinecap="round" strokeLinejoin="round" {...p}>
    <path d="M3 7a2 2 0 0 1 2-2h4l2 2.5h6a2 2 0 0 1 2 2V17a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z" />
  </svg>
);

export const PlayIcon = (p: P) => (
  <svg width="15" height="15" viewBox="0 0 24 24" fill="currentColor" {...p}>
    <path d="M7 5.5v13l11-6.5z" />
  </svg>
);

export const ArrowIcon = (p: P) => (
  <svg width="17" height="17" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2.6" strokeLinecap="round" strokeLinejoin="round" {...p}>
    <path d="M5 12h13M12 6l6 6-6 6" />
  </svg>
);

export const ArrowDownIcon = (p: P) => (
  <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2.6" strokeLinecap="round" strokeLinejoin="round" {...p}>
    <path d="M12 5v13M6 12l6 6 6-6" />
  </svg>
);

export const CheckIcon = (p: P) => (
  <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="3.2" strokeLinecap="round" strokeLinejoin="round" {...p}>
    <path d="M4 12.5l5 5 11-12" />
  </svg>
);

export const AppleIcon = (p: P) => (
  <svg width="15" height="15" viewBox="0 0 24 24" fill="currentColor" {...p}>
    <path d="M16.4 12.6c0-2.3 1.9-3.4 2-3.5-1.1-1.6-2.8-1.8-3.4-1.8-1.4-.1-2.8.9-3.5.9s-1.8-.8-3-.8c-1.5 0-3 .9-3.8 2.3-1.6 2.8-.4 7 1.2 9.3.8 1.1 1.7 2.4 2.9 2.3 1.2-.1 1.6-.7 3-.7s1.8.7 3 .7c1.2 0 2-1.1 2.8-2.2.9-1.3 1.2-2.5 1.3-2.6-.1 0-2.5-1-2.5-3.9zM14.2 5.4c.6-.8 1.1-1.9 1-3-.9.1-2.1.6-2.8 1.4-.6.7-1.1 1.8-1 2.9 1 .1 2.1-.5 2.8-1.3z" />
  </svg>
);

export const WindowsIcon = (p: P) => (
  <svg width="14" height="14" viewBox="0 0 24 24" fill="currentColor" {...p}>
    <path d="M3 5.4l7.5-1v7.1H3zM11.4 4.2L21 3v8.5h-9.6zM3 12.5h7.5v7.1L3 18.6zM11.4 12.5H21V21l-9.6-1.3z" />
  </svg>
);

export const CoffeeIcon = (p: P) => (
  <svg width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" {...p}>
    <path d="M4 9h13v6a5 5 0 0 1-5 5H9a5 5 0 0 1-5-5V9Z" />
    <path d="M17 11h2.5a2.5 2.5 0 0 1 0 5H17" />
    <path d="M8 4c0 1 .5 1.5.5 2.5S8 8 8 9" />
    <path d="M11.5 4c0 1 .5 1.5.5 2.5s-.5 1.5-.5 2.5" />
  </svg>
);

export const LinuxIcon = (p: P) => (
  // Tux drawn from scratch — rounder body, large white belly, close-set
  // eye patches, small triangular beak, two white feet at the base.
  // Explicit fills so the penguin keeps his canonical look on every
  // button background.
  <svg width="20" height="20" viewBox="0 0 24 24" {...p}>
    {/* black body silhouette */}
    <path
      fill="#0E0E0E"
      d="M12 1.6c-2.3 0-3.9 1.9-3.9 4.6 0 1 .2 1.9.4 2.6-1.7.9-3 3.1-3 5.9 0 2.6 1.1 4.5 2.4 5.6-1.2.3-1.9 1-1.9 1.8 0 .8.5 1.3 1.5 1.3.5 0 1.2-.1 1.8-.3 0 0 .8-.5 1.4-.6h4.6c.6.1 1.4.6 1.4.6.6.2 1.3.3 1.8.3 1 0 1.5-.5 1.5-1.3 0-.8-.7-1.5-1.9-1.8 1.3-1.1 2.4-3 2.4-5.6 0-2.8-1.3-5-3-5.9.2-.7.4-1.6.4-2.6C15.9 3.5 14.3 1.6 12 1.6Z"
    />
    {/* white belly */}
    <ellipse cx="12" cy="14.8" rx="3.2" ry="3.9" fill="#FFFFFF" />
    {/* eye patches (close-set) */}
    <ellipse cx="10.8" cy="6.3" rx="1" ry="1.4" fill="#FFFFFF" />
    <ellipse cx="13.2" cy="6.3" rx="1" ry="1.4" fill="#FFFFFF" />
    {/* pupils */}
    <circle cx="11" cy="6.6" r="0.42" fill="#0E0E0E" />
    <circle cx="13" cy="6.6" r="0.42" fill="#0E0E0E" />
    {/* small dark beak */}
    <path fill="#0E0E0E" d="M11.35 7.6h1.3L12 8.45Z" />
    {/* white feet at base */}
    <ellipse cx="9.1" cy="22.1" rx="1.9" ry="1" fill="#FFFFFF" stroke="#0E0E0E" strokeWidth="0.55" />
    <ellipse cx="14.9" cy="22.1" rx="1.9" ry="1" fill="#FFFFFF" stroke="#0E0E0E" strokeWidth="0.55" />
  </svg>
);

/* ---- Doodles (used by MemoriesCollage) ---- */

export const Squiggle = (p: P) => (
  <svg width="46" height="20" viewBox="0 0 46 20" fill="none" stroke="currentColor" strokeWidth="3.4" strokeLinecap="round" {...p}>
    <path d="M2 12C7 3 12 3 17 11s10 8 15 0 9-8 11-3" />
  </svg>
);

export const Star = (p: P) => (
  <svg width="30" height="30" viewBox="0 0 24 24" fill="currentColor" {...p}>
    <path d="M12 2l2.6 6.3L21 9l-5 4.3L17.7 20 12 16.4 6.3 20 8 13.3 3 9l6.4-.7z" />
  </svg>
);

export const Ring = (p: P) => (
  <svg width="34" height="34" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="3.2" {...p}>
    <circle cx="12" cy="12" r="9" strokeDasharray="3 5" strokeLinecap="round" />
  </svg>
);

export const DOODLES = { squiggle: Squiggle, star: Star, ring: Ring } as const;
export type DoodleName = keyof typeof DOODLES;

export const OS_GLYPHS = {
  mac: AppleIcon,
  windows: WindowsIcon,
  linux: LinuxIcon,
} as const;
