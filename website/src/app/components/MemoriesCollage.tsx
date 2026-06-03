"use client";

import { useEffect, useState, type CSSProperties } from "react";
import { useT } from "../lib/i18n";
import { CheckIcon, DOODLES, Squiggle, Star, type DoodleName } from "./Icons";

type Memory = {
  g: string;
  cap: string;
  date: string;
  d: DoodleName;
};

const MEMORIES: Memory[] = [
  { g: "linear-gradient(135deg,#FF6B9D 0%,#FF8A3B 100%)", cap: "golden hour", date: "03·09·2017", d: "star" },
  { g: "linear-gradient(135deg,#00C9B1 0%,#3BAFDA 100%)", cap: "beach day",   date: "15·07·2018", d: "squiggle" },
  { g: "linear-gradient(135deg,#A66BFF 0%,#FF6B9D 100%)", cap: "road trip",   date: "29·02·2020", d: "ring" },
  { g: "linear-gradient(135deg,#FFD93B 0%,#FF8A3B 100%)", cap: "first snow",  date: "08·11·2022", d: "squiggle" },
  { g: "linear-gradient(135deg,#3BD4A0 0%,#1FA8C9 100%)", cap: "birthday",    date: "21·06·2024", d: "star" },
];

const STACK: CSSProperties[] = [
  { transform: "translate(0px,0px) rotate(-3deg) scale(1)",      zIndex: 6, opacity: 1 },
  { transform: "translate(28px,-12px) rotate(6deg) scale(.95)",  zIndex: 5, opacity: 1 },
  { transform: "translate(-24px,-24px) rotate(-8deg) scale(.9)", zIndex: 4, opacity: 1 },
  { transform: "translate(16px,-34px) rotate(4deg) scale(.85)",  zIndex: 3, opacity: 0.96 },
  { transform: "translate(-8px,-44px) rotate(-2deg) scale(.8)",  zIndex: 2, opacity: 0 },
];

export default function MemoriesCollage() {
  const { t } = useT();
  const [order, setOrder] = useState<number[]>([0, 1, 2, 3, 4]);

  useEffect(() => {
    const id = window.setInterval(() => {
      setOrder((o) => [...o.slice(1), o[0]]);
    }, 2800);
    return () => window.clearInterval(id);
  }, []);

  return (
    <div className="memories-stage">
      <div className="deck">
        {order.map((idx, pos) => {
          const m = MEMORIES[idx];
          const s = STACK[pos] ?? STACK[STACK.length - 1];
          const Dd = DOODLES[m.d];
          const doodleStyle: CSSProperties =
            pos % 2 ? { top: 16, left: 16 } : { top: 18, right: 18 };
          return (
            <div className="polaroid" key={idx} style={s}>
              <div className="photo" style={{ background: m.g }}>
                <Dd className="doodle" style={doodleStyle} />
                <span className="stamp">{m.date}</span>
              </div>
              <span className="cap">{m.cap}</span>
            </div>
          );
        })}
      </div>

      <div className="mem-badge m1 pop" data-anim="b1">
        <span className="badge on-white">
          <CheckIcon style={{ color: "var(--color-primary)" }} /> {t("mem.b1")}
        </span>
      </div>
      <div className="mem-badge m2 pop" data-anim="b2">
        <span className="badge on-white">{t("mem.b2")}</span>
      </div>
      <Squiggle className="mem-doodle d1 pop" data-anim="b3" />
      <Star
        className="mem-doodle d2 pop"
        data-anim="b4"
        style={{ color: "var(--color-accent)" }}
      />
    </div>
  );
}
