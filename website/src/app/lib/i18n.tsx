"use client";

import {
  createContext,
  useCallback,
  useContext,
  useEffect,
  useState,
  type ReactNode,
} from "react";

export type Lang = "en" | "fr";

const DICT: Record<Lang, Record<string, string>> = {
  en: {
    "nav.how": "How it works",
    "nav.download": "Download",
    "nav.coffee": "Buy me a coffee",
    "nav.coffeeAria": "Buy me a coffee",
    "hero.badge": "Free · open source ·",
    "hero.t1": "Save your",
    "hero.t2": "memories on",
    "hero.t3": "the right date.",
    "hero.sub.pre":
      "A tiny desktop app that exports your Snapchat memories with their ",
    "hero.sub.strong": "real timestamps",
    "hero.sub.post":
      ", baking your overlays, captions and stickers right back in.",
    "dl.for": "Download for",
    "dl.all": "all versions →",
    "how.eyebrow": "how it works",
    "how.title": "Three clicks. No upload. Done.",
    "how.s1.t": "Pick the folder",
    "how.s1.b": "Drop in your unzipped Snapchat memories folder. As-is.",
    "how.s2.t": "Hit start",
    "how.s2.b":
      "The app reads dates, fixes EXIF and QuickTime, composites the overlays back. A few seconds per memory.",
    "how.s3.t": "Import & enjoy",
    "how.s3.b":
      "A clean snapchat_final/ folder appears. Drag it into Photos or Google Photos.",
    "app.eyebrow": "the app",
    "app.title": "No menus. No settings. Just the window.",
    "app.body":
      "Point it at your export, watch the counter climb, and get a clean folder back. That is the whole thing.",
    "app.link": "Download",
    "what.eyebrow": "what it does",
    "what.title.pre": "Made for memories that ",
    "what.title.em": "actually deserve",
    "what.title.post": " their date.",
    "what.f1.t": "Real dates",
    "what.f1.b":
      "Every photo gets its original timestamp baked back into EXIF. Photos and Google Photos sort them where they actually happened.",
    "what.f2.t": "Overlays baked in",
    "what.f2.b":
      "Captions, doodles and stickers come out as separate files. The app paints them back onto the source frame, pixel-perfect.",
    "what.f3.t": "Stays local",
    "what.f3.b":
      "No upload. No account. No telemetry. Reads your folder, writes a clean one next to it, done.",
    "cta.t1": "Free.",
    "cta.t2": "Every platform.",
    "cta.sub":
      "Apple silicon, Windows 10/11, Linux AppImage. The build is public on GitHub: hashes, source, every commit.",
    "cta.note.pre":
      "macOS build isn't signed (Apple Dev is $99/yr). First launch: ",
    "cta.note.strong": "right-click → Open",
    "cta.note.mid": ". On Windows, SmartScreen may warn, so click ",
    "cta.note.em": "more info → run anyway",
    "cta.note.post": ".",
    "support.eyebrow": "support",
    "support.title": "Did this save your memories?",
    "support.body":
      "GetSnapBack is free and stays free. If it helped, a small tip keeps it caffeinated.",
    "support.cta": "Buy me a coffee",
    "support.note": "Hosted by Buy Me a Coffee. One time, no account needed.",
    "foot.tagline": "· MIT licence · made for the memories",
    "foot.source": "source",
    "foot.bug": "report a bug",
    "foot.changelog": "changelog",
    "foot.support": "buy me a coffee",
    "pv.badge": "100% local · free",
    "pv.title": "Restore your Snapchat memories",
    "pv.sub": "Drop the export folder, hit start.",
    "pv.ready": "ready · 1.2 GB",
    "pv.memories": "memories",
    "pv.lastfixed": "last fixed",
    "pv.processing": "Processing…",
    "pv.b1": "overlay composited",
    "pv.b2": "EXIF · restored",
    "pv.b3": "100% local",
    "mem.b1": "date restored",
    "mem.b2": "overlay baked in",
  },
  fr: {
    "nav.how": "Comment ça marche",
    "nav.download": "Télécharger",
    "nav.coffee": "Offrez-moi un café",
    "nav.coffeeAria": "Offrez-moi un café",
    "hero.badge": "Gratuit · open source ·",
    "hero.t1": "Enregistrez vos",
    "hero.t2": "souvenirs à",
    "hero.t3": "la bonne date.",
    "hero.sub.pre":
      "Une petite app de bureau qui exporte vos souvenirs Snapchat avec leurs ",
    "hero.sub.strong": "vraies dates",
    "hero.sub.post":
      ", en réincrustant vos dessins, légendes et stickers.",
    "dl.for": "Télécharger pour",
    "dl.all": "toutes les versions →",
    "how.eyebrow": "comment ça marche",
    "how.title": "Trois clics. Aucun envoi. Terminé.",
    "how.s1.t": "Choisissez le dossier",
    "how.s1.b":
      "Glissez votre dossier de souvenirs Snapchat décompressé. Tel quel.",
    "how.s2.t": "Lancez",
    "how.s2.b":
      "L'app lit les dates, corrige l'EXIF et le QuickTime, et réincruste les overlays. Quelques secondes par souvenir.",
    "how.s3.t": "Importez, profitez",
    "how.s3.b":
      "Un dossier snapchat_final/ tout propre apparaît. Glissez-le dans Photos ou Google Photos.",
    "app.eyebrow": "l'app",
    "app.title": "Pas de menus. Pas de réglages. Juste la fenêtre.",
    "app.body":
      "Pointez-la vers votre export, regardez le compteur grimper, et récupérez un dossier propre. C'est tout.",
    "app.link": "Télécharger",
    "what.eyebrow": "ce qu'elle fait",
    "what.title.pre": "Pensée pour les souvenirs qui ",
    "what.title.em": "méritent vraiment",
    "what.title.post": " leur date.",
    "what.f1.t": "Vraies dates",
    "what.f1.b":
      "Chaque photo récupère sa date d'origine, réincrustée dans l'EXIF. Photos et Google Photos les rangent là où elles ont vraiment eu lieu.",
    "what.f2.t": "Overlays réincrustés",
    "what.f2.b":
      "Légendes, dessins et stickers ressortent en fichiers séparés. L'app les repeint sur l'image source, au pixel près.",
    "what.f3.t": "Reste en local",
    "what.f3.b":
      "Aucun envoi. Aucun compte. Aucune télémétrie. L'app lit votre dossier, en écrit un propre à côté, terminé.",
    "cta.t1": "Gratuit.",
    "cta.t2": "Toutes les plateformes.",
    "cta.sub":
      "Apple Silicon, Windows 10/11, Linux AppImage. Le build est public sur GitHub : hashes, source, chaque commit.",
    "cta.note.pre":
      "Le build macOS n'est pas signé (le compte Apple Dev coûte 99 $/an). Premier lancement : ",
    "cta.note.strong": "clic droit → Ouvrir",
    "cta.note.mid": ". Sous Windows, SmartScreen peut alerter, cliquez ",
    "cta.note.em": "informations complémentaires → exécuter quand même",
    "cta.note.post": ".",
    "support.eyebrow": "soutien",
    "support.title": "Cette app vous a aidé ?",
    "support.body":
      "GetSnapBack est gratuit et le restera. Si elle vous a aidé, un café m'aide à continuer.",
    "support.cta": "Offrez-moi un café",
    "support.note": "Hébergé par Buy Me a Coffee. Don ponctuel, sans compte.",
    "foot.tagline": "· licence MIT · fait pour les souvenirs",
    "foot.source": "source",
    "foot.bug": "signaler un bug",
    "foot.changelog": "journal des versions",
    "foot.support": "m'offrir un café",
    "pv.badge": "100% local · gratuit",
    "pv.title": "Restaurez vos souvenirs Snapchat",
    "pv.sub": "Glissez le dossier d'export, lancez.",
    "pv.ready": "prêt · 1,2 Go",
    "pv.memories": "souvenirs",
    "pv.lastfixed": "dernier corrigé",
    "pv.processing": "Traitement…",
    "pv.b1": "overlay incrusté",
    "pv.b2": "EXIF · restauré",
    "pv.b3": "100% local",
    "mem.b1": "date restaurée",
    "mem.b2": "overlay incrusté",
  },
};

