import { createWorkerFactory, terminate } from '@shopify/web-worker';

self.WorkerTestClass = class WorkerTestClass { }
self.memoryTracker = new WeakMap();
self.worker = createWorkerFactory(() => import('./worker'))();


self.retain = async () => {
    start();

    const func = () => { };
    self.memoryTracker.set(func, new self.WorkerTestClass());
    await self.worker.retain(func);

    done();
}

self.releaseAndTerminate = async () => {
    start();
    await self.worker.release();
    terminate(self.worker)
    done();
};

done();

function start() {
    for (const node of document.querySelectorAll('#' + "WorkerResult")) {
        node.remove();
    }
}

function done() {
    const element = document.createElement('div');
    element.setAttribute('id', "WorkerResult");
    document.body.appendChild(element);
}