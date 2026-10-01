<script>
  // Pure math illustration, no macroquad/WASM — same idea as
  // RadiansDemo.svelte. A single vector from the origin, its
  // direction given by (angle.cos(), angle.sin()) — nothing else: no
  // center, no second vector, no pivot. The "Крутить" button grows
  // the angle every frame, exactly like `angle += SPEED` inside a
  // real `loop`, so the vector sweeps around smoothly and endlessly.
  let { width = 560 } = $props();

  const VW = 600;
  const VH = 460;
  const ORIGIN_X = 300;
  const ORIGIN_Y = 230;
  const RADIUS_PX = 160;
  const SPEED = 0.02;

  let angle = $state(0);
  let spinning = $state(false);
  let animationFrame;

  function formatNum(value) {
    // Number(...).toString() on a tiny negative value (e.g. sin(0)
    // rounding to -0) prints "-0", which reads like a typo — add 0 to
    // collapse -0 back to a plain 0 first.
    return Number((value + 0).toFixed(2)).toString();
  }

  let dirX = $derived(Math.cos(angle));
  let dirY = $derived(Math.sin(angle));
  let tipX = $derived(ORIGIN_X + dirX * RADIUS_PX);
  let tipY = $derived(ORIGIN_Y + dirY * RADIUS_PX);
  // The line is drawn a few pixels short of the true tip so the
  // arrowhead marker's own point lands exactly on it, instead of the
  // line's stroke poking out past the marker.
  let lineEndX = $derived(ORIGIN_X + dirX * (RADIUS_PX - 6));
  let lineEndY = $derived(ORIGIN_Y + dirY * (RADIUS_PX - 6));

  // Label sits a bit further out than the tip, along the same
  // direction, so it never sits on top of the arrowhead.
  let labelX = $derived(ORIGIN_X + dirX * (RADIUS_PX + 34));
  let labelY = $derived(ORIGIN_Y + dirY * (RADIUS_PX + 34));

  function step() {
    angle += SPEED;
    animationFrame = requestAnimationFrame(step);
  }

  function toggleSpin() {
    spinning = !spinning;
    if (spinning) {
      animationFrame = requestAnimationFrame(step);
    } else {
      cancelAnimationFrame(animationFrame);
    }
  }

  $effect(() => {
    return () => cancelAnimationFrame(animationFrame);
  });
</script>

<div class="rotation-demo">
  <svg class="rotation-svg" viewBox={`0 0 ${VW} ${VH}`} style={`max-width: ${width}px;`}>
    <defs>
      <marker id="rotation-arrow" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="4.5" markerHeight="4.5" orient="auto">
        <path d="M0,1.5 L10,5 L0,8.5 Z" class="arrow" />
      </marker>
    </defs>

    <line x1="0" y1={ORIGIN_Y} x2={VW} y2={ORIGIN_Y} class="axis-line" />
    <line x1={ORIGIN_X} y1="0" x2={ORIGIN_X} y2={VH} class="axis-line" />
    <circle cx={ORIGIN_X} cy={ORIGIN_Y} r={RADIUS_PX} class="circle-outline" />

    <line x1={ORIGIN_X} y1={ORIGIN_Y} x2={lineEndX} y2={lineEndY} class="vector-line" marker-end="url(#rotation-arrow)" />
    <text x={labelX} y={labelY} class="vector-label" text-anchor="middle" dominant-baseline="middle"
      >({formatNum(dirX)}, {formatNum(dirY)})</text
    >

    <circle cx={ORIGIN_X} cy={ORIGIN_Y} r="4" class="origin-dot" />
  </svg>

  <p class="demo-legend">
    Вектор всегда начинается в нуле — его направление это
    <code>(angle.cos(), angle.sin())</code>. Кнопка ниже каждый кадр
    увеличивает <code>angle</code>, и вектор плавно крутится по кругу.
  </p>

  <div class="controls">
    <button type="button" class="spin-button" onclick={toggleSpin}>
      {spinning ? 'Стоп' : 'Крутить'}
    </button>
  </div>
</div>

<style>
  .rotation-demo {
    margin-block: 1rem;
  }

  .rotation-svg {
    display: block;
    width: 100%;
  }

  .demo-legend {
    margin: 0.75rem 0;
  }

  .axis-line {
    stroke: var(--sl-color-gray-3);
    stroke-width: 2;
    stroke-dasharray: 4 4;
  }

  .circle-outline {
    fill: none;
    stroke: var(--sl-color-gray-4);
    stroke-width: 2;
    stroke-dasharray: 4 4;
  }

  .vector-line {
    stroke: #ef4444;
    stroke-width: 3;
    stroke-linecap: round;
  }

  .arrow {
    fill: #ef4444;
  }

  .vector-label {
    fill: #ef4444;
    font-size: 15px;
    font-weight: 600;
    font-family: var(--__sl-font-mono, ui-monospace, monospace);
    /* A background-colored outline behind the text, drawn before the
       fill, so the numbers read cleanly over the circle or axes
       instead of blending into them. */
    paint-order: stroke;
    stroke: var(--sl-color-bg);
    stroke-width: 5px;
    stroke-linejoin: round;
  }

  .origin-dot {
    fill: var(--sl-color-text);
  }

  .controls {
    display: flex;
    margin-top: 0.75rem;
  }

  .spin-button {
    margin: 0;
    height: 1.75rem;
    padding: 0 0.75rem;
    border: 1.5px solid var(--sl-color-accent);
    border-radius: 0.25rem;
    background: var(--sl-color-bg);
    color: var(--sl-color-text);
    font: inherit;
    font-family: var(--__sl-font-mono, ui-monospace, monospace);
    font-size: var(--sl-text-sm);
    line-height: 1.75rem;
    cursor: pointer;
  }

  .spin-button:hover {
    background: var(--sl-color-gray-6);
  }
</style>
