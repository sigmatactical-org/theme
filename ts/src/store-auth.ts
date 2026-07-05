export type AuthStatus = {
  authenticated: boolean;
  username?: string;
  email?: string;
};

export function requireElement<T extends HTMLElement>(id: string): T {
  const element = document.getElementById(id);
  if (!element) {
    throw new Error(`missing element #${id}`);
  }
  return element as T;
}

export function identityStatusUrlFromBase(identityBase: string): string {
  return new URL("auth/status", identityBase).toString();
}
