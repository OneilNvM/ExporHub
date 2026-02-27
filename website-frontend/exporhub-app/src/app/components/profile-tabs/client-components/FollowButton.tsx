'use client'

import { UserRound } from 'lucide-react'
import React, { useEffect } from 'react'

export default function FollowButton({ isFollowed, sessionUserId, profileUserId }: { isFollowed: boolean, sessionUserId: number, profileUserId: number }) {

    useEffect(() => {
        const followBtn = document.getElementById('follow-btn')

        if (followBtn) {
            if (isFollowed) {
                followBtn.classList.add("followed")
                followBtn.classList.remove("unfollowed")
            } else {
                followBtn.classList.add("unfollowed")
                followBtn.classList.remove("followed")
            }
        }
    }, [isFollowed])

    const handleFollow = async () => {
        const followBtn = document.getElementById('follow-btn')

        try {
            if (!isFollowed) {
                const newFollow = await fetch(`https://api.exporhub.com:9000/api/follow/new?follower=${sessionUserId}&following=${profileUserId}`, {
                    method: 'post',
                    headers: {
                        "Content-Type": "application/json",
                        "Authorization": `Basic ${process.env.NEXT_PUBLIC_EXPORHUB_API_KEY}`
                    }
                })

                if (!newFollow.ok) {
                    throw new Error(`Failed to create follow`)
                }

                console.log("Successful follow")

                if (followBtn) {
                    followBtn.classList.add("followed")
                    followBtn.classList.remove("unfollowed")
                }
            } else {
                const deleteFollow = await fetch(`https://api.exporhub.com:9000/api/follow/unfollow?follower=${sessionUserId}&following=${profileUserId}`, {
                    method: 'post',
                    headers: {
                        "Content-Type": "application/json",
                        "Authorization": `Basic ${process.env.NEXT_PUBLIC_EXPORHUB_API_KEY}`
                    }
                })

                if (!deleteFollow.ok) {
                    throw new Error(`Failed to delete follow`)
                }

                console.log("Successful unfollow")

                if (followBtn) {
                    followBtn.classList.add("unfollowed")
                    followBtn.classList.remove("followed")
                }
            }
        } catch (error) {
            console.error(error)
        }
    }

    return (
        <button onClick={handleFollow} id='follow-btn' className='px-4 rounded-s-md before:block before:bg-pink-300 before:dark:bg-pink-950 before:absolute before:w-[0.05rem] before:h-6 before:left-14 before:top-0 transition-colors duration-300 ease-in-out'>
            <UserRound width={24} height={24} absoluteStrokeWidth={true} />
        </button>
    )
}
