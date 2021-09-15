import createResolver from '@shopify/async';

createResolver({
    id: () => './SomeComponent',
    load: () => import('../SomeComponent'),
  });