import { createWorkerFactory } from "@shopify/web-worker";
const worker = (()=>new Proxy({
    }, {
        get () {
            return ()=>{
                throw new Error("You can’t call a method on a noop worker");
            };
        }
    })
)();
(async ()=>{
    const element = document.createElement("div");
    element.setAttribute("id", "WorkerResult");
    try {
        await worker.willThrow();
    } catch (error) {
        element.textContent = error.message;
    }
    document.body.appendChild(element);
})();
