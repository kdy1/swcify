import { eslint } from "@sewing-kit/plugin-eslint";
import { prettier } from "@sewing-kit/plugin-prettier";
import { createPackage, Runtime } from "@sewing-kit/core";
import {
  buildLibrary,
  buildLibraryWorkspace,
} from "@sewing-kit/plugin-build-library";

export default createPackage((pkg) => {
  console.log("test");
  pkg.runtimes(Runtime.Node);
  pkg.entry({ root: "./js/index" });
  try {
    pkg.use(
      buildLibrary({
        jestEnvironment: "jsdom",
        // we don't have browser targets but this option isn't optional atm
        browserTargets: "",
        nodeTargets: "node 12.13",
      }),
      buildLibraryWorkspace({ graphql: false }),
      eslint(),
      prettier({ files: "**/*.{md,json,yaml,yml}" })
    );
  } catch (error) {
    // required because if this breaks skn will eat our errors :(
    console.error(error);
  }
});
