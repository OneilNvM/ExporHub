import { User } from "~/types/types";
import { createSession, deleteSession } from "../lib/session";

export async function signIn(user: User) {
    console.log("hey guys")
    await createSession(user.user_id)
}

export async function logOut() {
    deleteSession()
}