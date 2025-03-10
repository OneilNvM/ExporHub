import { Star } from 'lucide-react'
import Image from 'next/image'
import React from 'react'
import GlobeSVG from '~/public/globe.svg'

export default function ProjectItem() {
    return (
        <div className='flex w-full rounded-2xl max-w-[48rem] border border-pink-200 dark:border-pink-900'>
            <div className='flex flex-col items-center gap-4 p-3'>
                <div className='rounded-full self-center'>
                    <Image src={GlobeSVG} width={48} alt='Test Image' />
                </div>
                <div className='flex flex-col items-center'>
                    <p className='text-sm'>Followers</p>
                    <p className='text-sm'>1000</p>
                </div>
            </div>
            <div className='flex flex-col w-full py-4 justify-center'>
                <div className='flex self-stretch px-2 flex-col items-center gap-4 justify-between'>
                    <div className='flex flex-col items-center px-4'>
                        <p className='text-xl'>Project Name</p>
                        <div className='text-gray-400 line-clamp-2'>
                            <span>Lorem ipsum, dolor sit amet consectetur adipisicing elit. Odit consectetur molestiae iusto quos, repudiandae sunt minima ut possimus sit aspernatur et iure nemo expedita cum, temporibus odio aut omnis libero!</span>
                        </div>
                    </div>
                    <div className='flex self-stretch justify-between items-center px-4'>
                        <div className='border rounded-md border-pink-300 dark:border-pink-950'>
                            <button className='flex px-4 items-center gap-4 text-pink-400 dark:text-pink-950'>
                                <Star size={16} absoluteStrokeWidth={true}/>
                                <span>Favourite</span>
                            </button>
                        </div>
                        <p className='text-gray-700 text-sm'>Last Updated</p>
                    </div>
                </div>
            </div>
        </div>
    )
}
