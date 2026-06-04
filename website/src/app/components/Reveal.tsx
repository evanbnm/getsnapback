"use client";

import { useEffect, useRef, type ReactNode } from "react";

/**
 * Wraps content in a `<div>` that fades + slides up when scrolled into view.
 * Uses IntersectionObserver, disconnects after the first reveal, and respects
 * prefers-reduced-motion via CSS.
 */
export default function Reveal({
  children,
  delay,
  className = "",
  as: Tag = "div",
}: {
  children: ReactNode;
  delay?: number;
  className?: string;
  as?: "div" | "section" | "article";
}) {
  const ref = useRef<HTMLElement>(null);

  useEffect(() => {
    const el = ref.current;
    if (!el) return;
    // Trim the bottom 35% of the viewport so a section only counts as
    // "in view" once it's clearly scrolled into the upper part of the
    // screen. Without this, on tall monitors multiple sections trigger
    // at page-load and the user never sees the actual reveal motion.
    const io = new IntersectionObserver(
      (entries) => {
        for (const entry of entries) {
          if (entry.isIntersecting) {
            el.classList.add("in-view");
            io.disconnect();
          }
        }
      },
      { rootMargin: "0px 0px -35% 0px", threshold: 0.15 }
    );
    io.observe(el);
    return () => io.disconnect();
  }, []);

  return (
    <Tag
      ref={ref as never}
      className={`reveal ${className}`}
      style={delay ? ({ "--reveal-delay": `${delay}ms` } as React.CSSProperties) : undefined}
    >
      {children}
    </Tag>
  );
}
