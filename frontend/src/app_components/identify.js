import {AuthClient} from "@dfinity/auth-client";

export async function identify() {

    const authClient = await AuthClient.create();
    let id = await authClient.getIdentity();
    if (authClient.isAuthenticated()) {
        return id;
    } else {
        authClient.login({
            identityProvider: 'https://identity.ic0.app/#authorize',
            onSuccess: async () => {
                let identity = await authClient.getIdentity();
                return identity;
            },
        })
    }
}