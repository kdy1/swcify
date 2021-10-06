import _worker2 from "/Users/kdy1/projects/swcify/test-ref-gen/node_modules/@shopify/web-worker/build/cjs/webpack-parts/loader?{\"plain\":false}!./worker2";
import _worker from "/Users/kdy1/projects/swcify/test-ref-gen/node_modules/@shopify/web-worker/build/cjs/webpack-parts/loader?{\"name\":\"MyWorker\",\"plain\":false}!./worker";
import { createWorkerFactory } from '@shopify/web-worker';
const workerOne = createWorkerFactory(_worker)();
const workerTwo = createWorkerFactory(_worker2)();

(async () => {
  const results = await Promise.all([workerOne.default(), workerTwo.default()]);
  const element = document.createElement('div');
  element.setAttribute('id', "WorkerResult");
  element.textContent = results.join(' ');
  document.body.appendChild(element);
})();
