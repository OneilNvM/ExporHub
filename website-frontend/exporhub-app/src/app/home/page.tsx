import React from 'react'
import NavBarComponent from '../components/general/NavBarComponent'
import FooterComponent from '../components/general/FooterComponent'
import LatestProjectUpdates from './components/LatestProjectUpdates'
import { ChevronRight } from 'lucide-react'
import ProjectItem from '../components/project/ProjectItem'
import { cookies } from 'next/headers'
import { decrypt } from '../lib/session'
import fetchProjects from '~/fetch/fetchProjects'
import fetchFavourite from '~/fetch/fetchFavourite'

export default async function Home() {
  try {
    const cookieStore = await cookies()
    const session = cookieStore.get('session')
    const result = await decrypt(session?.value)

    if (result) {
      const projects = await fetchProjects()

      let favouritesArr = []

      if (projects) {
        for (const project of projects) {
          const favouriteObj = await fetchFavourite(result.userId, project.project_id)

          favouriteObj && favouritesArr.push(project.name)
        }
      }

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
                      <ChevronRight color='#fff' absoluteStrokeWidth={true} size={32} />
                    </button>
                  </div>
                </div>
                <div className='flex w-full flex-col gap-8'>
                  {projects?.length === 0 ? <p>Nothing here today</p> : projects?.map((project, index) => {
                    return <ProjectItem key={project.project_id} sessionUserId={result.userId} isFavourited={favouritesArr.includes(project.name) ? true : false} project={project} />
                  })}
                </div>
              </div>
            </div>
          </main>
          <FooterComponent />
        </div>
      )
    } else {

    }


  } catch (error) {

  }

}