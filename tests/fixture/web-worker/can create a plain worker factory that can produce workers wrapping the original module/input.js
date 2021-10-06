import { createPlainWorkerFactory } from '@shopify/web-worker';

const worker = createPlainWorkerFactory(() => import('./worker'))();

(async () => {
    const result = await new Promise((resolve) => {
        worker.addEventListener('message', ({ data }) => {
            resolve(data);
        });

        worker.postMessage("world");
    });

    const element = document.createElement('div');
    element.setAttribute('id', "WorkerResult");
    element.textContent = result;
    document.body.appendChild(element);
})();