<script>
  import WasmCanvas from './WasmCanvas.svelte';

  let { name, width = 720, height = 540 } = $props();

  const WORLD_WIDTH = 2560;

  let x = $state(Math.round(WORLD_WIDTH / 2));
  let iframeEl = $state(null);
  // While the input is focused, the user is typing into it — don't
  // overwrite what they're typing with the value read back from the
  // WASM module on the next poll.
  let focused = false;

  function applyPosition() {
    const exports = iframeEl?.contentWindow?.wasm_exports;
    if (!exports?.set_x) return;
    exports.set_x(x);
  }

  function readPosition() {
    if (focused) return;
    const exports = iframeEl?.contentWindow?.wasm_exports;
    if (!exports?.get_x) return;
    x = Math.round(exports.get_x());
  }

  $effect(() => {
    // Re-run whenever x changes, whether typed by the user or just read
    // back from the WASM module — pushing the same value back is a
    // harmless no-op, and pushing a newly typed one is the whole point.
    void x;
    applyPosition();
  });

  $effect(() => {
    // x increases on its own inside the WASM module every frame, so
    // unlike the coordinates demo (which only ever changes in response
    // to this component's own inputs), this one has to keep polling to
    // reflect that back into the input.
    const poll = setInterval(readPosition, 100);
    return () => clearInterval(poll);
  });
</script>

<div class="mutability-demo">
  <div class="controls" style={`max-width: ${width}px;`}>
    <label>
      <span class="label-text">x</span>
      <input
        type="number"
        bind:value={x}
        step="1"
        onfocus={() => (focused = true)}
        onblur={() => (focused = false)}
      />
    </label>
  </div>

  <WasmCanvas {name} {width} {height} bind:iframeEl />
</div>

<style>
  .mutability-demo {
    margin-block: 1rem;
  }

  .controls {
    display: flex;
    align-items: center;
    gap: 1.5rem;
    margin-bottom: 0.75rem;
  }

  .controls label {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    margin: 0;
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
</style>
