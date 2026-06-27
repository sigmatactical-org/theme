import { mountSigmaDial } from "./sigma-dial";
import { initEyeGlow } from "./eye-glow";

function mount(): void {
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
