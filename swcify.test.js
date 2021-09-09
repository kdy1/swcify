/* eslint-env jest */
const { trim, trimmed } = require("./tests/utilities");
const { transformSync } = require(".");

describe("swcify", () => {
  it("returns JS", () => {
    const { code } = transformSync(
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

  it("respects options", () => {
    const { code } = transformSync(
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
