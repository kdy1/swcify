import _en from "./translations/en.json";
import React from "react";
import { useI18n } from "@shopify/react-i18n";

export default function MyComponent() {
  const [i18n] = useI18n({
    id: "MyComponent_1asowhql4ye2g",
    fallback: _en,
    translations(locale) {
      const returnDefault = (dict) => dict && dict.default;

      switch (locale) {
        case "de":
          return import(
            /* webpackChunkName: "MyComponent_1asowhql4ye2g-i18n" */ "./translations/de.json"
          ).then(returnDefault);
        case "fr":
          return import(
            /* webpackChunkName: "MyComponent_1asowhql4ye2g-i18n" */ "./translations/fr.json"
          ).then(returnDefault);
        case "zh-TW":
          return import(
            /* webpackChunkName: "MyComponent_1asowhql4ye2g-i18n" */ "./translations/zh-TW.json"
          ).then(returnDefault);
      }
    },
  });
  return i18n.translate("key");
}