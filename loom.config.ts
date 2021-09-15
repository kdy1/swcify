import { createPackage, Runtime } from "@shopify/loom";
import { babel } from "@shopify/loom-plugin-babel";
import { eslint } from "@shopify/loom-plugin-eslint";
import { prettier } from "@shopify/loom-plugin-prettier";
import { packageBuild } from "@shopify/loom-plugin-package-build";
import { workspaceTypeScript } from "@shopify/loom-plugin-typescript";
import { jest } from "@shopify/loom-plugin-jest";

export default createPackage((pkg) => {
  pkg.runtimes(Runtime.Node);
  pkg.entry({ root: "./js/index" });

  try {
    pkg.use(
      babel({
        config: {
          presets: [
            [
              require.resolve("@shopify/babel-preset"),
              { typescript: true, react: true },
            ],
          ],
          configFile: false,
        },
      }),
      packageBuild({
        browserTargets: "defaults",
        nodeTargets: "node 12.20",

        /*
          We set this false because we are manually creating them
          (because otherwise they are created with invalid paths)
        */
        rootEntrypoints: false,
      }),
      jest(),
      eslint(),
      prettier({ files: "**/*.{md,json,yaml,yml}" }),
      workspaceTypeScript()
    );
  } catch (error) {
    // required because if this breaks skn will eat our errors :(
    // eslint-disable-next-line no-console
    console.error(error);
  }
});
