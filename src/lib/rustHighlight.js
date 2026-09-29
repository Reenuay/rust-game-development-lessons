// Shared by every live demo's code panel (CodePanel.svelte). Not tied
// to Starlight/Expressive Code in any way — see the big comment in
// CodePanel.svelte for why the panel doesn't reuse Starlight's own
// code-block rendering.
import { createHighlighterCore } from 'shiki/core';
import { createJavaScriptRegexEngine } from 'shiki/engine/javascript';

export function escapeHtml(s) {
  return s.replace(/[&<>"]/g, (c) => ({ '&': '&amp;', '<': '&lt;', '>': '&gt;', '"': '&quot;' })[c]);
}

// Cheap synchronous fallback shown before the async highlighter below
// has loaded — same demo-line/demo-line-code DOM shape as the real
// highlighted output, just without color spans, so there's always
// real code on screen instead of a blank panel.
export function plainCodeHtml(code) {
  const lines = code.replace(/\n$/, '').split('\n');
  const body = lines
    .map((line) => `<div class="demo-line"><div class="demo-line-code">${line.length ? escapeHtml(line) : '\n'}</div></div>`)
    .join('');
  return `<pre>${body}</pre>`;
}

// Reshapes Shiki's default per-line <span> output into simple
// <div class="demo-line">/<div class="demo-line-code"> wrappers —
// our own classes, not Expressive Code's — so CodePanel's own CSS can
// style them without depending on anything Starlight injects.
export const rustLineShapeTransformer = {
  pre(node) {
    node.properties = {};
  },
  code(node) {
    node.properties = {};
  },
  line(node) {
    const children = node.children.length > 0 ? node.children : [{ type: 'text', value: '\n' }];
    return {
      type: 'element',
      tagName: 'div',
      properties: { class: 'demo-line' },
      children: [
        {
          type: 'element',
          tagName: 'div',
          properties: { class: 'demo-line-code' },
          children,
        },
      ],
    };
  },
  // Shiki's default renderer joins line elements with a literal "\n"
  // text node — harmless for its own inline <span> lines, but our
  // .demo-line divs are block-level and don't need it; inside <pre>
  // that stray newline is whitespace:pre-preserved, showing up as an
  // extra blank line after every line.
  postprocess(html) {
    return html.replace(/\n(?=<div class="demo-line")/g, '');
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
  return highlighter.codeToHtml(code, {
    lang: 'rust',
    themes: { '0': 'night-owl', '1': 'night-owl-light' },
    defaultColor: false,
    cssVariablePrefix: '--',
    transformers: [rustLineShapeTransformer],
  });
}
