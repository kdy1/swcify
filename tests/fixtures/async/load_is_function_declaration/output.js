import createResolver from '@shopify/async';

createResolver({
  load: function load() {
    return import('./Foo');
  },
  id: () => require.resolveWeak('./Foo'),
});
