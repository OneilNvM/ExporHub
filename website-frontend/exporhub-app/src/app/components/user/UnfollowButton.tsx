'use client'

import React from 'react'
import { User } from '~/types/types'

export default function UnfollowButton({ user, profileUserId, sessionUserId, followedUsers, setFollowedUsers }: { user: User, followedUsers: User[] | null, profileUserId?: number, sessionUserId?: number, setFollowedUsers: React.Dispatch<React.SetStateAction<User[] | null>> }) {
    const handleUnfollow = async () => {
        try {
            if (profileUserId) {
                const deleteFollow = await fetch(`https://api.exporhub.com:9000/api/follow/unfollow?follower=${profileUserId}&following=${user.user_id}`, {
                    method: "post"
                })
    
                if (!deleteFollow.ok) {
                    throw new Error(`Failed to delete follow`)
                }
    
                if (followedUsers) {
                    let index = followedUsers.indexOf(user)
    
                    followedUsers.splice(index, 1)
                }
    
    
                setFollowedUsers(followedUsers)
    
                console.log("Successful delete")
            } else {
                const deleteFollow = await fetch(`https://api.exporhub.com:9000/api/follow/unfollow?follower=${sessionUserId}&following=${user.user_id}`, {
                    method: "post"
                })
    
                if (!deleteFollow.ok) {
                    throw new Error(`Failed to delete follow`)
                }
    
                if (followedUsers) {
                    let index = followedUsers.indexOf(user)
    
                    followedUsers.splice(index, 1)
                }
    
    
                setFollowedUsers(followedUsers)
    
                console.log("Successful delete")
            }

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
