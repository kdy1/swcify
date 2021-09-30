/* eslint-env jest */
import {transformSync} from '..';

import {trim, trimmed} from './utilities';

const swc = (code, options?) => {
  const output = transformSync(code, options);
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
      {filename: './file.js'},
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
        filename: './file.js',
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

describe('async transform', () => {
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
          filename: './file.js',
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

describe('i18n transform', () => {
  it('injects arguments into with i18n when adjacent exist', () => {
    const code = trim(`
    import React from "react";
    import { withI18n } from "@shopify/react-i18n";
    
    function MyComponent({ i18n }) {
      return i18n.translate("key");
    }
    
    export default withI18n()(MyComponent);    
      `);
    expect(
      trim(
        swc(code, {
          jsc: {
            target: 'es2020',
          },
          filename:
            './tests/fixtures/i18n/translations/adjacent/MyComponent.js',
        }),
      ),
    ).toBe(
      trim(`
      import _en from "./translations/en.json";
      import React from "react";
      import { withI18n } from "@shopify/react-i18n";
      
      function MyComponent({ i18n  }) {
        return i18n.translate("key");
      }
      
      export default withI18n({
        id: "MyComponent_TODOHASH",
        fallback: _en,
        translations (locale) {
          if ([
            "de",
            "fr",
            "zh-TW"
          ].indexOf(locale) < 0) {
            return;
          }
      
          return import(/* webpackChunkName: "MyComponent_TODOHASH-i18n", webpackMode: "lazy-once" */ \`./translations/\${locale}.json\`).then((dict)=>dict && dict.default
          );
        }
      })(MyComponent);      
      `),
    );
  });
});
