
import { createWorkerFactory } from '@shopify/web-worker';

const worker = createWorkerFactory(() => import(/* webpackChunkName: "myFancyWorker" */ './worker'))();

(async () => {
    const element = document.createElement('div');
    element.setAttribute('id', "WorkerResult");
    document.body.appendChild(element);
})();