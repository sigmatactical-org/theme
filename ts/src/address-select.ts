// Loads state/province/region options from the identity service when the user
// picks a country on the profile edit form.

type RegionsResponse = {
  prompt: string;
  regions: string[];
};

async function fetchRegions(country: string): Promise<RegionsResponse> {
  const url = new URL("/geo/regions", window.location.origin);
  url.searchParams.set("country", country);
  const response = await fetch(url);
  if (!response.ok) {
    throw new Error(`regions request failed: ${response.status}`);
  }
  return (await response.json()) as RegionsResponse;
}

function buildTextInput(value: string): HTMLInputElement {
  const input = document.createElement("input");
  input.className = "form-control";
  input.type = "text";
  input.id = "address-region";
  input.name = "region";
  input.autocomplete = "address-level1";
  input.placeholder = "State / province / region";
  input.value = value;
  return input;
}

function buildSelect(
  prompt: string,
  regions: string[],
  selected: string,
): HTMLSelectElement {
  const select = document.createElement("select");
  select.className = "form-select";
  select.id = "address-region";
  select.name = "region";
  select.autocomplete = "address-level1";

  const placeholder = document.createElement("option");
  placeholder.value = "";
  placeholder.textContent = prompt || "Select a region…";
  select.append(placeholder);

  for (const name of regions) {
    const option = document.createElement("option");
    option.value = name;
    option.textContent = name;
    if (name === selected) {
      option.selected = true;
    }
    select.append(option);
  }

  return select;
}

function currentRegionValue(container: HTMLElement, fallback: string): string {
  const field = container.querySelector<HTMLInputElement | HTMLSelectElement>(
    "#address-region",
  );
  return field?.value ?? fallback;
}

async function syncRegionField(
  container: HTMLElement,
  country: string,
  selected: string,
): Promise<void> {
  if (!country) {
    container.replaceChildren(buildTextInput(selected));
    return;
  }

  try {
    const data = await fetchRegions(country);
    if (data.regions.length === 0) {
      container.replaceChildren(buildTextInput(selected));
      return;
    }
    container.replaceChildren(
      buildSelect(data.prompt, data.regions, selected),
    );
  } catch {
    container.replaceChildren(buildTextInput(selected));
  }
}

function init(): void {
  const country = document.getElementById("address-country");
  const container = document.getElementById("address-region-field");
  if (!(country instanceof HTMLSelectElement) || !container) {
    return;
  }

  const initialRegion = container.dataset.initialRegion ?? "";

  const refresh = () => {
    const selected = currentRegionValue(container, initialRegion);
    void syncRegionField(container, country.value, selected);
  };

  country.addEventListener("change", refresh);
  void syncRegionField(container, country.value, initialRegion);
}

if (document.readyState === "loading") {
  document.addEventListener("DOMContentLoaded", init);
} else {
  init();
}

export {};
