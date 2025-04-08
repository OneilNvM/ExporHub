import Image from 'next/image'
import React from 'react'
import LogoSVG from '~/public/logo_draft_3.svg'
import FollowButton from './client-components/FollowButton'
import ConditionalButtons from './client-components/ConditionalButtons'
import { User } from '~/types/types'

export default function ProfileTab({ user }: { user: User | null }) {
    const ProjectNotification = () => {
        return (
            <>
                <div className='border px-8 py-4 rounded-2xl max-w-[75%] border-pink-300 dark:border-pink-900'>
                    <p className='text-2xl'>Project name</p>
                    <p className='text-xl'>Date Updated/ Added</p>
                </div>
            </>
        )
    }

    return (
        <>
            <section className='flex flex-1 w-full flex-col gap-4 my-4 md:ml-10 self-start'>
                <div className='flex flex-col gap-2 self-center md:self-start'>
                    <Image className='rounded-full' src={LogoSVG} width={256} alt='Profile Picture' />
                    <ConditionalButtons option='picture' />
                </div>
                <div className='flex flex-col w-4/5 md:w-[256px] ml-4 gap-2 self-center md:self-start'>
                    <p>Bio</p>
                    <ConditionalButtons option='bio' />
                    <p>{user ? user.bio : null}</p>
                </div>
                <div className='ml-4 self-center md:self-start'>
                    <div className='flex relative rounded-md items-center border text-pink-300 border-pink-300 dark:text-pink-950 dark:border-pink-950'>
                        <FollowButton />
                        <span className='px-2 text-black dark:text-white'>{user ? user.followers === 1 ? `${user.followers} follower` : `${user.followers} followers` : null}</span>
                    </div>
                </div>
            </section >
            <section className='flex flex-[2_2_0%] w-full flex-col self-start gap-28 mx-8 md:mx-0 mt-12'>
                <div className='flex flex-col'>
                    <div className='flex flex-col'>
                        <p className='text-5xl font-semibold'>{user ? user.username : "No Username"}</p>
                    </div>
                    <ConditionalButtons option='username' />
                </div>
                <div className='flex flex-col gap-12 m-2'>
                    <p className='text-4xl'>Recent Project Activity</p>
                    <ProjectNotification />
                </div>
            </section>
        </>
    )
}
