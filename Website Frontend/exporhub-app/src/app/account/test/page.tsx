'use client'

import { usePathname } from 'next/navigation'
import React from 'react'

export default function Test() {
    const pathname = usePathname()

    return (
        <div>{pathname.split("/")[1]}</div>
    )
}
