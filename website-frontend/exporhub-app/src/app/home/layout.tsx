import { Metadata } from 'next'
import React from 'react'

export const metadata: Metadata = {
  title: 'Home - ExporHub',
  description: 'Home page for ExporHub.'
}

export default function HomeLayout({children}: Readonly<{children: React.ReactNode}>) {
  return (
    <div className='bg-pink-950/10 size-full'>
        {children}
    </div>
  )
}
