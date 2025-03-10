'use client'

import Image from 'next/image'
import { usePathname } from 'next/navigation'
import React from 'react'
import GlobeSVG from '~/public/logo_draft_3.svg'

export default function ProfileItem() {
    const pathname = usePathname()

    return (
        <div className='flex rounded-2xl p-4 w-full max-w-[52rem] items-center gap-4 border border-pink-200 dark:border-pink-900'>
            <div className='self-start'>
                <Image src={GlobeSVG} className='rounded-full' width={56} alt='Profile Picture' />
            </div>
            <div className='flex flex-col w-full'>
                <div className='flex justify-between py-2 border-b-[1px] border-b-pink-200 dark:border-b-pink-900'>
                    <div className='flex flex-col gap-2'>
                        <p className='text-xl'>OneilNvM</p>
                        <p className='line-clamp-2 text-gray-500'>Lorem ipsum dolor, sit amet consectetur adipisicing elit. Vitae vero ea expedita at facere doloremque recusandae corporis, officiis saepe cumque nam quidem similique ipsa earum. At aliquam accusamus ratione esse!</p>
                    </div>
                    {
                        pathname == "/account" ? <UnfollowButton /> : null
                    }
                </div>
                <div className='flex py-2 justify-between'>
                    <p>12 projects</p>
                    <p>1000 Followers</p>
                </div>
            </div>
        </div>
    )
}

const UnfollowButton = () => {
    return (
        <div className='border self-center rounded-md border-pink-300 dark:border-pink-900'>
            <button className='px-2 py-1'>
                Unfollow
            </button>
        </div>
    )
}