// UI translations.
//
// Each language lives in its own JSON file under `./locales/`, so a translator
// never has to touch TypeScript. English is the reference: its keys are the
// `MessageKey` union, and every other file is type-checked against it, so a
// missing or misspelled key fails the build instead of showing a raw key at
// runtime.
import de from "./locales/de.json";
import en from "./locales/en.json";
import es from "./locales/es.json";
import fr from "./locales/fr.json";
import pt from "./locales/pt.json";
import tr from "./locales/tr.json";
import zh from "./locales/zh.json";
import zhTw from "./locales/zhTw.json";

export type MessageKey = keyof typeof en;

/** One language's dictionary: exactly the English keys, nothing else. */
export type Dict = Record<MessageKey, string>;

// A translation that is missing a key is a build error, not a runtime
// fallback: the app must never show a raw key to the user.
const dicts = { de, en, es, fr, pt, tr, zh, zhTw } satisfies Record<string, Dict>;

export const LANGUAGES = [
  { code: "en", name: "English" },
  { code: "zh", name: "简体中文" },
  { code: "zhTw", name: "繁體中文" },
  { code: "fr", name: "Français" },
  { code: "es", name: "Español" },
  { code: "tr", name: "Türkçe" },
  { code: "de", name: "Deutsch" },
  { code: "pt", name: "Português" },
] as const;

export type Lang = (typeof LANGUAGES)[number]["code"];

const DICTS = dicts as Record<Lang, Dict>;
const STORAGE_KEY = import.meta.env.DEV ? "leagueaccounts.dev.lang" : "leagueaccounts.lang";

/** BCP-47 tag for the document, so the OS picks the right CJK font and `Intl`
 * accepts the value. The internal codes are not valid tags ("zhTw"). */
const HTML_LANG: Record<Lang, string> = {
  en: "en",
  zh: "zh-CN",
  zhTw: "zh-TW",
  fr: "fr",
  es: "es",
  tr: "tr",
  de: "de",
  pt: "pt",
};

/** BCP-47 tag of the active language, for `Intl` and the document. */
export const localeTag = (): string => HTML_LANG[current] ?? current;

/** Chinese needs the script subtag: `zh-TW`/`zh-HK`/`zh-Hant` is Traditional. */
function fromBrowser(tag: string): Lang {
  const lower = tag.toLowerCase();
  if (lower.startsWith("zh")) {
    return /-(tw|hk|mo|hant)\b/.test(lower) ? "zhTw" : "zh";
  }
  const base = lower.slice(0, 2);
  return (base in DICTS ? base : "en") as Lang;
}

function detect(): Lang {
  try {
    const saved = localStorage.getItem(STORAGE_KEY);
    if (saved && saved in DICTS) return saved as Lang;
  } catch {
    // Storage unavailable.
  }
  return fromBrowser(navigator.language);
}

let current: Lang = detect();

export const lang = (): Lang => current;

export function setLang(next: Lang): void {
  current = next;
  document.documentElement.lang = HTML_LANG[next] ?? next;
  try {
    localStorage.setItem(STORAGE_KEY, next);
  } catch {
    // Non-essential.
  }
}

export function t(key: MessageKey, vars: Record<string, string | number> = {}): string {
  const text = DICTS[current][key] ?? en[key] ?? key;
  return text.replace(/\{(\w+)\}/g, (match, name: string) => (name in vars ? String(vars[name]) : match));
}

/** Translate a backend error (`{ code, detail }`) for display. */
export function errorText(code: string, detail?: string | null): string {
  const key = `err.${code}` as MessageKey;
  const message = key in en ? t(key) : t("err.unknown");
  return detail ? `${message} (${detail})` : message;
}

const TIER_WORD = /^(Challenger|Grandmaster|Master|Diamond|Emerald|Platinum|Gold|Silver|Bronze|Iron|Unranked)\b/i;

/** Localize a rank string from the backend, e.g. "Platinum I 12LP" or "N/A". */
export function localizeRank(text: string): string {
  const value = text.trim();
  if (!value || value === "N/A") return t("rank.na");
  if (value === "...") return "…";
  return value.replace(TIER_WORD, (word) => {
    const tier = word[0].toUpperCase() + word.slice(1).toLowerCase();
    return t(`tier.${tier === "Grandmaster" ? "Grandmaster" : tier}` as MessageKey);
  });
}

export const tierName = (tier: string): string => t(`tier.${tier}` as MessageKey);

/** Apply translations to static markup: data-i18n (text), -ph, -title, -aria. */
export function applyStatic(root: ParentNode = document): void {
  root.querySelectorAll<HTMLElement>("[data-i18n]").forEach((element) => {
    element.textContent = t(element.dataset.i18n as MessageKey);
  });
  root.querySelectorAll<HTMLInputElement>("[data-i18n-ph]").forEach((element) => {
    element.placeholder = t(element.dataset.i18nPh as MessageKey);
  });
  root.querySelectorAll<HTMLElement>("[data-i18n-title]").forEach((element) => {
    element.dataset.tooltip = t(element.dataset.i18nTitle as MessageKey);
    if (element.tagName === "BUTTON" && !element.dataset.i18nAria) {
      element.setAttribute("aria-label", element.dataset.tooltip);
    }
  });
  root.querySelectorAll<HTMLElement>("[data-i18n-aria]").forEach((element) => {
    element.setAttribute("aria-label", element.dataset.i18nAria as MessageKey);
  });
}
