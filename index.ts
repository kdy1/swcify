const path = require("path");
const { loadBinding } = require("@node-rs/helper");

// grabs the appropriate native code for our platform
// ("swcify" is the name defined in package.json)
const nativeBindings = loadBinding(
  path.join(__dirname, "native"),
  "swcify",
  "swcify"
);

export type Options = any;

export async function transform(src: any, options: Options = {}) {
  const isModule = typeof src !== "string";

  if (options?.jsc?.parser) {
    options.jsc.parser.syntax = options.jsc.parser.syntax ?? "ecmascript";
  }

  return nativeBindings.transform(
    isModule ? JSON.stringify(src) : src,
    isModule,
    toBuffer(options)
  );
}

export function transformSync(src: any, options: Options = {}) {
  const isModule = typeof src !== "string";

  if (options?.jsc?.parser) {
    options.jsc.parser.syntax = options.jsc.parser.syntax ?? "ecmascript";
  }

  return nativeBindings.transformSync(
    isModule ? JSON.stringify(src) : src,
    isModule,
    toBuffer(options)
  );
}

function toBuffer(raw: Options) {
  return Buffer.from(JSON.stringify(raw));
}
