import { Project, ResponseStatus } from "~/types/types"

export default async function fetchAllProjects(): Promise<Project[] | null> {
    try {
        const projectsResponse = await fetch(`https://api.exporhub.com:9000/api/projects`, {
            method: "GET",
            headers: {
                "Content-Type": "application/json",
                "Authorization": `Basic ${process.env.NEXT_PUBLIC_EXPORHUB_API_KEY}`
            }
        })

        if (!projectsResponse.ok) {
            const error = await projectsResponse.json() as ResponseStatus
            throw new Error(`\nCode: ${error.code}\nMessage: ${error.message}`)
        }

        const projects = await projectsResponse.json() as Array<Project>

        return projects
    } catch (error) {
        console.error(error)

        return null
    }
}
