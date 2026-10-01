<script>
  import NumberField from './NumberField.svelte';

  // Pure math illustration, no macroquad/WASM — same idea as
  // ScaleDemo.svelte. Gray vector with adjustable length/angle, a red
  // unit vector (fixed on-screen length, standing for length 1) drawn
  // in the same direction. Nothing is labeled by default — a button
  // reveals it step by step: first the gray vector's own x/y as
  // projections dropping onto the axes, then the same for the red
  // vector (its x/y being exactly cos/sin of the angle), together with
  // the arc marking that angle. Same "measure it, don't just label it"
  // idea as the arc-straightening animation in the radians lesson.
  let { width = 720 } = $props();

  let length = $state(4);
  let angle = $state(40);

  function clampLength(value) {
    return Math.min(6, Math.max(2, Math.round(value * 2) / 2));
  }

  function clampAngle(value) {
    return Math.min(360, Math.max(0, Math.round(value)));
  }

  $effect(() => {
    length = clampLength(length);
    angle = clampAngle(angle);
  });

  function formatNum(value) {
    return Number(value.toFixed(2)).toString();
  }

  // SVG viewBox geometry — fixed internal units, scaled to fit `width`
  // via CSS.
  const VW = 800;
  const VH = 640;
  const ORIGIN_X = 400;
  const ORIGIN_Y = 320;
  // How many pixels stand for "1" — the red vector is always exactly
  // this long, since it always has length 1.
  const PX_PER_UNIT = 40;
  const UNIT_PX = PX_PER_UNIT;
  const ARC_RADIUS = 28;

  let radians = $derived((angle * Math.PI) / 180);

  // The vector's own x, y — not pixels, the actual math values (length
  // times cos/sin) — same numbers the projections reveal below.
  let grayMathX = $derived(length * Math.cos(radians));
  let grayMathY = $derived(length * Math.sin(radians));
  let unitMathX = $derived(Math.cos(radians));
  let unitMathY = $derived(Math.sin(radians));

  let grayTipX = $derived(ORIGIN_X + grayMathX * PX_PER_UNIT);
  let grayTipY = $derived(ORIGIN_Y + grayMathY * PX_PER_UNIT);
  let unitTipX = $derived(ORIGIN_X + unitMathX * UNIT_PX);
  let unitTipY = $derived(ORIGIN_Y + unitMathY * UNIT_PX);

  // The <line> itself is drawn a few pixels past the true tip (the one
  // used for the projections above) — otherwise the line's own stroke
  // width poked out past the arrowhead marker's point, instead of the
  // marker fully covering it. Purely visual: the numbers shown always
  // come from the real, un-extended tip.
  let dirX = $derived(Math.cos(radians));
  let dirY = $derived(Math.sin(radians));
  let grayLineEndX = $derived(grayTipX + dirX * 6);
  let grayLineEndY = $derived(grayTipY + dirY * 6);
  let unitLineEndX = $derived(unitTipX + dirX * 5);
  let unitLineEndY = $derived(unitTipY + dirY * 5);

  let arcStartX = $derived(ORIGIN_X + ARC_RADIUS);
  let arcStartY = ORIGIN_Y;
  let arcEndX = $derived(ORIGIN_X + ARC_RADIUS * Math.cos(radians));
  let arcEndY = $derived(ORIGIN_Y + ARC_RADIUS * Math.sin(radians));
  let arcLargeFlag = $derived(angle > 180 ? 1 : 0);
  // At exactly 360° the start and end points of the arc coincide —
  // SVG's arc command can't draw a full circle from identical
  // endpoints, it just renders nothing. Draw the full sweep as two
  // half-circles instead, which does work.
  let arcPath = $derived(
    angle >= 360
      ? `M ${arcStartX} ${arcStartY} A ${ARC_RADIUS} ${ARC_RADIUS} 0 1 1 ${ORIGIN_X - ARC_RADIUS} ${ORIGIN_Y} A ${ARC_RADIUS} ${ARC_RADIUS} 0 1 1 ${arcStartX} ${arcStartY}`
      : `M ${arcStartX} ${arcStartY} A ${ARC_RADIUS} ${ARC_RADIUS} 0 ${arcLargeFlag} 1 ${arcEndX} ${arcEndY}`,
  );
  // Exact length of that path — radius times angle in radians, same
  // formula as the radians lesson — used below to "draw" the arc in
  // with a growing stroke instead of just switching it on.
  let arcLength = $derived(Math.max(ARC_RADIUS * radians, 0.01));

  // Revealing: 0 = nothing shown, 1 = fully shown. Two separate knobs,
  // animated one after the other, so the story reads "here's the gray
  // vector's own x/y" and only then "here's the same thing for the red
  // one — and that's exactly cos/sin of the angle" instead of dumping
  // both at once.
  let revealed = $state(false);
  let grayProgress = $state(0);
  let redProgress = $state(0);
  // Bumped on every click; an in-flight animation checks it and stops
  // updating (instead of fighting a newer one) once it no longer
  // matches — simpler than tracking individual animation-frame ids.
  let generation = 0;

  function animateValue(getCurrent, setValue, target, duration, myGeneration) {
    return new Promise((resolve) => {
      const start = getCurrent();
      const startTime = performance.now();

      function step(now) {
        if (myGeneration !== generation) {
          resolve();
          return;
        }
        const progress = Math.min(1, (now - startTime) / duration);
        const eased = progress < 0.5 ? 2 * progress * progress : 1 - (-2 * progress + 2) ** 2 / 2;
        setValue(start + (target - start) * eased);
        if (progress < 1) {
          requestAnimationFrame(step);
        } else {
          resolve();
        }
      }

      requestAnimationFrame(step);
    });
  }

  async function toggleReveal() {
    generation += 1;
    const myGeneration = generation;
    revealed = !revealed;

    if (revealed) {
      // Gray first, then red — one story at a time.
      await animateValue(() => grayProgress, (v) => (grayProgress = v), 1, 500, myGeneration);
      if (myGeneration !== generation) return;
      await animateValue(() => redProgress, (v) => (redProgress = v), 1, 500, myGeneration);
    } else {
      await Promise.all([
        animateValue(() => grayProgress, (v) => (grayProgress = v), 0, 400, myGeneration),
        animateValue(() => redProgress, (v) => (redProgress = v), 0, 400, myGeneration),
      ]);
    }
  }

  $effect(() => {
    return () => {
      generation += 1;
    };
  });

  // Projection endpoints — grow from the tip (at progress 0) out to the
  // axis (at progress 1), each vector using its own progress knob.
  let grayProjXEndY = $derived(grayTipY + (ORIGIN_Y - grayTipY) * grayProgress);
  let grayProjYEndX = $derived(grayTipX + (ORIGIN_X - grayTipX) * grayProgress);
  let redProjXEndY = $derived(unitTipY + (ORIGIN_Y - unitTipY) * redProgress);
  let redProjYEndX = $derived(unitTipX + (ORIGIN_X - unitTipX) * redProgress);

  // Which side of each axis the label should land on — continuing past
  // the axis in the same direction the projection line was already
  // travelling, so the number settles just beyond the crossing instead
  // of sitting on top of the axis itself. Same angle for both vectors,
  // so one pair of directions serves both.
  const GAP = 18;
  let axisOffsetDirY = $derived(dirY === 0 ? 1 : -Math.sign(dirY));
  let axisOffsetDirX = $derived(dirX === 0 ? 1 : -Math.sign(dirX));
  let yLabelAnchor = $derived(axisOffsetDirX > 0 ? 'start' : 'end');

  let grayXLabelY = $derived(grayProjXEndY + axisOffsetDirY * GAP * grayProgress);
  let grayYLabelX = $derived(grayProjYEndX + axisOffsetDirX * GAP * grayProgress);
  let redXLabelY = $derived(redProjXEndY + axisOffsetDirY * GAP * redProgress);
  let redYLabelX = $derived(redProjYEndX + axisOffsetDirX * GAP * redProgress);

  // A label's OTHER coordinate — the one it doesn't share with its own
  // projection line's growing end — is normally just the tip's own x
  // (for an x-label) or y (for a y-label). That's fine on its own, but
  // gray and red sit on the very same ray, so whenever the vector is
  // close to an axis (near 0°/90°/180°/...) both vectors' tips land
  // close together on that coordinate too, however different their
  // lengths are — and their labels would land right on top of each
  // other. Nudge red's the same way the gap above does, just along the
  // other coordinate — a wider one than GAP, since two whole strings
  // side by side (like "x=0" and "cos=0") need more room between their
  // centers than a single digit sitting past an axis does.
  //
  // Only apply the nudge when the tips are actually close enough on
  // that coordinate to be at risk — otherwise, near a diagonal angle,
  // both nudges (x and y) would fire at once and push red's own cos
  // and sin labels into each other instead of away from gray's.
  const SEPARATION = 46;
  const RISK_DISTANCE = 100;
  let xCollisionRisk = $derived(Math.max(0, 1 - Math.abs(grayTipX - unitTipX) / RISK_DISTANCE));
  let yCollisionRisk = $derived(Math.max(0, 1 - Math.abs(grayTipY - unitTipY) / RISK_DISTANCE));
  let redXLabelX = $derived(unitTipX + axisOffsetDirX * SEPARATION * redProgress * xCollisionRisk);
  let redYLabelY = $derived(unitTipY + axisOffsetDirY * SEPARATION * redProgress * yCollisionRisk);
