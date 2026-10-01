<script>
  import NumberField from './NumberField.svelte';

  // Pure math illustration, no macroquad/WASM — same idea as
  // SinCosDemo.svelte. A unit circle with an adjustable angle, the arc
  // it cuts off highlighted, and a button that morphs that arc into a
  // straight segment of the same length — the length it ends up being
  // is exactly the angle in radians.
  let { width = 720 } = $props();

  let angle = $state(90);
  let straightened = $state(false);
  // 0 = arc sitting on the circle, 1 = fully straightened out.
  let t = $state(0);

  function clampAngle(value) {
    return Math.min(360, Math.max(0, Math.round(value)));
  }

  $effect(() => {
    angle = clampAngle(angle);
  });

  function formatNum(value) {
    return Number(value.toFixed(2)).toString();
  }

  // SVG viewBox geometry — fixed internal units, scaled to fit `width`
  // via CSS.
  const VW = 900;
  const VH = 430;
  const ORIGIN_X = 150;
  const ORIGIN_Y = 230;
  // Pixels standing for the unit circle's radius, 1 — this is also how
  // many pixels stand for "1 radian" once the arc is straightened, so
  // the straightened length always matches the radians value.
  const RADIUS_PX = 85;

  let radians = $derived((angle * Math.PI) / 180);

  // The arc always starts at angle 0 — the circle's rightmost point —
  // and that's also where the straightened segment starts from.
  const anchorX = ORIGIN_X + RADIUS_PX;
  const anchorY = ORIGIN_Y;

  // Ruler the straightened segment lands on — whole units from 0 up to
  // 2π, plus π and 2π marked specially since the lesson text calls out
  // those two values by name.
  const RULER_Y = ORIGIN_Y + 120;
  const MAX_TICK = 6;
  const ticks = Array.from({ length: MAX_TICK + 1 }, (_, n) => ({ n, x: anchorX + RADIUS_PX * n }));
  const piX = anchorX + RADIUS_PX * Math.PI;
  const twoPiX = anchorX + RADIUS_PX * 2 * Math.PI;

  // How many sample points make up the arc. Spaced evenly by angle, at
  // a fixed density per full turn — generous enough that even a full
  // 360° sweep reads as a smooth curve rather than a faceted polygon,
  // but scaled down for small angles instead of always sampling the
  // same fixed count regardless of how little arc there is to show.
  const POINTS_PER_TURN = 72;
  let pointCount = $derived(Math.max(2, Math.round((radians / (2 * Math.PI)) * POINTS_PER_TURN) + 1));

  // Each sample point carries both where it sits on the circle and
  // where it lands once straightened — evenly spaced by angle, so the
  // spacing between neighbors (in pixels) stays the same whether it's
  // measured along the curve or along the straight line. That's what
  // keeps the total length the same throughout the animation, instead
  // of the line coming out shorter or longer than the arc it replaced.
  // The straightened position sits on the ruler (RULER_Y), not at the
  // circle's own height, so the arc visibly comes down and lies flat
  // along it rather than just floating straight in place.
  let arcPoints = $derived.by(() => {
    const points = [];
    for (let i = 0; i < pointCount; i++) {
      const s = (i / (pointCount - 1)) * radians;
      points.push({
        curvedX: ORIGIN_X + RADIUS_PX * Math.cos(s),
        curvedY: ORIGIN_Y + RADIUS_PX * Math.sin(s),
        straightX: anchorX + RADIUS_PX * s,
        straightY: RULER_Y,
      });
    }
    return points;
  });

  // The actual points drawn right now — each one somewhere between its
  // curved and straightened position, depending on t.
  let morphedPoints = $derived(
    arcPoints.map((p) => ({
      x: p.curvedX + (p.straightX - p.curvedX) * t,
      y: p.curvedY + (p.straightY - p.curvedY) * t,
    })),
  );

  let polylinePoints = $derived(morphedPoints.map((p) => `${p.x},${p.y}`).join(' '));

  // The length readout sits right past the far end of the arc/segment,
  // wherever that currently is.
  let lastPoint = $derived(morphedPoints[morphedPoints.length - 1] ?? { x: anchorX, y: anchorY });

  let animationFrame;

  // Animates t from its current value to `target` (0 or 1) — no tween
  // library in this project, so a small requestAnimationFrame loop
  // with a plain ease-in-out curve does the job.
  function animateTo(target) {
    cancelAnimationFrame(animationFrame);
    const start = t;
    const startTime = performance.now();
    const duration = 700;

    function step(now) {
      const progress = Math.min(1, (now - startTime) / duration);
      const eased = progress < 0.5 ? 2 * progress * progress : 1 - (-2 * progress + 2) ** 2 / 2;
      t = start + (target - start) * eased;
      if (progress < 1) {
        animationFrame = requestAnimationFrame(step);
      }
    }

    animationFrame = requestAnimationFrame(step);
  }

  function toggleStraighten() {
    straightened = !straightened;
    animateTo(straightened ? 1 : 0);
  }

  $effect(() => {
    return () => cancelAnimationFrame(animationFrame);
  });
</script>

