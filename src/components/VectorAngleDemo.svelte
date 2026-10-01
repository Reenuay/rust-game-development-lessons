<script>
  // Drag the gray vector's tip (same drag pattern as Rotate90Demo) and
  // watch its angle — computed live via atan2, exactly the formula the
  // lesson teaches — update alongside it. "Проверка" overlays a second
  // vector rebuilt from that same angle via cos/sin, to show it lands
  // right back on the original.
  let { width = 720 } = $props();

  const VW = 720;
  const VH = 560;
  const ORIGIN_X = 360;
  const ORIGIN_Y = 280;
  const PX_PER_UNIT = 20;
  const MAX_UNIT = 12;
  const ARC_RADIUS = 32;

  let vx = $state(9);
  let vy = $state(-6);
  let showCheck = $state(false);
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

  let length = $derived(Math.hypot(vx, vy));
  // The same order as the Rust code below: y first, x second.
  let angle = $derived(Math.atan2(vy, vx));
  let degrees = $derived((angle * 180) / Math.PI);

  // Rebuilt from the angle via cos/sin, for the "Проверка" overlay —
  // should land right back on (vx, vy).
  let checkX = $derived(Math.cos(angle) * length);
  let checkY = $derived(Math.sin(angle) * length);

  function tipOf(x, y) {
    return { x: ORIGIN_X + x * PX_PER_UNIT, y: ORIGIN_Y + y * PX_PER_UNIT };
  }

  let grayTip = $derived(tipOf(vx, vy));
  let checkTip = $derived(tipOf(checkX, checkY));

  let arcStartX = $derived(ORIGIN_X + ARC_RADIUS);
  let arcStartY = ORIGIN_Y;
  let arcEndX = $derived(ORIGIN_X + ARC_RADIUS * Math.cos(angle));
  let arcEndY = $derived(ORIGIN_Y + ARC_RADIUS * Math.sin(angle));
  let arcSweep = $derived(angle >= 0 ? 1 : 0);
  let arcPath = $derived(
    `M ${arcStartX} ${arcStartY} A ${ARC_RADIUS} ${ARC_RADIUS} 0 0 ${arcSweep} ${arcEndX} ${arcEndY}`,
  );

  function labelPos(tip) {
    const dx = tip.x - ORIGIN_X;
    const dy = tip.y - ORIGIN_Y;
    const len = Math.hypot(dx, dy) || 1;
    return { x: tip.x + (dx / len) * 30, y: tip.y + (dy / len) * 30 };
  }

  let grayLabel = $derived(labelPos(grayTip));

  function toggleCheck() {
    showCheck = !showCheck;
  }
</script>

<div class="vector-angle-demo">
  <svg class="vector-angle-svg" viewBox={`0 0 ${VW} ${VH}`} style={`max-width: ${width}px;`} bind:this={svgEl}>
    <line x1="0" y1={ORIGIN_Y} x2={VW} y2={ORIGIN_Y} class="axis-line" />
    <line x1={ORIGIN_X} y1="0" x2={ORIGIN_X} y2={VH} class="axis-line" />

    <path d={arcPath} class="arc-path" />

    {#if showCheck}
      <line x1={ORIGIN_X} y1={ORIGIN_Y} x2={checkTip.x} y2={checkTip.y} class="check-line" />
      <circle cx={checkTip.x} cy={checkTip.y} r="6" class="check-dot" />
    {/if}

    <line x1={ORIGIN_X} y1={ORIGIN_Y} x2={grayTip.x} y2={grayTip.y} class="vector-line" />
    <text x={grayLabel.x} y={grayLabel.y} class="vector-label" text-anchor="middle" dominant-baseline="middle"
      >({vx}, {vy})</text
    >

    <circle cx={ORIGIN_X} cy={ORIGIN_Y} r="4" class="origin-dot" />

    <circle
      cx={grayTip.x}
      cy={grayTip.y}
      r="10"
      class="drag-handle"
      class:dragging
      onpointerdown={onHandlePointerDown}
      onpointermove={onHandlePointerMove}
      onpointerup={onHandlePointerUp}
      onpointercancel={onHandlePointerUp}
    />
  </svg>

  <p class="demo-stats">
    <code>x = {vx}</code>, <code>y = {vy}</code> → <code>angle = {angle.toFixed(2)}</code> рад
    (≈ {Math.round(degrees)}°)
  </p>

  <p class="demo-legend">
    Тяни серый вектор за кончик мышью — угол под ним пересчитывается
    сразу по <code>x</code> и <code>y</code>. Кнопка ниже проверяет
    результат в обратную сторону: берёт этот самый угол и через
    <code>cos</code>/<code>sin</code> строит по нему вектор заново —
    зелёная точка должна лечь ровно на кончик серого.
  </p>

  <div class="controls" style={`max-width: ${width}px;`}>
    <button type="button" class="check-button" class:active={showCheck} onclick={toggleCheck}>
      {showCheck ? 'Скрыть проверку' : 'Показать проверку'}
    </button>
  </div>
</div>

<style>
  .vector-angle-demo {
    margin-block: 1rem;
  }

  .vector-angle-svg {
    display: block;
    width: 100%;
    touch-action: none;
  }

  .demo-stats {
    margin: 0.75rem 0;
    font-family: var(--__sl-font-mono, ui-monospace, monospace);
    font-size: var(--sl-text-sm);
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

  .vector-label {
    font-size: 15px;
    font-weight: 600;
    font-family: var(--__sl-font-mono, ui-monospace, monospace);
    fill: var(--sl-color-text);
    paint-order: stroke;
    stroke: var(--sl-color-bg);
    stroke-width: 5px;
    stroke-linejoin: round;
  }

  .check-line {
    stroke: #22c55e;
    stroke-width: 3;
    stroke-dasharray: 5 5;
  }

  .check-dot {
    fill: #22c55e;
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

  .check-button {
    margin: 0;
    height: 1.75rem;
    padding: 0 0.75rem;
    border: 1.5px solid var(--sl-color-gray-3);
    border-radius: 0.25rem;
    background: var(--sl-color-bg);
    color: var(--sl-color-text);
    font: inherit;
    font-family: var(--__sl-font-mono, ui-monospace, monospace);
    font-size: var(--sl-text-sm);
    line-height: 1.75rem;
    cursor: pointer;
  }

  .check-button:hover {
    background: var(--sl-color-gray-6);
  }

  .check-button.active {
    border-color: #22c55e;
    color: #22c55e;
  }
</style>
