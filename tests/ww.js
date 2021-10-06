
// @TODO: Disabled for now because these tests are flaky and take a long time to run
// eslint-disable-next-line jest/no-disabled-tests
describe.skip('web-worker', () => {
  it('can create a worker that communicates via an iframe without same-origin credentials', async () => {
    const cookie = 'MY_COOKIE';
    const noCookiesResult = 'NoCookies';
    const cookiesResult = 'Cookies';
    const testId = 'WorkerResult';

    await withContext('no-same-origin', async (context) => {
      const { workspace, browser, server } = context;

      const path = '/app/ping';

      server.use((ctx, next) => {
        if (ctx.originalUrl === path) {
          ctx.type = 'text';
          ctx.body = ctx.cookies.get(cookie) ? cookiesResult : noCookiesResult;
          ctx.set('Access-Control-Allow-Origin', '*');
          return;
        }

        return next();
      });

      await workspace.write(
        mainFile,
        `
           import {createWorkerFactory, createIframeWorkerMessenger} from '@shopify/web-worker';
 
           const worker = createWorkerFactory(() => import('./worker'))({
             createMessenger: createIframeWorkerMessenger,
           });
 
           (async () => {
             document.cookie = ${JSON.stringify(cookie)} + '=1';
             const result = await worker.default();
             const element = document.createElement('div');
             element.setAttribute('id', ${JSON.stringify(testId)});
             element.textContent = result;
             document.body.appendChild(element);
           })();
         `,
      );

      await workspace.write(
        workerFile,
        `
           export default async function run() {
             const result = await fetch(${JSON.stringify(
          server.url(path),
        )}, {method: 'GET'});
             return result.text();
           }
         `,
      );

      await runWebpack(context);

      const page = await browser.go();
      const workerElement = await page.waitForSelector(`#${testId}`);
      const textContent = await workerElement!.evaluate(
        (element) => element.innerHTML,
      );

      expect(textContent).toBe(noCookiesResult);
    });
  });
});
