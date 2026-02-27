import { Project, ResponseStatus } from "~/types/types"

export async function fetchProjects(identifier: string | number): Promise<Project | Project[] | null> {
    try {
        if (typeof identifier === "string") {
            const response = await fetch(`https://api.exporhub.com:9000/api/project/project-name?name=${identifier}`, {
                method: "GET",
                headers: {
                    "Content-Type": "application/json",
                    "Authorization": `Basic ${process.env.NEXT_PUBLIC_EXPORHUB_API_KEY}`
                },
            })

            if (!response.ok) {
                const error = await response.json() as ResponseStatus
                throw new Error(`\nCode: ${error.code}\nMessage: ${error.message}`)
            }

            const project = await response.json() as Project

            return project
        } else {
            const response = await fetch(`https://api.exporhub.com:9000/api/project/user-id?user_id=${identifier}`, {
                method: "GET",
                headers: {
                    "Content-Type": "application/json",
                    "Authorization": `Basic ${process.env.NEXT_PUBLIC_EXPORHUB_API_KEY}`
                },
            })

            if (!response.ok) {
                const error = await response.json() as ResponseStatus
                throw new Error(`\nCode: ${error.code}\nMessage: ${error.message}`)
            }

            const projects = await response.json() as Project[]

            return projects
        }
    } catch (error) {
        console.error(error)

        return null
    }
}