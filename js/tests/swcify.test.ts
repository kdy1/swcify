/* eslint-env jest */
import {transform, transformSync} from '..';

import {trim, trimmed} from './utilities';

const swc = (code, options?) => {
  const output = transformSync(code, options);
  return output.code;
};

const swcAsync = async (code, options?) => {
  const output = await transform(code, options);
  return output.code;
};

const defaultPackage = '@shopify/async';
const defaultImport = 'createResolver';

describe('swcify', () => {
  it('returns JS', () => {
    const code = swc(
      trimmed`
      import {foo} from 'bar';

      export function helloWorld() {
        console.log("hi ", foo);
      }
    `,
    );
    expect(trim(code)).toMatch(trimmed`
    import { foo } from 'bar';
    export function helloWorld() {
        console.log(\"hi \", foo);
    }
  `);
  });

  it('returns JS Async', async () => {
    const code = await swcAsync(
      trimmed`
      import {foo} from 'bar';

      export function helloWorld() {
        console.log("hi ", foo);
      }
    `,
    );
    expect(trim(code)).toMatch(trimmed`
    import { foo } from 'bar';
    export function helloWorld() {
        console.log(\"hi \", foo);
    }
  `);
  });

  it('respects options', () => {
    const code = swc(
      trimmed`
      async function f() {
      }
      await f();
    `,
      {
        jsc: {
          parser: {
            syntax: 'ecmascript',
            topLevelAwait: true,
          },
          target: 'es2017',
          externalHelpers: true,
        },
      },
    );

    expect(trim(code)).toMatch(trimmed`
    import * as swcHelpers from \"@swc/helpers\";
    function _f() {
    _f = swcHelpers.asyncToGenerator(function*() {
    });
    return _f.apply(this, arguments);
    }
    function f() {
    return _f.apply(this, arguments);
    }
    await f();
  `);
  });
});

describe('Custom AsyncTransform', () => {
  it('adds an id prop that returns the require.resolveWeak of the first dynamic import in load', () => {
    const code = trim(`
        import { ${defaultImport} } from '${defaultPackage}';
  
        ${defaultImport}({
          load: ()=> import("./Foo"),
        });
      `);
    expect(
      trim(
        swc(code, {
          jsc: {
            target: 'es2020',
          },
        }),
      ),
    ).toBe(
      trim(`
        import { ${defaultImport} } from '${defaultPackage}';
  
        ${defaultImport}({
              load: ()=>import("./Foo")
              ,
              id: ()=>require.resolveWeak("./Foo")
        });
      `),
    );
  });
});
