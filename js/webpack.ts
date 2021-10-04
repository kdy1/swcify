import {getOptions, OptionObject} from 'loader-utils';
import type {LoaderDefinitionFunction} from 'webpack';

import {Options} from './types';

import {transform, transformSync} from './index';

// eslint-disable-next-line func-style
const swcifyLoader: LoaderDefinitionFunction<Options> = function swcifyLoader(
  source,
  inputSourceMap,
) {
  // Make the loader async
  const callback = this.async();
  const filename = this.resourcePath;

  // We have to cast `this` here because the `getOptions` typing
  // does not match the webpack types for loader context.
  const loaderOptions: Options & OptionObject = getOptions(this as any) || {};

  const {sync, parseMap} = loaderOptions;

  const swcOptions = {
    ...removeWebpackOptions(loaderOptions),
    filename,
    inputSourceMap: inputSourceMap ? JSON.stringify(inputSourceMap) : undefined,

    // Support both ways of setting sourceMaps but default to webpacks global settings
    // we force it to boolean because we want "inline" to just act the same as true
    sourceMaps: Boolean(
      loaderOptions.sourceMaps === undefined
        ? this.sourceMap
        : loaderOptions.sourceMaps,
    ),
    sourceFileName: filename,
  };

  // auto detect development mode
  if (
    this.mode &&
    swcOptions.jsc &&
    swcOptions.jsc.transform &&
    swcOptions.jsc.transform.react &&
    !Object.prototype.hasOwnProperty.call(
      swcOptions.jsc.transform.react,
      'development',
    )
  ) {
    swcOptions.jsc.transform.react.development = this.mode === 'development';
  }

  try {
    if (sync) {
      const output = transformSync(source, swcOptions);
      callback(
        null,
        output.code,
        parseMap && output.map ? JSON.parse(output.map) : output.map,
      );
    } else {
      transform(source, swcOptions)
        .then((output) => {
          callback(
            null,
            output.code,
            parseMap && output.map ? JSON.parse(output.map) : output.map,
          );
        })
        .catch(callback);
    }
    // The typing for errors is technically unknown but the webpack
    // callback expects something more specific, so we cast.
  } catch (err: any) {
    callback(err);
  }
};

const WEBPACK_OPTIONS = [
  'sync',
  'parseMap',
  'customize',
  'cacheDirectory',
  'cacheIdentifier',
  'cacheCompression',
  'metadataSubscribers',
];

function removeWebpackOptions(loaderOptions: Options & OptionObject): Options {
  return Object.keys(loaderOptions)
    .filter((key) => !WEBPACK_OPTIONS.includes(key))
    .reduce((obj: any, key: string) => {
      obj[key] = loaderOptions[key];
      return obj;
    }, {});
}

export default swcifyLoader;
