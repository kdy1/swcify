import _worker from "/Users/kdy1/projects/swcify/test-ref-gen/node_modules/@shopify/web-worker/build/cjs/webpack-parts/loader?{\"plain\":false}!./worker";
import { createWorkerFactory } from '@shopify/web-worker';
const worker = createWorkerFactory(_worker)();

(async () => {
  let content = '';

  try {
    await worker.blowUp();
    content = 'All clear!';
  } catch (error) {
    content = error.message + error.stack;
  }

  const element = document.createElement('div');
  element.setAttribute('id', "WorkerResult");
  element.textContent = content;
  document.body.appendChild(element);
})();
