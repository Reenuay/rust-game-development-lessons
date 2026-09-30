<script>
  import NumberField from './NumberField.svelte';

  // Pure math illustration, no macroquad/WASM — same idea as
  // ScaleDemo.svelte. Gray vector with adjustable length/angle, a red
  // unit vector (fixed on-screen length, standing for length 1) drawn
  // over it in the same direction, and a red arc marking the angle
  // between the x-axis and the vector. The numbers themselves live in
  // a plain HTML readout under the SVG, not as in-drawing labels —
  // packing x/y text right next to the lines made them collide with
  // each other and with the axis at angles near 0°/180°/360°.
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
  const VW = 600;
  const VH = 540;
  const ORIGIN_X = 300;
  const ORIGIN_Y = 270;
  // How many pixels stand for "1" — the red vector is always exactly
  // this long, since it always has length 1.
  const PX_PER_UNIT = 40;
  const UNIT_PX = PX_PER_UNIT;
  const ARC_RADIUS = 28;

  let radians = $derived((angle * Math.PI) / 180);

  // The vector's own x, y — not pixels, the actual math values (length
  // times cos/sin) — same numbers the readout below shows.
  let grayMathX = $derived(length * Math.cos(radians));
  let grayMathY = $derived(length * Math.sin(radians));
  let unitMathX = $derived(Math.cos(radians));
  let unitMathY = $derived(Math.sin(radians));

  let grayTipX = $derived(ORIGIN_X + grayMathX * PX_PER_UNIT);
  let grayTipY = $derived(ORIGIN_Y + grayMathY * PX_PER_UNIT);
  let unitTipX = $derived(ORIGIN_X + unitMathX * UNIT_PX);
  let unitTipY = $derived(ORIGIN_Y + unitMathY * UNIT_PX);

  let arcStartX = $derived(ORIGIN_X + ARC_RADIUS);
  let arcStartY = ORIGIN_Y;
  let arcEndX = $derived(ORIGIN_X + ARC_RADIUS * Math.cos(radians));
  let arcEndY = $derived(ORIGIN_Y + ARC_RADIUS * Math.sin(radians));
  let arcLargeFlag = $derived(angle > 180 ? 1 : 0);
  let arcPath = $derived(
    `M ${arcStartX} ${arcStartY} A ${ARC_RADIUS} ${ARC_RADIUS} 0 ${arcLargeFlag} 1 ${arcEndX} ${arcEndY}`,
  );
</script>

<div class="sincos-demo">
  <svg class="sincos-svg" viewBox={`0 0 ${VW} ${VH}`} style={`max-width: ${width}px;`}>
    <defs>
      <marker id="sincos-arrow-gray" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="7" markerHeight="7" orient="auto">
        <path d="M0,0 L10,5 L0,10 Z" class="gray-arrow" />
      </marker>
      <marker id="sincos-arrow-red" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="7" markerHeight="7" orient="auto">
        <path d="M0,0 L10,5 L0,10 Z" class="unit-arrow" />
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

    <!-- Gray vector — adjustable length and angle. -->
    <line
      x1={ORIGIN_X}
      y1={ORIGIN_Y}
      x2={grayTipX}
      y2={grayTipY}
      class="gray-line"
      marker-end="url(#sincos-arrow-gray)"
    />

    <!-- Unit vector — same origin, same direction, fixed on-screen
         length, because its length is always exactly 1. -->
    <line
      x1={ORIGIN_X}
      y1={ORIGIN_Y}
      x2={unitTipX}
      y2={unitTipY}
      class="unit-line"
      marker-end="url(#sincos-arrow-red)"
    />

    <circle cx={ORIGIN_X} cy={ORIGIN_Y} r="4" class="origin-dot" />
  </svg>

  <p class="demo-legend">
    Серый — вектор с задаваемыми длиной и углом. Красный — единичный
    вектор (длина ровно 1) в том же самом направлении. Пунктир от
    красного вектора к осям — его проекции на `x` и `y`.
  </p>

  <div class="readout" style={`max-width: ${width}px;`}>
    <span class="readout-gray" data-pulse={highlightLength || highlightAngle ? '' : undefined}>
      серый: x={formatNum(grayMathX)}, y={formatNum(grayMathY)}
    </span>
    <span class="readout-unit" data-pulse={highlightAngle ? '' : undefined}>
      единичный: x={formatNum(unitMathX)}, y={formatNum(unitMathY)}
    </span>
    <span class="readout-angle" data-pulse={highlightAngle ? '' : undefined}>угол: {angle}°</span>
  </div>

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
    stroke-width: 4;
    stroke-linecap: round;
  }

  .gray-arrow {
    fill: var(--sl-color-gray-2);
  }

  .unit-line {
    stroke: #ef4444;
    stroke-width: 3;
    stroke-linecap: round;
  }

  .unit-arrow {
    fill: #ef4444;
  }

  .origin-dot {
    fill: var(--sl-color-text);
  }

  .readout {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem 1.5rem;
    margin: 0.5rem 0 0.75rem;
    font-family: var(--__sl-font-mono, ui-monospace, monospace);
    font-size: var(--sl-text-sm);
  }

  .readout-gray {
    color: var(--sl-color-text);
  }

  .readout-unit,
  .readout-angle {
    color: #ef4444;
    font-weight: 600;
  }

  .controls {
    display: flex;
    align-items: center;
    gap: 1.5rem;
    margin-top: 0.75rem;
  }
</style>
