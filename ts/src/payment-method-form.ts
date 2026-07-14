// Credit-card / bank-account form helpers for Sigma Payments:
// toggle field groups by method type, auto-detect brand from PAN, format digits.

function detectBrand(digits: string): string {
  if (/^3[47]/.test(digits)) return "American Express";
  if (/^4/.test(digits)) return "Visa";
  if (/^(5[1-5]|2[2-7])/.test(digits)) return "Mastercard";
  if (/^(6011|65|64[4-9])/.test(digits)) return "Discover";
  if (/^(36|38|30[0-5])/.test(digits)) return "Diners Club";
  if (/^35/.test(digits)) return "JCB";
  return "";
}

function formatPan(digits: string, brand: string): string {
  if (brand === "American Express") {
    const a = digits.slice(0, 4);
    const b = digits.slice(4, 10);
    const c = digits.slice(10, 15);
    return [a, b, c].filter(Boolean).join(" ");
  }
  return digits.replace(/(\d{4})(?=\d)/g, "$1 ").trim();
}

function init(): void {
  const form = document.getElementById("payment-method-form");
  if (!(form instanceof HTMLFormElement)) {
    return;
  }

  const typeEl = document.getElementById("method_type");
  const ccFields = document.getElementById("cc-fields");
  const bankFields = document.getElementById("bank-fields");
  const cardNumberEl = document.getElementById("card_number");
  const cardholderEl = document.getElementById("cardholder_name");
  const cvvEl = document.getElementById("cvv");
  const expiryMonthEl = document.getElementById("expiry_month");
  const expiryYearEl = document.getElementById("expiry_year");
  const last4El = document.getElementById("last4");
  const brandBadge = document.getElementById("detected-brand");
  if (
    !(typeEl instanceof HTMLSelectElement || typeEl instanceof HTMLInputElement) ||
    !ccFields ||
    !bankFields ||
    !(cardNumberEl instanceof HTMLInputElement) ||
    !(cardholderEl instanceof HTMLInputElement) ||
    !(cvvEl instanceof HTMLInputElement) ||
    !(expiryMonthEl instanceof HTMLInputElement) ||
    !(expiryYearEl instanceof HTMLInputElement) ||
    !(last4El instanceof HTMLInputElement) ||
    !brandBadge
  ) {
    return;
  }

  const typeControl = typeEl;
  const cc = ccFields;
  const bank = bankFields;
  const cardNumber = cardNumberEl;
  const cardholder = cardholderEl;
  const cvv = cvvEl;
  const expiryMonth = expiryMonthEl;
  const expiryYear = expiryYearEl;
  const last4 = last4El;
  const badge = brandBadge;
  const isEdit = form.dataset.isEdit === "true";

  function syncType(): void {
    const isCard = typeControl.value === "credit_card";
    cc.hidden = !isCard;
    bank.hidden = isCard;
    cardholder.required = isCard;
    expiryMonth.required = isCard;
    expiryYear.required = isCard;
    cardNumber.required = isCard && !isEdit;
    cvv.required = isCard && !isEdit;
    last4.required = !isCard;
    if (!isCard) {
      expiryMonth.value = "";
      expiryYear.value = "";
      cardNumber.value = "";
      cvv.value = "";
      cardholder.value = "";
      badge.textContent = "—";
    }
  }

  function syncBrand(): void {
    let digits = (cardNumber.value || "").replace(/[\s-]/g, "").replace(/\D/g, "");
    if (digits.length > 19) {
      digits = digits.slice(0, 19);
    }
    const brand = detectBrand(digits);
    const formatted = formatPan(digits, brand);
    if (cardNumber.value !== formatted) {
      const end = cardNumber.selectionEnd ?? formatted.length;
      const before = (cardNumber.value.slice(0, end) || "").replace(/[\s-]/g, "").length;
      cardNumber.value = formatted;
      let pos = 0;
      let seen = 0;
      while (pos < formatted.length && seen < before) {
        if (/\d/.test(formatted.charAt(pos))) {
          seen++;
        }
        pos++;
      }
      cardNumber.setSelectionRange(pos, pos);
    }
    badge.textContent = brand || (digits ? "…" : "—");
    if (brand === "American Express") {
      cvv.maxLength = 4;
      cvv.placeholder = "••••";
    } else {
      cvv.maxLength = 3;
      cvv.placeholder = "•••";
    }
    if (isEdit && digits.length > 0) {
      cvv.required = true;
    } else if (isEdit) {
      cvv.required = false;
    }
  }

  cardNumber.addEventListener("input", syncBrand);
  typeControl.addEventListener("change", syncType);
  syncType();
  syncBrand();
}

if (document.readyState === "loading") {
  document.addEventListener("DOMContentLoaded", init);
} else {
  init();
}

export {};
