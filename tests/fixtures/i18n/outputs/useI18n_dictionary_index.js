import __shopify__i18n_translations from "./translations";
import React from "react";
import {useI18n} from "@shopify/react-i18n";

export default function MyComponent() {
  const [i18n] = useI18n({
    id: "MyComponent_15lrxy207c54x",
    fallback: Object.values(__shopify__i18n_translations)[0],
    translations(locale) {
      return Promise.resolve(__shopify__i18n_translations[locale]);
    },
  });
  return i18n.translate("key");
}
