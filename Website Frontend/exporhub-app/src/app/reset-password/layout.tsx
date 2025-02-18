
import { Metadata } from 'next'
import React from 'react'

export const metadata: Metadata = {
    title: 'Reset Password',
    description: 'This page is used for resetting the password of a user.'
}

export default function ResetPasswordLayout({ children }: Readonly<{ children: React.ReactNode }>) {
    return (
        <div className='bg-gradient-to-b from-transparent via-transparent to-pink-900/20 flex size-full'>
            {children}
        </div>
    )
}
