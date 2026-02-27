'use client'

import React, { Suspense, useEffect, useState } from 'react'
import ProfileItem from '../user/ProfileItem'
import { User } from '~/types/types'

export default function FollowingTab({ followings, sessionUserId }: { followings: User[] | null, sessionUserId?: number }) {
    const [followedUsers, setFollowedUsers] = useState<User[] | null>(followings)

    useEffect(() => {
        setFollowedUsers(followedUsers)
    }, [followedUsers])

    return (
        <div className='flex flex-col w-full items-center m-8 gap-12'>
            <Suspense>
                {followedUsers?.length === 0 ? <p>No Follows</p> : followedUsers?.map(user => {
                    return sessionUserId ? <ProfileItem followedUsers={followedUsers} setFollowedUsers={setFollowedUsers} sessionUserId={sessionUserId} key={user.user_id} user={user} /> : null
                })}
            </Suspense>
        </div>
    )
}
