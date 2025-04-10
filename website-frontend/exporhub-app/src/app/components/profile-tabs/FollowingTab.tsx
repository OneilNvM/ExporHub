'use client'

import React, { useEffect, useState } from 'react'
import ProfileItem from '../user/ProfileItem'
import { User } from '~/types/types'

export default function FollowingTab({ followings, session }: { followings: User[] | null, session: {userId: number, expiresAt: Date} }) {
    const [followedUsers, setFollowedUsers] = useState<User[] | null>(followings)

    useEffect(() => {
        setFollowedUsers(followedUsers)
    }, [followings])

    return (
        <div className='flex flex-col w-full items-center m-8 gap-12'>
            {followings?.length === 0 ? <p>No Follows</p> : followings?.map((user, index) => {
                return <ProfileItem setFollowedUsers={setFollowedUsers} session={session} key={user.user_id} user={user}/>
            })}
        </div>
    )
}
