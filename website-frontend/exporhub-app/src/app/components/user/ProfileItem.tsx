'use client'

import Image from 'next/image'
import Link from 'next/link'
import { usePathname } from 'next/navigation'
import React from 'react'
import GlobeSVG from '~/public/logo_draft_3.svg'
import { User } from '~/types/types'
import UnfollowButton from './UnfollowButton'

export default function ProfileItem({ user, session, setFollowedUsers }: { user: User, session: {userId: number, expiresAt: Date}, setFollowedUsers: React.Dispatch<React.SetStateAction<User[] | null>> }) {
    const pathname = usePathname()

    return (
        <div className='flex rounded-2xl p-4 w-full max-w-[52rem] items-center gap-4 border border-pink-200 dark:border-pink-900'>
            <Link href={'/profile'} className='self-start'>
                <Image src={GlobeSVG} className='rounded-full' width={56} alt='Profile Picture' />
            </Link>
            <div className='flex flex-col w-full'>
                <div className='flex justify-between py-2 border-b-[1px] border-b-pink-200 dark:border-b-pink-900'>
                    <div className='flex flex-col gap-2'>
                        <Link href={'/profile'} className='text-xl hover:underline'>{user.username}</Link>
                        <p className='line-clamp-2 text-gray-500'>{user.bio}</p>
                    </div>
                    {
                        pathname == "/account" ? <UnfollowButton setFollowedUsers={setFollowedUsers} session={session} user={user} /> : null
                    }
                </div>
                <div className='flex py-2 justify-between'>
                    <p>12 projects</p>
                    <p>{user.followers}</p>
                </div>
            </div>
        </div>
    )
}