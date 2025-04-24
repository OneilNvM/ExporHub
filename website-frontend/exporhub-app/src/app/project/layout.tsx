import React from 'react'

export default function ProjectLayout({ children }: Readonly<{ children: React.ReactNode }>) {
  return (
    <div className='size-full bg-pink-950/10'>
        {children}
    </div>
  )
}
