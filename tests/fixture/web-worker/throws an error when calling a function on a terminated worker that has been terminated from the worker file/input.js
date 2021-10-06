import { createWorkerFactory } from '@shopify/web-worker';
self.worker = createWorkerFactory(() => import('./worker'))();

(async () => {
    await self.worker.terminateAttemptFromWorker();

    let result;
    try {
        result = await self.worker.greet("world");
    } catch (error) {
        result = error.toString();
    }
    const element = document.createElement('div');
    element.setAttribute('id', "WorkerResult");
    element.textContent = result;
    document.body.appendChild(element);
})();