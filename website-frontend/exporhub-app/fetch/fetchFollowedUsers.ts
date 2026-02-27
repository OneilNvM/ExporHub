import { Follow, ResponseStatus, User } from "~/types/types"

export default async function fetchFollowedUsers(userId: number): Promise<User[] | null> {
    try {
        const followings = await fetch(`https://api.exporhub.com:9000/api/follow/user-id?user_id=${userId}`, {
            method: "GET",
            headers: {
                "Content-Type": "application/json",
                "Authorization": `Basic ${process.env.NEXT_PUBLIC_EXPORHUB_API_KEY}`
            }
        })

        if (!followings.ok) {
            const error = await followings.json() as ResponseStatus
            throw new Error(`\nCode: ${error.code}\nMessage: ${error.message}`)
        }

        const follows = await followings.json() as Array<Follow>

        let results: Array<User> = []

        for (const follow of follows) {
            const userResponse = await fetch(`https://api.exporhub.com:9000/api/user/user-id?user_id=${follow.following}`, {
                method: "GET",
                headers: {
                    "Content-Type": "application/json",
                    "Authorization": `Basic ${process.env.NEXT_PUBLIC_EXPORHUB_API_KEY}`
                }
            })

            if (!userResponse.ok) {
                const error = await userResponse.json() as ResponseStatus
                throw new Error(`\nCode: ${error.code}\nMessage: ${error.message}`)            }

            const user = await userResponse.json() as User

            results.push(user)
        }

        return results
    } catch (error) {
        console.error(error)

        return null
    }

}
