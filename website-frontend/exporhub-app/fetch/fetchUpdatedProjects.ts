import { Project, ResponseStatus } from "~/types/types"

export default async function fetchUpdatedProjects(userId: number): Promise<Project[] | null> {
    try {
        const response = await fetch(`https://api.exporhub.com:9000/api/project/date-updated?user_id=${userId}`, {
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

        const json = await response.json() as Array<Project>

        return json
    } catch (error) {
        console.error()

        return null
    }
}
