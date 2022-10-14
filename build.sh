mkdir backend/wasm_file

dfx start --clean --background
dfx canister create --all

dfx build storage

mv ./target/wasm32-unknown-unknown/release/storage.wasm ./backend/wasm_file

dfx build registrar

dfx stop
