<script>
  import NumberField from './NumberField.svelte';

  // Pure math illustration, no macroquad/WASM — same idea as
  // RadiansDemo.svelte/SinCosDemo.svelte. Four congruent right
  // triangles (legs a, b) tile the same (a+b)-side square two
  // different ways: rotated around the edges, leaving a tilted
  // square of side c in the middle (area c²) — or packed into two
  // corners, leaving an a×a square and a b×b square (area a²+b²).
  // Toggling between the two rearranges the same four triangles; the
  // leftover area has to match either way, since the big square and
  // the triangles never change — that's the whole proof.
  let { width = 720 } = $props();

  // 3-4-5 by default — the first Pythagorean triple most people
  // already half-remember, so the demo opens on a clean, familiar
  // number instead of an arbitrary one.
  let a = $state(3);
  let b = $state(4);
  let rearranged = $state(false);
  let t = $state(0); // 0 = tilted-square layout, 1 = two-squares layout

  function clamp(value) {
    return Math.min(8, Math.max(1, Math.round(value)));
  }

  $effect(() => {
    a = clamp(a);
    b = clamp(b);
  });

  function formatNum(value) {
    return Number(value.toFixed(2)).toString();
  }

  // SVG viewBox geometry — fixed internal units/scale, scaled to fit
  // `width` via CSS, same approach as RadiansDemo.
  const SCALE = 42; // pixels per unit of a/b
  const PAD = 40;
  const MAX_SIDE = 16; // covers a,b up to 8 each
  const VW = PAD * 2 + MAX_SIDE * SCALE;
  const VH = VW;
  let ORIGIN_X = $derived(PAD + ((MAX_SIDE - (a + b)) * SCALE) / 2);
  // Bottom edge of the big square — centers it vertically instead of
  // always sitting at the bottom, so small a/b don't leave a big empty
  // gap above while large ones barely fit below.
  let BASE_Y = $derived(VH - PAD - ((MAX_SIDE - (a + b)) * SCALE) / 2);

  let c = $derived(Math.sqrt(a * a + b * b));

  // Math point (x, y), y pointing up, origin at the big square's
  // bottom-left corner — converted to SVG coordinates (y down).
  function toSvg([x, y]) {
    return { x: ORIGIN_X + x * SCALE, y: BASE_Y - y * SCALE };
  }

  function pointsAttr(pts) {
    return pts.map((p) => `${p.x},${p.y}`).join(' ');
  }

  // Each triangle listed as [rightAngleVertex, legAEnd, legBEnd] —
  // same role order in both layouts, so lerping vertex-by-vertex
  // moves the right angle to the right angle, not to some unrelated
  // corner.
  let trianglesA = $derived.by(() => {
    const s = a + b;
    return [
      [[0, 0], [a, 0], [0, b]],
      [[s, 0], [s, a], [a, 0]],
      [[s, s], [b, s], [s, a]],
      [[0, s], [0, b], [b, s]],
    ];
  });

  let trianglesB = $derived.by(() => {
    const s = a + b;
    return [
      [[0, 0], [a, 0], [0, b]],
      [[a, b], [0, b], [a, 0]],
      [[a, b], [a, s], [s, b]],
      [[s, s], [s, b], [a, s]],
    ];
  });

  function lerpPoint(p0, p1, progress) {
    return [p0[0] + (p1[0] - p0[0]) * progress, p0[1] + (p1[1] - p0[1]) * progress];
  }

  // Each triangle's three SVG vertices, in [rightAngle, legAEnd,
  // legBEnd] order, so the edges can be drawn as three separate
  // color-coded segments: rightAngle–legAEnd is the `a` leg (red),
  // rightAngle–legBEnd is the `b` leg (green), legAEnd–legBEnd is the
  // hypotenuse `c` (blue) — the same three colors PointsTriangleDemo
  // uses for dx/dy/distance, so the two demos read as the same idea.
  let triangleVertices = $derived(
    trianglesA.map((triA, i) => {
      const triB = trianglesB[i];
      return triA.map((p0, j) => toSvg(lerpPoint(p0, triB[j], t)));
    }),
  );

  let trianglePolylines = $derived(triangleVertices.map((pts) => pointsAttr(pts)));

  // Leftover regions — not morphed, just cross-faded: the tilted
  // square (visible at t=0) fades out as the two corner squares
  // (invisible at t=0) fade in.
  let tiltedSquare = $derived.by(() => {
    const s = a + b;
    const pts = [
      [a, 0],
      [s, a],
      [b, s],
      [0, b],
    ].map(toSvg);
    return pointsAttr(pts);
  });

  let squareA = $derived.by(() => {
    const s = a + b;
    const pts = [
      [0, b],
      [a, b],
      [a, s],
      [0, s],
    ].map(toSvg);
    return pointsAttr(pts);
  });

  let squareB = $derived.by(() => {
    const s = a + b;
    const pts = [
      [a, 0],
      [s, 0],
      [s, b],
      [a, b],
    ].map(toSvg);
    return pointsAttr(pts);
  });

  let outerSquare = $derived.by(() => {
    const s = a + b;
    const pts = [
      [0, 0],
      [s, 0],
      [s, s],
      [0, s],
    ].map(toSvg);
    return pointsAttr(pts);
  });

  // The tilted square's four corners are symmetric around the big
  // square's own center, so its centroid is just that center point.
  let tiltedCenter = $derived(toSvg([(a + b) / 2, (a + b) / 2]));
  let squareACenter = $derived(toSvg([a / 2, (a + 2 * b) / 2]));
  let squareBCenter = $derived(toSvg([(2 * a + b) / 2, b / 2]));

  function ease(x) {
    return x < 0.5 ? 2 * x * x : 1 - (-2 * x + 2) ** 2 / 2;
  }

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
        const progress = duration > 0 ? Math.min(1, (now - startTime) / duration) : 1;
        setValue(start + (target - start) * ease(progress));
        if (progress < 1) {
          requestAnimationFrame(step);
        } else {
          resolve();
        }
      }

      requestAnimationFrame(step);
    });
  }

  async function toggleRearrange() {
    generation += 1;
    const myGeneration = generation;
    rearranged = !rearranged;
    await animateValue(() => t, (v) => (t = v), rearranged ? 1 : 0, 700, myGeneration);
  }

  $effect(() => {
    return () => {
      generation += 1;
    };
  });