</script>

<div class="sincos-demo">
  <svg class="sincos-svg" viewBox={`0 0 ${VW} ${VH}`} style={`max-width: ${width}px;`}>
    <defs>
      <marker id="sincos-arrow-gray" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="4.5" markerHeight="4.5" orient="auto">
        <path d="M0,1.5 L10,5 L0,8.5 Z" class="gray-arrow" />
      </marker>
      <marker id="sincos-arrow-red" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="4.5" markerHeight="4.5" orient="auto">
        <path d="M0,1.5 L10,5 L0,8.5 Z" class="unit-arrow" />
      </marker>
    </defs>

    <!-- Full x/y axes through the origin — just orientation, not measured values. -->
    <line x1="0" y1={ORIGIN_Y} x2={VW} y2={ORIGIN_Y} class="axis-line" />
    <line x1={ORIGIN_X} y1="0" x2={ORIGIN_X} y2={VH} class="axis-line" />

    <!-- Gray vector's projections onto each axis — grow in on reveal. -->
    <line x1={grayTipX} y1={grayTipY} x2={grayTipX} y2={grayProjXEndY} class="projection-line-gray" />
    <line x1={grayTipX} y1={grayTipY} x2={grayProjYEndX} y2={grayTipY} class="projection-line-gray" />
    <text x={grayTipX} y={grayXLabelY} class="gray-label" text-anchor="middle" dominant-baseline="middle" style={`opacity: ${grayProgress}`}
      >x={formatNum(grayMathX)}</text
    >
    <text x={grayYLabelX} y={grayTipY} class="gray-label" text-anchor={yLabelAnchor} dominant-baseline="middle" style={`opacity: ${grayProgress}`}
      >y={formatNum(grayMathY)}</text
    >

    <!-- Red (unit) vector's projections — same idea, revealed after
         the gray ones, together with the arc marking the angle. -->
    <line x1={unitTipX} y1={unitTipY} x2={unitTipX} y2={redProjXEndY} class="projection-line-red" />
    <line x1={unitTipX} y1={unitTipY} x2={redProjYEndX} y2={unitTipY} class="projection-line-red" />

    <!-- Arc marking the angle — drawn in (not just faded in) together
         with the red vector's own reveal. Drawn before the cos/sin
         labels below (not after), so their background halo paints on
         top of the arc instead of the arc covering it. -->
    <path d={arcPath} class="arc-path" stroke-dasharray={arcLength} stroke-dashoffset={arcLength * (1 - redProgress)} />

    <text x={redXLabelX} y={redXLabelY} class="unit-label" text-anchor="middle" dominant-baseline="middle" style={`opacity: ${redProgress}`}
      >cos={formatNum(unitMathX)}</text
    >
    <text x={redYLabelX} y={redYLabelY} class="unit-label" text-anchor={yLabelAnchor} dominant-baseline="middle" style={`opacity: ${redProgress}`}
      >sin={formatNum(unitMathY)}</text
    >

    <!-- Gray vector — adjustable length and angle. -->
    <line
      x1={ORIGIN_X}
      y1={ORIGIN_Y}
      x2={grayLineEndX}
      y2={grayLineEndY}
      class="gray-line"
      marker-end="url(#sincos-arrow-gray)"
    />

    <!-- Unit vector — same origin, same direction, fixed on-screen
         length, because its length is always exactly 1. -->
    <line
      x1={ORIGIN_X}
      y1={ORIGIN_Y}
      x2={unitLineEndX}
      y2={unitLineEndY}
      class="unit-line"
      marker-end="url(#sincos-arrow-red)"
    />

    <circle cx={ORIGIN_X} cy={ORIGIN_Y} r="4" class="origin-dot" />
  </svg>

  <p class="demo-legend">
    Серый — вектор с задаваемыми длиной и углом. Красный — единичный
    вектор (длина ровно 1) в том же самом направлении. Кнопка ниже
    показывает их <code>x</code> и <code>y</code> — сначала у серого,
    потом у красного вместе с углом, который им обоим задан.
  </p>

  <div class="controls" style={`max-width: ${width}px;`}>
    <NumberField label="length" bind:value={length} min="2" max="6" step="0.5" />
    <NumberField label="angle°" bind:value={angle} min="0" max="360" step="1" />
    <button type="button" class="reveal-button" onclick={toggleReveal}>
      {revealed ? 'Скрыть' : 'Показать'}
    </button>
  </div>
