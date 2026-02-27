import { ResponseStatus } from "~/types/types"

export async function fetchProjectCount(user_id: number): Promise<number | null> {
    try {
        const response = await fetch(`https://api.exporhub.com:9000/api/project/num-of-projects?user_id=${user_id}`, {
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

        const projectCount = await response.text()

        return Number(projectCount)
    } catch (error) {
        console.error(error)

        return null
    }
}