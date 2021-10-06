import { createWorkerFactory } from '@shopify/web-worker';

const worker = createWorkerFactory(() => import('./worker'))();

(async () => {
    const element = document.createElement('div');
    element.setAttribute('id', "WorkerResult");
    element.textContent = await worker.magicVar();
    document.body.appendChild(element);
})();