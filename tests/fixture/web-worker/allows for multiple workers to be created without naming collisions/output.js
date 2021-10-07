import _worker from "@shopify/web-worker/webpack-loader?{\"name\":\"MyWorker\",\"plain\":false}!./worker";
import _worker1 from "@shopify/web-worker/webpack-loader?{\"plain\":false}!./worker2";
import { createWorkerFactory } from "@shopify/web-worker";
const workerOne = createWorkerFactory(_worker)();
const workerTwo = createWorkerFactory(_worker1)();
(async ()=>{
    const results = await Promise.all([
        workerOne.default(),
        workerTwo.default()
    ]);
    const element = document.createElement("div");
    element.setAttribute("id", "WorkerResult");
    element.textContent = results.join(" ");
    document.body.appendChild(element);
})();
