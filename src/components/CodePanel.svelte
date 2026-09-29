<script>
  // Renders a live-updating code snippet using Starlight's own
  // Expressive Code markup/classes (.expressive-code/.frame/.header/
  // .copy/...) so it's pixel-identical to the site's real static code
  // blocks and its copy button is wired up by Starlight's own script —
  // see the big comment at the top of ../lib/rustHighlight.js for the
  // tradeoff that comes with reusing them (this page must always keep
  // at least one real, static ```code``` fence somewhere on it).
  let { title = 'src/main.rs', html, code, highlightField = null } = $props();

  // Expressive Code's copy button reads the text to copy from a
  // data-code attribute, encoding newlines as \x7f instead of literal
  // "\n" (HTML attribute values collapse literal newlines to spaces).
  let copyCode = $derived(code.replace(/\n/g, '\x7f'));

  // `html` embeds a data-field="..." attribute on whichever token(s)
  // a given input field controls (see valueMarkTransformer in
  // ../lib/rustHighlight.js) — {@html} content is opaque to Svelte, so
  // rather than fight it, just walk the rendered DOM directly and
  // toggle the shared .demo-pulse class (src/styles/pulse.css) onto
  // whichever one matches the currently hovered/focused field.
  let bodyEl;

  $effect(() => {
    if (!bodyEl) return;
    // `html` isn't read directly below, but the marked elements it
    // produces are what this effect re-scans — without reading it
    // here, Svelte has no way to know this effect depends on it, and
    // a code update wouldn't re-run the highlight toggle.
    void html;
    for (const el of bodyEl.querySelectorAll('[data-field]')) {
      el.classList.toggle('demo-pulse', el.dataset.field === highlightField);
    }
  });
</script>

<div class="expressive-code">
  <figure class="frame has-title not-content">
    <figcaption class="header"><span class="title">{title}</span></figcaption>
    <div bind:this={bodyEl}>{@html html}</div>
    <div class="copy">
      <div aria-live="polite"></div>
      <button title="Копировать" data-copied="Скопировано!" data-code={copyCode}><div></div></button>
    </div>
  </figure>
</div>

<style>
  .expressive-code {
    margin-top: 0.75rem;
  }
</style>
