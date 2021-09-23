import _en from "./translations/en.json";
import React from 'react';
import {useI18n} from '@shopify/react-i18n';

export default function MyComponent() {
  const [i18n] = useI18n({
    id: "MyComponent_TODOHASH",
    fallback: _en,
    translations(locale) {
      if (locale !== "fr") {
        return;
      }

      return import(/* webpackChunkName: "MyComponent_TODOHASH-i18n" */ "./translations/fr.json").then((dict) => dict && dict.default);
    }
  });
  return i18n.translate('key');
}