import {
  type AuthStatus,
  identityStatusUrlFromBase,
  requireElement,
} from "./store-auth";

function setSignedIn(signedIn: boolean, username?: string): void {
  const signedOut = requireElement<HTMLElement>("store-nav-signed-out");
  const signedInEl = requireElement<HTMLElement>("store-nav-signed-in");
  const welcome = requireElement<HTMLElement>("store-nav-welcome");
  signedOut.classList.toggle("d-none", signedIn);
  signedInEl.classList.toggle("d-none", !signedIn);
  welcome.replaceChildren();
  if (signedIn && username) {
    const editProfileUrl =
      document.getElementById("store-nav-auth")?.dataset.editProfileUrl;
    welcome.append("Welcome, ");
    if (editProfileUrl) {
      const link = document.createElement("a");
      link.href = editProfileUrl;
      link.className = "store-nav-profile-link text-light text-decoration-underline";
      link.textContent = username;
      welcome.append(link);
    } else {
      welcome.append(username);
    }
  } else if (signedIn) {
    welcome.textContent = "Welcome";
  }
}

function identityStatusUrl(): string {
  const root = document.getElementById("store-nav-auth")?.dataset.identityBase;
  if (!root) {
    throw new Error("missing data-identity-base");
  }
  return identityStatusUrlFromBase(root);
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
