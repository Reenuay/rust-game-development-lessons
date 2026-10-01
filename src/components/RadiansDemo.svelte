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

  // A real wheel only touches a straight road at the point directly
  // below its center — not wherever the arc happens to start. So the
  // ruler's zero sits under the center (the "90°" direction, straight
  // down in our cos/sin convention), not under the circle's rightmost
  // point.
  const anchorX = ORIGIN_X;

  // Ruler the straightened segment lands on — whole units from 0 up to
  // 2π, plus π and 2π marked specially since the lesson text calls out
  // those two values by name.
  const MAX_TICK = 6;
  const ticks = Array.from({ length: MAX_TICK + 1 }, (_, n) => ({ n, x: anchorX + RADIUS_PX * n }));
  const piX = anchorX + RADIUS_PX * Math.PI;
  const twoPiX = anchorX + RADIUS_PX * 2 * Math.PI;

  // Two-phase roll. Phase 1 is a rotation in place, with no
  // translation yet: it swings the whole wheel (rigidly — both rays,
  // the full arc) until the far ray (at angle = radians) points
  // straight down, lined up with the ruler's zero, ready to touch
  // down. Phase 2 is the actual roll: the wheel turns clockwise and
  // moves right, peeling its arc onto the ruler, until the near ray
  // (angle 0) has swung all the way down to that same spot — at which
  // point every bit of the arc has touched down and the wheel vanishes.
  //
  // `psi` is the wheel's total rotation from its resting pose (positive
  // = clockwise, same sense as the angle/cos/sin convention
  // elsewhere). Phase 1 covers psi from 0 to `psi1` — negative (a
  // counter-clockwise swing) whenever the target angle is more than
  // 90°, positive otherwise. Phase 2 always adds a further `radians`
  // clockwise on top of that, ending at a fixed psi of 90° regardless
  // of the target angle — that's what makes the near ray land exactly
  // on the ruler's zero every time.
  let psi1 = $derived(Math.PI / 2 - radians);
  let phase1Share = $derived.by(() => {
    const phase1Work = Math.abs(psi1);
    const totalWork = phase1Work + radians;
    return totalWork > 0 ? phase1Work / totalWork : 0;
  });

  let phase2Progress = $derived(t <= phase1Share ? 0 : (t - phase1Share) / Math.max(1 - phase1Share, 0.0001));
  let psi = $derived(t <= phase1Share ? psi1 * (phase1Share > 0 ? t / phase1Share : 1) : psi1 + radians * phase2Progress);

  // The wheel only starts moving once phase 1's pre-swing is done.
  let wheelCenterX = $derived(ORIGIN_X + RADIUS_PX * radians * phase2Progress);

  // A point at `localAngle` on the circle's own rim (measured the same
  // way as the resting arc) ends up here at the current point in the
  // animation, carried along by the wheel's rotation and translation.
  function wheelPoint(localAngle) {
    const worldAngle = localAngle + psi;
    return {
      x: wheelCenterX + RADIUS_PX * Math.cos(worldAngle),
      y: ORIGIN_Y + RADIUS_PX * Math.sin(worldAngle),
    };
  }

  // How much of the original arc (measured from angle 0) hasn't
  // touched the ruler yet. Nothing peels during phase 1 — it's just a
  // rotation in place. During phase 2, the far end (closest to
  // touching down already, back at the end of phase 1) peels off
  // first, so the remaining arc shrinks from that end inward.
  let remainingHighEnd = $derived(t <= phase1Share ? radians : Math.max(0, Math.min(radians, Math.PI / 2 - psi)));

  // How many sample points make up the remaining arc on the wheel.
  // Spaced evenly by angle, at a fixed density per full turn —
  // generous enough that even a full 360° sweep reads as a smooth
  // curve rather than a faceted polygon, but scaled down for small
  // angles instead of always sampling the same fixed count.
  const POINTS_PER_TURN = 72;
  let pointCount = $derived(Math.max(2, Math.round((remainingHighEnd / (2 * Math.PI)) * POINTS_PER_TURN) + 1));

  let wheelArcPoints = $derived.by(() => {
    const points = [];
    for (let i = 0; i < pointCount; i++) {
      const s = (i / (pointCount - 1)) * remainingHighEnd;
      points.push(wheelPoint(s));
    }
    return points;
  });

  let wheelArcPolyline = $derived(wheelArcPoints.map((p) => `${p.x},${p.y}`).join(' '));

  // The straightened trail grows along the ruler as the wheel rolls —
  // its length always matches how much of the arc has touched down.
  let trailEndX = $derived(anchorX + RADIUS_PX * (radians - remainingHighEnd));

  // The length readout sits just past the leading edge of the trail.
  let labelX = $derived(trailEndX);
  let labelY = RULER_Y;

  // The two angle rays, rigidly attached to the wheel.
  let ray1Point = $derived(wheelPoint(0));
  let ray2Point = $derived(wheelPoint(radians));

  // The ruler appears the instant the animation starts (not a gradual
  // fade), and the wheel itself only starts fading once phase 2 — the
  // actual rolling — gets underway; nothing is "disappearing" yet
  // during phase 1's in-place swing.
  let rulerVisible = $derived(t > 0);
  let wheelOpacity = $derived(t <= phase1Share ? 1 : 1 - phase2Progress);

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
    <!-- The wheel — rotates in place first (phase 1), then rolls right
         (phase 2), fading away once its whole tire has peeled off. -->
    <circle cx={wheelCenterX} cy={ORIGIN_Y} r={RADIUS_PX} class="circle-outline" style={`opacity: ${wheelOpacity}`} />

    <!-- Ruler baseline + ticks — appear the instant the animation
         starts, zero lined up under the circle's center. -->
    {#if rulerVisible}
      <line x1={anchorX} y1={RULER_Y} x2={twoPiX} y2={RULER_Y} class="ruler-line" />
      {#each ticks as tick (tick.n)}
        <line x1={tick.x} y1={RULER_Y - 6} x2={tick.x} y2={RULER_Y + 6} class="ruler-tick" />
        <text x={tick.x} y={RULER_Y + 22} class="ruler-label" text-anchor="middle">{tick.n}</text>
      {/each}
      <line x1={piX} y1={RULER_Y - 12} x2={piX} y2={RULER_Y + 12} class="ruler-mark" />
      <text x={piX} y={RULER_Y - 18} class="ruler-mark-label" text-anchor="middle">π</text>
      <line x1={twoPiX} y1={RULER_Y - 12} x2={twoPiX} y2={RULER_Y + 12} class="ruler-mark" />
      <text x={twoPiX} y={RULER_Y - 18} class="ruler-mark-label" text-anchor="middle">2π</text>
    {/if}

    <!-- Two rays marking the angle, rigidly attached to the wheel —
         they roll and fade away together with it. -->
    <line x1={wheelCenterX} y1={ORIGIN_Y} x2={ray1Point.x} y2={ray1Point.y} class="ray-line" style={`opacity: ${wheelOpacity}`} />
    <line x1={wheelCenterX} y1={ORIGIN_Y} x2={ray2Point.x} y2={ray2Point.y} class="ray-line" style={`opacity: ${wheelOpacity}`} />

    <!-- The part of the arc still on the wheel, not yet peeled off. -->
    {#if remainingHighEnd > 0.001}
      <polyline points={wheelArcPolyline} class="arc-line" />
    {/if}

    <!-- The straightened trail the wheel leaves behind as it rolls. -->
    {#if rulerVisible}
      <line x1={anchorX} y1={RULER_Y} x2={trailEndX} y2={RULER_Y} class="arc-line" />
    {/if}

    <text x={labelX + 14} y={labelY - 16} class="length-label" dominant-baseline="middle"
      >{formatNum(radians)}</text
    >

    <circle cx={wheelCenterX} cy={ORIGIN_Y} r="4" class="origin-dot" style={`opacity: ${wheelOpacity}`} />
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
