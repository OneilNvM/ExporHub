import React from 'react'
import UserProject from '../project/UserProject'
import Link from 'next/link'
import { Project, User } from '~/types/types'
import fetchFavourite from '~/fetch/fetchFavourite'

export default async function ProjectsTab({ sessionUserId, projects, user }: { sessionUserId: number, projects: Project[] | null, user: User | null }) {
    const favouritesArr: boolean[] = []
    if (user && projects) {
        for (const project of projects) {
            const favouriteObj = await fetchFavourite(user.user_id, project.project_id)

            favouriteObj && favouritesArr.push(true)
        }
    }

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
        </div>
    )
}
