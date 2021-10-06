import { createWorkerFactory } from '@shopify/web-worker';

const worker = createWorkerFactory(() => import('./worker'))();

(async () => {
    const result = await worker.greet("world");
    const element = document.createElement('div');
    element.setAttribute('id', "WorkerResult");
    element.textContent = result;
    document.body.appendChild(element);
})();