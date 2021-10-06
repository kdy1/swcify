
// @TODO: Disabled for now because these tests are flaky and take a long time to run
// eslint-disable-next-line jest/no-disabled-tests
describe.skip('web-worker', () => {
  it('allows for multiple workers to be created without naming collisions', async () => {
    const workerOneMessage = 'Hello';
    const workerTwoMessage = 'world';
    const testId = 'WorkerResult';

    await withContext('multiple-workers', async (context) => {
      const { workspace, browser } = context;

      await workspace.write(
        mainFile,
        `
           import {createWorkerFactory} from '@shopify/web-worker';
 
           const workerOne = createWorkerFactory(() => import(/* webpackChunkName: 'MyWorker' */ './worker'))();
           const workerTwo = createWorkerFactory(() => import('./worker2'))();
 
           (async () => {
             const results = await Promise.all([
               workerOne.default(),
               workerTwo.default(),
             ]);
 
             const element = document.createElement('div');
             element.setAttribute('id', ${JSON.stringify(testId)});
             element.textContent = results.join(' ');
             document.body.appendChild(element);
           })();
         `,
      );

      await workspace.write(
        workerFile,
        `
           export default function(name) {
             return ${JSON.stringify(workerOneMessage)};
           }
         `,
      );

      await workspace.write(
        secondWorkerFile,
        `
           export default function(name) {
             return ${JSON.stringify(workerTwoMessage)};
           }
         `,
      );

      await runWebpack(context);

      const page = await browser.go();
      const workerElement = await page.waitForSelector(`#${testId}`);

      const textContent = await workerElement!.evaluate(
        (element) => element.innerHTML,
      );
      expect(textContent).toBe(`${workerOneMessage} ${workerTwoMessage}`);
    });
  });

  it('allows setting a custom worker file name using the webpackChunkName directive', async () => {
    const name = 'myFancyWorker';
    const testId = 'WorkerResult';

    await withContext('custom-worker-name', async (context) => {
      const { workspace, browser } = context;

      await workspace.write(
        mainFile,
        `
           import {createWorkerFactory} from '@shopify/web-worker';
 
           const worker = createWorkerFactory(() => import(/* webpackChunkName: ${JSON.stringify(
          name,
        )} */ './worker'))();
 
           (async () => {
             const element = document.createElement('div');
             element.setAttribute('id', ${JSON.stringify(testId)});
             document.body.appendChild(element);
           })();
         `,
      );

      await workspace.write(
        workerFile,
        `
           export default function(name) {
             return 'Hello world';
           }
         `,
      );

      await runWebpack(context);

      const page = await browser.go();
      await page.waitForSelector(`#${testId}`);

      expect(await getWorkerSource(page.workers()[0], page)).toContain(
        `${name}.worker`,
      );
    });
  });

  it('can create a "plain" worker factory that can produce workers wrapping the original module', async () => {
    const greetingPrefix = 'Hello ';
    const greetingTarget = 'world';
    const testId = 'WorkerResult';

    await withContext('plain', async (context) => {
      const { workspace, browser } = context;

      await workspace.write(
        mainFile,
        `
           import {createPlainWorkerFactory} from '@shopify/web-worker';
 
           const worker = createPlainWorkerFactory(() => import('./worker'))();
 
           (async () => {
             const result = await new Promise((resolve) => {
               worker.addEventListener('message', ({data}) => {
                 resolve(data);
               });
 
               worker.postMessage(${JSON.stringify(greetingTarget)});
             });
 
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
           self.addEventListener('message', ({data}) => {
             self.postMessage(\`${greetingPrefix}\${data}\`);
           });
         `,
      );

      await runWebpack(context);

      const page = await browser.go();
      const workerElement = await page.waitForSelector(`#${testId}`);
      const textContent = await workerElement!.evaluate(
        (element) => element.innerHTML,
      );

      expect(textContent).toBe(`${greetingPrefix}${greetingTarget}`);
    });
  });

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
