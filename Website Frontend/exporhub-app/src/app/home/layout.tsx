import React from 'react'

export default function HomeLayout({children}: Readonly<{children: React.ReactNode}>) {
  return (
    <div className='flex bg-pink-950/10 w-screen h-screen'>
        {children}
    </div>
  )
}
