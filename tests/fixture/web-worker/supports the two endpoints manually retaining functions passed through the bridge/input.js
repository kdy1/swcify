import { createWorkerFactory } from '@shopify/web-worker';

// See previous test for details of these test utilities
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

self.release = async () => {
    start();
    await self.worker.release();
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