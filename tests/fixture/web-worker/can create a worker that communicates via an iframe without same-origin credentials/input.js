import { createWorkerFactory, createIframeWorkerMessenger } from '@shopify/web-worker';

const worker = createWorkerFactory(() => import('./worker'))({
    createMessenger: createIframeWorkerMessenger,
});

(async () => {
    document.cookie = "MY_COOKIE" + '=1';
    const result = await worker.default();
    const element = document.createElement('div');
    element.setAttribute('id', "WorkerResult");
    element.textContent = result;
    document.body.appendChild(element);
})();