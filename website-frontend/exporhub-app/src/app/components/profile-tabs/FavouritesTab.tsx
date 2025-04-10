import React from 'react'
import ProjectItem from '../project/ProjectItem'
import { Project, User } from '~/types/types'

export default function FavouritesTab({ favouriteProjects, user }: { favouriteProjects: Project[] | null, user: User | null, }) {
    return (
        <div className='flex flex-col gap-8 m-4 w-full items-center'>
            {favouriteProjects?.length === 0 ? <p>No favourites</p> : favouriteProjects?.map((project, index) => {
                return <ProjectItem user={user} isFavourited={true} project={project} key={project.project_id}/>
            }) }
        </div>
    )
}
