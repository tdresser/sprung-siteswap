import { default as wasmbin } from '../rust/pkg/sprung_siteswap_bg.wasm';
import init, {parse, Pattern} from "../rust/pkg/sprung_siteswap";

async function run() {
	await init(wasmbin);
    const input = document.getElementById("input") as HTMLInputElement;
    const canonical = document.getElementById("canonical");
    const siteswap = document.getElementById("siteswap");
    const anim = document.getElementById("anim") as HTMLImageElement;
    const error = document.getElementById("error");

    input.value = "icziczcaB";

    function updateOutput() {
        const pattern = parse(input.value.trim()); 
        if (pattern.error) {
            canonical.textContent = "";
            siteswap.textContent = "";
            anim.src = "";
            error.textContent = pattern.error;
            return;
        }
        canonical.textContent = pattern.canonical;
        siteswap.textContent = pattern.siteswap;
        anim.src = pattern.juggle_anim_url;
    }

    input.addEventListener("input", updateOutput);
    updateOutput();
}
run();

