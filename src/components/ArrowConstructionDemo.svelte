<script>
  import NumberField from './NumberField.svelte';

  // A fixed vertical vector, so "step back" is literally "down" and
  // the two wings are literally "left" and "right" — no rotation math
  // needed to follow along. Pressing the button animates the actual
  // construction: the shaft retracts from the tip down to `back`,
  // then a stub grows left to `wing1`, then right to `wing2`, then
  // both wings sweep up to the tip and the triangle fills in.
  // Pressing it again reverses the same animation, back to a plain line.
  let { width = 560 } = $props();

  const VW = 480;
  const VH = 420;
  const ORIGIN_X = 240;
  const ORIGIN_Y = 360;
  const TIP_X = 240;
  const TIP_Y = 80;

  let headLength = $state(50);
  let headWidth = $state(30);
  let buildProgress = $state(0); // 0 = plain line, 1 = fully built triangle
  let showConstruction = $state(false);
  let generation = 0;

  function clampHeadLength(value) {
    return Math.round(Math.min(120, Math.max(20, value)));
  }

  function clampHeadWidth(value) {
    return Math.round(Math.min(70, Math.max(10, value)));
  }

  $effect(() => {
    headLength = clampHeadLength(headLength);
    headWidth = clampHeadWidth(headWidth);
  });

  // The four stages share the one animated value — each owns a quarter
  // of it, so they play in order and reverse in order for free.
  let p1 = $derived(Math.min(1, Math.max(0, buildProgress / 0.25))); // tip -> back
  let p2 = $derived(Math.min(1, Math.max(0, (buildProgress - 0.25) / 0.25))); // back -> wing1 (left)
  let p3 = $derived(Math.min(1, Math.max(0, (buildProgress - 0.5) / 0.25))); // back -> wing2 (right)
  let p4 = $derived(Math.min(1, Math.max(0, (buildProgress - 0.75) / 0.25))); // wings -> tip, fill

  let backX = $derived(TIP_X);
  let backY = $derived(TIP_Y + headLength);
  let wing1X = $derived(backX - headWidth); // left
  let wing1Y = $derived(backY);
  let wing2X = $derived(backX + headWidth); // right
  let wing2Y = $derived(backY);

  function lerp(a, b, t) {
    return a + (b - a) * t;
  }

  // The shaft's own end point — slides from the tip down to `back` as
  // stage 1 plays, then stays put.
  let shaftEndX = $derived(TIP_X);
  let shaftEndY = $derived(lerp(TIP_Y, backY, p1));

  let leftStubX = $derived(lerp(backX, wing1X, p2));
  let leftStubY = $derived(lerp(backY, wing1Y, p2));
  let rightStubX = $derived(lerp(backX, wing2X, p3));
  let rightStubY = $derived(lerp(backY, wing2Y, p3));

  let leftEdgeX = $derived(lerp(wing1X, TIP_X, p4));
  let leftEdgeY = $derived(lerp(wing1Y, TIP_Y, p4));
  let rightEdgeX = $derived(lerp(wing2X, TIP_X, p4));
  let rightEdgeY = $derived(lerp(wing2Y, TIP_Y, p4));

  function ease(x) {
    return x < 0.5 ? 2 * x * x : 1 - (-2 * x + 2) ** 2 / 2;
  }

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

  function toggleConstruction() {
    generation += 1;
    const myGeneration = generation;
    showConstruction = !showConstruction;
    const target = showConstruction ? 1 : 0;
    animateValue(() => buildProgress, (v) => (buildProgress = v), target, 1400, myGeneration);
  }
</script>

<div class="arrow-construction-demo">
  <svg class="arrow-construction-svg" viewBox={`0 0 ${VW} ${VH}`} style={`max-width: ${width}px;`}>
    <!-- Сам «тело» стрелки — тянется от начала до текущего конца. -->
    <line x1={ORIGIN_X} y1={ORIGIN_Y} x2={shaftEndX} y2={shaftEndY} class="shaft-line" />

    {#if p2 > 0.001}
      <line x1={backX} y1={backY} x2={leftStubX} y2={leftStubY} class="wing-line" />
    {/if}
    {#if p3 > 0.001}
      <line x1={backX} y1={backY} x2={rightStubX} y2={rightStubY} class="wing-line" />
    {/if}

    {#if p4 > 0.001}
      <line x1={wing1X} y1={wing1Y} x2={leftEdgeX} y2={leftEdgeY} class="wing-line" />
      <line x1={wing2X} y1={wing2Y} x2={rightEdgeX} y2={rightEdgeY} class="wing-line" />
      <polygon
        points={`${TIP_X},${TIP_Y} ${wing1X},${wing1Y} ${wing2X},${wing2Y}`}
        class="head-triangle"
        style={`opacity: ${p4}`}
      />
    {/if}

    <circle cx={ORIGIN_X} cy={ORIGIN_Y} r="4" class="origin-dot" />
  </svg>

  <p class="demo-legend">
    Стрелка смотрит вертикально вверх — так направления видно сразу:
    «назад» — это вниз, а крылья наконечника — влево и вправо. Кнопка
    ниже проигрывает само построение: шаг назад до <code>back</code>,
    потом крыло влево, потом вправо, а потом оба крыла соединяются с
    кончиком — и получается треугольник.
  </p>

  <div class="controls" style={`max-width: ${width}px;`}>
    <NumberField label="head_length" bind:value={headLength} min="20" max="120" step="1" />
    <NumberField label="head_width" bind:value={headWidth} min="10" max="70" step="1" />
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
  }

  .demo-legend {
    margin: 0.75rem 0;
  }

  .shaft-line {
    stroke: var(--sl-color-gray-2);
    stroke-width: 4;
    stroke-linecap: round;
  }

  .head-triangle {
    fill: #facc15;
    stroke: none;
  }

  .wing-line {
    stroke: #7dd3fc;
    stroke-width: 3;
    stroke-linecap: round;
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
