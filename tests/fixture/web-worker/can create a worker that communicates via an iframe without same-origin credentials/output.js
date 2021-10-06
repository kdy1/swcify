import _worker from "@shopify/web-worker/webpack-loader?{\"plain\":false}!./worker";
import { createWorkerFactory, createIframeWorkerMessenger } from "@shopify/web-worker";
const worker = createWorkerFactory(_worker)({
    createMessenger: createIframeWorkerMessenger
});
(async ()=>{
    document.cookie = "MY_COOKIE" + "=1";
    const result = await worker.default();
    const element = document.createElement("div");
    element.setAttribute("id", "WorkerResult");
    element.textContent = result;
    document.body.appendChild(element);
})();
