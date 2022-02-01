import esbuild from "esbuild";
import process from "process";
import wasmpack from "esbuild-plugin-wasm-pack";
import console from "console";
import fs from "fs/promises"

const prod = process.argv[2] === "production";

esbuild
  .build({
    platform: "browser",
    plugins: [
      wasmpack.wasmPack({
        path: "rust",
        profile: "dev",//prod ? "release " : "dev", // TODO
        target: "web",
      }),
    ],
    entryPoints: ["src/main.ts"],
    bundle: true,
    format: "esm",
    watch: !prod,
    target: "esnext",
    logLevel: "info",
    sourcemap: prod ? false : "inline",
    treeShaking: true,
    outdir: "build",
    loader: {
        ".wasm": "binary",
        ".jar.js": "js",
    },
  })
  .catch((e) => {
    console.log(e);
    process.exit(1);
  });