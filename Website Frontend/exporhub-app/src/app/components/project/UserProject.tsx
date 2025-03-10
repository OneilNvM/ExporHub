import { Star } from 'lucide-react'
import Link from 'next/link'
import React from 'react'

export default function UserProject() {
  return (
    <div className='flex p-6 border-b-[1px] max-w-[48rem] w-full justify-between gap-4 border-b-pink-200 dark:border-b-pink-900'>
        <div className='flex flex-col gap-4'>
            <Link href={"/project"} className='text-3xl self-start'>Project Name</Link>
            <p className='text-gray-400 line-clamp-2'>Lorem, ipsum dolor sit amet consectetur adipisicing elit. Consequatur repellendus aut corrupti aspernatur obcaecati nesciunt dolorum doloribus earum unde officiis sequi et cumque provident, nulla quaerat ea est excepturi similique!</p>
        </div>
        <div className='flex flex-col justify-between items-center'>
            <div className='w-fit border rounded-md border-pink-300 dark:border-pink-950'>
                <button className='flex px-4  items-center gap-4 text-pink-400 dark:text-pink-950'>
                    <Star size={16} absoluteStrokeWidth={true}/>
                    <span>Favourite</span>
                </button>
            </div>
            <p className='text-xs'>Last Updated on 10th March 2025</p>
        </div>
    </div>
  )
}
