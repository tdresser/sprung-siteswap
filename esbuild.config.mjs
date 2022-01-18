import esbuild from "esbuild";
import process from "process";
import wasmpack from "esbuild-plugin-wasm-pack";
import console from "console";
import path from "path";
import fs from "fs";

const prod = process.argv[2] === "production";

let wasmPlugin = {
  name: "wasm",
  setup(build) {
    // Resolve ".wasm" files to a path with a namespace
    build.onResolve({ filter: /\.wasm$/ }, (args) => {
      // Resolve relative paths to absolute paths here since this
      // resolve callback is given "resolveDir", the directory to
      // resolve imports against.
      if (args.resolveDir === "") {
        return; // Ignore unresolvable paths
      }
      return {
        path: path.isAbsolute(args.path)
          ? args.path
          : path.join(args.resolveDir, args.path),
        namespace: "wasm-binary",
      };
    });

    // Virtual modules in the "wasm-binary" namespace contain the
    // actual bytes of the WebAssembly file. This uses esbuild's
    // built-in "binary" loader instead of manually embedding the
    // binary data inside JavaScript code ourselves.
    build.onLoad({ filter: /.*/, namespace: "wasm-binary" }, async (args) => ({
      contents: await fs.promises.readFile(args.path),
      loader: "binary",
    }));
  },
};

esbuild
  .build({
    platform: "browser",
    plugins: [
      wasmPlugin,
      wasmpack.wasmPack({
        path: "rust",
        profile: "dev",
        target: "web",
      }),
    ],
    entryPoints: ["src/main.ts"],
    bundle: true,
    format: "esm",
    watch: true,
    target: "esnext",
    logLevel: "info",
    sourcemap: prod ? false : "inline",
    treeShaking: true,
    outdir: "build",
  })
  .catch((e) => {
    console.log(e);
    process.exit(1);
  });
