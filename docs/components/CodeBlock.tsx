"use client";

import { useState, type ReactNode } from "react";

interface CodeBlockProps {
  children: ReactNode;
  /** Optional filename / language label shown in the header bar. */
  title?: string;
  /** Language label shown in the header bar. */
  language?: string;
  /** Raw text to copy; falls back to the rendered children when omitted. */
  code?: string;
}

/**
 * A titled code container with a copy button. For fenced code inside MDX, the
 * plain `pre`/`code` mapping is used; this component is for callouts where a
 * header and copy affordance add value.
 */
export function CodeBlock({ children, title, language, code }: CodeBlockProps) {
  const [copied, setCopied] = useState(false);
  const [failed, setFailed] = useState(false);

  async function copy() {
    const text = code ?? (typeof children === "string" ? children : "");
    if (!text) return;
    try {
      await navigator.clipboard.writeText(text);
      setFailed(false);
      setCopied(true);
      setTimeout(() => setCopied(false), 1400);
    } catch {
      setFailed(true);
      setTimeout(() => setFailed(false), 1400);
    }
  }

  return (
    <div className="my-6 overflow-hidden rounded-xl border border-white/10 bg-[#0a0c11]">
      <div className="flex items-center justify-between border-b border-white/5 px-4 py-2">
        <span className="flex items-center gap-2">
          <span className="font-mono text-xs text-base-300">{title ?? "code"}</span>
          {language ? (
            <span className="rounded border border-white/10 px-1.5 py-0.5 font-mono text-[10px] uppercase tracking-wide text-base-400">
              {language}
            </span>
          ) : null}
        </span>
        <button
          type="button"
          onClick={copy}
          aria-label={copied ? "Copied to clipboard" : "Copy code to clipboard"}
          className="text-xs font-medium text-base-300 transition-colors hover:text-brand-400"
        >
          {copied ? "Copied" : "Copy"}
        </button>
      </div>
      <pre className="overflow-x-auto p-4 text-sm leading-relaxed">
        <code className="font-mono text-base-100">{children}</code>
      </pre>
      <span role="status" aria-live="polite" className="sr-only">
        {copied ? "Code copied to clipboard" : failed ? "Copy failed" : ""}
      </span>
    </div>
  );
}

export default CodeBlock;
