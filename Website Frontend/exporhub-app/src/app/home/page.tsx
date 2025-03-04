import React from 'react'
import NavBarComponent from '../components/NavBarComponent'
import FooterComponent from '../components/FooterComponent'
import LatestProjectUpdates from './components/LatestProjectUpdates'
import Image from 'next/image'
import ArrowSVG from '../../../public/right-arrow-white.svg'
import GlobeSVG from '../../../public/globe.svg'

export default function Home() {
  return (
    <div className='grid auto-rows-auto size-full overflow-x-hidden'>
      <NavBarComponent />
      <section className='grid lg:grid-cols-6 row-span-1'>
        <div className='flex col-span-2 hidden lg:block'>
          <LatestProjectUpdates />
        </div>
        <main className='flex justify-center lg:block w-full col-span-4'>
          <div className='flex flex-col gap-4 mt-24 lg:mt-28 lg:ml-36 lg:m-4'>
            <div className='flex gap-4'>
              <div className='text-xl'>Trending Projects</div>
              <div className='flex justify-center size-8 rounded-full bg-black'>
                <button>
                  <Image src={ArrowSVG} title='Filter projects' width={24} alt='Arrow image' />
                </button>
              </div>
            </div>
            <div className='flex w-full flex-col gap-8'>
              <ProjectItem />
              <ProjectItem />
              <ProjectItem />
              <ProjectItem />
              <ProjectItem />
            </div>
          </div>
        </main>
      </section>
      <FooterComponent />
    </div>
  )
}

const ProjectItem = () => {
  return (
    <div id='project-container' className='flex max-w-[48rem] bg-pink-400 rounded-2xl'>
      <div id='profile' className='flex  flex-col items-center gap-4 p-3'>
        <div className='rounded-full self-center'>
          <Image src={GlobeSVG} width={48} alt='Test Image' />
        </div>
        <div className='flex flex-col items-center'>
          <p className='text-sm'>Followers</p>
          <p className='text-sm'>1000</p>
        </div>
      </div>
      <div id="meta-data" className='flex flex-col w-full justify-center'>
        <div className='flex self-stretch px-2 flex-col items-center gap-4 justify-between'>
          <div className='flex flex-col items-center px-4'>
            <p className='text-xl'>Project Name</p>
            <div className='max-w-[24rem] text-gray-600 truncate'>
              <span className=''>Lorem ipsum, dolor sit amet consectetur adipisicing elit. Odit consectetur molestiae iusto quos, repudiandae sunt minima ut possimus sit aspernatur et iure nemo expedita cum, temporibus odio aut omnis libero!</span>
            </div>
          </div>
          <div className='flex self-stretch justify-between items-center px-4'>
            <p>Favourite</p>
            <p>Last Updated</p>
          </div>
        </div>
      </div>
    </div>
  )
}