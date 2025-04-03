'use client'

import Link from 'next/link'
import React from 'react'
import FavouriteButton from './FavouriteButton'

export default function UserProject() {

    return (
        <div className='flex p-6 border-b-[1px] max-w-[48rem] w-full justify-between gap-4 border-b-pink-200 dark:border-b-pink-900'>
            <div className='flex flex-col gap-4'>
                <Link href={"/project"} className='text-3xl self-start'>Project Name</Link>
                <p className='text-gray-400 line-clamp-2'>Lorem, ipsum dolor sit amet consectetur adipisicing elit. Consequatur repellendus aut corrupti aspernatur obcaecati nesciunt dolorum doloribus earum unde officiis sequi et cumque provident, nulla quaerat ea est excepturi similique!</p>
            </div>
            <div className='flex flex-col justify-between items-center'>
                <FavouriteButton />
                <p className='text-xs'>Last Updated on 10th March 2025</p>
            </div>
        </div>
    )
}
