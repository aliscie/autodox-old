import {defineConfig} from 'vite';
import wasmPack from 'vite-plugin-wasm-pack';
import wasm from "vite-plugin-wasm";
import topLevelAwait from "vite-plugin-top-level-await";

export default defineConfig({
    build: {
        minify: false
    },
    plugins: [
        wasmPack(["../frontend"])
    ]
});
