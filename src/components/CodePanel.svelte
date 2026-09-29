<script>
  // Renders a live-updating code snippet using Starlight's own
  // Expressive Code markup/classes (.expressive-code/.frame/.header/
  // .copy/...) so it's pixel-identical to the site's real static code
  // blocks and its copy button is wired up by Starlight's own script —
  // see the big comment at the top of ../lib/rustHighlight.js for the
  // tradeoff that comes with reusing them (this page must always keep
  // at least one real, static ```code``` fence somewhere on it).
  let { title = 'src/main.rs', html, code } = $props();

  // Expressive Code's copy button reads the text to copy from a
  // data-code attribute, encoding newlines as \x7f instead of literal
  // "\n" (HTML attribute values collapse literal newlines to spaces).
  let copyCode = $derived(code.replace(/\n/g, '\x7f'));
</script>

<div class="expressive-code">
  <figure class="frame has-title not-content">
    <figcaption class="header"><span class="title">{title}</span></figcaption>
    {@html html}
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
