import React from "react";
import {useI18n as useFunI18n} from "@shopify/react-i18n";

export default function MyComponent() {
  const [i18n] = useFunI18n();
  return i18n.translate("key");
}
