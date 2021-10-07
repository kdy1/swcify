import _worker from "@shopify/web-worker/webpack-loader?{\"name\":\"myFancyWorker\",\"plain\":false}!./worker";
import { createWorkerFactory } from "@shopify/web-worker";
const worker = createWorkerFactory(_worker)();
(async ()=>{
    const element = document.createElement("div");
    element.setAttribute("id", "WorkerResult");
    document.body.appendChild(element);
})();
