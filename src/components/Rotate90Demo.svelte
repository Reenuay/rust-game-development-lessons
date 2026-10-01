<script>
  // Pure math illustration, no macroquad/WASM — same idea as
  // SinCosDemo.svelte, but the gray vector is set by dragging its tip
  // around with the pointer instead of typing into a NumberField
  // (there's no single number to type here — it's a point in 2D).
  // Three buttons each reveal one of the three "free" 90°-multiple
  // rotations, computed from the gray vector's own x/y by the exact
  // swap-and-negate formulas the lesson teaches, so the picture is the
  // proof: drag the vector anywhere, the numbers on the rotated copies
  // always match the formula.
  let { width = 720 } = $props();

  // SVG viewBox geometry — fixed internal units, scaled to fit `width`
  // via CSS.
  const VW = 720;
  const VH = 560;
  const ORIGIN_X = 360;
  const ORIGIN_Y = 280;
  // How many pixels stand for "1" in the vector's own x/y.
  const PX_PER_UNIT = 20;
  const MAX_UNIT = 12;

  let vx = $state(10);
  let vy = $state(3);

  let show180 = $state(false);
  let show90 = $state(false);
  let showMinus90 = $state(false);

  let dragging = $state(false);

  let svgEl = $state(null);

  function clampUnit(value) {
    return Math.max(-MAX_UNIT, Math.min(MAX_UNIT, Math.round(value)));
  }

  // A vector collapsed to (0, 0) has no direction to rotate — nudge it
  // back out along whatever direction it was last headed, or straight
  // right if it lands exactly on the origin.
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

  // The three "free" rotations — exactly the formulas the lesson text
  // teaches, applied live to whatever the gray vector currently is.
  let v180 = $derived({ x: -vx, y: -vy });
  let v90 = $derived({ x: -vy, y: vx });
  let vMinus90 = $derived({ x: vy, y: -vx });

  function tipOf(v) {
    return { x: ORIGIN_X + v.x * PX_PER_UNIT, y: ORIGIN_Y + v.y * PX_PER_UNIT };
  }

  let grayTip = $derived(tipOf({ x: vx, y: vy }));
  let tip180 = $derived(tipOf(v180));
  let tip90 = $derived(tipOf(v90));
  let tipMinus90 = $derived(tipOf(vMinus90));

  // Label sits further out than the tip, along the same direction the
  // vector already points — far enough to clear the draggable handle
  // (whose own radius is 10) without needing per-case collision
  // nudging.
  function labelPos(tip) {
    const dx = tip.x - ORIGIN_X;
    const dy = tip.y - ORIGIN_Y;
    const len = Math.hypot(dx, dy) || 1;
    return { x: tip.x + (dx / len) * 36, y: tip.y + (dy / len) * 36 };
  }

  let grayLabel = $derived(labelPos(grayTip));
  let label180 = $derived(labelPos(tip180));
  let label90 = $derived(labelPos(tip90));
  let labelMinus90 = $derived(labelPos(tipMinus90));
</script>