</div>

<style>
  .sincos-demo {
    margin-block: 1rem;
  }

  .sincos-svg {
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

  .projection-line-gray {
    stroke: var(--sl-color-gray-2);
    stroke-width: 1.5;
    stroke-dasharray: 3 3;
    opacity: 0.7;
  }

  .projection-line-red {
    stroke: #ef4444;
    stroke-width: 1.5;
    stroke-dasharray: 3 3;
    opacity: 0.7;
  }

  .arc-path {
    fill: none;
    stroke: #ef4444;
    stroke-width: 2;
  }

  .gray-line {
    stroke: var(--sl-color-gray-2);
    stroke-width: 2;
    stroke-linecap: round;
  }

  .gray-arrow {
    fill: var(--sl-color-gray-2);
  }

  .unit-line {
    stroke: #ef4444;
    stroke-width: 2;
    stroke-linecap: round;
  }

  .unit-arrow {
    fill: #ef4444;
  }

  .origin-dot {
    fill: var(--sl-color-text);
  }

  .gray-label,
  .unit-label {
    /* A background-colored outline behind the text, drawn before the
       fill (paint-order), so the numbers read cleanly over the axes,
       the arc, or the vectors instead of blending into them. */
    paint-order: stroke;
    stroke: var(--sl-color-bg);
    stroke-width: 5px;
    stroke-linejoin: round;
  }

  .gray-label {
    fill: var(--sl-color-text);
    color: var(--sl-color-text);
    font-size: 15px;
    font-family: var(--__sl-font-mono, ui-monospace, monospace);
  }

  .unit-label {
    fill: #ef4444;
    color: #ef4444;
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

  .reveal-button {
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

  .reveal-button:hover {
    background: var(--sl-color-gray-6);
  }
</style>
