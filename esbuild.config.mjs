import esbuild from "esbuild";
import process from "process";
import wasmpack from "esbuild-plugin-wasm-pack";
import console from "console";

const prod = process.argv[2] === "production";


esbuild
	.build({
		platform: "browser",
		plugins: [
			wasmpack.wasmPack({
				path: "rust",
				profile: "dev",
				target: "no-modules",
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
