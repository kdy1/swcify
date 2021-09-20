import createResolver from '@shopify/async';

createResolver({
  ...otherOptions,
  [complexExpression()]: value,
  'non-identifier': () => import('./Bar'),
  notLoad: () => import('./Foo'),
});
