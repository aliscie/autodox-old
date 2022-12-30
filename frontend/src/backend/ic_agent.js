import {AuthClient} from "@dfinity/auth-client";
import {createActor, canisterId, idlFactory} from './../../../../../src/declarations/backend';
import {Actor, HttpAgent} from "@dfinity/agent";

export async function identify() {
    const authClient = await AuthClient.create();
    if (await authClient.isAuthenticated()) {
        return authClient.getIdentity();
    }
    return await authClient.login({identityProvider: "https://identity.ic0.app"});
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
    // test http://localhost:8000 main https://ic0.app
    const authClient = await AuthClient.create();
    const identity = await authClient.getIdentity();
    console.log("identity", identity);
    const backend = createActor(canisterId, {
        identity,
        host: "https://ic0.app",
        // host: window.location.href,
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