</script>

<div class="pythagoras-demo">
  <svg class="pythagoras-svg" viewBox={`0 0 ${VW} ${VH}`} style={`max-width: ${width}px;`}>
    <polygon points={outerSquare} class="outer-square" />

    <polygon points={tiltedSquare} class="leftover-square c-square" style={`opacity: ${1 - t}`} />
    <polygon points={squareA} class="leftover-square a-square" style={`opacity: ${t}`} />
    <polygon points={squareB} class="leftover-square b-square" style={`opacity: ${t}`} />

    {#each trianglePolylines as pts}
      <polygon points={pts} class="triangle" />
    {/each}
    {#each triangleVertices as [rightAngle, legAEnd, legBEnd]}
      <line x1={rightAngle.x} y1={rightAngle.y} x2={legAEnd.x} y2={legAEnd.y} class="edge-a" />
      <line x1={rightAngle.x} y1={rightAngle.y} x2={legBEnd.x} y2={legBEnd.y} class="edge-b" />
      <line x1={legAEnd.x} y1={legAEnd.y} x2={legBEnd.x} y2={legBEnd.y} class="edge-c" />
    {/each}
    {#each triangleVertices as pts, i}
      <text
        x={(pts[0].x + pts[1].x + pts[2].x) / 3}
        y={(pts[0].y + pts[1].y + pts[2].y) / 3}
        class="triangle-number"
        text-anchor="middle"
        dominant-baseline="middle"
      >{i + 1}</text>
    {/each}

    <text x={tiltedCenter.x} y={tiltedCenter.y} class="area-label c-label" text-anchor="middle" dominant-baseline="middle" style={`opacity: ${1 - t}`}>c²</text>
    <text x={squareACenter.x} y={squareACenter.y} class="area-label a-label" text-anchor="middle" dominant-baseline="middle" style={`opacity: ${t}`}>a²</text>
    <text x={squareBCenter.x} y={squareBCenter.y} class="area-label b-label" text-anchor="middle" dominant-baseline="middle" style={`opacity: ${t}`}>b²</text>
  </svg>

  <p class="demo-numbers">
    c² = {formatNum(c * c)} &nbsp;&nbsp; a² + b² = {formatNum(a * a + b * b)}
  </p>

  <div class="controls" style={`max-width: ${width}px;`}>
    <NumberField label="a" bind:value={a} min="1" max="8" step="1" />
    <NumberField label="b" bind:value={b} min="1" max="8" step="1" />
    <button type="button" class="rearrange-button" onclick={toggleRearrange}>
      {rearranged ? 'Вернуть' : 'Переставить'}
    </button>
  </div>
</div>

<style>
  .pythagoras-demo {
    margin-block: 1rem;
  }

  .pythagoras-svg {
    display: block;
    width: 100%;
  }

  .demo-numbers {
    margin: 0.75rem 0;
  }

  .demo-numbers {
    font-family: var(--__sl-font-mono, ui-monospace, monospace);
    font-size: var(--sl-text-sm);
  }

  .outer-square {
    fill: none;
    stroke: var(--sl-color-gray-4);
    stroke-width: 2;
    stroke-dasharray: 4 4;
  }

  .triangle {
    fill: var(--sl-color-gray-5, rgba(140, 140, 150, 0.2));
    stroke: none;
  }

  .leftover-square {
    stroke-width: 2;
    stroke-linejoin: round;
  }

  .c-square {
    fill: rgba(59, 130, 246, 0.25);
    stroke: #3b82f6;
  }

  .a-square {
    fill: rgba(239, 68, 68, 0.25);
    stroke: #ef4444;
  }

  .b-square {
    fill: rgba(34, 197, 94, 0.25);
    stroke: #22c55e;
  }

  .triangle-number {
    font-size: 20px;
    font-weight: 700;
    font-family: var(--__sl-font-mono, ui-monospace, monospace);
    fill: var(--sl-color-gray-2, #555);
  }

  .area-label {
    font-size: 26px;
    font-weight: 700;
    font-family: var(--__sl-font-mono, ui-monospace, monospace);
  }

  .c-label {
    fill: #3b82f6;
  }

  .a-label {
    fill: #ef4444;
  }

  .b-label {
    fill: #22c55e;
  }

  .edge-a {
    stroke: #ef4444;
    stroke-width: 3;
    stroke-linecap: round;
  }

  .edge-b {
    stroke: #22c55e;
    stroke-width: 3;
    stroke-linecap: round;
  }

  .edge-c {
    stroke: #3b82f6;
    stroke-width: 3;
    stroke-linecap: round;
  }

  .controls {
    display: flex;
    align-items: center;
    gap: 1.5rem;
    margin-top: 0.75rem;
  }

  .rearrange-button {
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

  .rearrange-button:hover {
    background: var(--sl-color-gray-6);
  }
</style>
