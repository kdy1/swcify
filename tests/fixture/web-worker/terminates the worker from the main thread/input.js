import { createWorkerFactory, terminate } from '@shopify/web-worker';
self.worker = createWorkerFactory(() => import('./worker'))();

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
}