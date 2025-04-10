import { Follow, User } from "~/types/types"

export default async function fetchFollowedUsers(userId: number): Promise<User[] | null> {
    try {
        const followings = await fetch(`https://api.exporhub.com:9000/api/follow/user-id?user_id=${userId}`)

        if (!followings.ok) {
            throw new Error(`Failed to find followings`)
        }

        const follows = await followings.json() as Array<Follow>

        let results: Array<User> = []

        for (const follow of follows) {
            const userResponse = await fetch(`https://api.exporhub.com:9000/api/user/user-id?user_id=${follow.following}`)

            if (!userResponse.ok) {
                throw new Error(`Failed to find user`)
            }

            const user = await userResponse.json() as User

            results.push(user)
        }

        return results
    } catch (error) {
        console.error(error)

        return null
    }

}
