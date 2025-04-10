'use client'

import React from 'react'
import { User } from '~/types/types'

export default function UnfollowButton({ user, session, setFollowedUsers }: { user: User, session: {userId: number, expiresAt: Date}, setFollowedUsers: React.Dispatch<React.SetStateAction<User[] | null>> }) {
    const handleUnfollow = async () => {
        try {
            const deleteFollow = await fetch(`https://api.exporhub.com:9000/api/follow/unfollow?follower=${session.userId}&following=${user.user_id}`)

            if (!deleteFollow.ok) {
                throw new Error(`Failed to delete follow`)
            }

            console.log("Successful delete")
        } catch (error) {
            console.error(error)
        }
    }
    return (
        <div className='border self-center rounded-md border-pink-300 dark:border-pink-900'>
            <button onClick={handleUnfollow} className='px-2 py-1'>
                Unfollow
            </button>
        </div>
    )
}
