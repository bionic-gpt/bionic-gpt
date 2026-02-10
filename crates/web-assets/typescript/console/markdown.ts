import DOMPurify from "dompurify";
import { marked } from "marked";

const SAFE_PROTOCOLS = new Set(["http:", "https:", "mailto:"]);

function isSafeHref(href: string): boolean {
    const trimmed = href.trim();
    if (!trimmed) return false;
    if (trimmed.startsWith("/") || trimmed.startsWith("#")) return true;

    try {
        const parsed = new URL(trimmed, window.location.origin);
        return SAFE_PROTOCOLS.has(parsed.protocol);
    } catch (_e) {
        return false;
    }
}

function hardenLinks(html: string): string {
    const template = document.createElement("template");
    template.innerHTML = html;

    template.content.querySelectorAll("a").forEach((anchor) => {
        const href = anchor.getAttribute("href") || "";
        if (!isSafeHref(href)) {
            anchor.removeAttribute("href");
            return;
        }

        // Add common hardening attributes for all remaining links.
        anchor.setAttribute("rel", "noopener noreferrer nofollow");
        anchor.setAttribute("target", "_blank");
    });

    return template.innerHTML;
}

export function renderMarkdownSafe(src: string): string {
    const rawHtml = marked.parse(src, {
        async: false,
        gfm: true,
        breaks: true,
    }) as string;

    const sanitized = DOMPurify.sanitize(rawHtml, {
        USE_PROFILES: { html: true },
    });

    return hardenLinks(sanitized);
}
