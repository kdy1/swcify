import {join} from 'path';
import {existsSync} from 'fs';

import {loadBinding} from '@node-rs/helper';

import type {Source, Options} from './types';

export type {Source, Options};

// grabs the appropriate dist code for our platform
// ("swcify" is the name defined in package.json)
const bindings = loadBinding(getBinaryDir(), 'swcify', 'swcify');

export async function transform(src: Source, options: Options = {}) {
  const isModule = typeof src !== 'string';

  if (options && options.jsc && options.jsc.parser) {
    options.jsc.parser.syntax = options.jsc.parser.syntax || 'ecmascript';
  }

  return bindings.transform(
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

  return bindings.transformSync(
    isModule ? JSON.stringify(src) : src,
    isModule,
    toBuffer(options),
  );
}

function toBuffer(raw: any) {
  return Buffer.from(JSON.stringify(raw));
}

function getBinaryDir() {
  // 💩 we know that in built code we are nested an extra level from root.
  const pathToRoot = __dirname.endsWith('build/cjs')
    ? join(__dirname, '..', '..')
    : join(__dirname, '..');

  // use the temp gitignored local builds if we have them otherwise use the canonical builds
  return existsSync(join(pathToRoot, 'dev'))
    ? join(pathToRoot, 'dev')
    : join(pathToRoot, 'dist');
}
