import { User } from '~/types/types'

export async function fetchUser(userId: number): Promise<User | null> {
    try {
        const response = await fetch(`https://api.exporhub.com:9000/api/user/user-id?user_id=${userId}`)

        if (!response.ok) {
            throw new Error(`Failed to find account`)
        }

        const json = await response.json() as User

        return json
    } catch (error) {
        console.error()

        return null
    }
}
