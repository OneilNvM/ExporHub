import { Project } from "~/types/types"

export default async function fetchProjects(): Promise<Project[] | null> {
    try {
        const projectsResponse = await fetch(`https://api.exporhub.com:9000/api/projects`)

        if (!projectsResponse.ok) {
            throw new Error(`Failed to find projects`)
        }
    
        const projects = await projectsResponse.json() as Array<Project>
    
        return projects
    } catch (error) {
        console.error(error)

        return null
    }
}
