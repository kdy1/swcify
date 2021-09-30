import React from "react";
import { withFoo } from "@shopify/react-i18n";

function MyComponent({ i18n }) {
  return i18n.translate("key");
}
export const key = translate("key");

export default withFoo()(MyComponent);
