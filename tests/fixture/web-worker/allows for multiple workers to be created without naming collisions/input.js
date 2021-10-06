import { createWorkerFactory } from '@shopify/web-worker';

const workerOne = createWorkerFactory(() => import(/* webpackChunkName: 'MyWorker' */ './worker'))();
const workerTwo = createWorkerFactory(() => import('./worker2'))();

(async () => {
    const results = await Promise.all([
        workerOne.default(),
        workerTwo.default(),
    ]);

    const element = document.createElement('div');
    element.setAttribute('id', "WorkerResult");
    element.textContent = results.join(' ');
    document.body.appendChild(element);
})();