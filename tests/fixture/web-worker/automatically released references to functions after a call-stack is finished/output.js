import _worker from "@shopify/web-worker/webpack-loader?{\"plain\":false}!./worker";
import { createWorkerFactory } from "@shopify/web-worker";
self.prepare = ()=>{
    start();
    self.func = ()=>{
    };
    self.worker = createWorkerFactory(_worker)();
    self.WorkerTestClass = class WorkerTestClass {
        constructor(id){
            this.id = id;
        }
    };
    self.memoryTracker = new WeakMap();
    self.memoryTracker.set(self.func, new self.WorkerTestClass("foo"));
    done();
};
self.run = async ()=>{
    start();
    await self.worker.run(self.func);
    delete self.func;
    done();
};
done();
function start() {
    for (const node of document.querySelectorAll("#" + "WorkerResult")){
        node.remove();
    }
}
function done() {
    const element = document.createElement("div");
    element.setAttribute("id", "WorkerResult");
    document.body.appendChild(element);
}
