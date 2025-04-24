import { User } from '~/types/types'

export async function fetchUser(identifier: number | string): Promise<User | null> {
    try {
        if (typeof identifier === "string") {
            if (!identifier.includes("@")) {
                const response = await fetch(`https://api.exporhub.com:9000/api/user/username?username=${identifier}`)

                if (!response.ok) {
                    throw new Error(`Failed to find account`)
                }
    
                const json = await response.json() as User
    
                return json
            } else {
                const response = await fetch(`https://api.exporhub.com:9000/api/user/email?email=${identifier}`)

                if (!response.ok) {
                    throw new Error(`Failed to find account`)
                }
    
                const json = await response.json() as User
    
                return json
            }
        } else {
            const response = await fetch(`https://api.exporhub.com:9000/api/user/user-id?user_id=${identifier}`)

            if (!response.ok) {
                throw new Error(`Failed to find account`)
            }

            const json = await response.json() as User

            return json
        }
    } catch (error) {
        console.error()

        return null
    }
}
