<script>
  import { untrack } from 'svelte';

  // Pure math illustration, no macroquad/WASM. Same two-panel idea as
  // ScalePivotDemo.svelte, but rotating continuously instead of
  // scaling once: left panel rotates B around A's tip correctly,
  // right panel applies the same rotation to the absolute sum
  // directly — which spins everything around the true origin instead.
  //
  // Rotation keeps going forever once started (unlike a one-shot
  // scale factor), so both panels are centered on their own origin
  // with equal room in every direction, not biased toward one corner.
  let { width = 340 } = $props();

  const VW = 400;
  const VH = 400;
  const ORIGIN_X = 200;
  const ORIGIN_Y = 200;
  const PX_PER_UNIT = 12;
  const MAX_UNIT = 9;

  // Big vector A — draggable, shared by both panels.
  let ax = $state(6);
  let ay = $state(-3);

  // Small vector B — fixed offset from A's tip, at rest (angle = 0).
  const bx0 = 2;
  const by0 = 1;
  const blen = Math.hypot(bx0, by0);
  const b0Angle = Math.atan2(by0, bx0);

  let angle = $state(0);
  let spinning = $state(false);
  let animationFrame;
  const SPEED = 0.02;

  function clampUnit(value) {
    return Math.max(-MAX_UNIT, Math.min(MAX_UNIT, Math.round(value)));
  }

  function updateFromPointer(svgEl, clientX, clientY) {
    if (!svgEl) return;
    const rect = svgEl.getBoundingClientRect();
    const mathX = (clientX - rect.left) * (VW / rect.width);
    const mathY = (clientY - rect.top) * (VH / rect.height);
    ax = clampUnit((mathX - ORIGIN_X) / PX_PER_UNIT);
    ay = clampUnit((mathY - ORIGIN_Y) / PX_PER_UNIT);
  }

  let leftSvgEl = $state(null);
  let dragging = $state(false);

  function onHandlePointerDown(event) {
    dragging = true;
    event.target.setPointerCapture(event.pointerId);
    updateFromPointer(leftSvgEl, event.clientX, event.clientY);
  }

  function onHandlePointerMove(event) {
    if (!dragging) return;
    updateFromPointer(leftSvgEl, event.clientX, event.clientY);
  }

  function onHandlePointerUp() {
    dragging = false;
  }

  function step() {
    angle += SPEED;
    animationFrame = requestAnimationFrame(step);
  }

  function toggleSpin() {
    spinning = !spinning;
    if (spinning) {
      animationFrame = requestAnimationFrame(step);
    } else {
      cancelAnimationFrame(animationFrame);
    }
  }

  // Dragging A starts over — resets the angle instead of leaving the
  // two panels mid-spin relative to a pivot that just moved.
  $effect(() => {
    ax;
    ay;
    if (!untrack(() => spinning)) {
      angle = 0;
    }
  });

  $effect(() => {
    return () => cancelAnimationFrame(animationFrame);
  });

  function toTip(x, y) {
    return { x: ORIGIN_X + x * PX_PER_UNIT, y: ORIGIN_Y + y * PX_PER_UNIT };
  }

  // Left (correct): B keeps its own length and orbits A's tip — its
  // direction is just b0's starting angle plus however far we've
  // spun, same idea as RotationDemo's single vector.
  let leftBx = $derived(Math.cos(b0Angle + angle) * blen);
  let leftBy = $derived(Math.sin(b0Angle + angle) * blen);

  // Right (wrong): the absolute starting point (A0 + B0) gets rotated
  // directly — which is the same as rotating A0 and B0 each on their
  // own and keeping them stacked tip to tail, since rotation
  // distributes over addition just like scaling did.
  let sumLen0 = $derived(Math.hypot(ax + bx0, ay + by0));
  let sumAngle0 = $derived(Math.atan2(ay + by0, ax + bx0));
  let aLen0 = $derived(Math.hypot(ax, ay));
  let aAngle0 = $derived(Math.atan2(ay, ax));

  let rightAx = $derived(Math.cos(aAngle0 + angle) * aLen0);
  let rightAy = $derived(Math.sin(aAngle0 + angle) * aLen0);
  let rightSumX = $derived(Math.cos(sumAngle0 + angle) * sumLen0);
  let rightSumY = $derived(Math.sin(sumAngle0 + angle) * sumLen0);

  let aTip = $derived(toTip(ax, ay));
  let leftSumTip = $derived(toTip(ax + leftBx, ay + leftBy));
  let leftBTip = $derived(leftSumTip);
  let rightATip = $derived(toTip(rightAx, rightAy));
  let rightSumTip = $derived(toTip(rightSumX, rightSumY));
</script>

