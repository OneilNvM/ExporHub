<<<<<<< HEAD
import Image from 'next/image'
import React from 'react'
import GlobeSVG from '~/public/globe.svg'
import FavouriteButton from './FavouriteButton'
import Link from 'next/link'

export default function ProjectItem() {
    return (
        <div className='flex w-full rounded-2xl max-w-[52rem] border border-pink-200 dark:border-pink-900'>
            <div className='flex flex-col items-center gap-4 p-3'>
                <Link href={'/profile'} className='rounded-full self-center'>
=======
'use client'

import Image from 'next/image'
import React, { useEffect, useState } from 'react'
import GlobeSVG from '~/public/globe.svg'
import FavouriteButton from '../profile-tabs/client-components/FavouriteButton'
import Link from 'next/link'
import { Project, User } from '~/types/types'

export default function ProjectItem({ sessionUserId, project, isFavourited }: { sessionUserId: number, project: Project, isFavourited: boolean }) {
    const [user, setUser] = useState<User | null>(null)

    useEffect(() => {
        fetch(`https://api.exporhub.com:9000/api/user/user-id?user_id=${project.user_id}`)
            .then(async res => {
                if (!res.ok) {
                    throw new Error(`Failed to find user`)
                }

                return await res.json() as User
            })
            .then(user => {
                setUser(user)
            })
    }, [project])

    return (
        <div className='flex w-full rounded-2xl max-w-[52rem] border border-pink-200 dark:border-pink-900'>
            <div className='flex flex-col items-center gap-4 p-3'>
                <Link href={`/profile/${user?.username}`} className='rounded-full self-center'>
>>>>>>> origin/main
                    <Image src={GlobeSVG} width={48} alt='Test Image' />
                </Link>
                <div className='flex flex-col items-center'>
                    <p className='text-sm'>Followers</p>
<<<<<<< HEAD
                    <p className='text-sm'>1000</p>
=======
                    <p className='text-sm'>{user ? user.followers : null}</p>
>>>>>>> origin/main
                </div>
            </div>
            <div className='flex flex-col w-full py-4 justify-center'>
                <div className='flex self-stretch px-2 flex-col items-center gap-4 justify-between'>
                    <div className='flex flex-col items-center px-4'>
<<<<<<< HEAD
                        <Link href={'/project'} className='text-xl'>Project Name</Link>
                        <div className='text-gray-400 line-clamp-2'>
                            <span>Lorem ipsum, dolor sit amet consectetur adipisicing elit. Odit consectetur molestiae iusto quos, repudiandae sunt minima ut possimus sit aspernatur et iure nemo expedita cum, temporibus odio aut omnis libero!</span>
                        </div>
                    </div>
                    <div className='flex self-stretch justify-between items-center px-4'>
                        <FavouriteButton />
                        <p className='text-gray-700 text-sm'>Last Updated</p>
=======
                        <Link href={`/project/${project.name}`} className='text-xl'>{project.name}</Link>
                        <div className='text-gray-400 line-clamp-2'>
                            <span>{project.description}</span>
                        </div>
                    </div>
                    <div className='flex self-stretch justify-between items-center px-4'>
                        <FavouriteButton sessionUserId={sessionUserId} project={project} isFavourited={isFavourited} />
                        <p className='text-gray-700 text-sm'>{project.date_updated ? "Last Updated on " + new Date(project.date_updated).toDateString() : "Created on " + new Date(project.date_created).toDateString()}</p>
>>>>>>> origin/main
                    </div>
                </div>
            </div>
        </div>
    )
}
