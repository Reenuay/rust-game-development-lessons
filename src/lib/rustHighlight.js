// Shared by every live demo's code panel (CodePanel.svelte).
//
// This reuses Starlight's own Expressive Code class names (ec-line,
// code, ...) on purpose, so the live panels are pixel-identical to
// the site's real static code blocks and never drift from whatever
// Starlight's theme currently looks like. The cost: Astro's Expressive
// Code integration only injects its stylesheet/script into a page
// that has at least one *real*, statically-rendered ```code``` fence
// for it to process — every .mdx file that embeds a live demo using
// this module MUST also contain at least one ordinary fenced code
// block elsewhere on the page (which, in practice, every lesson
// already has as part of its normal "Код" section). If a page is ever
// edited down to zero static fences, every live panel on it silently
// loses its styling and copy button — this bit us once already
// (see git history: "Fix offset not tracked as a dependency, and
// missing Expressive Code assets").
import { createHighlighterCore } from 'shiki/core';
import { createJavaScriptRegexEngine } from 'shiki/engine/javascript';

export function escapeHtml(s) {
  return s.replace(/[&<>"]/g, (c) => ({ '&': '&amp;', '<': '&lt;', '>': '&gt;', '"': '&quot;' })[c]);
}

// Cheap synchronous fallback shown before the async highlighter below
// has loaded — same ec-line/code DOM shape as the real highlighted
// output, just without color spans, so there's always real code on
// screen instead of a blank panel.
export function plainCodeHtml(code) {
  const lines = code.replace(/\n$/, '').split('\n');
  const body = lines
    .map((line) => `<div class="ec-line"><div class="code">${line.length ? escapeHtml(line) : '\n'}</div></div>`)
    .join('');
  return `<pre data-language="rust"><code>${body}</code></pre>`;
}

// Reshapes Shiki's default per-line <span> output into the exact
// ec-line/code DOM shape Expressive Code's own static blocks use, so
// this live snippet is styled by the exact same sitewide CSS instead
// of a hand-rolled lookalike.
export const rustLineShapeTransformer = {
  pre(node) {
    node.properties = { 'data-language': 'rust' };
  },
  code(node) {
    node.properties = {};
  },
  line(node) {
    // A truly empty line has no children, and an empty block box has
    // no line box to size itself by — it collapses to 0 height. Give
    // it the same single "\n" text placeholder Expressive Code's own
    // empty lines have.
    const children = node.children.length > 0 ? node.children : [{ type: 'text', value: '\n' }];
    return {
      type: 'element',
      tagName: 'div',
      properties: { class: 'ec-line' },
      children: [
        {
          type: 'element',
          tagName: 'div',
          properties: { class: 'code' },
          children,
        },
      ],
    };
  },
  // Shiki's default renderer joins line elements with a literal "\n"
  // text node (its usual per-line wrapper is an inline <span>, which
  // needs that newline to actually break the line). Our .ec-line divs
  // are block-level and don't need it, but inside <pre> that stray
  // newline is still whitespace:pre-preserved — showing up as a whole
  // extra blank line after every line, exactly doubling the vertical
  // rhythm the static code blocks have. Strip them.
  postprocess(html) {
    return html.replace(/\n(?=<div class="ec-line")/g, '');
  },
};

let highlighterPromise;

// One highlighter instance shared by every demo on a page, instead of
// each component loading its own copy of the theme/language data.
export function loadRustHighlighter() {
  if (!highlighterPromise) {
    highlighterPromise = createHighlighterCore({
      themes: [import('@shikijs/themes/night-owl'), import('@shikijs/themes/night-owl-light')],
      langs: [import('@shikijs/langs/rust')],
      engine: createJavaScriptRegexEngine(),
    });
  }
  return highlighterPromise;
}

export function highlightRust(highlighter, code) {
  // Our codeFor() template literals always end with a trailing "\n"
  // right before the closing backtick — Shiki turns that into a
  // genuine extra empty line at the bottom of the highlighted output.
  // Markdown's own fenced code blocks don't have that problem (their
  // content doesn't include a trailing newline to begin with), which
  // is why only the live panels grew the extra blank line. Strip it so
  // both match.
  const trimmedCode = code.replace(/\n+$/, '');

  // Two colors per token (--0 for dark, --1 for light) instead of one,
  // matching Expressive Code's own dual-theme output exactly (down to
  // the variable names) so its existing CSS picks the right one off
  // the site's <html data-theme> — no separate light/dark render pass
  // or theme-change listener needed here.
  return highlighter.codeToHtml(trimmedCode, {
    lang: 'rust',
    themes: { '0': 'night-owl', '1': 'night-owl-light' },
    defaultColor: false,
    cssVariablePrefix: '--',
    transformers: [rustLineShapeTransformer],
  });
}
