import { Metadata } from 'next'
import React from 'react'

export const metadata: Metadata = {
    title: "Search - ExporHub",
    description: "Search page for viewing search results on ExporHub"
}

export default function SearchLayout({ children }: Readonly<{ children: React.ReactNode }>) {
  return (
    <div className='size-full bg-pink-950/10'>
        {children}
    </div>
  )
}
