'use client'

import Link from 'next/link'
import React from 'react'
<<<<<<< HEAD
import FavouriteButton from './FavouriteButton'

export default function UserProject() {
=======
import FavouriteButton from '../profile-tabs/client-components/FavouriteButton'
import { Project } from '~/types/types'

export default function UserProject({ sessionUserId, isFavourited, project }: { sessionUserId: number, isFavourited: boolean, project: Project }) {
>>>>>>> origin/main

    return (
        <div className='flex p-6 border-b-[1px] max-w-[48rem] w-full justify-between gap-4 border-b-pink-200 dark:border-b-pink-900'>
            <div className='flex flex-col gap-4'>
<<<<<<< HEAD
                <Link href={"/project"} className='text-3xl self-start'>Project Name</Link>
                <p className='text-gray-400 line-clamp-2'>Lorem, ipsum dolor sit amet consectetur adipisicing elit. Consequatur repellendus aut corrupti aspernatur obcaecati nesciunt dolorum doloribus earum unde officiis sequi et cumque provident, nulla quaerat ea est excepturi similique!</p>
            </div>
            <div className='flex flex-col justify-between items-center'>
                <FavouriteButton />
                <p className='text-xs'>Last Updated on 10th March 2025</p>
=======
                <Link href={`/project/${project.name}`} className='text-3xl self-start'>{project.name}</Link>
                <p className='text-gray-400 line-clamp-2'>{project.description}</p>
            </div>
            <div className='flex flex-col justify-between items-center'>
                <FavouriteButton sessionUserId={sessionUserId} isFavourited={isFavourited} project={project} />
                <p className='text-xs'>{project.date_updated ? "Last Updated on " + new Date(project.date_updated).toDateString() : "Created on " + new Date(project.date_created).toDateString()}</p>
>>>>>>> origin/main
            </div>
        </div>
    )
}
