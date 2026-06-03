import type { Metadata } from "next";
import "./globals.css";

const SITE_URL = "https://getsnapback.vercel.app";

export const metadata: Metadata = {
  metadataBase: new URL(SITE_URL),
  title: "GetSnapBack · Save your Snapchat memories on the right date",
  description:
    "A free desktop app that exports your Snapchat memories with their real dates, baking the overlays you drew, captioned and stickered right back in. Free, local, three platforms.",
  icons: {
    icon: [
      { url: "/icon.png", sizes: "any" },
      { url: "/favicon.ico" },
    ],
    apple: "/icon.png",
  },
  openGraph: {
    title: "GetSnapBack",
    description: "Save your Snapchat memories on the right date. Free, local app.",
    url: SITE_URL,
    siteName: "GetSnapBack",
    images: [{ url: "/icon-large.png", width: 1254, height: 1254 }],
    type: "website",
  },
  twitter: {
    card: "summary_large_image",
    title: "GetSnapBack",
    description: "Save your Snapchat memories on the right date.",
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
