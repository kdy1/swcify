/* eslint-env jest */
const { trim, trimmed } = require("./tests/utilities");
const { transform } = require(".");

const swc = async (code, options) => {
  let output = await transform(code, options);
  return output.code;
};

const defaultPackage = "@shopify/async";
const defaultImport = "createResolver";

describe("swcify", () => {
  it("returns JS", async () => {
    const code = await swc(
      trimmed`
      import {foo} from 'bar';

      export function helloWorld() {
        console.log("hi ", foo);
      }
    `
    );
    expect(trim(code)).toMatch(trimmed`
    import { foo } from 'bar';
    export function helloWorld() {
        console.log(\"hi \", foo);
    }
  `);
  });

  it("respects options", async () => {
    const code = await swc(
      trimmed`
      async function f() {
      }
      await f();
    `,
      {
        jsc: {
          parser: { topLevelAwait: true },
          target: "es2017",
          externalHelpers: true,
        },
      }
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

describe("async transform", () => {
  it("adds an id prop that returns the require.resolveWeak of the first dynamic import in load", async () => {
    const code = trim(`
        import { ${defaultImport} } from '${defaultPackage}';
  
        ${defaultImport}({
          load: ()=> import(\"./Foo\"),
        });
      `);
    expect(
      trim(await swc(code, {
        jsc: {
          target: "es2020",
        },
      }))
    ).toBe(
      trim(`
        import { ${defaultImport} } from '${defaultPackage}';
  
        ${defaultImport}({
              load: ()=>import(\"./Foo\")
              ,
              id: ()=>require.resolveWeak(\"./Foo\")
        });
      `)
    );
  });
});
