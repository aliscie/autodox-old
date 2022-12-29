import {defineConfig} from 'vite';
import wasmPack from 'vite-plugin-wasm-pack';
import EnvironmentPlugin from 'vite-plugin-environment'

export default defineConfig({
    publicDir: './public',
    server: {
        watch: {
            usePolling: true,
            // path: "./src",  TODO use rollup because vite does not reload on changes.
        }
    },
    build: {
        minify: true,
        outDir: "./dist",
        emptyOutDir: true,
        copyPublicDir: true,
    },
    plugins: [
        wasmPack(["../frontend"]),
        // EnvironmentPlugin(['BACKEND_CANISTER_ID',"DFX_NETWORK","FRONTEND_CANISTER_ID"]),

    ],
    define: {
        'process.env': {"BACKEND_CANISTER_ID": "ryjl3-tyaaa-aaaaa-aaaba-cai"}
    },
});
