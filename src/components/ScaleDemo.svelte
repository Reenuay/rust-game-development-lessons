<script>
  // Pure math illustration — no macroquad/WASM involved, just an SVG
  // drawn from N and P. Shows the trapezoid you get when you line up
  // the [0, 1] range against the [0, N] range it's meant to scale to:
  // the [0, 1] line only takes up 1/N of the space the [0, N] line
  // does, because both use the same pixels-per-unit scale.
  let { width = 720 } = $props();

  let n = $state(5);
  let p = $state(0.5);

  function clampN(value) {
    return Math.min(50, Math.max(1, Math.round(value)));
  }

  function clampP(value) {
    return Math.min(1, Math.max(0, value));
  }

  $effect(() => {
    n = clampN(n);
    p = clampP(p);
  });

  function formatLabel(value) {
    return Number.isInteger(value) ? `${value}` : value.toFixed(1);
  }

  // SVG viewBox geometry — fixed internal units, scaled to fit `width`
  // via CSS.
  const VW = 600;
  const PAD = 50;
  const DRAW_WIDTH = VW - PAD * 2;
  const Y_TOP = 55;
  const Y_BOTTOM = 150;

  let pxPerUnit = $derived(DRAW_WIDTH / n);
  let topWidth = $derived(pxPerUnit * 1);

  function topX(t) {
    return PAD + t * topWidth;
  }

  function bottomX(t) {
    return PAD + t * DRAW_WIDTH;
  }

  // 0, 0.1, 0.2 ... 1 — the thin projection lines fanning out across
  // the trapezoid.
  const ticks = Array.from({ length: 11 }, (_, i) => i / 10);

  // Keep the dynamic P labels from running off the edge of the SVG at
  // the extremes (P near 0 or 1).
  function clampLabelX(x) {
    return Math.min(VW - PAD + 20, Math.max(PAD - 20, x));
  }
</script>

<div class="scale-demo">
  <svg class="scale-svg" viewBox={`0 0 ${VW} 215`} style={`max-width: ${width}px;`}>
    {#each ticks as t}
      <line
        x1={topX(t)}
        y1={Y_TOP}
        x2={bottomX(t)}
        y2={Y_BOTTOM}
        class="projection-line"
      />
    {/each}

    <line
      x1={topX(p)}
      y1={Y_TOP}
      x2={bottomX(p)}
      y2={Y_BOTTOM}
      class="percent-line"
    />
    <text x={clampLabelX(topX(p))} y={Y_TOP - 14} class="percent-label">{formatLabel(p)}</text>
    <text x={clampLabelX(bottomX(p))} y={Y_BOTTOM + 26} class="percent-label">{formatLabel(p * n)}</text>

    <line x1={PAD} y1={Y_TOP} x2={PAD + topWidth} y2={Y_TOP} class="axis-line" />
    <text x={PAD} y={Y_TOP - 10} class="tick-label">0</text>
    <text x={PAD + topWidth} y={Y_TOP - 10} class="tick-label">1</text>

    <line x1={PAD} y1={Y_BOTTOM} x2={PAD + DRAW_WIDTH} y2={Y_BOTTOM} class="axis-line" />
    <text x={PAD} y={Y_BOTTOM + 18} class="tick-label">0</text>
    <text x={PAD + DRAW_WIDTH} y={Y_BOTTOM + 18} class="tick-label">{n}</text>
  </svg>

  <div class="controls" style={`max-width: ${width}px;`}>
    <label>
      <span class="label-text">N</span>
      <input type="number" bind:value={n} min="1" max="50" step="1" />
    </label>
    <label>
      <span class="label-text">P</span>
      <input type="number" bind:value={p} min="0" max="1" step="0.1" />
    </label>
  </div>
</div>

<style>
  .scale-demo {
    margin-block: 1rem;
  }

  .scale-svg {
    display: block;
    width: 100%;
  }

  .axis-line {
    stroke: var(--sl-color-text);
    stroke-width: 4;
    stroke-linecap: round;
  }

  .projection-line {
    stroke: var(--sl-color-gray-3);
    stroke-width: 1;
    opacity: 0.6;
  }

  .percent-line {
    stroke: #ef4444;
    stroke-width: 2;
    opacity: 0.85;
  }

  .tick-label {
    fill: var(--sl-color-text);
    font-size: 15px;
    font-family: var(--__sl-font-mono, ui-monospace, monospace);
    text-anchor: middle;
  }

  .percent-label {
    fill: #ef4444;
    font-size: 16px;
    font-weight: 600;
    font-family: var(--__sl-font-mono, ui-monospace, monospace);
    text-anchor: middle;
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
