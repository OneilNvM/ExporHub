import React from 'react'
import NavBarComponent from '../components/general/NavBarComponent'
import FooterComponent from '../components/general/FooterComponent'
import LatestProjectUpdates from './components/LatestProjectUpdates'
import { ChevronRight } from 'lucide-react'
import ProjectItem from '../components/project/ProjectItem'

export default function Home() {
  return (
    <div className='grid auto-rows-auto size-full overflow-auto'>
      <NavBarComponent />
      <main className='grid grid-cols-1 lg:grid-cols-6 row-span-1'>
          <div className='col-span-2 hidden lg:block'>
            <LatestProjectUpdates />
          </div>
          <div className='flex justify-center lg:block w-full col-span-4'>
            <div className='flex flex-col gap-4 mt-24 lg:mt-28 lg:ml-36 lg:m-4'>
              <div className='flex gap-4'>
                <div className='text-xl'>Trending Projects</div>
                <div className='flex justify-center size-8 rounded-full bg-gray-950'>
                  <button>
                    <ChevronRight color='#fff' absoluteStrokeWidth={true} size={32}/>
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
          </div>
      </main>
      <FooterComponent />
    </div>
  )
}