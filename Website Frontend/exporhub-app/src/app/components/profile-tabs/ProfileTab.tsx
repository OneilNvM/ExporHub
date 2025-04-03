'use client'

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

    return (
        <>
            <section className='flex flex-1 w-full flex-col gap-4 my-4 md:ml-10 self-start'>
                <div className='flex flex-col gap-2 self-center md:self-start'>
                    <Image className='rounded-full' src={LogoSVG} width={256} alt='Profile Picture' />
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
                    </div>
                </div>
            </section >
            <section className='flex flex-[2_2_0%] w-full flex-col self-start gap-28 mx-8 md:mx-0 mt-12'>
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
                </div>
            </section>
        </>
    )
}
