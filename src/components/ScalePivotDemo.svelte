<script>
  import { untrack } from 'svelte';

  // Pure math illustration, no macroquad/WASM. Two side-by-side SVG
  // panels, same idea as Rotate90Demo's draggable vector, but now
  // showing point + vector (as in vector.mdx) and what goes wrong if
  // you scale the absolute sum instead of the small vector on its own.
  //
  // Panel A (correct): to stretch B relative to A's tip, slide B back
  // to the origin, scale it there, slide it back to A's tip.
  // Panel B (wrong): scale the absolute sum (origin to A+B) directly.
  //
  // Only panel A's big vector is draggable; panel B always mirrors the
  // same A and B. One button drives both animations together so the
  // difference is visible side by side, not just described.
  let { width = 340 } = $props();

  const VW = 480;
  const VH = 420;
  const ORIGIN_X = 120;
  const ORIGIN_Y = 330;
  const PX_PER_UNIT = 16;
  const MAX_UNIT = 12;

  // Big vector A — draggable, shared by both panels.
  let ax = $state(8);
  let ay = $state(-5);

  // Small vector B — fixed, drawn from A's tip to A's tip + B.
  const bx = 2;
  const by = 1;
  const SCALE = 2;

  let animating = $state(false);
  // 0 = at rest (B sitting at A's tip, unscaled), 1 = fully animated
  // (B slid to origin, scaled, slid back).
  let progress = $state(0);
  let scaledOnce = $state(false);

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

  // Phases of the "correct" animation, driven by one 0..1 progress:
  // 0.0-0.33 slide B from A's tip to the origin, 0.33-0.66 scale it in
  // place, 0.66-1.0 slide it back to A's tip (now scaled).
  function phase(t, start, end) {
    return Math.max(0, Math.min(1, (t - start) / (end - start)));
  }

  function ease(t) {
    return t < 0.5 ? 2 * t * t : 1 - (-2 * t + 2) ** 2 / 2;
  }

  // How far B has slid from "based at A" (0) to "based at origin" (1).
  let slideOut = $derived(ease(phase(progress, 0, 0.33)));
  // How much of the scale has been applied, 0 = original length, 1 = full SCALE.
  let scaleAmount = $derived(ease(phase(progress, 0.33, 0.66)));
  // How far B has slid back from origin (0) to A's tip (1).
  let slideBack = $derived(ease(phase(progress, 0.66, 1)));

  let currentScale = $derived(1 + (SCALE - 1) * scaleAmount);
  // B's base point: starts at A, slides to origin, stays there while
  // scaling, then slides back to A.
  let baseT = $derived(slideOut - slideBack);
  let bBaseX = $derived(ax * (1 - baseT));
  let bBaseY = $derived(ay * (1 - baseT));

  let leftBx = $derived(bBaseX + bx * currentScale);
  let leftBy = $derived(bBaseY + by * currentScale);

  // The dashed sum line only makes sense once B is back at rest
  // (either fully unscaled at the start, or fully scaled at the end)
  // — mid-animation there's no "final answer" to show yet.
  let leftSumVisible = $derived(progress === 0 || progress === 1);
  let leftSumX = $derived(progress === 1 ? leftBx : ax + bx);
  let leftSumY = $derived(progress === 1 ? leftBy : ay + by);

  // Right panel: scales the absolute sum directly — A itself gets
  // dragged into the multiplication along with B, which is the bug.
  let rightScale = $derived(1 + (SCALE - 1) * ease(progress));
  let rightSumX = $derived((ax + bx) * rightScale);
  let rightSumY = $derived((ay + by) * rightScale);

  let animationFrame;

  function runAnimation() {
    if (animating) return;
    animating = true;
    progress = 0;
    const duration = 2200;
    const startTime = performance.now();

    function step(now) {
      const t = Math.min(1, (now - startTime) / duration);
      progress = t;
      if (t < 1) {
        animationFrame = requestAnimationFrame(step);
      } else {
        animating = false;
        scaledOnce = true;
      }
    }

    animationFrame = requestAnimationFrame(step);
  }

  function reset() {
    cancelAnimationFrame(animationFrame);
    animating = false;
    progress = 0;
    scaledOnce = false;
  }

  $effect(() => {
    // Dragging A mid-demo invalidates whatever scaled state we were
    // showing — back to the unscaled starting picture. Reads
    // `animating` through untrack so this effect reruns only when ax
    // or ay actually change (a drag) — not merely because `animating`
    // itself flips off when the animation finishes on its own, which
    // would otherwise immediately reset the result we just animated to.
    ax;
    ay;
    if (!untrack(() => animating)) reset();
  });

  $effect(() => {
    return () => cancelAnimationFrame(animationFrame);
  });

  function toTip(x, y) {
    return { x: ORIGIN_X + x * PX_PER_UNIT, y: ORIGIN_Y + y * PX_PER_UNIT };
  }

  let aTip = $derived(toTip(ax, ay));
  let leftSumTip = $derived(toTip(leftSumX, leftSumY));
  let leftBBaseTip = $derived(toTip(bBaseX, bBaseY));
  let leftBTip = $derived(toTip(leftBx, leftBy));
  let rightSumTip = $derived(toTip(rightSumX, rightSumY));
  let rightBTip = $derived(toTip(ax + bx, ay + by));
