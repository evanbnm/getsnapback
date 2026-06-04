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
    "nav.coffee": "Buy me a coffee",
    "nav.coffeeAria": "Buy me a coffee",
    "hero.t1": "Save your",
    "hero.t2": "memories on",
    "hero.t3": "the right date.",
    "hero.sub.pre":
      "Snapchat dates your memories export to today. GetSnapBack restores the ",
    "hero.sub.strong": "real dates",
    "hero.sub.post":
      " of every photo and video — EXIF for photos, QuickTime for videos — and bakes your overlays, captions and stickers back on. Ready for Apple Photos and Google Photos.",
    "dl.for": "Download for",
    "dl.all": "all versions →",
    "app.eyebrow": "the app",
    "app.title": "No menus. No settings. Just the window.",
    "app.body":
      "Point it at your export, watch the counter climb, and get a clean folder back. That is the whole thing.",
    "app.link": "Download",
    "what.eyebrow": "what it does",
    "what.f1.t": "Real dates",
    "what.f1.b":
      "Drop your Snapchat export, a zip or an unzipped folder. Every photo gets its original timestamp written back into EXIF, so Photos and Google Photos sort them where they actually happened.",
    "what.f2.t": "Overlays baked in",
    "what.f2.b":
      "Captions, doodles and stickers come out as separate files. The app paints them back onto the source frame, pixel-perfect.",
    "what.f3.t": "Stays local",
    "what.f3.b":
      "No upload. No account. No telemetry. Reads your folder, writes a clean one next to it, done.",
    "export.eyebrow": "first step",
    "export.title": "Get your export from Snapchat",
    "export.body":
      "Open the Snapchat page below and tick only « Export my memories ». Leave the other boxes unchecked.",
    "export.cta": "Open the Snapchat export page",
    "export.note":
      "Snapchat emails you a download link, usually within an hour (sometimes up to 24h). If the archive is over 2 GB it gets split into several zips: unzip them all into the same folder before running GetSnapBack.",
    "cta.t1": "Free.",
    "cta.t2": "Every platform.",
    "cta.sub": "Apple silicon, Windows 10/11, Linux AppImage.",
    "cta.note.intro": "First launch:",
    "cta.note.mac.label": "macOS",
    "cta.note.mac.pre":
      " — try opening the app once, then in System Settings → Privacy & Security, click ",
    "cta.note.mac.em": "Open Anyway",
    "cta.note.mac.post":
      ". macOS will let you launch GetSnapBack normally from then on.",
    "cta.note.win.label": "Windows",
    "cta.note.win.pre":
      " — SmartScreen may warn you; click ",
    "cta.note.win.em": "more info → run anyway",
    "cta.note.win.post": ".",
    "faq.eyebrow": "common questions",
    "faq.title": "Snapchat memories, wrong dates — sorted",
    "faq.q1.q": "Why are my Snapchat memories all dated 2024 instead of when I took them?",
    "faq.q1.a": "Snapchat stamps every exported file with the date of the export, not the date the photo or video was taken. The real date is preserved only in the filename prefix (YYYY-MM-DD). GetSnapBack reads that prefix and writes it back into EXIF (photos) and QuickTime (videos) metadata, so Apple Photos and Google Photos sort your memories under the year they actually happened.",
    "faq.q2.q": "Will my photos and videos be uploaded anywhere?",
    "faq.q2.a": "No. GetSnapBack runs entirely on your computer. There is no account, no server, no telemetry. The app reads your folder and writes a clean folder next to it. You can run it offline.",
    "faq.q3.q": "Snapchat split my export into several ZIPs — can the app handle that?",
    "faq.q3.a": "Yes. When a memories archive is over 2 GB Snapchat splits it across several ZIPs. Drop all of them onto the app together (or extract them into the same folder) and GetSnapBack merges them into one clean output.",
    "faq.q4.q": "What about the overlays — the text, captions, stickers I drew?",
    "faq.q4.a": "Snapchat exports overlays as separate transparent PNG files next to each main photo or video. GetSnapBack composites them back onto the source media so the result imported into Apple Photos or Google Photos looks exactly like the original Snap.",
    "faq.q5.q": "Does it also fix the date on videos?",
    "faq.q5.a": "Yes. Photo dates go into EXIF, video dates go into the QuickTime atoms (creation_time and com.apple.quicktime.creationdate). Both Apple Photos and Google Photos read those fields.",
    "faq.q6.q": "Is it really free?",
    "faq.q6.a": "Yes — free and open source under MIT. If it helped, there is a Buy Me a Coffee link, but no paid version, no plan to add one.",

    "support.eyebrow": "support",
    "support.title": "Did this save your memories?",
    "support.body":
      "GetSnapBack is free and stays free. If it helped, a small tip keeps it caffeinated.",
    "support.cta": "Buy me a coffee",
    "foot.source": "source",
    "foot.bug": "report a bug",
    "foot.changelog": "changelog",
    "foot.support": "buy me a coffee",
    "pv.badge": "100% local · free",
    "pv.title": "Restore your Snapchat memories",
    "pv.sub": "Drop the zip or folder, hit start.",
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
    "nav.coffee": "Offrez-moi un café",
    "nav.coffeeAria": "Offrez-moi un café",
    "hero.t1": "Enregistrez vos",
    "hero.t2": "souvenirs à",
    "hero.t3": "la bonne date.",
    "hero.sub.pre":
      "Snapchat date votre export au jour du téléchargement. GetSnapBack restaure les ",
    "hero.sub.strong": "vraies dates",
    "hero.sub.post":
      " de chaque photo et vidéo — EXIF pour les photos, QuickTime pour les vidéos — et réincruste textes, dessins et stickers. Prêt pour Photos d'Apple et Google Photos.",
    "dl.for": "Télécharger pour",
    "dl.all": "toutes les versions →",
    "app.eyebrow": "l'app",
    "app.title": "Pas de menus. Pas de réglages. Juste la fenêtre.",
    "app.body":
      "Pointez-la vers votre export, regardez le compteur grimper, et récupérez un dossier propre. C'est tout.",
    "app.link": "Télécharger",
    "what.eyebrow": "ce qu'elle fait",
    "what.f1.t": "Vraies dates",
    "what.f1.b":
      "Glissez votre export Snapchat, un zip ou un dossier décompressé. Chaque photo récupère sa date d'origine, réinscrite dans l'EXIF, pour que Photos et Google Photos la rangent là où elle a vraiment eu lieu.",
    "what.f2.t": "Overlays réincrustés",
    "what.f2.b":
      "Légendes, dessins et stickers ressortent en fichiers séparés. L'app les repeint sur l'image source, au pixel près.",
    "what.f3.t": "Reste en local",
    "what.f3.b":
      "Aucun envoi. Aucun compte. Aucune télémétrie. L'app lit votre dossier, en écrit un propre à côté, terminé.",
    "export.eyebrow": "première étape",
    "export.title": "Récupérez votre export Snapchat",
    "export.body":
      "Ouvrez la page Snapchat ci-dessous et cochez uniquement « Exporter mes souvenirs ». Laissez le reste décoché.",
    "export.cta": "Ouvrir la page d'export Snapchat",
    "export.note":
      "Snapchat envoie un lien de téléchargement par email, généralement en moins d'une heure (parfois jusqu'à 24h). Au-dessus de 2 Go, l'archive est découpée en plusieurs ZIP : décompressez-les tous dans le même dossier avant de lancer GetSnapBack.",
    "cta.t1": "Gratuit.",
    "cta.t2": "Toutes les plateformes.",
    "cta.sub": "Apple Silicon, Windows 10/11, Linux AppImage.",
    "cta.note.intro": "Premier lancement :",
    "cta.note.mac.label": "macOS",
    "cta.note.mac.pre":
      " — essayez d'ouvrir l'app une première fois, puis dans Réglages Système → Confidentialité et sécurité, cliquez sur ",
    "cta.note.mac.em": "Ouvrir quand même",
    "cta.note.mac.post":
      ". macOS vous laissera lancer GetSnapBack normalement les fois suivantes.",
    "cta.note.win.label": "Windows",
    "cta.note.win.pre":
      " — SmartScreen peut afficher une alerte ; cliquez sur ",
    "cta.note.win.em":
      "informations complémentaires → exécuter quand même",
    "cta.note.win.post": ".",
    "faq.eyebrow": "questions fréquentes",
    "faq.title": "Souvenirs Snapchat, mauvaises dates — réglé",
    "faq.q1.q": "Pourquoi mes souvenirs Snapchat sont tous datés de 2024 au lieu de l'année où je les ai pris ?",
    "faq.q1.a": "Snapchat met sur chaque fichier exporté la date du téléchargement, pas la date de prise de vue. La vraie date n'est conservée que dans le préfixe du nom de fichier (AAAA-MM-JJ). GetSnapBack lit ce préfixe et le réécrit dans l'EXIF (photos) et dans les atomes QuickTime (vidéos), pour que Photos d'Apple et Google Photos classent vos souvenirs à l'année où ils ont vraiment eu lieu.",
    "faq.q2.q": "Mes photos et vidéos sont-elles envoyées quelque part ?",
    "faq.q2.a": "Non. GetSnapBack tourne entièrement sur votre machine. Aucun compte, aucun serveur, aucune télémétrie. L'app lit votre dossier et en écrit un propre à côté. Vous pouvez l'utiliser hors-ligne.",
    "faq.q3.q": "Snapchat a coupé mon export en plusieurs ZIP — l'app gère ?",
    "faq.q3.a": "Oui. Au-delà de 2 Go, Snapchat découpe l'archive en plusieurs ZIP. Déposez-les tous ensemble sur l'app (ou décompressez-les dans un même dossier) et GetSnapBack les fusionne dans une seule sortie propre.",
    "faq.q4.q": "Et les overlays — les textes, légendes et stickers que j'avais dessinés ?",
    "faq.q4.a": "Snapchat les exporte comme des PNG transparents séparés, à côté de chaque photo ou vidéo. GetSnapBack les réincruste sur le média d'origine, pour que le résultat importé dans Photos ou Google Photos ressemble exactement au Snap d'origine.",
    "faq.q5.q": "Ça remet aussi la bonne date sur les vidéos ?",
    "faq.q5.a": "Oui. Les dates photo vont dans l'EXIF, les dates vidéo dans les atomes QuickTime (creation_time et com.apple.quicktime.creationdate). Photos et Google Photos lisent ces deux champs.",
    "faq.q6.q": "C'est vraiment gratuit ?",
    "faq.q6.a": "Oui — gratuit et open source sous licence MIT. Si l'app vous a aidé, il y a un bouton Buy Me a Coffee, mais pas de version payante et aucune en projet.",

    "support.eyebrow": "soutien",
    "support.title": "Cette app vous a aidé ?",
    "support.body":
      "GetSnapBack est gratuit et le restera. Si elle vous a aidé, un café m'aide à continuer.",
    "support.cta": "Offrez-moi un café",
    "foot.source": "source",
    "foot.bug": "signaler un bug",
    "foot.changelog": "journal des versions",
    "foot.support": "m'offrir un café",
    "pv.badge": "100% local · gratuit",
    "pv.title": "Restaurez vos souvenirs Snapchat",
    "pv.sub": "Glissez le zip ou le dossier, lancez.",
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
