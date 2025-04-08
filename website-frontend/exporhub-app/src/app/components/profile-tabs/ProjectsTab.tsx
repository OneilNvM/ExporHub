import React from 'react'
import UserProject from '../project/UserProject'

export default function ProjectsTab() {
    return (
        <div className='flex flex-col items-center gap-16 w-full m-8'>
            <UserProject />
            <UserProject />
            <UserProject />
            <UserProject />
            <UserProject />
            <UserProject />
        </div>
    )
}
