import _worker from "@shopify/web-worker/webpack-loader?{\"plain\":false}!./worker";
import { createWorkerFactory } from "@shopify/web-worker";
self.WorkerTestClass = class WorkerTestClass {
};
self.memoryTracker = new WeakMap();
self.worker = createWorkerFactory(_worker)();
self.retain = async ()=>{
    start();
    const funcOne = ()=>{
    };
    const funcTwo = ()=>{
    };
    self.memoryTracker.set(funcOne, new self.WorkerTestClass());
    self.memoryTracker.set(funcTwo, new self.WorkerTestClass());
    await self.worker.retain({
        funcOne,
        funcs: [
            funcOne,
            funcTwo
        ]
    });
    done();
};
self.release = async ()=>{
    start();
    await self.worker.release();
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
