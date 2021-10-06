import _worker from "/Users/kdy1/projects/swcify/test-ref-gen/node_modules/@shopify/web-worker/build/cjs/webpack-parts/loader?{\"plain\":false}!./worker";
import { createWorkerFactory } from '@shopify/web-worker';

self.prepare = () => {
  start(); // Store this on self so we can use it when we call the worker

  self.func = () => {}; // Store this on self so we retain it and its function store,
  // which should lead to memory leaks if the function store is not cleaned.


  self.worker = createWorkerFactory(_worker)(); // Store this on self so we can get access to it to
  // count the non-GC'ed instances

  self.WorkerTestClass = class WorkerTestClass {
    constructor(id) {
      this.id = id;
    }

  }; // Store this on self so we have a retained reference that
  // references both the function and an instance of the test class

  self.memoryTracker = new WeakMap();
  self.memoryTracker.set(self.func, new self.WorkerTestClass('foo'));
  done();
};

self.run = async () => {
  start();
  await self.worker.run(self.func); // Delete the reference so we no longer have any
  // direct retain paths to the function, allowing it
  // to be GC'ed (and, by extension, to be removed from the
  // memoryTracker WeakMap).

  delete self.func;
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
