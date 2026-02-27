import { Image, ResponseStatus } from "~/types/types"

export async function fetchProjectImages(user_id: number, project_id: number): Promise<Image[] | null> {
    try {
        const response = await fetch(`https://api.exporhub.com:9000/api/image/project-images?user_id=${user_id}&project_id=${project_id}`, {
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

        const images = await response.json() as Image[]

        return images
    } catch (error) {
        console.error(error)

        return null
    }
}