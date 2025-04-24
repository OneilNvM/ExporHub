<<<<<<< HEAD
import React from 'react'
import ProfileItem from '../user/ProfileItem'

export default function FollowingTab() {
    return (
        <div className='flex flex-col w-full items-center m-8 gap-12'>
            <ProfileItem />
            <ProfileItem />
            <ProfileItem />
            <ProfileItem />
            <ProfileItem />
            <ProfileItem />
=======
'use client'

import React, { useEffect, useState } from 'react'
import ProfileItem from '../user/ProfileItem'
import { User } from '~/types/types'

export default function FollowingTab({ followings, profileUserId, sessionUserId }: { followings: User[] | null, profileUserId?: number, sessionUserId?: number }) {
    const [followedUsers, setFollowedUsers] = useState<User[] | null>(followings)

    useEffect(() => {
        setFollowedUsers(followedUsers)
    }, [followedUsers])

    return (
        <div className='flex flex-col w-full items-center m-8 gap-12'>
            {followedUsers?.length === 0 ? <p>No Follows</p> : followedUsers?.map((user, index) => {
                return profileUserId ? <ProfileItem followedUsers={followedUsers} setFollowedUsers={setFollowedUsers} profileUserId={profileUserId} key={user.user_id} user={user} /> : sessionUserId ? <ProfileItem followedUsers={followedUsers} setFollowedUsers={setFollowedUsers} sessionUserId={sessionUserId} key={user.user_id} user={user} /> : null
            })}
>>>>>>> origin/main
        </div>
    )
}
