import createResolver from '@shopify/async';

createResolver({
  ...otherOptions,
  load: Foo,
});
