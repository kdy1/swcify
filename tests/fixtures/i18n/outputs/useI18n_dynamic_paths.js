import _en from "./translations/en.json";
import React from "react";
import { useI18n } from "@shopify/react-i18n";
export default function MyComponent() {
  const [i18n] = useI18n({
    id: "MyComponent_1asowhql4ye2g",
    fallback: _en,
    translations(locale) {
      if (["de", "fr", "zh-TW"].indexOf(locale) < 0) {
        return;
      }

      return import(
        /* webpackChunkName: "MyComponent_1asowhql4ye2g-i18n", webpackMode: "lazy-once" */ `./translations/${locale}.json`
      ).then((dict) => dict && dict.default);
    },
  });
  return i18n.translate("key");
}
