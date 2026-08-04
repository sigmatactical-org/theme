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

  // src/contact-form-auth.ts
  function contactFormRoot() {
    const root = document.getElementById("contact-form");
    if (!root) {
      throw new Error("missing #contact-form");
    }
    return root;
  }
  function applySignedInFields(displayName, email) {
    const nameInput = requireElement("display_name");
    const emailInput = requireElement("email");
    nameInput.value = displayName;
    emailInput.value = email;
    nameInput.readOnly = true;
    emailInput.readOnly = true;
    nameInput.classList.add("bg-body-secondary");
    emailInput.classList.add("bg-body-secondary");
  }
  async function prefillFromIdentity() {
    const identityBase = contactFormRoot().dataset.identityBase;
    if (!identityBase) {
      return;
    }
    try {
      const response = await fetch(identityStatusUrlFromBase(identityBase), {
        credentials: "include"
      });
      if (!response.ok) {
        return;
      }
      const status = await response.json();
      if (!status.authenticated) {
        return;
      }
      applySignedInFields(status.username ?? "", status.email ?? "");
    } catch {
    }
  }
  void prefillFromIdentity();
})();
