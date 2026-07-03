function initRegisterSuccessRedirect(): void {
  const config = document.getElementById("register-success-redirect");
  if (!config) {
    return;
  }

  const url = config.dataset.redirectUrl;
  const delay = Number(config.dataset.redirectDelay ?? "3");
  if (!url || !Number.isFinite(delay) || delay < 1) {
    return;
  }

  const secondsEl = document.getElementById("register-success-seconds");
  let remaining = delay;

  const tick = (): void => {
    remaining -= 1;
    if (secondsEl) {
      secondsEl.textContent = String(Math.max(remaining, 0));
    }
    if (remaining <= 0) {
      window.location.assign(url);
      return;
    }
    window.setTimeout(tick, 1000);
  };

  window.setTimeout(tick, 1000);
}

if (document.readyState === "loading") {
  document.addEventListener("DOMContentLoaded", initRegisterSuccessRedirect);
} else {
  initRegisterSuccessRedirect();
}
