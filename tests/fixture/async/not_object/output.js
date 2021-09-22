import createResolver from '@shopify/async';

createResolver(() => import('./Foo'));
