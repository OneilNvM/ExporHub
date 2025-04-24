import { Favourite } from "~/types/types"

export default async function fetchFavourite(userId: number | null, projectId: number): Promise<Favourite | null> {
    try {
        const favRes = await fetch(`https://api.exporhub.com:9000/api/favourite/u-p-id?user_id=${userId}&project_id=${projectId}`)

        if (!favRes.ok) {
            throw new Error(`Failed to find favourite`)
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
