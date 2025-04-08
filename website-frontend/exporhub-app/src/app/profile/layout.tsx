import { Metadata } from 'next'
import React from 'react'

export const metadata: Metadata = {
    title: 'Profile - ExporHub',
    description: 'Profile for a user on ExporHub.'
}

export default function ProfileLayout({ children }: Readonly<{ children: React.ReactNode }>) {
  return (
    <div className='size-full bg-pink-950/10'>
        {children}
    </div>
  )
}
