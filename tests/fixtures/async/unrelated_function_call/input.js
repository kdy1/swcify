import createResolver from 'unrelated-package';

createResolver({
  load: () => import('../SomeComponent'),
});
