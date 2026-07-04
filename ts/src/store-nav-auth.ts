type AuthStatus = {
  authenticated: boolean;
  username?: string;
};

function requireElement<T extends HTMLElement>(id: string): T {
  const element = document.getElementById(id);
  if (!element) {
    throw new Error(`missing element #${id}`);
  }
  return element as T;
}

function setSignedIn(signedIn: boolean, username?: string): void {
  const signedOut = requireElement<HTMLElement>("store-nav-signed-out");
  const signedInEl = requireElement<HTMLElement>("store-nav-signed-in");
  const welcome = requireElement<HTMLElement>("store-nav-welcome");
  signedOut.classList.toggle("d-none", signedIn);
  signedInEl.classList.toggle("d-none", !signedIn);
  if (signedIn && username) {
    welcome.textContent = `Welcome, ${username}`;
  } else if (signedIn) {
    welcome.textContent = "Welcome";
  } else {
    welcome.textContent = "";
  }
}

function identityStatusUrl(): string {
  const root = document.getElementById("store-nav-auth")?.dataset.identityBase;
  if (!root) {
    throw new Error("missing data-identity-base");
  }
  return new URL("auth/status", root).toString();
}

async function pollAuthStatus(): Promise<void> {
  try {
    const response = await fetch(identityStatusUrl(), { credentials: "include" });
    if (!response.ok) {
      setSignedIn(false);
      return;
    }
    const status = (await response.json()) as AuthStatus;
    setSignedIn(Boolean(status.authenticated), status.username);
  } catch {
    setSignedIn(false);
  }
}

pollAuthStatus();
document.addEventListener("visibilitychange", () => {
  if (document.visibilityState === "visible") {
    void pollAuthStatus();
  }
});
window.addEventListener("focus", () => {
  void pollAuthStatus();
});
