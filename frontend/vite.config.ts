import {defineConfig} from 'vite';
import wasmPack from 'vite-plugin-wasm-pack';

export default defineConfig({
    server: {
        watch: {
            usePolling: true,
        }
    },
    build: {
        minify: true,
        outDir : "./dist",
        assetsDir: "./",
    },
    plugins: [
        wasmPack(["../frontend"])
    ]
});