</script>

<div class="scale-pivot-demo">
  <div class="panels" style={`max-width: ${width * 2 + 24}px;`}>
    <div class="panel">
      <p class="panel-title">Правильно</p>
      <svg class="pivot-svg" viewBox={`0 0 ${VW} ${VH}`} style={`max-width: ${width}px;`} bind:this={leftSvgEl}>
        <defs>
          <marker id="pivot-arrow-a" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="4.5" markerHeight="4.5" orient="auto">
            <path d="M0,1.5 L10,5 L0,8.5 Z" class="arrow-a" />
          </marker>
          <marker id="pivot-arrow-b-left" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="4.5" markerHeight="4.5" orient="auto">
            <path d="M0,1.5 L10,5 L0,8.5 Z" class="arrow-b" />
          </marker>
          <marker id="pivot-arrow-sum" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="4.5" markerHeight="4.5" orient="auto">
            <path d="M0,1.5 L10,5 L0,8.5 Z" class="arrow-sum" />
          </marker>
        </defs>

        <line x1="0" y1={ORIGIN_Y} x2={VW} y2={ORIGIN_Y} class="axis-line" />
        <line x1={ORIGIN_X} y1="0" x2={ORIGIN_X} y2={VH} class="axis-line" />

        {#if leftSumVisible}
          <line x1={ORIGIN_X} y1={ORIGIN_Y} x2={leftSumTip.x} y2={leftSumTip.y} class="sum-line" marker-end="url(#pivot-arrow-sum)" />
        {/if}

        <!-- Big vector A. -->
        <line x1={ORIGIN_X} y1={ORIGIN_Y} x2={aTip.x} y2={aTip.y} class="vector-a" marker-end="url(#pivot-arrow-a)" />

        <!-- Small vector B, from its current (possibly mid-slide) base. -->
        <line x1={leftBBaseTip.x} y1={leftBBaseTip.y} x2={leftBTip.x} y2={leftBTip.y} class="vector-b" marker-end="url(#pivot-arrow-b-left)" />

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
          <marker id="pivot-arrow-a-right" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="4.5" markerHeight="4.5" orient="auto">
            <path d="M0,1.5 L10,5 L0,8.5 Z" class="arrow-a" />
          </marker>
          <marker id="pivot-arrow-b-right" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="4.5" markerHeight="4.5" orient="auto">
            <path d="M0,1.5 L10,5 L0,8.5 Z" class="arrow-b" />
          </marker>
          <marker id="pivot-arrow-sum-right" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="4.5" markerHeight="4.5" orient="auto">
            <path d="M0,1.5 L10,5 L0,8.5 Z" class="arrow-sum" />
          </marker>
        </defs>

        <line x1="0" y1={ORIGIN_Y} x2={VW} y2={ORIGIN_Y} class="axis-line" />
        <line x1={ORIGIN_X} y1="0" x2={ORIGIN_X} y2={VH} class="axis-line" />

        <line x1={ORIGIN_X} y1={ORIGIN_Y} x2={rightSumTip.x} y2={rightSumTip.y} class="sum-line" marker-end="url(#pivot-arrow-sum-right)" />

        <!-- A and B themselves never change here — only the dashed
             sum line reacts, because the bug multiplies the sum
             directly instead of touching B on its own. -->
        <line x1={ORIGIN_X} y1={ORIGIN_Y} x2={aTip.x} y2={aTip.y} class="vector-a" marker-end="url(#pivot-arrow-a-right)" />

        <line x1={aTip.x} y1={aTip.y} x2={rightBTip.x} y2={rightBTip.y} class="vector-b" marker-end="url(#pivot-arrow-b-right)" />

        <circle cx={ORIGIN_X} cy={ORIGIN_Y} r="4" class="origin-dot" />
      </svg>
    </div>
  </div>

  <p class="demo-legend">
    Серый — вектор A, тащи его мышью (в любой из двух картинок, вторая
    повторяет за первой). Бледно-голубой — вектор B, фиксированный,
    растёт из кончика A. Зелёный пунктир — их сумма: место, где
    на самом деле окажется то, что мы рисуем. Кнопка увеличивает B в
    {SCALE} раза двумя разными способами одновременно.
  </p>

  <div class="controls">
    <button type="button" class="scale-button" onclick={runAnimation} disabled={animating}>
      {scaledOnce ? 'Повторить' : `Увеличить B в ${SCALE} раза`}
    </button>
  </div>
</div>

<style>
  .scale-pivot-demo {
    margin-block: 1rem;
  }

  .panels {
    display: flex;
    gap: 1.5rem;
    flex-wrap: wrap;
  }

  .panel {
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

  .scale-button {
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

  .scale-button:hover:not(:disabled) {
    background: var(--sl-color-gray-6);
  }

  .scale-button:disabled {
    opacity: 0.6;
    cursor: default;
  }
</style>
