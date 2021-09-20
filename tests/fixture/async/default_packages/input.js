import createResolver from '@shopify/async';

createResolver({
  load: () => import('../SomeComponent'),
});
