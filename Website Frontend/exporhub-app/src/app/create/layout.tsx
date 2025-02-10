import { Metadata } from 'next'
import React from 'react'

export const metadata: Metadata = {
    title: 'Create Account',
    description: 'Page for creating an account.',
}

export default function CreateAccountLayout({ children }: Readonly<{ children: React.ReactNode }>) {
    return (
        <div className={`size-full flex`}>
            {children}
        </div>
    )
}
