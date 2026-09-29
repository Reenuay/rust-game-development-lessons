<script>
  // Shared by every demo's control row (x/y, percent, radius, ...) —
  // was duplicated label+input markup and CSS in seven different
  // components before this.
  let { label, value = $bindable(), min, max, step, onfocus, onblur, onmouseenter, onmouseleave } = $props();
</script>

<label {onmouseenter} {onmouseleave}>
  <span class="label-text">{label}</span>
  <input type="number" bind:value {min} {max} {step} {onfocus} {onblur} />
</label>

<style>
  label {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    /* Starlight's prose CSS adds margin-top between adjacent content
       elements (for normal paragraph/list spacing); it doesn't know
       these <label>s are a flex row, not stacked text, so without
       this it pushes the second one down. */
    margin: 0;
    line-height: 1;
    font-family: var(--__sl-font-mono, ui-monospace, monospace);
    font-size: var(--sl-text-sm);
    color: var(--sl-color-text);
  }

  input {
    width: 5.5rem;
    height: 1.75rem;
    padding: 0 0.5rem;
    /* Plain hairline borders on the inputs were too subtle to notice
       against the page — a friend of the site owner's couldn't spot
       them at a glance. The theme's own accent blue reads as an
       obvious "this is interactive" cue without introducing a new
       color. */
    border: 1.5px solid var(--sl-color-accent);
    border-radius: 0.25rem;
    background: var(--sl-color-bg);
    color: var(--sl-color-text);
    font: inherit;
    line-height: 1.75rem;
  }
</style>
