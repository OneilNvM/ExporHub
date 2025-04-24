import { Metadata } from 'next'
import React from 'react'

export const metadata: Metadata = {
    title: 'Create Account - ExporHub',
    description: 'Page for creating an account.',
}

export default function CreateAccountLayout({ children }: Readonly<{ children: React.ReactNode }>) {
    return (
        <div className={`bg-gradient-to-b from-transparent via-transparent to-pink-900/20 size-full flex`}>
            {children}
        </div>
    )
}
