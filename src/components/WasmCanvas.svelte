<script>
  // Shared by every embedded WASM demo on the site: the iframe pointing
  // at the compiled example, the fullscreen toggle, and the sizing/
  // letterboxing that goes with it. Demo-specific extras (like the x/y
  // inputs in the coordinates lesson) are passed in as children, and
  // `iframeEl` is exposed via bind: so a parent can reach into the
  // iframe's window (e.g. to call its exported WASM functions).
  let { name, width = 720, height = 540, iframeEl = $bindable(null), children } = $props();

  const base = import.meta.env.BASE_URL.replace(/\/$/, '');
  const src = `${base}/wasm-examples/${name}/`;

  let containerEl;
  let isFullscreen = $state(false);

  $effect(() => {
    const onFullscreenChange = () => {
      isFullscreen = document.fullscreenElement === containerEl;
    };
    document.addEventListener('fullscreenchange', onFullscreenChange);
    document.addEventListener('webkitfullscreenchange', onFullscreenChange);
    return () => {
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
  class="wasm-canvas"
  bind:this={containerEl}
  style={`--demo-width: ${width}px; --demo-ratio: ${(width / height).toFixed(6)};`}
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

  {@render children?.()}
</div>

<style>
  .wasm-canvas {
    width: 100%;
    max-width: var(--demo-width);
  }

  .canvas-wrap {
    position: relative;
    width: 100%;
    aspect-ratio: var(--demo-ratio);
    border: 1px solid var(--sl-color-hairline);
    border-radius: 0.5rem;
    overflow: hidden;
    background: black;
  }

  /* Letterboxed instead of a plain 100vw/100vh stretch: that distorted
     the demo (a circle into an ellipse, etc.) on any screen whose
     aspect ratio didn't happen to match the demo's own. Fit within the
     screen at the same ratio instead, centered, with the (already
     black) canvas background doubling as the letterbox bars. */
  :global(.wasm-canvas:fullscreen) {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 0.75rem;
    width: 100vw;
    height: 100vh;
    max-width: none;
    background: black;
  }

  :global(.wasm-canvas:fullscreen .canvas-wrap) {
    width: min(100vw, calc(100vh * var(--demo-ratio)));
    height: min(100vh, calc(100vw / var(--demo-ratio)));
    max-width: 100vw;
    max-height: 100vh;
    border: 0;
    border-radius: 0;
  }

  /* Any extra content passed in as children (e.g. the coordinates
     lesson's x/y inputs) would otherwise compete with the canvas for
     space in the fullscreen flex column, shrinking it below the
     letterbox size computed above (flex items shrink by default) — so
     it's hidden there instead of reserving space for it. */
  :global(.wasm-canvas:fullscreen > :not(.canvas-wrap)) {
    display: none;
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
    /* Starlight's prose CSS adds margin-top between adjacent content
       elements (for normal paragraph/list spacing); it doesn't know
       this button is one of two absolutely-positioned corner controls,
       not stacked text, so it was pushing it down and throwing off the
       top/right offsets that are meant to match. */
    margin: 0;
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
</style>
