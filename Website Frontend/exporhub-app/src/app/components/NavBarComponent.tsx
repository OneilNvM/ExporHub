import React from 'react'

export default function NavBarComponent() {
  return (
    <nav className='flex flex-[0_0_12%] items-center justify-between px-16 bg-pink-950/15 border-b-[1px] border-b-pink-300'>
        <div>Logo</div>
        <input type="text" className='w-1/2' placeholder='search'/>
        <div>profile</div>
    </nav>
  )
}
