"use strict";
(() => {
  // src/sigma-dial.ts
  function buildTicks() {
    const ticks = [];
    for (let i = 0; i < 360; i++) {
      let len;
      let op;
      if (i % 30 === 0) {
        len = 24;
        op = 0.95;
      } else if (i % 10 === 0) {
        len = 16;
        op = 0.75;
      } else if (i % 5 === 0) {
        len = 10;
        op = 0.5;
      } else {
        len = 5;
        op = 0.35;
      }
      ticks.push({ deg: i, len, op });
    }
    return ticks;
  }
  var CARDINAL_LABELS = [
    "N",
    "30",
    "60",
    "E",
    "120",
    "150",
    "S",
    "210",
    "240",
    "W",
    "300",
    "330"
  ];
  function createSigmaDialSvg() {
    const ticks = buildTicks();
    const tickLines = ticks.map((t) => {
      const sw = t.deg % 30 === 0 ? 1.8 : t.deg % 10 === 0 ? 1.1 : t.deg % 5 === 0 ? 0.8 : 0.6;
      const y2 = -380 + t.len;
      return `<line x1="0" y1="-380" x2="0" y2="${y2}" stroke="var(--sigma-brass)" stroke-width="${sw}" opacity="${t.op}" transform="rotate(${t.deg})" />`;
    }).join("\n");
    const labelRadius = 348;
    const labelEls = CARDINAL_LABELS.map((label, i) => {
      const deg = i * 30;
      const isCardinal = i % 3 === 0;
      const op = isCardinal ? 1 : 0.6;
      const size = isCardinal ? 22 : 16;
      return `<g transform="rotate(${deg})"><text x="0" y="${-labelRadius}" transform="rotate(180 0 ${-labelRadius})" text-anchor="middle" dominant-baseline="central" opacity="${op}" font-family="var(--sigma-mono)" font-size="${size}" fill="var(--sigma-brass)" font-weight="600">${label}</text></g>`;
    }).join("\n");
    return `<svg class="sigma-dial" viewBox="-400 -400 800 800" aria-hidden="true" focusable="false">
<circle r="380" fill="none" stroke="var(--sigma-ink-3)" stroke-width="1" />
<circle r="340" fill="none" stroke="var(--sigma-ink-3)" stroke-width="0.6" />
<circle r="290" fill="none" stroke="var(--sigma-ink-4)" stroke-width="1" />
<circle r="260" fill="none" stroke="var(--sigma-ink-3)" stroke-width="0.4" />
<circle r="200" fill="none" stroke="var(--sigma-ink-4)" stroke-width="1" />
<circle r="160" fill="none" stroke="var(--sigma-ink-3)" stroke-width="0.4" />
<circle r="120" fill="none" stroke="var(--sigma-ink-4)" stroke-width="0.8" />
<g>${tickLines}</g>
<g>${labelEls}</g>
<line x1="-400" y1="0" x2="400" y2="0" stroke="var(--sigma-ink-4)" stroke-width="0.5" stroke-dasharray="2 4" />
<line x1="0" y1="-400" x2="0" y2="400" stroke="var(--sigma-ink-4)" stroke-width="0.5" stroke-dasharray="2 4" />
<g class="sigma-sight">
<line x1="0" y1="-360" x2="0" y2="360" stroke="var(--sigma-brass)" stroke-width="1.2" opacity="0.9" />
<circle r="8" fill="var(--sigma-brass)" />
<circle r="3" fill="var(--sigma-ink)" />
<polygon points="0,-380 -8,-360 8,-360" fill="var(--sigma-brass)" />
<polygon points="0,380 -8,360 8,360" fill="var(--sigma-brass)" opacity="0.4" />
</g>
<circle r="14" fill="var(--sigma-ink-2)" stroke="var(--sigma-ink-4)" stroke-width="1" />
<circle r="2" fill="var(--sigma-brass)" />
</svg>`;
  }
  function mountSigmaDial(container) {
    container.innerHTML = createSigmaDialSvg();
  }

  // src/eye-glow.ts
  function initEyeGlow() {
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
    const period = 3.8;
    function loop(now) {
      const t = (now - start) / 1e3;
      const base = (1 - Math.cos(t / period * 2 * Math.PI)) / 2;
      const flicker = 0.82 + 0.18 * Math.sin(t * 31) * Math.sin(t * 11.7) + 0.09 * Math.sin(t * 61);
      const intensity = Math.max(0, Math.min(1, Math.pow(base, 0.7) * flicker)) * 0.32;
      eyeGlow.style.opacity = intensity.toFixed(3);
      const s = 0.7 + 0.7 * base + 0.1 * Math.sin(t * 37);
      glow.setAttribute(
        "transform",
        `translate(66 36) scale(${s.toFixed(3)}) translate(-66 -36)`
      );
      requestAnimationFrame(loop);
    }
    requestAnimationFrame(loop);
  }

  // src/index.ts
  function mount() {
    const host = document.getElementById("sigma-dial-root");
    if (host) {
      mountSigmaDial(host);
    }
    initEyeGlow();
  }
  if (document.readyState === "loading") {
    document.addEventListener("DOMContentLoaded", mount);
  } else {
    mount();
  }
})();
