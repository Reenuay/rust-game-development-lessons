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
  // Pixels standing for the unit circle's radius, 1 — this is also how
  // many pixels stand for "1 radian" once the circle rolls out flat,
  // so the straightened length always matches the radians value.
  const RADIUS_PX = 85;
  // The circle's center sits exactly RADIUS_PX above the ruler, so at
  // rest it's already touching the ruler — like a wheel sitting on a
  // road, ready to roll.
  const RULER_Y = 350;
  const ORIGIN_Y = RULER_Y - RADIUS_PX;

  let radians = $derived((angle * Math.PI) / 180);

  // The arc always starts at angle 0 — the circle's rightmost point —
  // and that's also where the straightened trail on the ruler starts.
  const anchorX = ORIGIN_X + RADIUS_PX;

  // Ruler the straightened segment lands on — whole units from 0 up to
  // 2π, plus π and 2π marked specially since the lesson text calls out
  // those two values by name.
  const MAX_TICK = 6;
  const ticks = Array.from({ length: MAX_TICK + 1 }, (_, n) => ({ n, x: anchorX + RADIUS_PX * n }));
  const piX = anchorX + RADIUS_PX * Math.PI;
  const twoPiX = anchorX + RADIUS_PX * 2 * Math.PI;

  // How far the circle has rolled, in radians, at the current point in
  // the animation — also how much of the arc has peeled onto the
  // ruler. Rolling without slipping: the center moves right by exactly
  // RADIUS_PX for every radian rolled, same formula as the arc length
  // itself (radius × angle).
  let rolled = $derived(radians * t);
  let wheelCenterX = $derived(ORIGIN_X + RADIUS_PX * rolled);

  // A point at `localAngle` on the circle's own rim (measured the same
  // way as the resting arc) ends up here once the wheel has rolled —
  // rotated by `rolled` together with the whole wheel, and carried
  // along as the center translates.
  function wheelPoint(localAngle) {
    const worldAngle = localAngle + rolled;
    return {
      x: wheelCenterX + RADIUS_PX * Math.cos(worldAngle),
      y: ORIGIN_Y + RADIUS_PX * Math.sin(worldAngle),
    };
  }

  // How many sample points make up the remaining arc on the wheel.
  // Spaced evenly by angle, at a fixed density per full turn —
  // generous enough that even a full 360° sweep reads as a smooth
  // curve rather than a faceted polygon, but scaled down for small
  // angles instead of always sampling the same fixed count.
  const POINTS_PER_TURN = 72;
  let remainingSpan = $derived(radians * (1 - t));
  let pointCount = $derived(Math.max(2, Math.round((remainingSpan / (2 * Math.PI)) * POINTS_PER_TURN) + 1));

  // The part of the arc still "on the tire" — it hasn't touched the
  // ruler yet. Shrinks from the near end (angle 0, the first bit to
  // touch down as the wheel starts turning) while the far end (angle
  // = radians) stays put in the wheel's own rotating frame, right up
  // until there's nothing left.
  let wheelArcPoints = $derived.by(() => {
    const points = [];
    for (let i = 0; i < pointCount; i++) {
      const s = rolled + (i / (pointCount - 1)) * remainingSpan;
      points.push(wheelPoint(s - rolled));
    }
    return points;
  });

  let wheelArcPolyline = $derived(wheelArcPoints.map((p) => `${p.x},${p.y}`).join(' '));

  // The straightened trail just grows along the ruler as the wheel
  // rolls — no per-point morphing needed, its length always matches
  // how far the wheel has rolled.
  let trailEndX = $derived(anchorX + RADIUS_PX * rolled);

  // The length readout sits just past the leading edge of the trail.
  let labelX = $derived(trailEndX);
  let labelY = RULER_Y;

  // The two angle rays, rigidly attached to the wheel.
  let ray1Point = $derived(wheelPoint(0));
  let ray2Point = $derived(wheelPoint(radians));

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
    <!-- The wheel — same circle as before, but it rolls to the right
         as it unrolls, and fades away once its whole tire has peeled
         off onto the ruler. -->
    <circle cx={wheelCenterX} cy={ORIGIN_Y} r={RADIUS_PX} class="circle-outline" style={`opacity: ${1 - t}`} />

    <!-- Ruler baseline + ticks — always there, so the rolling wheel
         visibly sits right down on it from the very start. -->
    <line x1={anchorX} y1={RULER_Y} x2={twoPiX} y2={RULER_Y} class="ruler-line" />
    {#each ticks as tick (tick.n)}
      <line x1={tick.x} y1={RULER_Y - 6} x2={tick.x} y2={RULER_Y + 6} class="ruler-tick" />
      <text x={tick.x} y={RULER_Y + 22} class="ruler-label" text-anchor="middle">{tick.n}</text>
    {/each}
    <line x1={piX} y1={RULER_Y - 12} x2={piX} y2={RULER_Y + 12} class="ruler-mark" />
    <text x={piX} y={RULER_Y - 18} class="ruler-mark-label" text-anchor="middle">π</text>
    <line x1={twoPiX} y1={RULER_Y - 12} x2={twoPiX} y2={RULER_Y + 12} class="ruler-mark" />
    <text x={twoPiX} y={RULER_Y - 18} class="ruler-mark-label" text-anchor="middle">2π</text>

    <!-- Two rays marking the angle, rigidly attached to the wheel —
         they roll and fade away together with it. -->
    <line x1={wheelCenterX} y1={ORIGIN_Y} x2={ray1Point.x} y2={ray1Point.y} class="ray-line" style={`opacity: ${1 - t}`} />
    <line x1={wheelCenterX} y1={ORIGIN_Y} x2={ray2Point.x} y2={ray2Point.y} class="ray-line" style={`opacity: ${1 - t}`} />

    <!-- The part of the arc still on the wheel, not yet peeled off. -->
    {#if remainingSpan > 0.001}
      <polyline points={wheelArcPolyline} class="arc-line" />
    {/if}

    <!-- The straightened trail the wheel leaves behind as it rolls. -->
    <line x1={anchorX} y1={RULER_Y} x2={trailEndX} y2={RULER_Y} class="arc-line" />

    <text x={labelX + 14} y={labelY - 16} class="length-label" dominant-baseline="middle"
      >{formatNum(radians)}</text
    >

    <circle cx={wheelCenterX} cy={ORIGIN_Y} r="4" class="origin-dot" style={`opacity: ${1 - t}`} />
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
