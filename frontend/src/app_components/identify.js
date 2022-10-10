
import { AuthClient } from "@dfinity/auth-client";

export async function identify() {
    const authClient = await AuthClient.create();
    console.log(authClient);
    if (authClient.isAuthenticated()) {
        return "already logged in";
    } else {
        authClient.login({
            identityProvider: "https://identity.ic0.app/#authorize",
            onSuccess: () => {
                return "login success";
            },
        });
    }
}