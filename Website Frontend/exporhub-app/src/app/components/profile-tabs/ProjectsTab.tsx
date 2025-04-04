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
        </div>
    )
}
