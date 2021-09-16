import createResolver from '@shopify/async';

{
  const createResolver = UNRELATED_FUNCTION;

  createResolver({
    load: () => import('./Foo'),
  });
}

createResolver({
  load: () => import('./Foo'),
});