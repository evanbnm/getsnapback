"use client";

import AppPreview from "./components/AppPreview";
import DownloadButtons, {
  type ReleaseAssets,
} from "./components/DownloadButtons";
import MemoriesCollage from "./components/MemoriesCollage";
import {
  ArrowDownIcon,
  BrushIcon,
  CalendarIcon,
  CoffeeIcon,
  LockIcon,
} from "./components/Icons";

const BMC_URL = "https://www.buymeacoffee.com/evanbnm";
import { LangProvider, LangToggle, useT } from "./lib/i18n";

export default function Site({ assets }: { assets: ReleaseAssets }) {
  return (
    <LangProvider>
      <Nav />
      <Hero assets={assets} />
      <HowItWorks />
      <AppShowcase />
      <WhatItDoes />
      <DownloadCTA assets={assets} />
      <Support />
      <Footer version={assets.tag} />
    </LangProvider>
  );
}

function Nav() {
  const { t } = useT();
  return (
    <nav className="nav">
      <div className="wrap nav-inner">
        <a className="brand" href="#top">
          {/* eslint-disable-next-line @next/next/no-img-element */}
          <img src="/icon.png" alt="GetSnapBack" />
          <span>GetSnapBack</span>
        </a>
        <div className="nav-right">
          <a className="nav-link" href="#how">
            {t("nav.how")}
          </a>
          <a className="nav-link" href="#download">
            {t("nav.download")}
          </a>
          <a
            className="pill"
            href="https://github.com/evanbnm/getsnapback"
          >
            Github
          </a>
          <a
            className="coffee-pill"
            href={BMC_URL}
            target="_blank"
            rel="noopener noreferrer"
            aria-label={t("nav.coffeeAria")}
          >
            <span className="cup">
              <span className="steam" aria-hidden>
                <i />
                <i />
                <i />
              </span>
              <CoffeeIcon width={15} height={15} />
            </span>
            <span className="coffee-label">{t("nav.coffee")}</span>
          </a>
          <LangToggle />
        </div>
      </div>
    </nav>
  );
}

function Hero({ assets }: { assets: ReleaseAssets }) {
  const { t } = useT();
  return (
    <header className="hero" id="top">
      <div className="wrap hero-grid">
        <div>
          <span className="badge rise" data-anim="h0">
            <span className="dot" />
            {t("hero.badge")} {assets.tag}
          </span>
          <h1 className="h1 rise" data-anim="h1">
            {t("hero.t1")}
            <br />
            {t("hero.t2")}
            <br />
            <span className="hl">{t("hero.t3")}</span>
          </h1>
          <p className="subtitle rise" data-anim="h2">
            {t("hero.sub.pre")}
            <strong>{t("hero.sub.strong")}</strong>
            {t("hero.sub.post")}
          </p>
          <div className="rise" data-anim="h3">
            <DownloadButtons assets={assets} />
          </div>
        </div>
        <div className="rise" data-anim="h4">
          <MemoriesCollage />
        </div>
      </div>
    </header>
  );
}

function HowItWorks() {
  const { t } = useT();
  const steps: Array<[string, string, string]> = [
    ["1", t("how.s1.t"), t("how.s1.b")],
    ["2", t("how.s2.t"), t("how.s2.b")],
    ["3", t("how.s3.t"), t("how.s3.b")],
  ];
  return (
    <section className="section" id="how">
      <div className="wrap">
        <div className="card how-card">
          <span className="eyebrow dark">{t("how.eyebrow")}</span>
          <h2 className="h2" style={{ marginTop: 16 }}>
            {t("how.title")}
          </h2>
          <div className="steps">
            {steps.map(([n, title, body]) => (
              <div className="step" key={n}>
                <div className="step-head">
                  <div className="step-num">{n}</div>
                  <div className="step-rule" />
                </div>
                <h3>{title}</h3>
                <p>{body}</p>
              </div>
            ))}
          </div>
        </div>
      </div>
    </section>
  );
}

function AppShowcase() {
  const { t } = useT();
  return (
    <section className="appshow" id="app">
      <div className="wrap appshow-grid">
        <div className="appshow-copy">
          <span className="eyebrow dark">{t("app.eyebrow")}</span>
          <h2 className="h2">{t("app.title")}</h2>
          <p>{t("app.body")}</p>
          <a className="jump-link" href="#download">
            {t("app.link")} <ArrowDownIcon />
          </a>
        </div>
        <div>
          <AppPreview />
        </div>
      </div>
    </section>
  );
}