<div class="rotate90-demo">
  <svg
    class="rotate90-svg"
    viewBox={`0 0 ${VW} ${VH}`}
    style={`max-width: ${width}px;`}
    bind:this={svgEl}
  >
    <defs>
      <marker id="rotate90-arrow-180" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="4.5" markerHeight="4.5" orient="auto">
        <path d="M0,1.5 L10,5 L0,8.5 Z" class="arrow-180" />
      </marker>
      <marker id="rotate90-arrow-90" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="4.5" markerHeight="4.5" orient="auto">
        <path d="M0,1.5 L10,5 L0,8.5 Z" class="arrow-90" />
      </marker>
      <marker id="rotate90-arrow-minus90" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="4.5" markerHeight="4.5" orient="auto">
        <path d="M0,1.5 L10,5 L0,8.5 Z" class="arrow-minus90" />
      </marker>
    </defs>

    <!-- Full x/y axes through the origin — just orientation. -->
    <line x1="0" y1={ORIGIN_Y} x2={VW} y2={ORIGIN_Y} class="axis-line" />
    <line x1={ORIGIN_X} y1="0" x2={ORIGIN_X} y2={VH} class="axis-line" />

    {#if show180}
      <line x1={ORIGIN_X} y1={ORIGIN_Y} x2={tip180.x} y2={tip180.y} class="vector-line line-180" marker-end="url(#rotate90-arrow-180)" />
      <text x={label180.x} y={label180.y} class="vector-label label-180" text-anchor="middle" dominant-baseline="middle"
        >({v180.x}, {v180.y})</text
      >
    {/if}
    {#if show90}
      <line x1={ORIGIN_X} y1={ORIGIN_Y} x2={tip90.x} y2={tip90.y} class="vector-line line-90" marker-end="url(#rotate90-arrow-90)" />
      <text x={label90.x} y={label90.y} class="vector-label label-90" text-anchor="middle" dominant-baseline="middle"
        >({v90.x}, {v90.y})</text
      >
    {/if}
    {#if showMinus90}
      <line x1={ORIGIN_X} y1={ORIGIN_Y} x2={tipMinus90.x} y2={tipMinus90.y} class="vector-line line-minus90" marker-end="url(#rotate90-arrow-minus90)" />
      <text x={labelMinus90.x} y={labelMinus90.y} class="vector-label label-minus90" text-anchor="middle" dominant-baseline="middle"
        >({vMinus90.x}, {vMinus90.y})</text
      >
    {/if}

    <!-- Gray vector — the one you drag. No arrowhead marker: the drag
         handle sits right at the tip and would hide it anyway. -->
    <line x1={ORIGIN_X} y1={ORIGIN_Y} x2={grayTip.x} y2={grayTip.y} class="vector-line line-gray" />
    <text x={grayLabel.x} y={grayLabel.y} class="vector-label label-gray" text-anchor="middle" dominant-baseline="middle"
      >({vx}, {vy})</text
    >

    <circle cx={ORIGIN_X} cy={ORIGIN_Y} r="4" class="origin-dot" />

    <!-- Draggable handle on the gray vector's tip. -->
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

  <p class="demo-legend">
    Тяни серый вектор за кончик мышью — его числа меняются. Кнопки
    ниже показывают остальные три вектора, посчитанные по формулам из
    урока прямо из текущих чисел серого — сравни их с тем, что
    получается на картинке.
  </p>

  <div class="controls" style={`max-width: ${width}px;`}>
    <button type="button" class="rotate-button button-180" class:active={show180} onclick={() => (show180 = !show180)}>
      180°
    </button>
    <button type="button" class="rotate-button button-90" class:active={show90} onclick={() => (show90 = !show90)}>
      90°
    </button>
    <button type="button" class="rotate-button button-minus90" class:active={showMinus90} onclick={() => (showMinus90 = !showMinus90)}>
      −90°
    </button>
  </div>
</div>

<style>
  .rotate90-demo {
    margin-block: 1rem;
  }

  .rotate90-svg {
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

  .vector-line {
    stroke-width: 3;
    stroke-linecap: round;
  }

  .line-gray {
    stroke: var(--sl-color-gray-2);
  }

  .line-180 {
    stroke: #a855f7;
  }

  .arrow-180 {
    fill: #a855f7;
  }

  .line-90 {
    stroke: #ef4444;
  }

  .arrow-90 {
    fill: #ef4444;
  }

  .line-minus90 {
    stroke: #3b82f6;
  }

  .arrow-minus90 {
    fill: #3b82f6;
  }

  .vector-label {
    font-size: 15px;
    font-weight: 600;
    font-family: var(--__sl-font-mono, ui-monospace, monospace);
    /* Background-colored outline behind the text, drawn before the
       fill, so the numbers read cleanly over the axes or other
       vectors instead of blending into them. */
    paint-order: stroke;
    stroke: var(--sl-color-bg);
    stroke-width: 5px;
    stroke-linejoin: round;
  }

  .label-gray {
    fill: var(--sl-color-text);
  }

  .label-180 {
    fill: #a855f7;
  }

  .label-90 {
    fill: #ef4444;
  }

  .label-minus90 {
    fill: #3b82f6;
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
    gap: 0.75rem;
    margin-top: 0.75rem;
  }

  .rotate-button {
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

  .rotate-button:hover {
    background: var(--sl-color-gray-6);
  }

  .button-180.active {
    border-color: #a855f7;
    color: #a855f7;
  }

  .button-90.active {
    border-color: #ef4444;
    color: #ef4444;
  }

  .button-minus90.active {
    border-color: #3b82f6;
    color: #3b82f6;
  }
</style>
