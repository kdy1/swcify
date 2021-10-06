import _worker from "/Users/kdy1/projects/swcify/test-ref-gen/node_modules/@shopify/web-worker/build/cjs/webpack-parts/loader?{\"plain\":false}!./worker";
import { createWorkerFactory, createIframeWorkerMessenger } from '@shopify/web-worker';
const worker = createWorkerFactory(_worker)({
  createMessenger: createIframeWorkerMessenger
});

(async () => {
  document.cookie = "MY_COOKIE" + '=1';
  const result = await worker.default();
  const element = document.createElement('div');
  element.setAttribute('id', "WorkerResult");
  element.textContent = result;
  document.body.appendChild(element);
})();
