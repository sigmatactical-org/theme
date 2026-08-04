"use strict";
(() => {
  // src/store-auth.ts
  function requireElement(id) {
    const element = document.getElementById(id);
    if (!element) {
      throw new Error(`missing element #${id}`);
    }
    return element;
  }
  function identityStatusUrlFromBase(identityBase) {
    return new URL("auth/status", identityBase).toString();
  }

  // src/store-nav-auth.ts
  function setSignedIn(signedIn, username) {
    const signedOut = requireElement("store-nav-signed-out");
    const signedInEl = requireElement("store-nav-signed-in");
    const welcome = requireElement("store-nav-welcome");
    signedOut.classList.toggle("d-none", signedIn);
    signedInEl.classList.toggle("d-none", !signedIn);
    welcome.replaceChildren();
    if (signedIn && username) {
      const editProfileUrl = document.getElementById("store-nav-auth")?.dataset.editProfileUrl;
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
  function identityStatusUrl() {
    const root = document.getElementById("store-nav-auth")?.dataset.identityBase;
    if (!root) {
      throw new Error("missing data-identity-base");
    }
    return identityStatusUrlFromBase(root);
  }
  async function pollAuthStatus() {
    try {
      const response = await fetch(identityStatusUrl(), { credentials: "include" });
      if (!response.ok) {
        setSignedIn(false);
        return;
      }
      const status = await response.json();
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
})();
