import Site from "./Site";
import { getReleaseAssets } from "@/lib/release";

const BASE = "https://getsnapback.vercel.app";
const REPO = "https://github.com/evanbnm/getsnapback";

export default async function Page() {
  const assets = await getReleaseAssets();

  // Schema.org SoftwareApplication. Google's Rich Results spec; lets the
  // site qualify for the "software" rich snippet in search results
  // (price, OS, version) and pre-populates app-discovery channels.
  const jsonLd = {
    "@context": "https://schema.org",
    "@type": "SoftwareApplication",
    name: "GetSnapBack",
    description:
      "A free desktop app that exports your Snapchat memories with their real dates and bakes the overlays you drew, captioned and stickered right back in. Free, local, three platforms.",
    applicationCategory: "MultimediaApplication",
    operatingSystem: "macOS, Windows, Linux",
    softwareVersion: assets.tag.replace(/^v/, ""),
    downloadUrl: `${REPO}/releases/latest`,
    license: `${REPO}/blob/main/LICENSE`,
    url: BASE,
    image: `${BASE}/icon-large.png`,
    offers: {
      "@type": "Offer",
      price: "0",
      priceCurrency: "USD",
    },
    author: {
      "@type": "Person",
      name: "Evan Benhamou",
      url: "https://github.com/evanbnm",
    },
  };

  // FAQPage schema — eligible for Google rich results when the questions
  // appear verbatim on the page. Kept short and problem-language to match
  // how people actually search ("snapchat memories wrong date" rather
  // than "EXIF metadata restoration").
  const faqLd = {
    "@context": "https://schema.org",
    "@type": "FAQPage",
    mainEntity: [
      {
        "@type": "Question",
        name: "Why are my Snapchat memories all dated 2024 instead of when I took them?",
        acceptedAnswer: {
          "@type": "Answer",
          text: "Snapchat stamps every exported file with the date of the export, not the date the photo or video was taken. The real date is preserved only in the filename prefix (YYYY-MM-DD). GetSnapBack reads that prefix and writes it back into EXIF (photos) and QuickTime (videos) metadata, so Apple Photos, Google Photos and any file manager will sort your memories under the year they actually happened.",
        },
      },
      {
        "@type": "Question",
        name: "Will my photos and videos be uploaded anywhere?",
        acceptedAnswer: {
          "@type": "Answer",
          text: "No. GetSnapBack runs entirely on your computer. There is no account, no server, no telemetry. The app reads your folder and writes a clean folder next to it. You can run it offline.",
        },
      },
      {
        "@type": "Question",
        name: "Snapchat split my export into several ZIP files. Can GetSnapBack handle that?",
        acceptedAnswer: {
          "@type": "Answer",
          text: "Yes. When a memories archive is over 2 GB Snapchat splits it into several ZIPs. Just drop all of them onto the app together (or extract them into the same folder) and GetSnapBack merges them into one clean output.",
        },
      },
      {
        "@type": "Question",
        name: "What about the overlays — the text I drew, the captions, the stickers?",
        acceptedAnswer: {
          "@type": "Answer",
          text: "Snapchat exports overlays as separate transparent PNG files next to each main photo or video. GetSnapBack composites them back onto the source media so what you see imported into Apple Photos or Google Photos looks exactly like the original Snap.",
        },
      },
      {
        "@type": "Question",
        name: "Does GetSnapBack also fix the date on videos?",
        acceptedAnswer: {
          "@type": "Answer",
          text: "Yes. Photo dates go into EXIF, video dates go into the QuickTime atoms (creation_time and com.apple.quicktime.creationdate). Both Apple Photos and Google Photos read those fields.",
        },
      },
      {
        "@type": "Question",
        name: "Is it free?",
        acceptedAnswer: {
          "@type": "Answer",
          text: "Yes — free and open source under the MIT license. If it helped you, the author has a Buy Me a Coffee link, but there is no paid version and no plan to add one.",
        },
      },
    ],
  };

  return (
    <>
      <script
        type="application/ld+json"
        dangerouslySetInnerHTML={{ __html: JSON.stringify(jsonLd) }}
      />
      <script
        type="application/ld+json"
        dangerouslySetInnerHTML={{ __html: JSON.stringify(faqLd) }}
      />
      <Site assets={assets} />
    </>
  );
}
