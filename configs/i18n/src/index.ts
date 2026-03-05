import zhCN from "./locales/zh-CN.json";
import zhTW from "./locales/zh-TW.json";
import enUS from "./locales/en-US.json";

export const resources = {
  "zh-CN": { translation: zhCN, key: "zh_CN", label: "简体中文" },
  "zh-TW": { translation: zhTW, key: "zh_TW", label: "繁體中文" },
  "en-US": { translation: enUS, key: "en_US", label: "English" },
} as const;

export type LanguageType = keyof typeof resources;

export const LANGUAGES = Object.keys(resources) as LanguageType[];

export const DEFAULT_LANGUAGE = "zh-CN";
