import React from 'react'
import { Project } from '~/types/types'

export default function ProjectNotification({ project }: { project: Project }) {
    return (
        <>
            <div className='border px-8 py-4 rounded-2xl max-w-[75%] border-pink-300 dark:border-pink-900'>
                <p className='text-2xl'>{project.name}</p>
                <p className='text-xl'>Date Updated: {new Date(project.date_updated).toDateString()}</p>
            </div>
        </>
    )
}
