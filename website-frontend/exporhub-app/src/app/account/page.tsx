import React from 'react'
import NavBarComponent from '../components/general/NavBarComponent'
import FooterComponent from '../components/general/FooterComponent'
import AccountTab from './components/AccountTab'
import { cookies } from 'next/headers'
import { decrypt } from '../lib/session'
import { fetchUser } from '~/fetch/fetchUser'
import fetchUpdatedProjects from '~/fetch/fetchUpdatedProjects'
import fetchUserProjects from '~/fetch/fetchUserProjects'
import fetchFavouriteProjects from '~/fetch/fetchFavouriteProjects'

export default async function Account() {
  const cookieStore = await cookies()
  const session = cookieStore.get('session')
  const result = await decrypt(session?.value)

  const user = await fetchUser(result ? result.userId : null)
  const projectNotifications = await fetchUpdatedProjects(result ? result.userId : null)
  const projects = await fetchUserProjects(result ? result.userId : null)
  const favouriteProjects = await fetchFavouriteProjects(result ? result.userId : null)

  console.log(result)

  return (
    <div className='grid auto-rows-auto size-full overflow-auto'>
      <NavBarComponent />
      <main className='flex flex-col md:flex-row items-center row-span-1'>
        <AccountTab favouriteProjects={favouriteProjects} user={user} projects={projects} projectNotifications={projectNotifications} />
      </main>
      <FooterComponent />
    </div>
  )
}