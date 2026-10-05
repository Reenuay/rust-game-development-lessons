<script>
  import NumberField from './NumberField.svelte';

  // Grounds the abstract a/b/c triangle in something concrete first:
  // two actual points, with dx/dy/distance drawn as three
  // color-coded segments (red/green/blue) — the same three colors
  // PythagorasDemo.svelte reuses for its square-tiling proof, so the
  // two demos visibly connect.
  let { width = 560 } = $props();

  let dx = $state(3);
  let dy = $state(4);

  function clamp(value) {
    return Math.min(6, Math.max(1, Math.round(value)));
  }

  $effect(() => {
    dx = clamp(dx);
    dy = clamp(dy);
  });

  function formatNum(value) {
    return Number(value.toFixed(2)).toString();
  }

  const SCALE = 45;
  const VW = 560;
  const VH = 420;
  const P1X = 90;
  const P1Y = 360;

  let p2x = $derived(P1X + dx * SCALE);
  let p2y = $derived(P1Y - dy * SCALE);
  let distance = $derived(Math.sqrt(dx * dx + dy * dy));
</script>

<div class="points-triangle-demo">
  <svg class="points-triangle-svg" viewBox={`0 0 ${VW} ${VH}`} style={`max-width: ${width}px;`}>
    <!-- dx — красная линия, строго по горизонтали. -->
    <line x1={P1X} y1={P1Y} x2={p2x} y2={P1Y} class="leg-a-line" />
    <text x={(P1X + p2x) / 2} y={P1Y + 24} class="leg-a-label" text-anchor="middle">dx</text>

    <!-- dy — зелёная линия, строго по вертикали. -->
    <line x1={p2x} y1={P1Y} x2={p2x} y2={p2y} class="leg-b-label-line" />
    <text x={p2x + 14} y={(P1Y + p2y) / 2} class="leg-b-label" dominant-baseline="middle">dy</text>

    <!-- Расстояние — синяя линия, напрямую между точками. -->
    <line x1={P1X} y1={P1Y} x2={p2x} y2={p2y} class="hyp-line" />

    <circle cx={P1X} cy={P1Y} r="5" class="point-dot" />
    <circle cx={p2x} cy={p2y} r="5" class="point-dot" />
  </svg>

  <p class="demo-legend">
    Красная линия — это <code>dx</code>, шаг по горизонтали между
    точками. Зелёная — <code>dy</code>, шаг по вертикали. Синяя —
    расстояние между точками напрямую, то, что мы и хотим посчитать.
  </p>

  <p class="demo-numbers">
    dx = {dx} &nbsp;&nbsp; dy = {dy} &nbsp;&nbsp; расстояние = {formatNum(distance)}
  </p>

  <div class="controls" style={`max-width: ${width}px;`}>
    <NumberField label="dx" bind:value={dx} min="1" max="6" step="1" />
    <NumberField label="dy" bind:value={dy} min="1" max="6" step="1" />
  </div>
</div>

<style>
  .points-triangle-demo {
    margin-block: 1rem;
  }

  .points-triangle-svg {
    display: block;
    width: 100%;
  }

  .demo-legend,
  .demo-numbers {
    margin: 0.75rem 0;
  }

  .demo-numbers {
    font-family: var(--__sl-font-mono, ui-monospace, monospace);
    font-size: var(--sl-text-sm);
  }

  .leg-a-line {
    stroke: #ef4444;
    stroke-width: 4;
    stroke-linecap: round;
  }

  .leg-b-label-line {
    stroke: #22c55e;
    stroke-width: 4;
    stroke-linecap: round;
  }

  .hyp-line {
    stroke: #3b82f6;
    stroke-width: 4;
    stroke-linecap: round;
  }

  .leg-a-label {
    fill: #ef4444;
    font-size: 20px;
    font-weight: 700;
    font-family: var(--__sl-font-mono, ui-monospace, monospace);
  }

  .leg-b-label {
    fill: #22c55e;
    font-size: 20px;
    font-weight: 700;
    font-family: var(--__sl-font-mono, ui-monospace, monospace);
  }

  .point-dot {
    fill: var(--sl-color-text);
  }

  .controls {
    display: flex;
    align-items: center;
    gap: 1.5rem;
    margin-top: 0.75rem;
  }
</style>
