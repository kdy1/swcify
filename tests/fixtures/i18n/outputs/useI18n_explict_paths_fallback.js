import _en from "./translations/en.json";
import React from "react";
import {useI18n} from "@shopify/react-i18n";

export default function MyComponent() {
  const [i18n] = useI18n({
    id: "MyComponent_15lrxy207c54x",
    fallback: _en,
    translations(locale) {
      return;
    },
  });
  return i18n.translate("key");
}
