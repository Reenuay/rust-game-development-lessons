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

// Builds a code string from a mix of plain text and "marked" values,
// while recording exactly where each marked value landed (1-indexed
// line, 0-indexed column, length) — used to highlight the number(s)
// in a live code panel that a given input field controls, when
// hovering/focusing it (see valueMarkTransformer below).
//
// This exists instead of just searching the generated code for each
// value's text after the fact because that's ambiguous: two fields
// can easily hold the same value at the same time (e.g. x and y both
// defaulting to 0.5), and a plain text search would highlight
// whichever one appears first for both. Recording the exact position
// while building the string sidesteps that entirely.
//
// A field can legitimately appear more than once (e.g. RelativeOffsetDemo's
// `radius` is used both for the big circle and inside the offset math) —
// marks is field -> array of positions, not a single position.
//
// `parts` is a mix of plain strings and { field, value } markers, e.g.:
//   markedCode(['let x = ', { field: 'x', value: '0.5' }, ';'])
export function markedCode(parts) {
  let code = '';
  let line = 1;
  let col = 0;
  const marks = {};

  const advance = (text) => {
    for (const ch of text) {
      if (ch === '\n') {
        line += 1;
        col = 0;
      } else {
        col += 1;
      }
    }
  };

  for (const part of parts) {
    if (typeof part === 'string') {
      code += part;
      advance(part);
    } else {
      const text = String(part.value);
      (marks[part.field] ??= []).push({ line, col, length: text.length });
      code += text;
      advance(text);
    }
  }

  return { code, marks };
}

// Wraps whichever token(s) fall inside a mark's (line, col) range in
// <span data-field="...">, using Shiki's own per-token position info
// instead of text matching — reliable even when the marked value and
// some unrelated bit of code happen to look identical.
export function valueMarkTransformer(marks) {
  const entries = Object.entries(marks ?? {});
  if (entries.length === 0) return {};
  return {
    span(hast, line, col, _lineEl, token) {
      const tokenEnd = col + token.content.length;
      for (const [field, ranges] of entries) {
        for (const mark of ranges) {
          const markEnd = mark.col + mark.length;
          const overlaps = col < markEnd && tokenEnd > mark.col;
          if (line === mark.line && overlaps) {
            hast.properties['data-field'] = field;
            return;
          }
        }
      }
    },
  };
}

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

export function highlightRust(highlighter, code, marks) {
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
    transformers: [rustLineShapeTransformer, valueMarkTransformer(marks)],
  });
}
