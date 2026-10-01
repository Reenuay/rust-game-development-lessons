<script>
  import NumberField from './NumberField.svelte';

  // Pure math illustration, no macroquad/WASM. A draggable vector from
  // the origin (same drag pattern as Rotate90Demo/ScalePivotDemo), with
  // a triangular arrowhead always drawn at its tip. A toggle reveals
  // the construction behind that triangle: step back along the
  // direction, then two perpendicular "wings" from that point — the
  // same three ingredients the lesson's draw_arrow() uses.
  let { width = 560 } = $props();

  const VW = 640;
  const VH = 460;
  const ORIGIN_X = 160;
  const ORIGIN_Y = 230;
  const PX_PER_UNIT = 20;
  const MAX_UNIT = 12;

  let vx = $state(9);
  let vy = $state(-6);
  let headLength = $state(30);
  let headWidth = $state(15);
  let showConstruction = $state(false);
  let dragging = $state(false);
  let svgEl = $state(null);

  function clampUnit(value) {
    return Math.max(-MAX_UNIT, Math.min(MAX_UNIT, Math.round(value)));
  }

  function pushOutIfZero(x, y) {
    if (x !== 0 || y !== 0) return { x, y };
    return { x: 1, y: 0 };
  }

  function updateFromPointer(clientX, clientY) {
    if (!svgEl) return;
    const rect = svgEl.getBoundingClientRect();
    const mathX = (clientX - rect.left) * (VW / rect.width);
    const mathY = (clientY - rect.top) * (VH / rect.height);
    const next = pushOutIfZero(
      clampUnit((mathX - ORIGIN_X) / PX_PER_UNIT),
      clampUnit((mathY - ORIGIN_Y) / PX_PER_UNIT),
    );
    vx = next.x;
    vy = next.y;
  }

  function onHandlePointerDown(event) {
    dragging = true;
    event.target.setPointerCapture(event.pointerId);
    updateFromPointer(event.clientX, event.clientY);
  }

  function onHandlePointerMove(event) {
    if (!dragging) return;
    updateFromPointer(event.clientX, event.clientY);
  }

  function onHandlePointerUp() {
    dragging = false;
  }

  function clampHeadLength(value) {
    return Math.round(Math.min(60, Math.max(8, value)));
  }

  function clampHeadWidth(value) {
    return Math.round(Math.min(30, Math.max(4, value)));
  }

  $effect(() => {
    headLength = clampHeadLength(headLength);
    headWidth = clampHeadWidth(headWidth);
  });

  let tipX = $derived(ORIGIN_X + vx * PX_PER_UNIT);
  let tipY = $derived(ORIGIN_Y + vy * PX_PER_UNIT);
  let vectorLengthPx = $derived(Math.hypot(vx, vy) * PX_PER_UNIT);
  let dirX = $derived((vx * PX_PER_UNIT) / vectorLengthPx);
  let dirY = $derived((vy * PX_PER_UNIT) / vectorLengthPx);

  // Never let the head eat into more of the shaft than is actually
  // there — otherwise dragging the tip in close would flip the
  // triangle past the origin.
  let effectiveHeadLength = $derived(Math.min(headLength, vectorLengthPx * 0.9));

  let backX = $derived(tipX - dirX * effectiveHeadLength);
  let backY = $derived(tipY - dirY * effectiveHeadLength);

  // Perpendicular to the direction — the 90° trick from «Поворот на 90°».
  let perpX = $derived(-dirY);
  let perpY = $derived(dirX);

  let wing1X = $derived(backX + perpX * headWidth);
  let wing1Y = $derived(backY + perpY * headWidth);
  let wing2X = $derived(backX - perpX * headWidth);
  let wing2Y = $derived(backY - perpY * headWidth);

  function toggleConstruction() {
    showConstruction = !showConstruction;
  }
</script>

