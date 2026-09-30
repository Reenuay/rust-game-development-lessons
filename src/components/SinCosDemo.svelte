<script>
  import NumberField from './NumberField.svelte';

  // Pure math illustration, no macroquad/WASM — same idea as
  // ScaleDemo.svelte. Gray vector with adjustable length/angle, a red
  // unit vector (fixed on-screen length, standing for length 1) drawn
  // over it in the same direction, and a red arc marking the angle
  // between the x-axis and the vector. x/y labels live right on the
  // drawing, placed along each vector's own direction (see
  // grayLabelX/Y, redLabelX/Y below) rather than at a fixed spot, so
  // they never collide with the axes or each other as angle sweeps
  // through 0°/180°/360°.
  let { width = 720 } = $props();

  let length = $state(4);
  let angle = $state(40);

  let highlightLength = $state(false);
  let highlightAngle = $state(false);

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
  // times cos/sin) — same numbers the on-drawing labels show.
  let grayMathX = $derived(length * Math.cos(radians));
  let grayMathY = $derived(length * Math.sin(radians));
  let unitMathX = $derived(Math.cos(radians));
  let unitMathY = $derived(Math.sin(radians));

  let grayTipX = $derived(ORIGIN_X + grayMathX * PX_PER_UNIT);
  let grayTipY = $derived(ORIGIN_Y + grayMathY * PX_PER_UNIT);
  let unitTipX = $derived(ORIGIN_X + unitMathX * UNIT_PX);
  let unitTipY = $derived(ORIGIN_Y + unitMathY * UNIT_PX);

  // The <line> itself is drawn a few pixels past the true tip (the one
  // used for the labels and the projections above) — otherwise the
  // line's own stroke width poked out past the arrowhead marker's
  // point, instead of the marker fully covering it. Purely visual:
  // the numbers shown always come from the real, un-extended tip.
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

  // Gray label: further out along the vector's own direction, past its
  // tip — never "always below", always where the vector itself points.
  // Once the vector gets close to horizontal, though, that ray-based
  // spot sits right where the line (or the x-axis right behind it)
  // passes through the text — lift the label above the line there, by
  // more the more horizontal the vector is, tapering back to no lift
  // at all once it's steep enough that the ray already clears on its
  // own.
  const GRAY_LABEL_GAP = 26;
  const GRAY_LABEL_MAX_LIFT = 20;
  let grayLabelDist = $derived(length * PX_PER_UNIT + GRAY_LABEL_GAP);
  let grayLabelLift = $derived(Math.max(0, 1 - Math.abs(dirY) * 2) * GRAY_LABEL_MAX_LIFT);
  let grayLabelX = $derived(ORIGIN_X + dirX * grayLabelDist);
  let grayLabelY = $derived(ORIGIN_Y + dirY * grayLabelDist - grayLabelLift);

  // Text-anchor follows the same direction: a label sitting past a
  // mostly-horizontal vector should grow away from it (start/end), not
  // spill back across it (middle only for the near-vertical case).
  function anchorFor(dx) {
    if (dx > 0.15) return 'start';
    if (dx < -0.15) return 'end';
    return 'middle';
  }
  let grayLabelAnchor = $derived(anchorFor(dirX));

  // Red label: offset sideways from the unit vector's tip — left when
  // it points left, right when it points right. Unlike the gray label
  // this never follows the vector out along its own ray: the unit
  // vector is short, so "past the tip" would land right next to (or
  // inside) the gray vector and arc.
  const RED_LABEL_OFFSET = 34;
  const RED_LABEL_VERTICAL_BIAS = 18;
  // The more horizontal the vector, the closer its tip sits to the
  // "angle" label's own spot (just right of the arc, right above the
  // x-axis) — so the nearer to horizontal, the harder this pushes the
  // label away from the axis, on top of the fixed bias below.
  const RED_LABEL_EXTRA_BIAS = 24;
  let unitPointsRight = $derived(dirX >= 0);
  let redLabelX = $derived(unitTipX + (unitPointsRight ? RED_LABEL_OFFSET : -RED_LABEL_OFFSET));
  let redLabelHorizontalness = $derived(Math.max(0, Math.abs(dirX) - 0.3) / 0.7);
  let redLabelBias = $derived(RED_LABEL_VERTICAL_BIAS + redLabelHorizontalness * RED_LABEL_EXTRA_BIAS);
  // A nudge down (or up, once the vector points mostly upward) — without
  // it, the label sat right on top of the x-axis (and the "angle" label
  // next to the arc) whenever the vector was close to horizontal, i.e.
  // angle near 0°/180°/360°.
  let redLabelY = $derived(unitTipY + (dirY >= 0 ? redLabelBias : -redLabelBias));
  let redLabelAnchor = $derived(unitPointsRight ? 'start' : 'end');

  // Angle label: a short fixed identifier next to the arc — not a
  // number (the angle field above already shows that), just "angle"
  // pointing at what the arc means. Fixed just to the right of the
  // arc's own starting point (always on the positive x-axis, angle=0),
  // rather than tracking the sweep — a position that follows the
  // current angle kept drifting onto the arc itself.
  const ANGLE_LABEL_X = ORIGIN_X + ARC_RADIUS + 24;
  const ANGLE_LABEL_Y = ORIGIN_Y - 10;
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

    <!-- Unit vector's projection onto each axis — drawn first, so the
         vectors and their arrowheads sit on top of them. -->
    <line x1={unitTipX} y1={unitTipY} x2={unitTipX} y2={ORIGIN_Y} class="projection-line" />
    <line x1={unitTipX} y1={unitTipY} x2={ORIGIN_X} y2={unitTipY} class="projection-line" />

    <!-- Arc marking the angle between the x-axis and the vector. -->
    <path d={arcPath} class="arc-path" />
    <text
      x={ANGLE_LABEL_X}
      y={ANGLE_LABEL_Y}
      class="angle-label"
      text-anchor="start"
      data-pulse={highlightAngle ? '' : undefined}
    >angle</text>

    <!-- Gray vector — adjustable length and angle. -->
    <line
      x1={ORIGIN_X}
      y1={ORIGIN_Y}
      x2={grayLineEndX}
      y2={grayLineEndY}
      class="gray-line"
      marker-end="url(#sincos-arrow-gray)"
    />
    <text
      x={grayLabelX}
      y={grayLabelY}
      class="gray-label"
      text-anchor={grayLabelAnchor}
      dominant-baseline="middle"
      data-pulse={highlightLength || highlightAngle ? '' : undefined}
    >x={formatNum(grayMathX)}, y={formatNum(grayMathY)}</text>

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
    <text
      x={redLabelX}
      y={redLabelY}
      class="unit-label"
      text-anchor={redLabelAnchor}
      dominant-baseline="middle"
      data-pulse={highlightAngle ? '' : undefined}
    >x={formatNum(unitMathX)}, y={formatNum(unitMathY)}</text>

    <circle cx={ORIGIN_X} cy={ORIGIN_Y} r="4" class="origin-dot" />
  </svg>

  <p class="demo-legend">
    Серый — вектор с задаваемыми длиной и углом. Красный — единичный
    вектор (длина ровно 1) в том же самом направлении. Пунктир от
    красного вектора к осям — его проекции на `x` и `y`.
  </p>

  <div class="controls" style={`max-width: ${width}px;`}>
    <NumberField
      label="length"
      bind:value={length}
      min="2"
      max="6"
      step="0.5"
      onfocus={() => (highlightLength = true)}
      onblur={() => (highlightLength = false)}
    />
    <NumberField
      label="angle"
      bind:value={angle}
      min="0"
      max="360"
      step="1"
      onfocus={() => (highlightAngle = true)}
      onblur={() => (highlightAngle = false)}
    />
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

  .projection-line {
    stroke: #ef4444;
    stroke-width: 1.5;
    stroke-dasharray: 3 3;
    opacity: 0.6;
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
    stroke-width: 1.5;
    stroke-linecap: round;
  }

  .unit-arrow {
    fill: #ef4444;
  }

  .origin-dot {
    fill: var(--sl-color-text);
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

  .angle-label {
    fill: #ef4444;
    color: #ef4444;
    font-size: 14px;
    font-weight: 600;
    font-family: var(--__sl-font-mono, ui-monospace, monospace);
  }

  .controls {
    display: flex;
    align-items: center;
    gap: 1.5rem;
    margin-top: 0.75rem;
  }
</style>
