import type { Metadata } from "next";
import { Inter } from "next/font/google";
import "./globals.css";
import { Sidebar } from "../components/Sidebar";

const inter = Inter({ subsets: ["latin"] });

export const metadata: Metadata = {
  title: {
    default: "Docs",
    template: "%s | Docs",
  },
  description: "Documentation site",
  openGraph: {
    type: "website",
    siteName: "Docs",
    title: "Docs",
    description: "Documentation site",
  },
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en">
      <body className={inter.className}>
        <div className="flex min-h-screen flex-col md:flex-row">
          <Sidebar />
          <main className="min-w-0 flex-1 overflow-x-hidden px-4 py-6 md:px-8">
            {children}
          </main>
        </div>
      </body>
    </html>
  );
}