<div class="arrow-construction-demo">
  <svg class="arrow-construction-svg" viewBox={`0 0 ${VW} ${VH}`} style={`max-width: ${width}px;`} bind:this={svgEl}>
    <line x1="0" y1={ORIGIN_Y} x2={VW} y2={ORIGIN_Y} class="axis-line" />
    <line x1={ORIGIN_X} y1="0" x2={ORIGIN_X} y2={VH} class="axis-line" />

    {#if showConstruction}
      <circle cx={backX} cy={backY} r="4" class="back-dot" />
      <text x={backX} y={backY - 12} class="back-label" text-anchor="middle">back</text>
      <line x1={backX} y1={backY} x2={wing1X} y2={wing1Y} class="wing-line" />
      <line x1={backX} y1={backY} x2={wing2X} y2={wing2Y} class="wing-line" />
      <text x={wing1X} y={wing1Y - 8} class="wing-label" text-anchor="middle">wing1</text>
      <text x={wing2X} y={wing2Y + 18} class="wing-label" text-anchor="middle">wing2</text>
    {/if}

    <!-- Shaft — only to `back`, same as the real draw_arrow(). -->
    <line x1={ORIGIN_X} y1={ORIGIN_Y} x2={backX} y2={backY} class="shaft-line" />

    <!-- The arrowhead itself, always visible. -->
    <polygon points={`${tipX},${tipY} ${wing1X},${wing1Y} ${wing2X},${wing2Y}`} class="head-triangle" />

    <circle
      cx={tipX}
      cy={tipY}
      r="10"
      class="drag-handle"
      class:dragging
      onpointerdown={onHandlePointerDown}
      onpointermove={onHandlePointerMove}
      onpointerup={onHandlePointerUp}
      onpointercancel={onHandlePointerUp}
    />
    <circle cx={ORIGIN_X} cy={ORIGIN_Y} r="4" class="origin-dot" />
  </svg>

  <p class="demo-legend">
    Тащи кончик стрелки мышью. Кнопка ниже показывает, как устроен сам
    наконечник: точка <code>back</code> — шаг назад от кончика вдоль
    направления, а <code>wing1</code>/<code>wing2</code> — два
    перпендикулярных отступа от неё в разные стороны.
  </p>

  <div class="controls" style={`max-width: ${width}px;`}>
    <NumberField label="head_length" bind:value={headLength} min="8" max="60" step="1" />
    <NumberField label="head_width" bind:value={headWidth} min="4" max="30" step="1" />
    <button type="button" class="construction-button" onclick={toggleConstruction}>
      {showConstruction ? 'Скрыть построение' : 'Показать построение'}
    </button>
  </div>
</div>

<style>
  .arrow-construction-demo {
    margin-block: 1rem;
  }

  .arrow-construction-svg {
    display: block;
    width: 100%;
    touch-action: none;
  }

  .demo-legend {
    margin: 0.75rem 0;
  }

  .axis-line {
    stroke: var(--sl-color-gray-3);
    stroke-width: 2;
    stroke-dasharray: 4 4;
  }

  .shaft-line {
    stroke: var(--sl-color-gray-2);
    stroke-width: 3;
    stroke-linecap: round;
  }

  .head-triangle {
    fill: #facc15;
    stroke: none;
  }

  .back-dot {
    fill: #7dd3fc;
  }

  .back-label {
    fill: #7dd3fc;
    font-size: 13px;
    font-family: var(--__sl-font-mono, ui-monospace, monospace);
    paint-order: stroke;
    stroke: var(--sl-color-bg);
    stroke-width: 4px;
  }

  .wing-line {
    stroke: #7dd3fc;
    stroke-width: 2;
    stroke-dasharray: 3 3;
  }

  .wing-label {
    fill: #7dd3fc;
    font-size: 13px;
    font-family: var(--__sl-font-mono, ui-monospace, monospace);
    paint-order: stroke;
    stroke: var(--sl-color-bg);
    stroke-width: 4px;
  }

  .origin-dot {
    fill: var(--sl-color-text);
  }

  .drag-handle {
    fill: var(--sl-color-bg);
    stroke: var(--sl-color-gray-2);
    stroke-width: 2;
    cursor: grab;
    touch-action: none;
  }

  .drag-handle.dragging {
    cursor: grabbing;
    fill: var(--sl-color-gray-2);
  }

  .controls {
    display: flex;
    align-items: center;
    flex-wrap: wrap;
    gap: 1.5rem;
    margin-top: 0.75rem;
  }

  .construction-button {
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

  .construction-button:hover {
    background: var(--sl-color-gray-6);
  }
</style>
