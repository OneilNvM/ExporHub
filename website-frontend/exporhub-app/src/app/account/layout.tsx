import { Metadata } from 'next'
import React from 'react'

export const metadata: Metadata = {
    title: 'Account - ExporHub',
    description: 'Account page for a user on ExporHub.'
}

export default function AccountLayout({ children }: Readonly<{ children: React.ReactNode }>) {
    return (
        <div className='size-full bg-pink-950/10'>{children}</div>
    )
}
