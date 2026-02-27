import { ResponseStatus, User } from '~/types/types'

export async function fetchUser(identifier: number | string): Promise<User | null> {
    try {
        if (typeof identifier === "string") {
            if (!identifier.includes("@")) {
                const response = await fetch(`https://api.exporhub.com:9000/api/user/username?username=${identifier}`, {
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

                const json = await response.json() as User

                return json
            } else {
                const response = await fetch(`https://api.exporhub.com:9000/api/user/email?email=${identifier}`, {
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

                const json = await response.json() as User

                return json
            }
        } else {
            const response = await fetch(`https://api.exporhub.com:9000/api/user/user-id?user_id=${identifier}`, {
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

            const json = await response.json() as User

            return json
        }
    } catch (error) {
        console.error()

        return null
    }
}
