import React from 'react'

export default function AboutUsLayout({ children }: Readonly<{ children: React.ReactNode }>) {
    return (
        <div className='size-full bg-gradient-to-b from-pink-200 to-pink-400 dark:from-pink-950/30 dark:to-pink-700/50'>
            {children}
        </div>
    )
}
