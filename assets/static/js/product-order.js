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

  // src/product-order.ts
  function orderRoot() {
    const root = document.getElementById("product-order");
    if (!root) {
      throw new Error("missing #product-order");
    }
    return root;
  }
  function identityStatusUrl() {
    const root = orderRoot().dataset.identityBase;
    if (!root) {
      throw new Error("missing data-identity-base");
    }
    return identityStatusUrlFromBase(root);
  }
  function signInUrl() {
    const url = orderRoot().dataset.signInUrl;
    if (!url) {
      throw new Error("missing data-sign-in-url");
    }
    return url;
  }
  function showOrderForm(username) {
    requireElement("product-order-auth-pending").classList.add("d-none");
    const formWrap = requireElement("product-order-form-wrap");
    formWrap.classList.remove("d-none");
    requireElement("product-order-username").value = username;
  }
  async function gateOrderPage() {
    const pending = requireElement("product-order-auth-pending");
    try {
      const response = await fetch(identityStatusUrl(), { credentials: "include" });
      if (!response.ok) {
        window.location.assign(signInUrl());
        return;
      }
      const status = await response.json();
      if (!status.authenticated) {
        window.location.assign(signInUrl());
        return;
      }
      showOrderForm(status.username ?? "customer");
    } catch {
      pending.textContent = "Unable to verify sign-in. Try again or sign in.";
      pending.classList.add("text-danger");
    }
  }
  void gateOrderPage();
})();
