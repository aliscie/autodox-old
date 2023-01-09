import {AuthClient} from "@dfinity/auth-client";
import {createActor as backendActor, canisterId, idlFactory} from './../../../../../src/declarations/backend';
import {Actor, HttpAgent} from "@dfinity/agent";

console.log(process.env)
console.log("canisterId",canisterId)

export async function identify() {
    const authClient = await AuthClient.create();
    if (await authClient.isAuthenticated()) {
        return authClient.getIdentity();
    }

    let identityProvider = "https://identity.ic0.app/#authorize";
    console.log("DFX_NETWORK",process.env.DFX_NETWORK);
    if (process.env.DFX_NETWORK != "ic") {
        identityProvider = `http://${process.env.IDENTITY_PROVIDER_ID}.localhost:4943/#authorize`
    }
    return await authClient.login({
        identityProvider,
        onSuccess: () => {
            window.location.reload()
        }
    });
}


export async function logout() {
    const authClient = await AuthClient.create();
    await authClient.logout()
}

export async function update_profile(image) {
    const actor = await get_actor()
    return await actor.update_profile({image: image});
}

export async function get_profile() {
    const actor = await get_actor()
    let result = await actor.get_profile();
    result = result[0];
    if ( typeof(result.username) == "object") {
        result.username = result.username[0] || "";
    } 
    if ( typeof(result.image) == "object") {
        result.image = result.image[0] || "";
    } 
    return result;
}


export async function is_logged() {
    const authClient = await AuthClient.create();
    return await authClient.isAuthenticated()
}


export const get_actor = async () => {
    const authClient = await AuthClient.create();
    const identity = await authClient.getIdentity();


    const backend = backendActor(canisterId, {
        agentOptions: {
            identity,
            host: "http://127.0.0.1:4943", //This is the canister host // My frontend rus on http://localhost:5173/
        }
    });

    return backend
}


export async function test_connect_wasm_bindgen() {
    let actor = await get_actor()
    return await actor.test_ic();
}

export async function create_directory() {
    let actor = await get_actor()
    return await actor.create_directory();
}

export async function get_directories() {
    let actor = await get_actor()
    return await actor.get_directories();
}


// export async function create_file() {
//     let actor = await get_actor()
//     return await actor.create_file();
// }

export async function register(username) {
    const backend = await get_actor()
    return await backend.register(username);
}

