import { Favourite, ResponseStatus } from "~/types/types"

export default async function fetchFavourite(userId: number | null, projectId: number): Promise<Favourite | null> {
    try {
        const favRes = await fetch(`https://api.exporhub.com:9000/api/favourite/u-p-id?user_id=${userId}&project_id=${projectId}`, {
            method: "GET",
            headers: {
                "Content-Type": "application/json",
                "Authorization": `Basic ${process.env.NEXT_PUBLIC_EXPORHUB_API_KEY}`
            }
        })

        if (!favRes.ok) {
            const error = await favRes.json() as ResponseStatus
            throw new Error(`\nCode: ${error.code}\nMessage: ${error.message}`)
        }

        const json = await favRes.json()
        if (json.code) {
            return null
        } else {
            return json as Favourite
        }
    } catch (error) {
        console.error(error)

        return null
    }

}
