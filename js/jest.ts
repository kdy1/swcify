import {cwd} from 'process';
import {existsSync, readFileSync} from 'fs';
import {join} from 'path';

import {Options} from './types';

import {transformSync} from './index';

interface JestConfig26 {
  transform: [match: string, transformerPath: string, options: Options][];
}

interface JestConfig27 {
  transformerConfig: Options;
}

let memoizedOptions: Options;
export function process(
  src: string,
  filename: string,
  jestConfig: JestConfig26 | JestConfig27,
) {
  if (memoizedOptions == null) {
    const options = getInlineConfig(jestConfig) || swcRcConfig();
    set(options, 'jsc.transform.hidden.jest', true);
    memoizedOptions = options;
  }

  return transformSync(src, {...memoizedOptions, filename});
}

function getInlineConfig(
  jestConfig: JestConfig26 | JestConfig27,
): Options | undefined {
  if ('transformerConfig' in jestConfig) {
    // jest 27
    return jestConfig.transformerConfig;
  }

  if ('transform' in jestConfig) {
    // jest 26
    return jestConfig.transform.find(
      ([, transformerPath]) => transformerPath === __filename,
    )?.[2];
  }

  return undefined;
}

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