function WhatItDoes() {
  const { t } = useT();
  const feats = [
    { Icon: CalendarIcon, title: t("what.f1.t"), body: t("what.f1.b") },
    { Icon: BrushIcon, title: t("what.f2.t"), body: t("what.f2.b") },
    { Icon: LockIcon, title: t("what.f3.t"), body: t("what.f3.b") },
  ];
  return (
    <section className="section section-pad">
      <div className="wrap">
        <span className="eyebrow dark">{t("what.eyebrow")}</span>
        <h2 className="h2" style={{ marginTop: 16, maxWidth: "20ch" }}>
          {t("what.title.pre")}
          <em style={{ fontStyle: "italic", fontWeight: 800 }}>
            {t("what.title.em")}
          </em>
          {t("what.title.post")}
        </h2>
        <div className="feat-grid">
          {feats.map(({ Icon, title, body }) => (
            <div className="feat" key={title}>
              <div className="feat-tile">
                <Icon />
              </div>
              <h3>{title}</h3>
              <p>{body}</p>
            </div>
          ))}
        </div>
      </div>
    </section>
  );
}

function DownloadCTA({ assets }: { assets: ReleaseAssets }) {
  const { t } = useT();
  return (
    <section className="section section-pad" id="download">
      <div className="wrap">
        <div className="card cta-card">
          <div className="deco deco-teal" />
          <div className="deco deco-pink" />
          <div className="cta-grid">
            <div>
              <h2 className="h2">
                {t("cta.t1")}
                <br />
                {t("cta.t2")}
              </h2>
              <p className="cta-sub">{t("cta.sub")}</p>
            </div>
            <div>
              <DownloadButtons assets={assets} />
              <p className="note">
                {t("cta.note.pre")}
                <strong>{t("cta.note.strong")}</strong>
                {t("cta.note.mid")}
                <em>{t("cta.note.em")}</em>
                {t("cta.note.post")}
              </p>
            </div>
          </div>
        </div>
      </div>
    </section>
  );
}

function Support() {
  const { t } = useT();
  return (
    <section className="section section-pad" id="support">
      <div className="wrap">
        <div className="card support-card">
          <div className="support-grid">
            <div className="support-glyph">
              <CoffeeIcon />
            </div>
            <div className="support-copy">
              <span className="eyebrow dark">{t("support.eyebrow")}</span>
              <h2 className="h2" style={{ marginTop: 14 }}>
                {t("support.title")}
              </h2>
              <p>{t("support.body")}</p>
              <a
                className="btn btn-primary support-cta"
                href={BMC_URL}
                target="_blank"
                rel="noopener noreferrer"
              >
                <CoffeeIcon width={17} height={17} /> {t("support.cta")}
              </a>
              <p className="support-note">{t("support.note")}</p>
            </div>
          </div>
        </div>
      </div>
    </section>
  );
}

function Footer({ version }: { version: string }) {
  const { t } = useT();
  return (
    <footer className="footer">
      <div className="wrap">
        <div className="foot-rule" />
        <div className="foot-inner">
          <div className="foot-brand">
            {/* eslint-disable-next-line @next/next/no-img-element */}
            <img src="/icon.png" alt="" />
            <div>
              <div className="t">GetSnapBack</div>
              <div className="s">{version} {t("foot.tagline")}</div>
            </div>
          </div>
          <div className="foot-links">
            <a
              className="link"
              href="https://github.com/evanbnm/getsnapback"
            >
              {t("foot.source")}
            </a>
            <a
              className="link"
              href="https://github.com/evanbnm/getsnapback/issues"
            >
              {t("foot.bug")}
            </a>
            <a
              className="link"
              href="https://github.com/evanbnm/getsnapback/releases"
            >
              {t("foot.changelog")}
            </a>
            <a
              className="link"
              href={BMC_URL}
              target="_blank"
              rel="noopener noreferrer"
            >
              {t("foot.support")}
            </a>
          </div>
        </div>
      </div>
    </footer>
  );
}
