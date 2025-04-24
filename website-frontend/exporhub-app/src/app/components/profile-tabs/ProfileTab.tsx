'use client'

<<<<<<< HEAD
import { UserRound } from 'lucide-react'
import Image from 'next/image'
import { usePathname } from 'next/navigation'
import React from 'react'
import LogoSVG from '~/public/logo_draft_3.svg'

export default function ProfileTab() {
    const pathname = usePathname()

    const handleClick = () => {
        const followBtn = document.getElementById('follow-btn')

        if (followBtn) {
            if (followBtn.classList.contains("followed")) {
                followBtn.classList.add("unfollowed")
                followBtn.classList.remove("followed")
            } else {
                followBtn.classList.add("followed")
                followBtn.classList.remove("unfollowed")
            }
        }
    }

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
=======
import Image from 'next/image'
import React from 'react'
import LogoSVG from '~/public/logo_draft_3.svg'
import FollowButton from './client-components/FollowButton'
import ConditionalButtons from './client-components/ConditionalButtons'
import { Project, User } from '~/types/types'
import ProjectNotification from './client-components/ProjectNotification'
import { usePathname } from 'next/navigation'

export default function ProfileTab({ user, projectNotifications, profileUserId, sessionUserId, isFollowed }: { user: User | null, projectNotifications: Project[] | null, sessionUserId?: number, profileUserId?: number, isFollowed?: boolean }) {
    const pathname = usePathname();
>>>>>>> origin/main

    return (
        <>
            <section className='flex flex-1 w-full flex-col gap-4 my-4 md:ml-10 self-start'>
                <div className='flex flex-col gap-2 self-center md:self-start'>
                    <Image className='rounded-full' src={LogoSVG} width={256} alt='Profile Picture' />
<<<<<<< HEAD
                    {
                        pathname == "/account" && <p className='text-center text-gray-400'>Change profile picture</p>
                    }
                </div>
                <div className='flex flex-col w-4/5 md:w-[256px] ml-4 gap-2 self-center md:self-start'>
                    <p>Bio</p>
                    {
                        pathname == "/account" && <p className='text-gray-400'>Edit bio</p>
                    }
                    <p>Lorem ipsum dolor sit amet consectetur, adipisicing elit. Adipisci magnam quaerat aliquid ea labore nam facere eaque obcaecati sed nesciunt dolorum molestiae dolor expedita repudiandae perferendis porro, placeat ullam aspernatur?</p>
                </div>
                <div className='ml-4 self-center md:self-start'>
                    <div className='flex relative rounded-md items-center border text-pink-300 border-pink-300 dark:text-pink-950 dark:border-pink-950'>
                        <button onClick={handleClick} id='follow-btn' className='px-4 rounded-s-md before:block before:bg-pink-300 before:dark:bg-pink-950 before:absolute before:w-[0.05rem] before:h-6 before:left-14 before:top-0 transition-colors duration-300 ease-in-out'>
                            <UserRound width={24} height={24} absoluteStrokeWidth={true} />
                        </button>
                        <span className='px-2 text-black dark:text-white'>1000 followers</span>
=======
                    <ConditionalButtons option='picture' />
                </div>
                <div className='flex flex-col w-4/5 md:w-[256px] ml-4 gap-2 self-center md:self-start'>
                    <p>Bio</p>
                    <ConditionalButtons option='bio' />
                    <p>{user ? user.bio : null}</p>
                </div>
                <div className='ml-4 self-center md:self-start'>
                    <div className='flex relative rounded-md items-center border text-pink-300 border-pink-300 dark:text-pink-950 dark:border-pink-950'>
                        {
                            pathname.includes("/profile") && profileUserId && sessionUserId ?
                                user?.user_id !== sessionUserId ?
                                    isFollowed ?
                                        <FollowButton isFollowed={isFollowed} profileUserId={profileUserId} sessionUserId={sessionUserId} /> :
                                        <FollowButton isFollowed={false} profileUserId={profileUserId} sessionUserId={sessionUserId} /> :
                                    null :
                                null
                        }
                        <span className='px-2 text-black dark:text-white'>{user ? user.followers === 1 ? `${user.followers} follower` : `${user.followers} followers` : null}</span>
>>>>>>> origin/main
                    </div>
                </div>
            </section >
            <section className='flex flex-[2_2_0%] w-full flex-col self-start gap-28 mx-8 md:mx-0 mt-12'>
<<<<<<< HEAD
                <div className='flex flex-col '>
                    <p className='text-5xl font-semibold'>OneilNvM</p>
                    {
                        pathname == "/account" && <p className='text-gray-400'>Change username</p>
                    }
                </div>
                <div className='flex flex-col gap-12 m-2'>
                    <p className='text-4xl'>Recent Project Activity</p>
                    <ProjectNotification />
                    <ProjectNotification />
                    <ProjectNotification />
                    <ProjectNotification />
                    <ProjectNotification />
=======
                <div className='flex flex-col'>
                    <div className='flex flex-col'>
                        <p className='text-5xl font-semibold'>{user ? user.username : "No Username"}</p>
                    </div>
                    <ConditionalButtons option='username' />
                </div>
                <div className='flex flex-col gap-12 m-2'>
                    <p className='text-4xl'>Recent Project Activity</p>
                    {
                        projectNotifications?.length !== 0 ? projectNotifications?.map((project, index) => {
                            return <ProjectNotification key={index} project={project} />
                        }) : <p>No Projects</p>
                    }
>>>>>>> origin/main
                </div>
            </section>
        </>
    )
}
