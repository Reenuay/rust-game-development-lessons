<script>
  // Renders a live-updating code snippet (title bar + highlighted
  // code + copy button) for the interactive demos.
  //
  // This used to reuse Starlight's own Expressive Code classes
  // (.expressive-code/.frame/.header/.copy/...) so it would pick up
  // Expressive Code's real CSS and copy-button script for free. That
  // broke on the "Относительные координаты" lesson: Astro's Expressive
  // Code integration only injects its stylesheet/script on pages that
  // have at least one *static* ```code``` fence for it to process —
  // remove the last one (as an earlier edit did, to stop showing the
  // same code twice) and every live panel on that page silently loses
  // its styling and its copy button, with nothing in the markup
  // pointing at why.
  //
  // So this component is now fully self-contained: its own classes
  // (prefixed "demo-" so they can never collide with Starlight's own,
  // even on a page that also has real Expressive Code blocks), its
  // own CSS approximating Starlight's current frame/header/copy-button
  // look, and its own copy-to-clipboard handler. It no longer depends
  // on anything Astro decides to inject — it always works, on any
  // page, regardless of what other content is on it.
  //
  // The tradeoff: this CSS is a hand-copied approximation of
  // Starlight's Expressive Code theme as it looks today. If Starlight
  // changes that look significantly, this won't follow along
  // automatically — come back and update the rules below to match.
  let { title = 'src/main.rs', html, code } = $props();

  let copied = $state(false);
  let copyTimer;

  async function copyCode() {
    try {
      await navigator.clipboard.writeText(code);
    } catch {
      return;
    }
    copied = true;
    clearTimeout(copyTimer);
    copyTimer = setTimeout(() => (copied = false), 1500);
  }
</script>

<div class="demo-code-panel">
  <div class="demo-code-header">{title}</div>
  <div class="demo-code-body">{@html html}</div>
  <button type="button" class="demo-code-copy" onclick={copyCode} title="Копировать" aria-label="Копировать код">
    {#if copied}
      <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
        <path d="M20 6 9 17l-5-5" />
      </svg>
    {:else}
      <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
        <rect x="9" y="9" width="13" height="13" rx="2" />
        <path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1" />
      </svg>
    {/if}
  </button>
</div>

<style>
  .demo-code-panel {
    position: relative;
    margin-top: 0.75rem;
    border: 1px solid var(--sl-color-hairline);
    border-radius: 0.5rem;
    overflow: hidden;
    background: var(--sl-color-bg-inline-code);
  }

  .demo-code-header {
    padding: 0.5rem 1rem;
    background: var(--sl-color-gray-6);
    border-bottom: 1px solid var(--sl-color-hairline);
    font-family: var(--__sl-font-mono, ui-monospace, monospace);
    font-size: var(--sl-text-xs, 0.8rem);
    color: var(--sl-color-text);
  }

  .demo-code-body {
    overflow-x: auto;
  }

  .demo-code-body :global(pre) {
    margin: 0;
    padding: 0.75rem 0;
    font-family: var(--__sl-font-mono, ui-monospace, monospace);
    font-size: var(--sl-text-code, 0.875rem);
    line-height: var(--sl-line-height, 1.7);
  }

  .demo-code-body :global(.demo-line) {
    padding-inline: 1rem;
  }

  .demo-code-copy {
    position: absolute;
    top: 0.5rem;
    right: 0.5rem;
    width: 2rem;
    height: 2rem;
    padding: 0.4rem;
    border: 1px solid var(--sl-color-hairline);
    border-radius: 0.3rem;
    background: var(--sl-color-bg-inline-code);
    color: var(--sl-color-text);
    opacity: 0;
    cursor: pointer;
    transition: opacity 0.2s ease;
  }

  .demo-code-panel:hover .demo-code-copy,
  .demo-code-copy:focus-visible {
    opacity: 0.85;
  }

  .demo-code-copy:hover {
    opacity: 1;
  }

  .demo-code-copy svg {
    width: 100%;
    height: 100%;
  }
</style>
