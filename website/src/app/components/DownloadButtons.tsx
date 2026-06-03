"use client";

import { useEffect, useState } from "react";
import { useT } from "../lib/i18n";
import { ArrowIcon, OS_GLYPHS } from "./Icons";

export type OS = "mac" | "windows" | "linux";

export type ReleaseAssets = {
  tag: string;
  mac: string;
  windows: string;
  linux: string;
  fallback: string;
};

const NAMES: Record<OS, string> = {
  mac: "macOS",
  windows: "Windows",
  linux: "Linux",
};

function detectOS(): OS {
  if (typeof navigator === "undefined") return "mac";
  const ua = (navigator.userAgent || "").toLowerCase();
  if (/mac|iphone|ipad/.test(ua)) return "mac";
  if (/win/.test(ua)) return "windows";
  if (/linux|x11/.test(ua)) return "linux";
  return "mac";
}

export default function DownloadButtons({
  assets,
}: {
  assets: ReleaseAssets;
}) {
  const { t } = useT();
  const [os, setOs] = useState<OS>("mac");

  useEffect(() => setOs(detectOS()), []);

  const order: OS[] = [
    os,
    ...(["mac", "windows", "linux"] as OS[]).filter((k) => k !== os),
  ];

  return (
    <div>
      <div className="dl-buttons">
        {order.map((key, idx) => {
          const Glyph = OS_GLYPHS[key];
          const href = assets[key] || assets.fallback;
          if (idx === 0) {
            return (
              <a key={key} className="btn btn-primary" href={href}>
                <Glyph /> {t("dl.for")} {NAMES[key]} <ArrowIcon />
              </a>
            );
          }
          return (
            <a key={key} className="btn btn-outline" href={href}>
              <Glyph /> {NAMES[key]}
            </a>
          );
        })}
      </div>
      <div className="dl-meta">
        <span>{assets.tag}</span>
        <span>·</span>
        <span>{t("dl.free")}</span>
        <span>·</span>
        <a
          className="link"
          href="https://github.com/evanbnm/getsnapback/releases"
        >
          {t("dl.all")}
        </a>
      </div>
    </div>
  );
}
