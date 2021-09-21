import transformer from '../jest';

const originalFs = jest.requireActual('fs');
jest.mock('fs', () => {
  // we can't use the identical ref outside of this because of weird jest hoisting
  const originalFs = jest.requireActual('fs');
  return {
    existsSync: jest.fn(originalFs.existsSync),
    readFileSync: jest.fn(),
  };
});
jest.mock('../index');

const mockFs: {existsSync: jest.Mock; readFileSync: jest.Mock} =
  jest.requireMock('fs');
const mockSwc: {transformSync: jest.Mock} = jest.requireMock('..');

describe('jest transform', () => {
  beforeEach(() => {
    mockFs.readFileSync.mockReset();
    mockFs.readFileSync.mockImplementation(() => '');
    mockSwc.transformSync.mockReset();
    mockSwc.transformSync.mockImplementation(() => ({code: ''}));
  });

  it('passes src, filename, and jest:hidden to swcify', () => {
    const code = 'console.log("hi")';
    const filename = 'code.js';
    transformer.process(code, filename, {} as any);
    expect(mockSwc.transformSync).toHaveBeenCalledWith(code, {
      filename,
      jsc: {transform: {hidden: {jest: true}}},
    });
  });

  it('uses a .swcrc if the file exists', () => {
    const code = 'console.log("hello")';
    const filename = 'code2.js';
    const config = {
      jsc: {
        externalHelpers: false,
        parser: {
          syntax: 'typescript',
          tsx: true,
          decorators: true,
        },
      },
    };
    mockSwcRc(config);
    transformer.process(code, filename, {} as any);
    expect(mockSwc.transformSync).toHaveBeenCalledWith(code, {
      ...config,
      filename,
      jsc: {transform: {hidden: {jest: true}}},
    });
  });

  it('overrides swcrc with config passed through jest', () => {
    const code = 'console.log("hello")';
    const filename = 'code.js';
    const config = {
      jsc: {
        externalHelpers: true,
        parser: {
          syntax: 'ecmascript',
          jsx: true,
        },
      },
    };
    mockSwcRc({
      jsc: {
        externalHelpers: false,
        parser: {
          syntax: 'typescript',
          tsx: true,
          decorators: true,
        },
      },
    });
    transformer.process(code, filename, {transformerConfig: config} as any);
    expect(mockSwc.transformSync).toHaveBeenCalledWith(code, {
      filename,
      jsc: {transform: {hidden: {jest: true}}},
    });
  });
});

function mockSwcRc(config: any) {
  mockFs.existsSync.mockImplementation((input: string) => {
    if (input === '.swcrc') {
      return true;
    }
    return originalFs.existsSync(input);
  });
  mockFs.readFileSync.mockImplementation((input: string) => {
    if (input === '.swcrc') {
      return JSON.stringify(config);
    }
    return originalFs.readFileSync(input);
  });
}
