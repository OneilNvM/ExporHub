import Image from 'next/image'
import React from 'react'
import LogoSVG from '../../../public/logo_draft_3.svg'

export default function NavBarComponent() {
  return (
    <nav className='flex h-[100px] items-center justify-between px-16 bg-pink-950/15 border-b-[1px] border-b-pink-300'>
        <div><Image src={LogoSVG} width={56} className='rounded-full' alt='Logo Image'/></div>
        <input type="text" className='w-1/2 rounded-full p-2 bg-transparent border border-pink-400' placeholder='search'/>
        <div>profile</div>
    </nav>
  )
}
