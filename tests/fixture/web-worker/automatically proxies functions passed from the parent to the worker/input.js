import { createWorkerFactory } from '@shopify/web-worker';

const worker = createWorkerFactory(() => import('./worker'))();

const users = [
    { getName: () => "Gord" },
    { getName: () => "Michelle" },
];

(async () => {
    const result = await worker.greet(users);
    const element = document.createElement('div');
    element.setAttribute('id', "WorkerResult");
    element.textContent = result;
    document.body.appendChild(element);
})();