function identityRegisterBase(): string | null {
  const { protocol, hostname, port, host } = window.location;
  if (hostname === "127.0.0.1" || hostname === "localhost") {
    return `${protocol}//${host}`;
  }
  if (hostname.startsWith("keycloak.")) {
    const identityHost = hostname.replace(/^keycloak\./, "identity.");
    return `${protocol}//${identityHost}${port ? `:${port}` : ""}`;
  }
  return null;
}

function installRegisterLink(): void {
  const returnUrl = new URLSearchParams(window.location.search).get("sigma_return_url");
  const identityBase = identityRegisterBase();
  if (!returnUrl || !identityBase) {
    return;
  }

  const password = document.querySelector<HTMLInputElement>(
    'input[name="password"], #password',
  );
  if (!password || document.querySelector(".sigma-register-link")) {
    return;
  }

  const registerUrl = `${identityBase}/register?return_url=${encodeURIComponent(returnUrl)}`;
  const wrapper = document.createElement("div");
  wrapper.className = "sigma-register-link";
  const link = document.createElement("a");
  link.href = registerUrl;
  link.textContent = "Create an account";
  wrapper.append(link);

  const anchor =
    password.closest(".pf-v5-c-form__group") ??
    password.closest(".form-group") ??
    password.parentElement;
  anchor?.insertAdjacentElement("afterend", wrapper);
}

function init(): void {
  document.documentElement.classList.add("pf-v5-theme-dark");
  installRegisterLink();
}

if (document.readyState === "loading") {
  document.addEventListener("DOMContentLoaded", init);
} else {
  init();
}

export {};
