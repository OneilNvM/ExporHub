export default async function fetchFollowedUser(follower: number, following: number) {
    try {
        const followingResponse = await fetch(`https://api.exporhub.com:9000/api/follow/unique-follow?follower=${follower}&following=${following}`)

        if (!followingResponse.ok) {
            throw new Error(`Failed to find followings ${followingResponse.status} ${followingResponse.statusText}`)
        }

        const follow = await followingResponse.json()

        console.log(follow)

        return follow.code ? false : true
    } catch (error) {
        console.error(error)

        return false
    }
}
