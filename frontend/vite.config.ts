import { defineConfig } from 'vite';
import wasmPack from 'vite-plugin-wasm-pack';

export default defineConfig({
    build:{
        minify : false
    },
    // pass your local crate path to the plugin
    plugins: [wasmPack(["../frontend"])]
});
