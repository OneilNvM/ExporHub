'use client'

import Image from 'next/image'
import React, { useEffect } from 'react'
import LogoSVG from '~/public/logo_draft_3.svg'
import { Search } from 'lucide-react'
import Link from 'next/link'
import { usePathname, useSearchParams } from 'next/navigation'

export default function NavBarComponent() {
  const pathName = usePathname()
  const queryParams = useSearchParams()

  if (pathName == "/account") {
    useEffect(() => {
      const profileTab = document.getElementById('profile-tab');
      const projectsTab = document.getElementById('projects-tab');
      const favouritesTab = document.getElementById('favourites-tab');
      const followingTab = document.getElementById('following-tab');

      console.log(queryParams.get("tab"))

      switch (queryParams.get("tab")) {
        case 'projects':
          profileTab?.classList.add("inactive-tab")
          projectsTab?.classList.add("active-tab")
          favouritesTab?.classList.add("inactive-tab")
          followingTab?.classList.add("inactive-tab")

          profileTab?.classList.remove("active-tab")
          favouritesTab?.classList.remove("active-tab")
          followingTab?.classList.remove("active-tab")
          break
        case 'favourites':
          profileTab?.classList.add("inactive-tab")
          projectsTab?.classList.add("inactive-tab")
          favouritesTab?.classList.add("active-tab")
          followingTab?.classList.add("inactive-tab")

          profileTab?.classList.remove("active-tab")
          projectsTab?.classList.remove("active-tab")
          followingTab?.classList.remove("active-tab")
          break
        case 'following':
          profileTab?.classList.add("inactive-tab")
          projectsTab?.classList.add("inactive-tab")
          favouritesTab?.classList.add("inactive-tab")
          followingTab?.classList.add("active-tab")

          profileTab?.classList.remove("active-tab")
          projectsTab?.classList.remove("active-tab")
          favouritesTab?.classList.remove("active-tab")
          break
        default:
          profileTab?.classList.add("active-tab")
          projectsTab?.classList.add("inactive-tab")
          favouritesTab?.classList.add("inactive-tab")
          followingTab?.classList.add("inactive-tab")

          projectsTab?.classList.remove("active-tab")
          favouritesTab?.classList.remove("active-tab")
          followingTab?.classList.remove("active-tab")
          break
      }
    }, [queryParams])

    return (
      <nav className='flex flex-col justify-between h-[150px] border-b-[1px] border-b-pink-200 dark:border-b-pink-900'>
        <div className='flex items-center justify-between px-16 p-4'>
          <Link href={"/"}>
            <Image src={LogoSVG} width={36} className='rounded-full' alt='Logo Image' />
          </Link>
          <div className='flex items-center gap-4 w-1/2 rounded-full pl-6 p-2 bg-transparent border border-pink-200 dark:border-pink-900'>
            <button className='text-pink-200 dark:text-pink-950'>
              <Search size={24} absoluteStrokeWidth={true} />
            </button>
            <input type="text" className='w-full bg-transparent outline-none' placeholder='Explore and indulge' />
          </div>
          <button>
            <Image src={LogoSVG} className='rounded-full' width={56} alt='Profile Pic' />
          </button>
        </div>
        <div className='flex relative bottom-[1.8rem] text-lg'>
          <div id='profile-tab' className='absolute border-e border-b border-t rounded-e-full z-30 transition-colors duration-500 ease-in-out border-pink-300 bg-pink-200 dark:border-pink-800 dark:bg-pink-950'>
            <Link href={"/account"} className='px-14'>
              Profile
            </Link>
          </div>
          <div id='projects-tab' className='absolute left-32 border rounded-full z-20 transition-colors duration-500 ease-in-out border-pink-300 bg-pink-200 dark:border-pink-800 dark:bg-pink-950'>
            <Link href={"/account?tab=projects"} className='px-14'>
              Projects
            </Link>
          </div>
          <div id='favourites-tab' className='absolute left-[16.5rem] border rounded-full z-10 transition-colors duration-500 ease-in-out border-pink-300 bg-pink-200 dark:border-pink-800 dark:bg-pink-950'>
            <Link href={"/account?tab=favourites"} className='px-14'>
              Favourites
            </Link>
          </div>
          <div id='following-tab' className='absolute left-[26rem] border rounded-full z-0 transition-colors duration-500 ease-in-out border-pink-300 bg-pink-200 dark:border-pink-800 dark:bg-pink-950'>
            <Link href={"/account?tab=following"} className='px-14'>
              Following
            </Link>
          </div>
        </div>
      </nav>
    )
  } else {
    return (
      <nav className='flex h-[100px] items-center justify-between px-16 border-b-[1px] border-b-pink-200 dark:border-b-pink-900'>
        <Link href={"/"}>
          <Image src={LogoSVG} width={36} className='rounded-full' alt='Logo Image' />
        </Link>
        <div className='flex items-center gap-4 w-1/2 rounded-full pl-6 p-2 bg-transparent border border-pink-200 dark:border-pink-900'>
          <button className='text-pink-200 dark:text-pink-950'>
            <Search size={24} absoluteStrokeWidth={true} />
          </button>
          <input type="text" className='w-full bg-transparent outline-none' placeholder='Explore and indulge' />
        </div>
        <button>
          <Image src={LogoSVG} className='rounded-full' width={56} alt='Profile Pic' />
        </button>
      </nav>
    )
  }
}
