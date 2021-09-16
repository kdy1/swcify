import {join} from 'path';

import {loadBinding} from '@node-rs/helper';

import type {Source, Options} from './types';

export type {Source, Options};

// grabs the appropriate native code for our platform
// ("swcify" is the name defined in package.json)
const nativeBindings = loadBinding(getNativeBinaryDir(), 'swcify', 'swcify');

export async function transform(src: Source, options: Options = {}) {
  const isModule = typeof src !== 'string';

  if (options && options.jsc && options.jsc.parser) {
    options.jsc.parser.syntax = options.jsc.parser.syntax || 'ecmascript';
  }

  return nativeBindings.transform(
    isModule ? JSON.stringify(src) : src,
    isModule,
    toBuffer(options),
  );
}

export function transformSync(src: Source, options: Options = {}) {
  const isModule = typeof src !== 'string';

  if (options && options.jsc && options.jsc.parser) {
    options.jsc.parser.syntax = options.jsc.parser.syntax || 'ecmascript';
  }

  return nativeBindings.transformSync(
    isModule ? JSON.stringify(src) : src,
    isModule,
    toBuffer(options),
  );
}

function toBuffer(raw: any) {
  return Buffer.from(JSON.stringify(raw));
}

function getNativeBinaryDir() {
  // 💩 we know that in built code we are nested an extra level from root.
  if (__dirname.endsWith('build/cjs')) {
    return join(__dirname, '..', '..', 'native');
  } else {
    return join(__dirname, '..', 'native');
  }
}
