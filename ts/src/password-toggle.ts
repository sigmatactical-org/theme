function initPasswordToggles(): void {
  for (const button of document.querySelectorAll<HTMLButtonElement>(
    ".password-toggle-btn",
  )) {
    const targetId = button.dataset.passwordTarget;
    if (!targetId) {
      continue;
    }

    const input = document.getElementById(targetId);
    if (!(input instanceof HTMLInputElement)) {
      continue;
    }

    button.addEventListener("click", () => {
      const show = input.type === "password";
      input.type = show ? "text" : "password";
      button.setAttribute("aria-pressed", String(show));
      button.setAttribute("aria-label", show ? "Hide password" : "Show password");
      button.setAttribute("title", show ? "Hide password" : "Show password");
      button.querySelector(".password-icon--show")?.toggleAttribute("hidden", show);
      button.querySelector(".password-icon--hide")?.toggleAttribute("hidden", !show);
    });
  }
}

if (document.readyState === "loading") {
  document.addEventListener("DOMContentLoaded", initPasswordToggles);
} else {
  initPasswordToggles();
}
