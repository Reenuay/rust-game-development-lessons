<script>
  import NumberField from './NumberField.svelte';

  // Pure math illustration — no macroquad/WASM involved, just an SVG
  // drawn from N and P. Shows the trapezoid you get when you line up
  // the [0, 1] range against the [0, N] range it's meant to scale to:
  // the [0, 1] line only takes up 1/N of the space the [0, N] line
  // does, because both use the same pixels-per-unit scale.
  let { width = 720 } = $props();

  let n = $state(5);
  let p = $state(0.5);

  // Highlights which number in the SVG a field controls, while the
  // field is hovered or focused — with N and P both changing several
  // numbers on screen at once (the trapezoid's width, the red line,
  // two labels), it wasn't obvious which one belongs to which input.
  let highlightN = $state(false);
  let highlightP = $state(false);

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

  // Rounding to 1 decimal made nearby values (0.5 vs 0.55) display as
  // the exact same text, looking like the label wasn't updating at
  // all — round to 2 instead, trimming any trailing zero.
  function formatLabel(value) {
    return Number(value.toFixed(2)).toString();
  }

  // SVG viewBox geometry — fixed internal units, scaled to fit `width`
  // via CSS. The dynamic P / P*N labels sit further out from the axis
  // lines than the static tick labels do, so they don't collide.
  const VW = 600;
  const PAD = 50;
  const DRAW_WIDTH = VW - PAD * 2;
  const Y_TOP = 60;
  const Y_BOTTOM = 150;
  const TICK_TOP_Y = Y_TOP - 16;
  const TICK_BOTTOM_Y = Y_BOTTOM + 22;
  const PERCENT_LABEL_TOP_Y = Y_TOP - 38;
  const PERCENT_LABEL_BOTTOM_Y = Y_BOTTOM + 50;

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
  <svg class="scale-svg" viewBox={`0 0 ${VW} 230`} style={`max-width: ${width}px;`}>
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
    <text
      x={clampLabelX(topX(p))}
      y={PERCENT_LABEL_TOP_Y}
      class="percent-label"
      class:demo-pulse={highlightP}
    >{formatLabel(p)}</text>
    <text x={clampLabelX(bottomX(p))} y={PERCENT_LABEL_BOTTOM_Y} class="percent-label">{formatLabel(p * n)}</text>

    <line x1={PAD} y1={Y_TOP} x2={PAD + topWidth} y2={Y_TOP} class="axis-line" />
    <text x={PAD} y={TICK_TOP_Y} class="tick-label">0</text>
    <text x={PAD + topWidth} y={TICK_TOP_Y} class="tick-label">1</text>

    <line x1={PAD} y1={Y_BOTTOM} x2={PAD + DRAW_WIDTH} y2={Y_BOTTOM} class="axis-line" />
    <text x={PAD} y={TICK_BOTTOM_Y} class="tick-label">0</text>
    <text
      x={PAD + DRAW_WIDTH}
      y={TICK_BOTTOM_Y}
      class="tick-label"
      class:demo-pulse={highlightN}
    >{n}</text>
  </svg>

  <div class="controls" style={`max-width: ${width}px;`}>
    <NumberField
      label="N"
      bind:value={n}
      min="1"
      max="50"
      step="1"
      onmouseenter={() => (highlightN = true)}
      onmouseleave={() => (highlightN = false)}
      onfocus={() => (highlightN = true)}
      onblur={() => (highlightN = false)}
    />
    <NumberField
      label="P"
      bind:value={p}
      min="0"
      max="1"
      step="0.01"
      onmouseenter={() => (highlightP = true)}
      onmouseleave={() => (highlightP = false)}
      onfocus={() => (highlightP = true)}
      onblur={() => (highlightP = false)}
    />
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
    /* .demo-pulse's glow (src/styles/pulse.css) reads the CSS `color`
       property via currentColor — SVG's `fill` doesn't set it, so set
       both to the same value or the glow would default to black. */
    color: var(--sl-color-text);
    font-size: 15px;
    font-family: var(--__sl-font-mono, ui-monospace, monospace);
    text-anchor: middle;
  }

  .percent-label {
    fill: #ef4444;
    color: #ef4444;
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

</style>
