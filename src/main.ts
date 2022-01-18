import { default as wasmbin } from '../rust/pkg/sprung_siteswap_bg.wasm';
import init, {get_canonical_form} from "../rust/pkg/sprung_siteswap";

async function run() {
	await init(wasmbin);
    console.log(get_canonical_form("icziczcB"));
}
run();

