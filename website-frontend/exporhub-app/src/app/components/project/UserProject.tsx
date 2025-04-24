'use client'

import Link from 'next/link'
import React from 'react'
import FavouriteButton from '../profile-tabs/client-components/FavouriteButton'
import { Project } from '~/types/types'

export default function UserProject({ sessionUserId, isFavourited, project }: { sessionUserId: number, isFavourited: boolean, project: Project }) {

    return (
        <div className='flex p-6 border-b-[1px] max-w-[48rem] w-full justify-between gap-4 border-b-pink-200 dark:border-b-pink-900'>
            <div className='flex flex-col gap-4'>
                <Link href={`/project/${project.name}`} className='text-3xl self-start'>{project.name}</Link>
                <p className='text-gray-400 line-clamp-2'>{project.description}</p>
            </div>
            <div className='flex flex-col justify-between items-center'>
                <FavouriteButton sessionUserId={sessionUserId} isFavourited={isFavourited} project={project} />
                <p className='text-xs'>{project.date_updated ? "Last Updated on " + new Date(project.date_updated).toDateString() : "Created on " + new Date(project.date_created).toDateString()}</p>
            </div>
        </div>
    )
}
