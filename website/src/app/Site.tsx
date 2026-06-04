"use client";

import AppPreview from "./components/AppPreview";
import DownloadButtons, {
  type ReleaseAssets,
} from "./components/DownloadButtons";
import MemoriesCollage from "./components/MemoriesCollage";
import Reveal from "./components/Reveal";
import {
  ArrowDownIcon,
  ArrowIcon,
  BrushIcon,
  CalendarIcon,
  CoffeeIcon,
  LockIcon,
} from "./components/Icons";
import { LangProvider, LangToggle, useT } from "./lib/i18n";

const BMC_URL = "https://www.buymeacoffee.com/evanbnm";
const SNAP_EXPORT_URL = "https://accounts.snapchat.com/v2/download-my-data";

export default function Site({ assets }: { assets: ReleaseAssets }) {
  return (
    <LangProvider>
      <Nav />
      <Hero assets={assets} />
      <Reveal>
        <AppShowcase />
      </Reveal>
      <Reveal>
        <WhatItDoes />
      </Reveal>
      <Reveal>
        <HowToExport />
      </Reveal>
      <Reveal>
        <DownloadCTA assets={assets} />
      </Reveal>
      <Reveal>
        <FAQ />
      </Reveal>
      <Reveal>
        <Support />
      </Reveal>
      <Footer />
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

function HowToExport() {
  const { t } = useT();
  return (
    <section className="section section-pad" id="export">
      <div className="wrap">
        <div className="export-layout">
          <div className="export-copy">
            <span className="eyebrow dark">{t("export.eyebrow")}</span>
            <h2 className="h2" style={{ marginTop: 14 }}>
              {t("export.title")}
            </h2>
            <p className="export-body">{t("export.body")}</p>
            <a
              className="btn btn-primary export-cta"
              href={SNAP_EXPORT_URL}
              target="_blank"
              rel="noopener noreferrer"
            >
              {t("export.cta")} <ArrowIcon />
            </a>
            <p className="export-note">{t("export.note")}</p>
          </div>
          <ExportArt />
        </div>
      </div>
    </section>
  );
}

function ExportArt() {
  return (
    <div className="export-art" aria-hidden>
      <div className="art-card art-c1">
        <div
          className="art-photo"
          style={{ background: "linear-gradient(135deg,#FF6B9D 0%,#FF8A3B 100%)" }}
        />
        <span className="art-cap">03·09·17</span>
      </div>
      <div className="art-card art-c2">
        <div
          className="art-photo"
          style={{ background: "linear-gradient(135deg,#00C9B1 0%,#3BAFDA 100%)" }}
        />
        <span className="art-cap">15·07·18</span>
      </div>
      <div className="art-card art-c3">
        <div
          className="art-photo"
          style={{ background: "linear-gradient(135deg,#A66BFF 0%,#FF6B9D 100%)" }}
        />
        <span className="art-cap">21·06·24</span>
      </div>
      <div className="art-zip">
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
          <path d="M3 7a2 2 0 0 1 2-2h4l2 2h8a2 2 0 0 1 2 2v9a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z" />
          <path d="M13 9v2M13 13v2M13 17v2" />
        </svg>
        <span>snapchat_memories.zip</span>
      </div>
      <div className="art-arrow" aria-hidden>
        <svg width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2.6" strokeLinecap="round" strokeLinejoin="round">
          <path d="M12 5v14" />
          <path d="M6 13l6 6 6-6" />
        </svg>
      </div>
    </div>
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
                <strong>{t("cta.note.intro")}</strong>
                <br />
                <strong>{t("cta.note.mac.label")}</strong>
                {t("cta.note.mac.pre")}
                <em>{t("cta.note.mac.em")}</em>
                {t("cta.note.mac.post")}
                <br />
                <strong>{t("cta.note.win.label")}</strong>
                {t("cta.note.win.pre")}
                <em>{t("cta.note.win.em")}</em>
                {t("cta.note.win.post")}
              </p>
            </div>
          </div>
        </div>
      </div>
    </section>
  );
}

/* FAQ — Q/R verbatim from the FAQPage JSON-LD in page.tsx so Google
   considers them eligible for rich-result rendering. Uses <details> so
   each row is keyboard-accessible and crawlable without JS. */
function FAQ() {
  const { t } = useT();
  const items = [1, 2, 3, 4, 5, 6].map((n) => ({
    q: t(`faq.q${n}.q`),
    a: t(`faq.q${n}.a`),
  }));
  return (
    <section className="section section-pad" id="faq">
      <div className="wrap">
        <span className="eyebrow dark">{t("faq.eyebrow")}</span>
        <h2 className="h2" style={{ marginTop: 14, maxWidth: "22ch" }}>
          {t("faq.title")}
        </h2>
        <div className="faq-list">
          {items.map(({ q, a }, i) => (
            <details key={i} className="faq-item">
              <summary>{q}</summary>
              <p>{a}</p>
            </details>
          ))}
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
                className="coffee-pill big support-cta"
                href={BMC_URL}
                target="_blank"
                rel="noopener noreferrer"
              >
                <span className="cup">
                  <span className="steam" aria-hidden>
                    <i />
                    <i />
                    <i />
                  </span>
                  <CoffeeIcon width={22} height={22} />
                </span>
                <span>{t("support.cta")}</span>
              </a>
            </div>
          </div>
        </div>
      </div>
    </section>
  );
}

function Footer() {
  const { t } = useT();
  return (
    <footer className="footer">
      <div className="wrap">
        <div className="foot-rule" />
        <div className="foot-inner">
          <div className="foot-brand">
            {/* eslint-disable-next-line @next/next/no-img-element */}
            <img src="/icon.png" alt="" />
            <div className="t">GetSnapBack</div>
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
