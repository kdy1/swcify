import {useI18n} from "some-other-lib";

export default function MyComponent() {
  const [i18n] = useI18n();
  return i18n.translate("key");
}