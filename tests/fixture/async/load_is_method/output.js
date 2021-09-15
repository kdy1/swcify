import createResolver from "@shopify/async";

createResolver({
  load() {
    return import("./Foo");
  },
  id: () => require.resolveWeak("./Foo"),
});
