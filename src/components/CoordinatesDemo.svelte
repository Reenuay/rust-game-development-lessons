<script>
  import { createHighlighterCore } from 'shiki/core';
  import { createJavaScriptRegexEngine } from 'shiki/engine/javascript';

  let { name, width = 720, height = 540 } = $props();

  const base = import.meta.env.BASE_URL.replace(/\/$/, '');
  const src = `${base}/wasm-examples/${name}/`;

  let x = $state(Math.round(width / 2));
  let y = $state(Math.round(height / 2));
  let codeHtml = $state('');
  let isFullscreen = $state(false);

  let iframeEl;
  let containerEl;
  let highlighter;
  let ready = false;

  function codeFor(x, y) {
    return `use macroquad::prelude::*;

#[macroquad::main("Координаты")]
async fn main() {
    loop {
        clear_background(BLACK);

        draw_circle(${x}.0, ${y}.0, 20.0, YELLOW);

        next_frame().await;
    }
}
`;
  }

  // Reshapes Shiki's default output into the same DOM shape Starlight's
  // Expressive Code uses for its static code blocks (.expressive-code >
  // .ec-line > .code), so this live snippet is styled by the exact same
  // sitewide CSS instead of a hand-rolled lookalike.
  const ecShapeTransformer = {
    pre(node) {
      node.properties = { 'data-language': 'rust' };
    },
    code(node) {
      node.properties = {};
    },
    line(node) {
      return {
        type: 'element',
        tagName: 'div',
        properties: { class: 'ec-line' },
        children: [
          {
            type: 'element',
            tagName: 'div',
            properties: { class: 'code' },
            children: node.children,
          },
        ],
      };
    },
  };

  function renderCode() {
    if (!highlighter) return;
    // Two colors per token (--0 for dark, --1 for light) instead of one,
    // matching Expressive Code's own dual-theme output exactly (down to
    // the variable names) so its existing CSS picks the right one off
    // the site's <html data-theme> — no separate light/dark render pass
    // or theme-change listener needed here.
    codeHtml = highlighter.codeToHtml(codeFor(x, y), {
      lang: 'rust',
      themes: { '0': 'night-owl', '1': 'night-owl-light' },
      defaultColor: false,
      cssVariablePrefix: '--',
      transformers: [ecShapeTransformer],
    });
  }

  function applyPosition() {
    const exports = iframeEl?.contentWindow?.wasm_exports;
    if (!exports?.set_x || !exports?.set_y) return;
    exports.set_x(x);
    exports.set_y(y);
    ready = true;
  }

  $effect(() => {
    // Re-run whenever x or y changes.
    void x;
    void y;
    applyPosition();
    renderCode();
  });

  $effect(() => {
    let cancelled = false;

    createHighlighterCore({
      themes: [import('@shikijs/themes/night-owl'), import('@shikijs/themes/night-owl-light')],
      langs: [import('@shikijs/langs/rust')],
      engine: createJavaScriptRegexEngine(),
    }).then((h) => {
      if (cancelled) return;
      highlighter = h;
      renderCode();
    });

    // The WASM module loads asynchronously inside the iframe; poll until
    // its exports are ready, then apply the current position once.
    const readyPoll = setInterval(() => {
      if (ready) {
        clearInterval(readyPoll);
        return;
      }
      applyPosition();
    }, 100);

    const onFullscreenChange = () => {
      isFullscreen = document.fullscreenElement === containerEl;
    };
    document.addEventListener('fullscreenchange', onFullscreenChange);
    document.addEventListener('webkitfullscreenchange', onFullscreenChange);

    return () => {
      cancelled = true;
      clearInterval(readyPoll);
      document.removeEventListener('fullscreenchange', onFullscreenChange);
      document.removeEventListener('webkitfullscreenchange', onFullscreenChange);
    };
  });

  function toggleFullscreen() {
    const request = containerEl.requestFullscreen?.bind(containerEl) ?? containerEl.webkitRequestFullscreen?.bind(containerEl);
    const exit = document.exitFullscreen?.bind(document) ?? document.webkitExitFullscreen?.bind(document);
    if (!request || !exit) return;
    if (document.fullscreenElement === containerEl) exit();
    else request();
  }
