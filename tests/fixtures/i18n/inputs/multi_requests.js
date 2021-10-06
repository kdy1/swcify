import React from "react";
import {useI18n} from "@shopify/react-i18n";

function MyOtherComponent() {
  const [i18n] = useI18n();
}

export default function MyComponent() {
  const [i18n] = useI18n();
  return i18n.translate("key");
}
