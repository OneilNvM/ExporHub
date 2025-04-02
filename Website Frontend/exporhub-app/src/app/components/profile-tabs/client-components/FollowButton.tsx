'use client'

import { UserRound } from 'lucide-react'
import React from 'react'

export default function FollowButton() {
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
    
    return (
        <button onClick={handleClick} id='follow-btn' className='px-4 rounded-s-md before:block before:bg-pink-300 before:dark:bg-pink-950 before:absolute before:w-[0.05rem] before:h-6 before:left-14 before:top-0 transition-colors duration-300 ease-in-out'>
            <UserRound width={24} height={24} absoluteStrokeWidth={true} />
        </button>
    )
}