<div class="rotate-pivot-demo">
  <div class="panels" style={`max-width: ${width * 2 + 24}px;`}>
    <div class="panel">
      <p class="panel-title">Правильно</p>
      <svg class="pivot-svg" viewBox={`0 0 ${VW} ${VH}`} style={`max-width: ${width}px;`} bind:this={leftSvgEl}>
        <defs>
          <marker id="rpivot-arrow-a" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="4.5" markerHeight="4.5" orient="auto">
            <path d="M0,1.5 L10,5 L0,8.5 Z" class="arrow-a" />
          </marker>
          <marker id="rpivot-arrow-b-left" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="4.5" markerHeight="4.5" orient="auto">
            <path d="M0,1.5 L10,5 L0,8.5 Z" class="arrow-b" />
          </marker>
          <marker id="rpivot-arrow-sum" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="4.5" markerHeight="4.5" orient="auto">
            <path d="M0,1.5 L10,5 L0,8.5 Z" class="arrow-sum" />
          </marker>
        </defs>

        <line x1="0" y1={ORIGIN_Y} x2={VW} y2={ORIGIN_Y} class="axis-line" />
        <line x1={ORIGIN_X} y1="0" x2={ORIGIN_X} y2={VH} class="axis-line" />

        <line x1={ORIGIN_X} y1={ORIGIN_Y} x2={leftSumTip.x} y2={leftSumTip.y} class="sum-line" marker-end="url(#rpivot-arrow-sum)" />
        <line x1={ORIGIN_X} y1={ORIGIN_Y} x2={aTip.x} y2={aTip.y} class="vector-a" marker-end="url(#rpivot-arrow-a)" />
        <line x1={aTip.x} y1={aTip.y} x2={leftBTip.x} y2={leftBTip.y} class="vector-b" marker-end="url(#rpivot-arrow-b-left)" />

        <circle
          cx={aTip.x}
          cy={aTip.y}
          r="9"
          class="drag-handle"
          class:dragging
          onpointerdown={onHandlePointerDown}
          onpointermove={onHandlePointerMove}
          onpointerup={onHandlePointerUp}
          onpointercancel={onHandlePointerUp}
        />
        <circle cx={ORIGIN_X} cy={ORIGIN_Y} r="4" class="origin-dot" />
      </svg>
    </div>

    <div class="panel">
      <p class="panel-title">Неправильно</p>
      <svg class="pivot-svg" viewBox={`0 0 ${VW} ${VH}`} style={`max-width: ${width}px;`}>
        <defs>
          <marker id="rpivot-arrow-a-right" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="4.5" markerHeight="4.5" orient="auto">
            <path d="M0,1.5 L10,5 L0,8.5 Z" class="arrow-a" />
          </marker>
          <marker id="rpivot-arrow-b-right" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="4.5" markerHeight="4.5" orient="auto">
            <path d="M0,1.5 L10,5 L0,8.5 Z" class="arrow-b" />
          </marker>
          <marker id="rpivot-arrow-sum-right" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="4.5" markerHeight="4.5" orient="auto">
            <path d="M0,1.5 L10,5 L0,8.5 Z" class="arrow-sum" />
          </marker>
        </defs>

        <line x1="0" y1={ORIGIN_Y} x2={VW} y2={ORIGIN_Y} class="axis-line" />
        <line x1={ORIGIN_X} y1="0" x2={ORIGIN_X} y2={VH} class="axis-line" />

        <line x1={ORIGIN_X} y1={ORIGIN_Y} x2={rightSumTip.x} y2={rightSumTip.y} class="sum-line" marker-end="url(#rpivot-arrow-sum-right)" />

        <!-- Both A and B swing around the true origin here — rotating
             the sum directly is the same as rotating A and B each on
             their own and keeping them stacked tip to tail. -->
        <line x1={ORIGIN_X} y1={ORIGIN_Y} x2={rightATip.x} y2={rightATip.y} class="vector-a" marker-end="url(#rpivot-arrow-a-right)" />
        <line x1={rightATip.x} y1={rightATip.y} x2={rightSumTip.x} y2={rightSumTip.y} class="vector-b" marker-end="url(#rpivot-arrow-b-right)" />

        <circle cx={ORIGIN_X} cy={ORIGIN_Y} r="4" class="origin-dot" />
      </svg>
    </div>
  </div>

  <p class="demo-legend">
    Серый — вектор A, тащи его мышью за кончик в левой картинке —
    правая повторяет за ней. Бледно-голубой — вектор B, растущий из
    кончика A. Зелёный пунктир — их сумма. Кнопка крутит B вокруг A
    двумя разными способами одновременно, пока не остановишь.
  </p>

  <div class="controls">
    <button type="button" class="spin-button" onclick={toggleSpin}>
      {spinning ? 'Стоп' : 'Крутить'}
    </button>
  </div>
</div>

<style>
  .rotate-pivot-demo {
    margin-block: 1rem;
  }

  .panels {
    display: flex;
    gap: 1.5rem;
    flex-wrap: wrap;
  }

  .panel {
    /* Starlight's prose CSS adds margin-top to any element that
       isn't the first child of its parent (for normal paragraph
       spacing) — it doesn't know these two <div>s are a side-by-side
       row, not stacked text, so without this the second panel sits
       lower than the first. */
    margin: 0;
    flex: 1 1 200px;
    min-width: 200px;
  }

  .panel-title {
    margin: 0 0 0.25rem;
    font-size: var(--sl-text-sm);
    font-weight: 600;
    color: var(--sl-color-text);
  }

  .pivot-svg {
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

  .vector-a {
    stroke: var(--sl-color-gray-2);
    stroke-width: 3;
    stroke-linecap: round;
  }

  .arrow-a {
    fill: var(--sl-color-gray-2);
  }

  .vector-b {
    stroke: #7dd3fc;
    stroke-width: 3;
    stroke-linecap: round;
  }

  .arrow-b {
    fill: #7dd3fc;
  }

  .sum-line {
    stroke: #22c55e;
    stroke-width: 2.5;
    stroke-dasharray: 6 5;
    stroke-linecap: round;
  }

  .arrow-sum {
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
    margin-top: 0.75rem;
  }

  .spin-button {
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

  .spin-button:hover {
    background: var(--sl-color-gray-6);
  }
</style>
