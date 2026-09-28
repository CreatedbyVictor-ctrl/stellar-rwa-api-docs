import type { MDXComponents } from "mdx/types";
import Link from "next/link";
import { CalloutBox } from "@/components/CalloutBox";
import { CodeBlock } from "@/components/CodeBlock";
import { ApiEndpoint } from "@/components/ApiEndpoint";
import { ErrorCodeTable } from "@/components/ErrorCodeTable";
import { createHeading } from "@/components/HeadingAnchor";

const h2 = createHeading(2);
const h3 = createHeading(3);
const h4 = createHeading(4);
const h5 = createHeading(5);
const h6 = createHeading(6);

/**
 * Global MDX component map. Custom components are available to every `.mdx`
 * page without per-file imports, internal links use the Next.js router, and
 * h2–h6 headings get a stable slug id plus a keyboard-accessible "#" anchor.
 */
export function useMDXComponents(components: MDXComponents): MDXComponents {
  return {
    a: ({ href = "", children, ...props }) => {
      const isInternal = href.startsWith("/") || href.startsWith("#");
      if (isInternal) {
        return (
          <Link href={href} {...props}>
            {children}
          </Link>
        );
      }
      return (
        <a href={href} target="_blank" rel="noopener noreferrer" {...props}>
          {children}
        </a>
      );
    },
    h2,
    h3,
    h4,
    h5,
    h6,
    CalloutBox,
    CodeBlock,
    ApiEndpoint,
    ErrorCodeTable,
    ...components,
  };
}
