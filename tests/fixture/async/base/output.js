import createResolver from '@shopify/async';

createResolver({
    load: () => import('../SomeComponent'),
    id: () => require.resolveWeak('../SomeComponent'),
  });