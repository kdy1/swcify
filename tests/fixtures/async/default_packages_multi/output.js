import createResolver from '@shopify/async';
import {createAsyncContext, createAsyncComponent} from '@shopify/react-async';
import {
  createAsyncQueryComponent,
  createAsyncQuery,
} from '@shopify/react-graphql';

createAsyncQueryComponent({
  load: () => import('../SomeComponent'),
  id: () => require.resolveWeak('../SomeComponent'),
});
