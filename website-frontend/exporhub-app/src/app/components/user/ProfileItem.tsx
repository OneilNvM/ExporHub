'use client'

import Image from 'next/image'
import Link from 'next/link'
import { usePathname } from 'next/navigation'
<<<<<<< HEAD
import React from 'react'
import GlobeSVG from '~/public/logo_draft_3.svg'

export default function ProfileItem() {
    const pathname = usePathname()

    return (
        <div className='flex rounded-2xl p-4 w-full max-w-[52rem] items-center gap-4 border border-pink-200 dark:border-pink-900'>
            <Link href={'/profile'} className='self-start'>
=======
import React, { useEffect, useState } from 'react'
import GlobeSVG from '~/public/logo_draft_3.svg'
import { User } from '~/types/types'
import UnfollowButton from './UnfollowButton'

export default function ProfileItem({ user, followedUsers, profileUserId, sessionUserId, setFollowedUsers }: { user: User, followedUsers: User[] | null, profileUserId?: number, sessionUserId?: number, setFollowedUsers?: React.Dispatch<React.SetStateAction<User[] | null>> }) {
    const [numOfProjects, setNumOfProjects] = useState<number | null>(null)
    const pathname = usePathname()

    useEffect(() => {
        if (profileUserId) {
            const fetchData = async () => {
                fetch(`https://api.exporhub.com:9000/api/project/num-of-projects?user_id=${profileUserId}`)
                    .then(res => {
                        if (!res.ok) {
                            throw new Error(`Failed to find projects`)
                        }
    
                        return res.text()
                    })
                    .then(num => {
                        setNumOfProjects(Number(num))
                    })
                    .catch(error => {
                        console.error(error)
                    })
            }
    
            fetchData()
        } else {
            const fetchData = async () => {
                fetch(`https://api.exporhub.com:9000/api/project/num-of-projects?user_id=${sessionUserId}`)
                    .then(res => {
                        if (!res.ok) {
                            throw new Error(`Failed to find projects`)
                        }
    
                        return res.text()
                    })
                    .then(num => {
                        setNumOfProjects(Number(num))
                    })
                    .catch(error => {
                        console.error(error)
                    })
            }
    
            fetchData()
        }

    }, [profileUserId, sessionUserId])

    return (
        <div className='flex rounded-2xl p-4 w-full max-w-[52rem] items-center gap-4 border border-pink-200 dark:border-pink-900'>
            <Link href={`/profile/${user.username}`} className='self-start'>
>>>>>>> origin/main
                <Image src={GlobeSVG} className='rounded-full' width={56} alt='Profile Picture' />
            </Link>
            <div className='flex flex-col w-full'>
                <div className='flex justify-between py-2 border-b-[1px] border-b-pink-200 dark:border-b-pink-900'>
                    <div className='flex flex-col gap-2'>
<<<<<<< HEAD
                        <Link href={'/profile'} className='text-xl hover:underline'>OneilNvM</Link>
                        <p className='line-clamp-2 text-gray-500'>Lorem ipsum dolor, sit amet consectetur adipisicing elit. Vitae vero ea expedita at facere doloremque recusandae corporis, officiis saepe cumque nam quidem similique ipsa earum. At aliquam accusamus ratione esse!</p>
                    </div>
                    {
                        pathname == "/account" ? <UnfollowButton /> : null
                    }
                </div>
                <div className='flex py-2 justify-between'>
                    <p>12 projects</p>
                    <p>1000 Followers</p>
=======
                        <Link href={`/profile/${user.username}`} className='text-xl hover:underline'>{user.username}</Link>
                        <p className='line-clamp-2 text-gray-500'>{user.bio}</p>
                    </div>
                    {
                        pathname == "/account" ? profileUserId && setFollowedUsers ? <UnfollowButton followedUsers={followedUsers} setFollowedUsers={setFollowedUsers} profileUserId={profileUserId} user={user} /> : setFollowedUsers ? <UnfollowButton followedUsers={followedUsers} setFollowedUsers={setFollowedUsers} sessionUserId={sessionUserId} user={user} /> : null : null
                    }
                </div>
                <div className='flex py-2 justify-between'>
                    <p>{numOfProjects !== 1 ? numOfProjects === null ? "" : `${numOfProjects} projects` : `1 Project`}</p>
                    <p>{user.followers}</p>
>>>>>>> origin/main
                </div>
            </div>
        </div>
    )
<<<<<<< HEAD
}

const UnfollowButton = () => {
    return (
        <div className='border self-center rounded-md border-pink-300 dark:border-pink-900'>
            <button className='px-2 py-1'>
                Unfollow
            </button>
        </div>
    )
=======
>>>>>>> origin/main
}