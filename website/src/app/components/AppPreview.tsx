"use client";

import {
  useEffect,
  useRef,
  useState,
  type ReactNode,
} from "react";
import { useT } from "../lib/i18n";
import {
  CheckIcon,
  FolderIcon,
  LockMiniIcon,
  PlayIcon,
} from "./Icons";

const COUNTS = [1247, 1248, 1249, 1250, 1251];
const DATES = [
  "03·09·2017 23:47",
  "15·07·2018 14:23",
  "29·02·2020 03:11",
  "08·11·2022 18:09",
  "21·06·2024 09:42",
];

export default function AppPreview() {
  const { t } = useT();
  const [i, setI] = useState(0);

  useEffect(() => {
    const id = window.setInterval(() => setI((v) => (v + 1) % 5), 2400);
    return () => window.clearInterval(id);
  }, []);

  return (
    <div className="preview-stage">
      <div className="win rise" data-anim="pwin">
        <div className="titlebar">
          <div className="dots">
            <i style={{ background: "#FF5F57" }} />
            <i style={{ background: "#FEBC2E" }} />
            <i style={{ background: "#28C840" }} />
          </div>
          <div className="title-c">Snapback</div>
          <div />
        </div>
        <div className="win-body">
          <span className="badge">
            <LockMiniIcon style={{ color: "var(--color-text)" }} /> {t("pv.badge")}
          </span>
          <h3>{t("pv.title")}</h3>
          <div className="win-sub">{t("pv.sub")}</div>

          <div className="dropzone">
            <FolderIcon style={{ color: "var(--color-primary)", margin: "0 auto" }} />
            <div className="fname">snapchat_export.zip</div>
            <div className="fmeta">{t("pv.ready")}</div>
          </div>

          <div className="stats">
            <div className="stat panel">
              <div className="lbl">{t("pv.memories")}</div>
              <div className="val">
                <CycleVal k={`c${i}`}>{COUNTS[i].toLocaleString()}</CycleVal>
              </div>
            </div>
            <div className="stat panel">
              <div className="lbl">{t("pv.lastfixed")}</div>
              <div className="val mono">
                <CycleVal k={`d${i}`}>{DATES[i]}</CycleVal>
              </div>
            </div>
          </div>

          <div className="prog">
            <div className="prog-fill" />
          </div>
          <button type="button" className="btn btn-primary btn-block disabled">
            <PlayIcon /> {t("pv.processing")}
          </button>
        </div>
      </div>

      <div className="float f1 pop" data-anim="p1">
        <span className="badge on-white">
          <CheckIcon style={{ color: "var(--color-primary)" }} /> {t("pv.b1")}
        </span>
      </div>
      <div className="float f2 pop" data-anim="p2">
        <span className="badge on-white">{t("pv.b2")}</span>
      </div>
      <div className="float f3 pop" data-anim="p3">
        <span className="badge on-white">{t("pv.b3")}</span>
      </div>
    </div>
  );
}

function CycleVal({ children, k }: { children: ReactNode; k: string }) {
  const ref = useRef<HTMLSpanElement>(null);
  useEffect(() => {
    const el = ref.current;
    if (!el) return;
    el.animate(
      [{ transform: "translateY(8px)" }, { transform: "none" }],
      { duration: 320, easing: "cubic-bezier(.22,1,.36,1)" }
    );
  }, [k]);
  return (
    <span ref={ref} key={k}>
      {children}
    </span>
  );
}
