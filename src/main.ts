import { default as wasmbin } from '../rust/pkg/sprung_siteswap_bg.wasm';
import init, {get_canonical_form, get_traditional_siteswap} from "../rust/pkg/sprung_siteswap";

async function run() {
	await init(wasmbin);
    const input = document.getElementById("input") as HTMLInputElement;
    const canonical = document.getElementById("canonical");
    const siteswap = document.getElementById("siteswap");

    input.value = "icziczcaB";

    function updateOutput() {
        try {
            canonical.textContent = get_canonical_form(input.value.trim());
            siteswap.textContent = get_traditional_siteswap(input.value.trim());
        } catch (e) {
            canonical.textContent = "Notation invalid.";
            siteswap.textContent = "";
        }
    }

    input.addEventListener("input", updateOutput);
    updateOutput();
}
run();

