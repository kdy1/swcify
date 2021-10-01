import { withI18n } from "some-other-lib";

function MyComponent({ i18n }) {
  return i18n.translate("key");
}

export default withI18n()(MyComponent);
