'use client'

import Image from 'next/image'
import Link from 'next/link'
import React, { useEffect, useState } from 'react'
import GlobeSVG from '~/public/logo_draft_3.svg'
import { User } from '~/types/types'
import UnfollowButton from './UnfollowButton'
import { fetchProjectCount } from '~/fetch/fetchProjectCount'
import { fetchUserFollowings } from '~/fetch/fethUserFollowings'

export default function ProfileItem({ user, followedUsers, sessionUserId, setFollowedUsers }: { user: User, followedUsers: User[] | null, sessionUserId?: number, setFollowedUsers?: React.Dispatch<React.SetStateAction<User[] | null>> }) {
    const [numOfProjects, setNumOfProjects] = useState<{ count: number | null }[] | null>(null)
    const [mutuals, setMutuals] = useState<{ username: string, followed: boolean }[] | null>(null)

    useEffect(() => {
        const fetchData = async () => {
            const projectCountMap: { count: number | null }[] = [];

            const count = await fetchProjectCount(user.user_id)

            console.log(user)

            projectCountMap.push({ count: count })


            console.dir(projectCountMap)

            setNumOfProjects(projectCountMap)

        }

        fetchData()

    }, [user])

    useEffect(() => {
        const fetchData = async () => {
            const mutuals: { username: string, followed: boolean }[] = []
            if (followedUsers && sessionUserId) {
                for (const user of followedUsers) {
                    const follows = await fetchUserFollowings(sessionUserId)

                    follows?.forEach(follow => {
                        if (user.user_id === follow.following) {
                            mutuals.push({ username: user.username, followed: true })
                        } else {
                            mutuals.push({ username: user.username, followed: false })
                        }
                    })
                }
            }

            setMutuals(mutuals)
        }

        fetchData()

    }, [followedUsers, sessionUserId])

    return (
        <div className='flex rounded-2xl p-4 w-full max-w-[52rem] items-center gap-4 border border-pink-200 dark:border-pink-900'>
            <Link href={`/profile/${user.username}`} className='self-start'>
                <Image src={GlobeSVG} className='rounded-full' width={56} alt='Profile Picture' />
            </Link>
            <div className='flex flex-col w-full'>
                <div className='flex justify-between py-2 border-b-[1px] border-b-pink-200 dark:border-b-pink-900'>
                    <div className='flex flex-col gap-2'>
                        <Link href={`/profile/${user.username}`} className='text-xl hover:underline'>{user.username}</Link>
                        <p className='line-clamp-2 text-gray-500'>{user.bio}</p>
                    </div>
                    {
                        mutuals?.map((mutual, index) => {
                            return mutual.followed && mutual.username === user.username && setFollowedUsers ? <UnfollowButton key={index} followedUsers={followedUsers} setFollowedUsers={setFollowedUsers} sessionUserId={sessionUserId} user={user} /> : null
                        })
                    }
                </div>
                <div className='flex py-2 justify-between'>
                    {numOfProjects?.map((value, index) => {
                        return <p key={index}>{value.count !== null ? value.count !== 1 ? `${value.count} projects` : `${value.count} project` : ""}</p>
                    })}
                    <p>{user.followers} followers</p>
                </div>
            </div>
        </div>
    )
}