<div class="radians-demo">
  <svg class="radians-svg" viewBox={`0 0 ${VW} ${VH}`} style={`max-width: ${width}px;`}>
    <!-- Full unit circle, just for reference. -->
    <circle cx={ORIGIN_X} cy={ORIGIN_Y} r={RADIUS_PX} class="circle-outline" />

    <!-- Ruler baseline + ticks — only relevant once the arc starts
         straightening out, so it fades in together with it. -->
    <g style={`opacity: ${t}`}>
      <line x1={anchorX} y1={RULER_Y} x2={twoPiX} y2={RULER_Y} class="ruler-line" />
      {#each ticks as tick (tick.n)}
        <line x1={tick.x} y1={RULER_Y - 6} x2={tick.x} y2={RULER_Y + 6} class="ruler-tick" />
        <text x={tick.x} y={RULER_Y + 22} class="ruler-label" text-anchor="middle">{tick.n}</text>
      {/each}
      <line x1={piX} y1={RULER_Y - 12} x2={piX} y2={RULER_Y + 12} class="ruler-mark" />
      <text x={piX} y={RULER_Y - 18} class="ruler-mark-label" text-anchor="middle">π</text>
      <line x1={twoPiX} y1={RULER_Y - 12} x2={twoPiX} y2={RULER_Y + 12} class="ruler-mark" />
      <text x={twoPiX} y={RULER_Y - 18} class="ruler-mark-label" text-anchor="middle">2π</text>
    </g>

    <!-- Two rays marking the angle — the second one fades out while
         straightening, since its far end is no longer on the circle. -->
    <line x1={ORIGIN_X} y1={ORIGIN_Y} x2={anchorX} y2={anchorY} class="ray-line" />
    <line
      x1={ORIGIN_X}
      y1={ORIGIN_Y}
      x2={ORIGIN_X + RADIUS_PX * Math.cos(radians)}
      y2={ORIGIN_Y + RADIUS_PX * Math.sin(radians)}
      class="ray-line"
      style={`opacity: ${1 - t}`}
    />

    <!-- The arc itself, morphing into a straight segment. -->
    <polyline points={polylinePoints} class="arc-line" />

    <text x={lastPoint.x + 14} y={lastPoint.y - 16} class="length-label" dominant-baseline="middle"
      >{formatNum(radians)}</text
    >

    <circle cx={ORIGIN_X} cy={ORIGIN_Y} r="4" class="origin-dot" />
  </svg>

  <p class="demo-legend">
    На картинке выше — окружность радиусом ровно 1 (можно считать, что
    это 1 метр, 1 сантиметр, что угодно — главное, что единица). Из
    точки в центре выходят две толстые серые линии и упираются в саму
    окружность — между ними получился уголок. Этот уголок отрезает от
    окружности (она нарисована пунктиром) кусочек её края — вот этот
    кусочек и называется дугой, мы выделили её красным. Рядом с дугой
    красным же числом записана её длина. А раз радиус окружности равен
    1, эта длина и есть угол в радианах — то есть это одно и то же
    число.
  </p>

  <div class="controls" style={`max-width: ${width}px;`}>
    <NumberField label="angle°" bind:value={angle} min="0" max="360" step="1" />
    <button type="button" class="straighten-button" onclick={toggleStraighten}>
      {straightened ? 'Свернуть' : 'Выпрямить'}
    </button>
  </div>
</div>

<style>
  .radians-demo {
    margin-block: 1rem;
  }

  .radians-svg {
    display: block;
    width: 100%;
  }

  .demo-legend {
    margin: 0.75rem 0;
  }

  .circle-outline {
    fill: none;
    stroke: var(--sl-color-gray-4);
    stroke-width: 2;
    stroke-dasharray: 4 4;
  }

  .ray-line {
    stroke: var(--sl-color-gray-2);
    stroke-width: 2;
  }

  .arc-line {
    fill: none;
    stroke: #ef4444;
    stroke-width: 3;
    stroke-linecap: round;
    stroke-linejoin: round;
  }

  .origin-dot {
    fill: var(--sl-color-text);
  }

  .length-label {
    fill: #ef4444;
    color: #ef4444;
    font-size: 16px;
    font-weight: 600;
    font-family: var(--__sl-font-mono, ui-monospace, monospace);
    /* A background-colored outline behind the text, drawn before the
       fill (paint-order), so the number reads cleanly over the arc or
       rays instead of blending into them. */
    paint-order: stroke;
    stroke: var(--sl-color-bg);
    stroke-width: 5px;
    stroke-linejoin: round;
  }

  .ruler-line {
    stroke: var(--sl-color-gray-3);
    stroke-width: 1.5;
  }

  .ruler-tick {
    stroke: var(--sl-color-gray-3);
    stroke-width: 1.5;
  }

  .ruler-label {
    fill: var(--sl-color-gray-2);
    color: var(--sl-color-gray-2);
    font-size: 13px;
    font-family: var(--__sl-font-mono, ui-monospace, monospace);
  }

  .ruler-mark {
    stroke: var(--sl-color-accent);
    stroke-width: 2;
  }

  .ruler-mark-label {
    fill: var(--sl-color-accent);
    color: var(--sl-color-accent);
    font-size: 15px;
    font-weight: 600;
    font-family: var(--__sl-font-mono, ui-monospace, monospace);
  }

  .controls {
    display: flex;
    align-items: center;
    gap: 1.5rem;
    margin-top: 0.75rem;
  }

  .straighten-button {
    /* Starlight's prose CSS adds margin-top between adjacent content
       elements — NumberField's own <label> resets this on itself (see
       the comment in that file), but a plain <button> next to it
       still gets pushed down without the same reset. */
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

  .straighten-button:hover {
    background: var(--sl-color-gray-6);
  }
</style>
