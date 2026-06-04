import type { Metadata } from "next";
import "./globals.css";

const SITE_URL = "https://getsnapback.vercel.app";

export const metadata: Metadata = {
  metadataBase: new URL(SITE_URL),
  title: "GetSnapBack — Fix wrong dates on your Snapchat memories export",
  description:
    "Your Snapchat memories export dates everything to today instead of when the photos were taken. GetSnapBack reads the real date from each filename, rewrites EXIF and QuickTime metadata, composites the overlays (captions, doodles, stickers) back onto each photo and video, and gives you a clean folder ready for Apple Photos or Google Photos. Free, 100% local, macOS / Windows / Linux.",
  keywords: [
    "snapchat memories wrong date",
    "snapchat memories export",
    "snapchat memories real date",
    "snapchat export dated 2024",
    "fix snapchat memories EXIF",
    "snapchat memories overlays",
    "restore snapchat memory timestamp",
    "snapchat memories import apple photos",
    "snapchat memories import google photos",
    "snapchat export ZIP date fix",
    "souvenirs snapchat mauvaise date",
    "date snapchat fausse export",
    "récupérer vraie date souvenirs snapchat",
  ],
  authors: [{ name: "Evan Benhamou", url: "https://github.com/evanbnm" }],
  creator: "Evan Benhamou",
  robots: {
    index: true,
    follow: true,
    googleBot: { index: true, follow: true, "max-snippet": -1, "max-image-preview": "large" },
  },
  alternates: { canonical: SITE_URL },
  icons: {
    icon: [
      { url: "/icon.png", sizes: "any" },
      { url: "/favicon.ico" },
    ],
    apple: "/icon.png",
  },
  openGraph: {
    title: "GetSnapBack — Fix wrong dates on your Snapchat memories",
    description:
      "Free local desktop app that restores the real dates on every photo and video from your Snapchat memories export, so they sort properly in Apple Photos or Google Photos.",
    url: SITE_URL,
    siteName: "GetSnapBack",
    images: [{ url: "/icon-large.png", width: 1254, height: 1254 }],
    type: "website",
  },
  twitter: {
    card: "summary_large_image",
    title: "GetSnapBack — Fix wrong dates on Snapchat memories",
    description:
      "Restore the real dates on every photo and video from your Snapchat memories export. Free, 100% local.",
    images: ["/icon-large.png"],
  },
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en">
      <body>{children}</body>
    </html>
  );
}
