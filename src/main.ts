import {wasm_bindgen} from '../rust/pkg/sprung_siteswap_bg';

rust
  .then(m => m.greet('World!'))
  .catch(console.error);