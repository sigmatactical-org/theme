import {
  type AuthStatus,
  identityStatusUrlFromBase,
  requireElement,
} from "./store-auth";

function contactFormRoot(): HTMLElement {
  const root = document.getElementById("contact-form");
  if (!root) {
    throw new Error("missing #contact-form");
  }
  return root;
}

function applySignedInFields(displayName: string, email: string): void {
  const nameInput = requireElement<HTMLInputElement>("display_name");
  const emailInput = requireElement<HTMLInputElement>("email");
  nameInput.value = displayName;
  emailInput.value = email;
  nameInput.readOnly = true;
  emailInput.readOnly = true;
  nameInput.classList.add("bg-body-secondary");
  emailInput.classList.add("bg-body-secondary");
}

async function prefillFromIdentity(): Promise<void> {
  const identityBase = contactFormRoot().dataset.identityBase;
  if (!identityBase) {
    return;
  }

  try {
    const response = await fetch(identityStatusUrlFromBase(identityBase), {
      credentials: "include",
    });
    if (!response.ok) {
      return;
    }
    const status = (await response.json()) as AuthStatus;
    if (!status.authenticated) {
      return;
    }
    applySignedInFields(status.username ?? "", status.email ?? "");
  } catch {
    // Leave fields editable when identity is unreachable.
  }
}

void prefillFromIdentity();
