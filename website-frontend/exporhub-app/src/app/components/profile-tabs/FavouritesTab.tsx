import React from 'react'
import ProjectItem from '../project/ProjectItem'
import { Project } from '~/types/types'

export default function FavouritesTab({ favouriteProjects, sessionUserId, favouritedProjectsArr }: { favouriteProjects: Project[] | null, sessionUserId: number, favouritedProjectsArr?: boolean[], }) {
    return (
        <div className='flex flex-col gap-8 m-4 w-full items-center'>
            {favouriteProjects?.length === 0 ? <p>No favourites</p> : favouriteProjects?.map((project, index) => {
                return <ProjectItem sessionUserId={sessionUserId} isFavourited={favouritedProjectsArr ? favouritedProjectsArr[index] : true} project={project} key={project.project_id} />
            })}
        </div>
    )
}
