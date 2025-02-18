import { Metadata } from 'next'
import React from 'react'

export const metadata: Metadata = {
    title: 'Forgot Password',
    description: 'This page is used for sending verification emails to user\'s that have forgotten their password.'
}

export default function ForgotPasswordLayout({ children }: Readonly<{ children: React.ReactNode }>) {
    return (
        <div className='bg-gradient-to-b from-transparent via-transparent to-pink-900/20 size-full flex'>
            {children}
        </div>
    )
}
