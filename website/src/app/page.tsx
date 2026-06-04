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

  return (
    <>
      <script
        type="application/ld+json"
        dangerouslySetInnerHTML={{ __html: JSON.stringify(jsonLd) }}
      />
      <Site assets={assets} />
    </>
  );
}