</script>

<div
  class="coordinates-demo"
  bind:this={containerEl}
  style={`--demo-width: ${width}px; --demo-aspect-ratio: ${width} / ${height};`}
>
  <div class="canvas-wrap">
    <iframe bind:this={iframeEl} {src} loading="lazy" title={`Живой пример: ${name}`}></iframe>
    <button type="button" class="fullscreen-toggle" onclick={toggleFullscreen} aria-label={isFullscreen ? 'Выйти из полноэкранного режима' : 'Развернуть на весь экран'}>
      {#if isFullscreen}
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M9 3v4a2 2 0 0 1-2 2H3m18 0h-4a2 2 0 0 1-2-2V3m0 18v-4a2 2 0 0 1 2-2h4M3 15h4a2 2 0 0 1 2 2v4" />
        </svg>
      {:else}
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M8 3H5a2 2 0 0 0-2 2v3m18 0V5a2 2 0 0 0-2-2h-3m0 18h3a2 2 0 0 0 2-2v-3M3 16v3a2 2 0 0 0 2 2h3" />
        </svg>
      {/if}
    </button>
  </div>

  <div class="controls">
    <label>
      <span class="label-text">x</span>
      <input type="number" bind:value={x} step="10" />
    </label>
    <label>
      <span class="label-text">y</span>
      <input type="number" bind:value={y} step="10" />
    </label>
  </div>

  <div class="expressive-code">
    <figure class="frame has-title not-content">
      <figcaption class="header"><span class="title">src/main.rs</span></figcaption>
      {@html codeHtml}
    </figure>
  </div>
</div>

<style>
  .coordinates-demo {
    width: 100%;
    max-width: var(--demo-width);
    margin-block: 1rem;
  }

  .canvas-wrap {
    position: relative;
    width: 100%;
    aspect-ratio: var(--demo-aspect-ratio);
    border: 1px solid var(--sl-color-hairline);
    border-radius: 0.5rem;
    overflow: hidden;
    background: black;
  }

  :global(.coordinates-demo:fullscreen .canvas-wrap) {
    max-width: none;
    width: 100vw;
    height: 100vh;
    border-radius: 0;
    aspect-ratio: auto;
  }

  iframe {
    display: block;
    width: 100%;
    height: 100%;
    border: 0;
  }

  .fullscreen-toggle {
    position: absolute;
    top: 0.5rem;
    right: 0.5rem;
    display: flex;
    align-items: center;
    justify-content: center;
    width: 2.25rem;
    height: 2.25rem;
    padding: 0;
    border: 0;
    border-radius: 0.375rem;
    background: rgba(0, 0, 0, 0.55);
    color: white;
    cursor: pointer;
  }

  .fullscreen-toggle:hover {
    background: rgba(0, 0, 0, 0.75);
  }

  .fullscreen-toggle svg {
    width: 1.15rem;
    height: 1.15rem;
  }

  .controls {
    display: flex;
    align-items: center;
    gap: 1.5rem;
    margin-top: 0.75rem;
  }

  .controls label {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    line-height: 1;
    font-family: var(--__sl-font-mono, ui-monospace, monospace);
    font-size: var(--sl-text-sm);
    color: var(--sl-color-text);
  }

  .controls input {
    width: 5.5rem;
    height: 1.75rem;
    padding: 0 0.5rem;
    border: 1px solid var(--sl-color-hairline);
    border-radius: 0.25rem;
    background: var(--sl-color-bg);
    color: var(--sl-color-text);
    font: inherit;
    line-height: 1.75rem;
  }

  .expressive-code {
    margin-top: 0.75rem;
  }
</style>
