import React from 'react'
import ProjectItem from '../project/ProjectItem'
import { Project, User } from '~/types/types'
import { fetchUser } from '~/fetch/fetchUser'
import fetchFavourite from '~/fetch/fetchFavourite'

export default async function FavouritesTab({ user, favouriteProjects, sessionUserId }: { favouriteProjects: Project[] | null, sessionUserId: number, user?: User | null }) {
    const favouritedProjectsArr: boolean[] = []
    if (user && favouriteProjects) {
        for (const project of favouriteProjects) {
            const favouriteObj = await fetchFavourite(sessionUserId, project.project_id)

            favouriteObj && favouritedProjectsArr.push(true)
        }
    }
    return (
        <div className='flex flex-col gap-8 m-4 w-full items-center'>
            {favouriteProjects?.length === 0 ? <p>No favourites</p> : favouriteProjects?.map(async (project, index) => {
                const user = await fetchUser(project.user_id)
                return <ProjectItem sessionUserId={sessionUserId} user={user} isFavourited={favouritedProjectsArr ? favouritedProjectsArr[index] : true} project={project} key={project.project_id} />
            })}
        </div>
    )
}
