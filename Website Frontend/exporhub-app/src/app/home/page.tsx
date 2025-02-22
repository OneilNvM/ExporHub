import React from 'react'
import NavBarComponent from '../components/NavBarComponent'
import FooterComponent from '../components/FooterComponent'
import LatestProjectUpdates from './components/LatestProjectUpdates'
import Image from 'next/image'
import ArrowSVG from '../../../public/right-arrow-white.svg'

export default function Home() {
  return (
    <div className='flex flex-col justify-between flex-1 overflow-auto gap-2'>
      <NavBarComponent />
      <section className='flex flex-1 gap-12'>
        <div className='flex-[2_1_0%] hidden lg:block'>
          <LatestProjectUpdates />
        </div>
        <main className='flex-[3_1_0%]'>
          <div className='mt-48 ml-12 m-4'>
            <div className='flex items-center gap-8'>
              <div className='text-xl'>Trending Projects</div>
              <div className='flex justify-center size-8 rounded-full bg-black'>
                <button>
                <Image src={ArrowSVG} title='Filter options' width={24} alt='Arrow image'/>
                </button>
              </div>
            </div>
            <div>

            </div>
          </div>
        </main>
      </section>
      <FooterComponent />
    </div>
  )
}
