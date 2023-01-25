import { defineConfig } from 'vite';
import wasmPack from 'vite-plugin-wasm-pack';

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
            "FRONTEND_CANISTER_ID": "r7inp-6aaaa-aaaaa-aaabq-cai",
            "INTERNET_IDENTITY_CANISTER_ID": "ryjl3-tyaaa-aaaaa-aaaba-cai",
            "IDENTITY_PROVIDER_ID": "ryjl3-tyaaa-aaaaa-aaaba-cai",
            "USE_WALLET": true,
        }
    },
});