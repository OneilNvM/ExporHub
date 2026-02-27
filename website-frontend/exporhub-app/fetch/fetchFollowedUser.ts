import { ResponseStatus } from "~/types/types"

export default async function fetchFollowedUser(follower: number, following: number): Promise<boolean> {
    try {
        const followingResponse = await fetch(`https://api.exporhub.com:9000/api/follow/unique-follow?follower=${follower}&following=${following}`, {
            method: "GET",
            headers: {
                "Content-Type": "application/json",
                "Authorization": `Basic ${process.env.NEXT_PUBLIC_EXPORHUB_API_KEY}`
            }
        })

        if (!followingResponse.ok) {
            const error = await followingResponse.json() as ResponseStatus
            throw new Error(`\nCode: ${error.code}\nMessage: ${error.message}`)
        }

        const follow = await followingResponse.json()

        console.log(follow)

        return follow.code ? false : true
    } catch (error) {
        console.error(error)

        return false
    }
}
