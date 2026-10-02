<script>
  import NumberField from './NumberField.svelte';
  import RangeSlider from './RangeSlider.svelte';

  // Pure math illustration, same family as VectorAngleDemo/Rotate90Demo
  // (SVG + JS, no macroquad/WASM involved). The slider sets a "raw"
  // angle in degrees that's allowed to wander far outside -180..180 —
  // simulating an angle that's been accumulating for a while (as in
  // Rotation/Satellite) rather than a single fresh atan2 result. That
  // range is exactly what breaks the naive "add 360 once" fix: it only
  // ever undoes one wrap, so it still misses once raw wanders past a
  // second one. rem_euclid stays correct everywhere, which is the
  // whole point of the demo.
  let { width = 720 } = $props();

  const VW = 720;
  const VH = 420;
  const ORIGIN_X = 360;
  const ORIGIN_Y = 210;
  const ARROW_LENGTH = 160;
  const ARC_RADIUS = 40;

  let rawDegrees = $state(450);
  let highlightField = $state(null);

  function clampRaw(value) {
    return Math.max(-720, Math.min(720, Math.round(value)));
  }

  $effect(() => {
    rawDegrees = clampRaw(rawDegrees);
  });

  // The naive fix from the lesson: works only if raw is already within
  // one wrap of 0..360, exactly like atan2's own output range.
  let naiveDegrees = $derived(rawDegrees < 0 ? rawDegrees + 360 : rawDegrees);

  // rem_euclid's JS equivalent — always lands in 0..360, no matter how
  // many times raw has wrapped around in either direction.
  let correctDegrees = $derived(((rawDegrees % 360) + 360) % 360);

  let naiveOk = $derived(naiveDegrees === correctDegrees);

  let angleRad = $derived((correctDegrees * Math.PI) / 180);
  let tipX = $derived(ORIGIN_X + Math.cos(angleRad) * ARROW_LENGTH);
  let tipY = $derived(ORIGIN_Y + Math.sin(angleRad) * ARROW_LENGTH);

  let arcEndX = $derived(ORIGIN_X + Math.cos(angleRad) * ARC_RADIUS);
  let arcEndY = $derived(ORIGIN_Y + Math.sin(angleRad) * ARC_RADIUS);
  let arcLarge = $derived(correctDegrees > 180 ? 1 : 0);
  let arcPath = $derived(
    `M ${ORIGIN_X + ARC_RADIUS} ${ORIGIN_Y} A ${ARC_RADIUS} ${ARC_RADIUS} 0 ${arcLarge} 1 ${arcEndX} ${arcEndY}`,
  );
</script>

<div class="angle-normalize-demo">
  <svg class="angle-normalize-svg" viewBox={`0 0 ${VW} ${VH}`} style={`max-width: ${width}px;`}>
    <line x1="0" y1={ORIGIN_Y} x2={VW} y2={ORIGIN_Y} class="axis-line" />
    <line x1={ORIGIN_X} y1="0" x2={ORIGIN_X} y2={VH} class="axis-line" />

    <path d={arcPath} class="arc-path" />

    <line x1={ORIGIN_X} y1={ORIGIN_Y} x2={tipX} y2={tipY} class="vector-line" />
    <circle cx={tipX} cy={tipY} r="9" class="tip-dot" />
    <circle cx={ORIGIN_X} cy={ORIGIN_Y} r="4" class="origin-dot" />
  </svg>

  <p class="demo-stats">
    <code>raw = {rawDegrees}°</code> →
    наивный способ: <code class:bad={!naiveOk}>{naiveDegrees}°</code>
    {naiveOk ? '(совпадает)' : '(мимо — всё ещё не 0..360)'},
    <code>rem_euclid: {correctDegrees}°</code>
  </p>

  <p class="demo-legend">
    Двигай ползунок — <code>raw</code> изображает угол, который успел
    накопиться за много кадров и мог обернуться вокруг оси не один раз.
    Стрелка на картинке всегда стоит под правильным углом,
    <code>rem_euclid</code>-результатом. «Наивный способ» — прибавить
    360° один раз, если угол отрицательный, — совпадает с ним, только
    пока <code>raw</code> не ушёл дальше одного оборота.
  </p>

  <div class="controls" style={`max-width: ${width}px;`}>
    <RangeSlider
      label="raw"
      bind:value={rawDegrees}
      min="-720"
      max="720"
      step="1"
      onfocus={() => (highlightField = 'raw')}
      onblur={() => (highlightField = null)}
    />
    <NumberField
      label="raw"
      bind:value={rawDegrees}
      min="-720"
      max="720"
      step="1"
      onfocus={() => (highlightField = 'raw')}
      onblur={() => (highlightField = null)}
    />
  </div>
</div>

<style>
  .angle-normalize-demo {
    margin-block: 1rem;
  }

  .angle-normalize-svg {
    display: block;
    width: 100%;
    touch-action: none;
  }

  .demo-stats {
    margin: 0.75rem 0;
    font-family: var(--__sl-font-mono, ui-monospace, monospace);
    font-size: var(--sl-text-sm);
  }

  .demo-stats code.bad {
    color: #ef4444;
  }

  .demo-legend {
    margin: 0.75rem 0;
  }

  .axis-line {
    stroke: var(--sl-color-gray-3);
    stroke-width: 2;
    stroke-dasharray: 4 4;
  }

  .arc-path {
    fill: none;
    stroke: #f97316;
    stroke-width: 3;
  }

  .vector-line {
    stroke: var(--sl-color-gray-2);
    stroke-width: 3;
    stroke-linecap: round;
  }

  .tip-dot {
    fill: #22c55e;
  }

  .origin-dot {
    fill: var(--sl-color-text);
  }

  .controls {
    display: flex;
    align-items: center;
    flex-wrap: wrap;
    gap: 1.5rem;
    margin-top: 0.75rem;
  }
</style>
