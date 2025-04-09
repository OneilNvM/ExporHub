import { Project } from "~/types/types"

export default async function fetchUserProjects(userId: number | null) {
    try {
        const response = await fetch(`https://api.exporhub.com:9000/api/project/user-id?user_id=${userId}`)

        if (!response.ok) {
            throw new Error(`Failed to find projects`)
        }

        const json = await response.json() as Array<Project>

        return json
    } catch (error) {
        console.error()

        return null
    }
}
