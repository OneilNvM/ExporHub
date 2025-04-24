<<<<<<< HEAD
import React from 'react'
import UserProject from '../project/UserProject'
import Link from 'next/link'

export default function ProjectsTab() {
    return (
        <div className='flex w-full'>
            <div className='flex flex-col items-center gap-16 w-full m-8'>
                <UserProject />
                <UserProject />
                <UserProject />
                <UserProject />
                <UserProject />
                <UserProject />
            </div>
            <div className='flex flex-col min-w-fit self-start rounded-lg mr-64 mt-6 transition-colors duration-300 ease-in-out bg-transparent text-pink-400 border-pink-400 border-2 hover:bg-pink-400 hover:text-white dark:text-pink-800 dark:hover:text-black dark:hover:bg-pink-800 dark:border-pink-800'>
                <Link href={'/account/create'} className='px-8 py-2'>
                    <span className=''>Create Project</span>
                </Link>
            </div>
=======
'use client'

import React from 'react'
import UserProject from '../project/UserProject'
import Link from 'next/link'
import { Project } from '~/types/types'

export default function ProjectsTab({ sessionUserId, favouritesArr, projects }: { sessionUserId: number, favouritesArr: boolean[], projects: Project[] | null }) {
    return (
        <div className='flex flex-col w-full items-center lg:flex-row '>
            <div className='flex flex-col min-w-fit self-center lg:self-start rounded-lg lg:ml-64 mt-6 transition-colors duration-300 ease-in-out bg-transparent text-pink-400 border-pink-400 border-2 hover:bg-pink-400 hover:text-white dark:text-pink-800 dark:hover:text-black dark:hover:bg-pink-800 dark:border-pink-800'>
                <Link href={'/account/create'} className='px-8 py-2'>
                    <span>Create Project</span>
                </Link>
            </div>
            <div className='flex flex-col items-center gap-16 w-full m-8'>
                {projects?.length === 0 ? <p>No Projects</p> : projects?.map((project, index) => {
                    return <UserProject key={project.project_id} sessionUserId={sessionUserId} isFavourited={favouritesArr[index]} project={project} />
                })}
            </div>
>>>>>>> origin/main
        </div>
    )
}
