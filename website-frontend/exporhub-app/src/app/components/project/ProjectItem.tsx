import Image from 'next/image'
import React, { Suspense } from 'react'
import GlobeSVG from '~/public/globe.svg'
import FavouriteButton from '../profile-tabs/client-components/FavouriteButton'
import Link from 'next/link'
import { Project, User } from '~/types/types'

export default function ProjectItem({ sessionUserId, project, isFavourited, user }: { sessionUserId: number, project: Project, isFavourited: boolean, user: User | null }) {

    return (
        <div className='flex w-full rounded-2xl max-w-[52rem] border border-pink-200 dark:border-pink-900'>
            <div className='flex flex-col items-center gap-4 p-3'>
                <Link href={`/profile/${user?.username}`} className='rounded-full self-center'>
                    <Image src={GlobeSVG} width={48} alt='Test Image' />
                </Link>
                <div className='flex flex-col items-center'>
                    <p className='text-sm'>Followers</p>
                    <p className='text-sm'>{user ? user.followers : null}</p>
                </div>
            </div>
            <div className='flex flex-col w-full py-4 justify-center'>
                <div className='flex self-stretch px-2 flex-col items-center gap-4 justify-between'>
                    <div className='flex flex-col items-center px-4'>
                        <Link href={`/project/${project.name}`} className='text-xl'>{project.name}</Link>
                        <div className='text-gray-400 line-clamp-2'>
                            <span>{project.description}</span>
                        </div>
                    </div>
                    <div className='flex self-stretch justify-between items-center px-4'>
                        <Suspense>
                            <FavouriteButton sessionUserId={sessionUserId} project={project} isFavourited={isFavourited} />
                        </Suspense>
                        <p className='text-gray-700 text-sm'>{project.date_updated ? "Last Updated on " + new Date(project.date_updated).toDateString() : "Created on " + new Date(project.date_created).toDateString()}</p>
                    </div>
                </div>
            </div>
        </div>
    )
}
