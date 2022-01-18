import { default as wasmbin } from '../rust/pkg/sprung_siteswap_bg.wasm';
import init, {get_canonical_form} from "../rust/pkg/sprung_siteswap";

async function run() {
	await init(wasmbin);
    const input = document.getElementById("input") as HTMLInputElement;
    const output = document.getElementById("output");
    input.value = "icziczcB";

    function updateOutput() {
        try {
            const canonical = get_canonical_form(input.value.trim());
            output.textContent = canonical;
        } catch (e) {
            output.textContent = "Notation invalid.";
        }
    }

    input.addEventListener("input", updateOutput);
    updateOutput();
}
run();

