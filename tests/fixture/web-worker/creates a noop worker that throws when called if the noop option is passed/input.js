import { createWorkerFactory } from '@shopify/web-worker';

const worker = createWorkerFactory(() => import('./worker'))();

(async () => {
    const element = document.createElement('div');
    element.setAttribute('id', "WorkerResult");

    try {
        await worker.willThrow();
    } catch (error) {
        element.textContent = error.message;
    }

    document.body.appendChild(element);
})();