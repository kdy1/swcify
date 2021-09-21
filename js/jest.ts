import {cwd} from 'process';
import {existsSync, readFileSync} from 'fs';
import {join} from 'path';

import type {Transformer} from '@jest/transform';

import {Options} from './types';

import {transformSync} from './index';

let memoizedOptions: Options;
const transformer: Transformer<Options> = {
  process(src, filename, jestConfig) {
    if (memoizedOptions == null) {
      const options = jestConfig.transformerConfig || swcRcConfig();
      set(options, 'jsc.transform.hidden.jest', true);
      memoizedOptions = options;
    }

    return transformSync(src, {...memoizedOptions, filename});
  },
};

function swcRcConfig(): Options {
  const swcrc = join(cwd(), '.swcrc');
  const options: Options = existsSync(swcrc)
    ? (JSON.parse(readFileSync(swcrc, 'utf-8')) as Options)
    : {};

  return options;
}

function set(obj: any, path: string, value: any) {
  const parents = path.split('.');
  const key = parents.pop() as string;

  let currentTarget = obj;
  for (const prop of parents) {
    if (currentTarget[prop] == null) {
      currentTarget[prop] = {};
    }
    currentTarget = currentTarget[prop];
  }

  currentTarget[key] = value;
}

export default transformer;
