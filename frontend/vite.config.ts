import {defineConfig} from 'vite';
import wasmPack from 'vite-plugin-wasm-pack';
import * as dotenv from 'dotenv';
dotenv.config();
const env = process.env;


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
        'process.env': env,
    },
});