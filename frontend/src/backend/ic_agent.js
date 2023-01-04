import {AuthClient} from "@dfinity/auth-client";
import {createActor as backendActor, canisterId, idlFactory} from './../../../../../src/declarations/backend';
import {Actor, HttpAgent} from "@dfinity/agent";

console.log(process.env)
console.log(canisterId)

export async function identify() {
    const authClient = await AuthClient.create();
    if (await authClient.isAuthenticated()) {
        return authClient.getIdentity();
    }

    let identityProvider = "https://identity.ic0.app/#authorize";
    if (process.env.DFX_NETWORK != "ic") {
        identityProvider = "http://127.0.0.1:4943/?canisterId=rkp4c-7iaaa-aaaaa-aaaca-cai&id=r7inp-6aaaa-aaaaa-aaabq-cai"
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

export async function update_profile(data) {
    const actor = await createActor(canisterId)
    return await actor.update_profile(data);
}

export async function get_profile() {
    const actor = await createActor(canisterId)
    return await actor.get_profile();
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
            // host,
        }
    });

    return backend
}


export async function test_connect_wasm_bindgen() {
    let actor = await get_actor()
    return await actor.test_ic();
}

export async function register(username) {
    const backend = await get_actor()
    return await backend.register(username);
}

