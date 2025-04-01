import { User } from "~/types/types";
import { createSession } from "../lib/session";

export async function signIn(user: User) {
    await createSession(user.user_id)
}
