if (typeof BigInt === 'undefined') global.BigInt = require('big-integer')
// the above line to solve the error
//     [Error] Unhandled Promise Rejection: ReferenceError: Can't find variable: BigInt
//         (anonymous function)
//         promiseReactionJob

import('./pkg').then((module) => {
    module.run()
})

// import { defineConfig } from 'vite';
// import wasmPack from 'vite-plugin-wasm-pack';
//
// export default defineConfig({
//   // pass your local crate path to the plugin
//   plugins: [wasmPack('./pkg')]
// });