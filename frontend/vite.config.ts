import { defineConfig } from 'vite';
import wasmPack from 'vite-plugin-wasm-pack';
import EnvironmentPlugin from 'vite-plugin-environment'

export default defineConfig({
    publicDir: './public',
    server: {
        watch: {
            usePolling: true,
            // path: "./src",
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
    ],
    define: {
        'process.env': {
            "BACKEND_CANISTER_ID": "rrkah-fqaaa-aaaaa-aaaaq-cai",
            "FRONTEND_CANISTER_ID": "ryjl3-tyaaa-aaaaa-aaaba-cai",
            "INTERNET_IDENTITY_CANISTER_ID": "r7inp-6aaaa-aaaaa-aaabq-cai",
            "IDENTITY_PROVIDER_ID": "r7inp-6aaaa-aaaaa-aaabq-cai",
            "USE_WALLET": true,
        }
    },
});
