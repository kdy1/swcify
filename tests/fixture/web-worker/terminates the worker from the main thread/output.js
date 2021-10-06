import _worker from "/Users/kdy1/projects/swcify/test-ref-gen/node_modules/@shopify/web-worker/build/cjs/webpack-parts/loader?{\"plain\":false}!./worker";
import { createWorkerFactory, terminate } from '@shopify/web-worker';
self.worker = createWorkerFactory(_worker)();

(async () => {
  const result = await self.worker.greet();
  const element = document.createElement('div');
  element.setAttribute('id', "WorkerResult");
  element.textContent = result;
  document.body.appendChild(element);
})();

self.terminateWorker = () => {
  terminate(self.worker);
  const element = document.createElement('div');
  element.setAttribute('id', "Terminate");
  document.body.appendChild(element);
};
