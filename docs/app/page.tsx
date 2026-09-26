import type { Metadata } from "next";
import Link from "next/link";

export const metadata: Metadata = {
  title: "Documentation",
  description:
    "Guides, API references, and examples to help you build with our platform.",
  openGraph: {
    title: "Documentation",
    description:
      "Guides, API references, and examples to help you build with our platform.",
    type: "website",
    url: "/",
  },
};

export default function HomePage() {
  return (
    <main>
      <h1>Documentation</h1>
      <p>
        Guides, API references, and examples to help you build with our
        platform.
      </p>
      <p>
        <Link href="/docs">Browse the docs</Link>
      </p>
    </main>
  );
}
