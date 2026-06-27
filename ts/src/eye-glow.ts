/**
 * Ember "eye" flicker on the background sigma mark. Pure CSS can't express the
 * pseudo-random flicker, so we animate opacity/scale here. Respects reduced motion.
 */
export function initEyeGlow(): void {
  const eyeGlow = document.getElementById("sigma-eye-glow");
  const glow = document.getElementById("sigma-eye-core");
  if (!eyeGlow || !glow) {
    return;
  }

  if (window.matchMedia?.("(prefers-reduced-motion: reduce)").matches) {
    eyeGlow.style.opacity = "0.28";
    return;
  }

  const start = performance.now();
  const period = 3.8; // seconds: dark -> burning -> dark

  function loop(now: number): void {
    const t = (now - start) / 1000;
    const base = (1 - Math.cos((t / period) * 2 * Math.PI)) / 2;
    const flicker =
      0.82 + 0.18 * Math.sin(t * 31.0) * Math.sin(t * 11.7) + 0.09 * Math.sin(t * 61.0);
    const intensity = Math.max(0, Math.min(1, Math.pow(base, 0.7) * flicker)) * 0.32;
    eyeGlow!.style.opacity = intensity.toFixed(3);
    const s = 0.7 + 0.7 * base + 0.1 * Math.sin(t * 37.0);
    glow!.setAttribute(
      "transform",
      `translate(66 36) scale(${s.toFixed(3)}) translate(-66 -36)`,
    );
    requestAnimationFrame(loop);
  }

  requestAnimationFrame(loop);
}
