"use client";

import { useEffect, useRef, useState } from "react";
import { useT } from "../lib/i18n";

const DEMO_SRC = "/demo.mp4";

export default function AppDemo() {
  const { t } = useT();
  const videoRef = useRef<HTMLVideoElement>(null);
  // Start with the placeholder (matches reality when demo.mp4 is missing).
  // On mount, HEAD-check the file; if it's there, swap to the actual video.
  const [hasVideo, setHasVideo] = useState(false);

  useEffect(() => {
    let cancelled = false;
    fetch(DEMO_SRC, { method: "HEAD" }).then(
      (r) => !cancelled && r.ok && setHasVideo(true),
      () => {}
    );
    return () => {
      cancelled = true;
    };
  }, []);

  return (
    <section className="section section-pad" id="demo">
      <div className="wrap">
        <div className="demo-grid">
          <div className="demo-copy">
            <span className="eyebrow dark">{t("demo.eyebrow")}</span>
            <h2 className="h2" style={{ marginTop: 14 }}>
              {t("demo.title")}
            </h2>
            <p className="demo-body">{t("demo.body")}</p>
          </div>
          <div className="demo-window">
            <div className="demo-titlebar">
              <div className="dots">
                <i style={{ background: "#FF5F57" }} />
                <i style={{ background: "#FEBC2E" }} />
                <i style={{ background: "#28C840" }} />
              </div>
              <div className="title-c">GetSnapBack</div>
              <div />
            </div>
            {hasVideo ? (
              <video
                ref={videoRef}
                className="demo-video"
                src={DEMO_SRC}
                autoPlay
                loop
                muted
                playsInline
                preload="metadata"
              />
            ) : (
              <div className="demo-placeholder">
                <span className="label">{t("demo.placeholder")}</span>
                <span className="hint">website/public/demo.mp4</span>
              </div>
            )}
          </div>
        </div>
      </div>
    </section>
  );
}
