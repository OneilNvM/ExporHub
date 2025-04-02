'use client'

import { usePathname } from 'next/navigation'
import React from 'react'

export default function ConditionalButtons({ option }: { option: "username" | "picture" | "bio" }) {
    const pathname = usePathname()

    if (option === "username") {
        return (
            <>
                {
                    pathname == "/account" && <p className='text-gray-400'>Change username</p>
                }
            </>
        )
    } else if (option === "picture") {
        return (
            <>
                {
                    pathname == "/account" && <p className='text-center text-gray-400'>Change profile picture</p>
                }
            </>
        )
    } else if (option === "bio") {
        return (
            <>
                {
                    pathname == "/account" && <p className='text-gray-400'>Edit bio</p>
                }
            </>
        )
    }
}
