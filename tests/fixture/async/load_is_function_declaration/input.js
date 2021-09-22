import createResolver from '@shopify/async';

createResolver({
  load: function load() {
    return import('./Foo');
  },
});