type Ctx = {
  lang: Lang;
  setLang: (l: Lang) => void;
  t: (k: string) => string;
};

const LangCtx = createContext<Ctx>({
  lang: "en",
  setLang: () => {},
  t: (k) => k,
});

export function useT() {
  return useContext(LangCtx);
}

export function LangProvider({ children }: { children: ReactNode }) {
  const [lang, setLangState] = useState<Lang>("en");

  useEffect(() => {
    let saved: string | null = null;
    try {
      saved = localStorage.getItem("sdf-lang");
    } catch {}
    if (!saved) {
      const nav = (navigator.language || "en").toLowerCase();
      saved = nav.startsWith("fr") ? "fr" : "en";
    }
    setLangState(saved === "fr" ? "fr" : "en");
  }, []);

  useEffect(() => {
    document.documentElement.lang = lang;
    try {
      localStorage.setItem("sdf-lang", lang);
    } catch {}
  }, [lang]);

  const setLang = useCallback((l: Lang) => setLangState(l), []);
  const t = useCallback(
    (k: string) => DICT[lang][k] ?? DICT.en[k] ?? k,
    [lang]
  );

  return (
    <LangCtx.Provider value={{ lang, setLang, t }}>{children}</LangCtx.Provider>
  );
}

export function LangToggle() {
  const { lang, setLang } = useT();
  return (
    <div className="lang-toggle" role="group" aria-label="Language">
      <button
        type="button"
        className={lang === "en" ? "on" : ""}
        onClick={() => setLang("en")}
      >
        EN
      </button>
      <button
        type="button"
        className={lang === "fr" ? "on" : ""}
        onClick={() => setLang("fr")}
      >
        FR
      </button>
    </div>
  );
}
