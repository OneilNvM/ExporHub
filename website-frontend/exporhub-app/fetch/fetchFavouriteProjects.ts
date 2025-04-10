import { Favourite, Project } from "~/types/types"

export default async function fetchFavouriteProjects(userId: number): Promise<Project[] | null> {
    try {
        let projects: Array<Project> = []
        const favouritesResponse = await fetch(`https://api.exporhub.com:9000/api/favourite/user-id?user_id=${userId}`)

        if (!favouritesResponse.ok) {
            throw new Error(`Failed to find favourites`)
        }

        const favouritesJSON = await favouritesResponse.json() as Array<Favourite>

        for (let i = 0; i < favouritesJSON.length; i++) {
            const projectsResponse = await fetch(`https://api.exporhub.com:9000/api/project/project-id?project_id=${favouritesJSON[i].project_id}`)

            if (!projectsResponse.ok) {
                throw new Error(`Failed to find project`)
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
