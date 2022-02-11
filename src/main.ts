import { default as wasmbin } from '../rust/pkg/sprung_siteswap_bg.wasm';
import init, {parse, Pattern} from "../rust/pkg/sprung_siteswap";

async function run() {
	await init(wasmbin);
    const input = document.getElementById("input") as HTMLInputElement;
    const canonical = document.getElementById("canonical");
    const siteswap = document.getElementById("siteswap");
    const hands = document.getElementById("hands");
    const anim = document.getElementById("anim") as HTMLImageElement;
    const error = document.getElementById("error");
    const bps = document.getElementById("bps") as HTMLInputElement;
    const bps_display = document.getElementById("bps_display");

    function updateBps() {
        bps_display.textContent = bps.value;
        updateOutput();
    }

    bps.addEventListener("change", updateBps);

    const queryString = window.location.search;
    const queryParams = new URLSearchParams(queryString);
    const pattern = queryParams.get("q");

    input.value = pattern ?? "icziczcaB";

    function updateOutput() {
        const pattern = parse(input.value.trim()); 
        if (pattern.error) {
            canonical.textContent = "";
            siteswap.textContent = "";
            hands.textContent = "";
            anim.src = "";
            error.textContent = pattern.error;
            return;
        }
        error.textContent = "";
        canonical.textContent = pattern.canonical;
        siteswap.textContent = pattern.siteswap;
        hands.textContent = pattern.hands;
        anim.src = "";
        let url = pattern.juggle_anim_url;
        url = url.replace(/bps=\d*/, "bps=" + bps.value);
        anim.src = url;

        window.history.pushState('', '', "?q=" + input.value);

    }

    input.addEventListener("input", updateOutput);
    //updateOutput();
    updateBps();

}
run();

