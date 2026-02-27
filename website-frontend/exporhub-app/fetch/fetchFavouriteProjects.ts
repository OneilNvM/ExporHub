import { Favourite, Project, ResponseStatus } from "~/types/types"

export default async function fetchFavouriteProjects(userId: number): Promise<Project[] | null> {
    try {
        let projects: Array<Project> = []
        const favouritesResponse = await fetch(`https://api.exporhub.com:9000/api/favourite/user-id?user_id=${userId}`, {
            method: "GET",
            headers: {
                "Content-Type": "application/json",
                "Authorization": `Basic ${process.env.NEXT_PUBLIC_EXPORHUB_API_KEY}`
            }
        })

        if (!favouritesResponse.ok) {
            const error = await favouritesResponse.json() as ResponseStatus
            throw new Error(`\nCode: ${error.code}\nMessage: ${error.message}`)
        }

        const favouritesJSON = await favouritesResponse.json() as Array<Favourite>

        for (let i = 0; i < favouritesJSON.length; i++) {
            const projectsResponse = await fetch(`https://api.exporhub.com:9000/api/project/project-id?project_id=${favouritesJSON[i].project_id}`, {
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

            const projectJson = await projectsResponse.json() as Project

            projects.push(projectJson)
        }

        return projects
    } catch (error) {
        console.error()

        return null
    }
}
