import _worker from "/Users/kdy1/projects/swcify/test-ref-gen/node_modules/@shopify/web-worker/build/cjs/webpack-parts/loader?{\"plain\":false}!./worker";
import { createWorkerFactory } from '@shopify/web-worker';
const worker = createWorkerFactory(_worker)();

(async () => {
  const element = document.createElement('div');
  element.setAttribute('id', "WorkerResult");
  element.textContent = await worker.magicVar();
  document.body.appendChild(element);
})();
