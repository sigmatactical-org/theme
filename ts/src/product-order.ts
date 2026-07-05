import {
  type AuthStatus,
  identityStatusUrlFromBase,
  requireElement,
} from "./store-auth";

function orderRoot(): HTMLElement {
  const root = document.getElementById("product-order");
  if (!root) {
    throw new Error("missing #product-order");
  }
  return root;
}

function identityStatusUrl(): string {
  const root = orderRoot().dataset.identityBase;
  if (!root) {
    throw new Error("missing data-identity-base");
  }
  return identityStatusUrlFromBase(root);
}

function signInUrl(): string {
  const url = orderRoot().dataset.signInUrl;
  if (!url) {
    throw new Error("missing data-sign-in-url");
  }
  return url;
}

function showOrderForm(username: string): void {
  requireElement<HTMLElement>("product-order-auth-pending").classList.add("d-none");
  const formWrap = requireElement<HTMLElement>("product-order-form-wrap");
  formWrap.classList.remove("d-none");
  requireElement<HTMLInputElement>("product-order-username").value = username;
}

async function gateOrderPage(): Promise<void> {
  const pending = requireElement<HTMLElement>("product-order-auth-pending");
  try {
    const response = await fetch(identityStatusUrl(), { credentials: "include" });
    if (!response.ok) {
      window.location.assign(signInUrl());
      return;
    }
    const status = (await response.json()) as AuthStatus;
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
