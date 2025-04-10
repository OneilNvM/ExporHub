import { Project } from "~/types/types"

export default async function fetchUpdatedProjects(userId: number): Promise<Project[] | null> {
    try {
        const response = await fetch(`https://api.exporhub.com:9000/api/project/date-updated?user_id=${userId}`)

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
