import {AuthClient} from "@dfinity/auth-client";

export async function identify() {
    const authClient = await AuthClient.create();
    if (await authClient.isAuthenticated()) {
        return authClient.getIdentity();
    }
    return await authClient.login({identityProvider: "https://identity.ic0.app"});
}
