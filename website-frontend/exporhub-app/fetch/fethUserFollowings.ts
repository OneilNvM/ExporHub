import { Follow, ResponseStatus } from "~/types/types"

export async function fetchUserFollowings(user_id: number): Promise<Follow[] | null> {
    try {
        const response = await fetch(`https://api.exporhub.com:9000/api/follow/user-id?user_id=${user_id}`, {
            method: "GET",
            headers: {
                "Content-Type": "application/json",
                "Authorization": `Basic ${process.env.NEXT_PUBLIC_EXPORHUB_API_KEY}`
            }
        })

        if (!response.ok) {
            const error = await response.json() as ResponseStatus
            throw new Error(`\nCode: ${error.code}\nMessage: ${error.message}`)
        }

        const follows = await response.json() as Follow[]

        return follows

    } catch (error) {
        console.error(error)

        return null
    }
}