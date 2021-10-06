import { createWorkerFactory } from '@shopify/web-worker';

const worker = createWorkerFactory(() => import('./worker'))();

(async () => {
    let content = '';

    try {
        await worker.blowUp();
        content = 'All clear!';
    } catch (error) {
        content = error.message + error.stack;
    }

    const element = document.createElement('div');
    element.setAttribute('id', "WorkerResult");
    element.textContent = content;
    document.body.appendChild(element);
})();