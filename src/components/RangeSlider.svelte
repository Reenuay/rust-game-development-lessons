<script>
  // Same shape as NumberField.svelte, but a <input type="range"> — for
  // demos where dragging a slider makes more sense than typing a number
  // (e.g. a 0..1 fraction like lerp's t). Bind both a slider and a
  // NumberField to the same $state value in a demo and they stay in
  // sync automatically, with no extra wrapper component needed.
  let { label, value = $bindable(), min, max, step, onfocus, onblur } = $props();
</script>

<label>
  <span class="label-text">{label}</span>
  <input type="range" bind:value {min} {max} {step} {onfocus} {onblur} />
</label>

<style>
  label {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    margin: 0;
    line-height: 1;
    font-family: var(--__sl-font-mono, ui-monospace, monospace);
    font-size: var(--sl-text-sm);
    color: var(--sl-color-text);
  }

  input {
    width: 10rem;
    accent-color: var(--sl-color-accent);
  }
</style>
