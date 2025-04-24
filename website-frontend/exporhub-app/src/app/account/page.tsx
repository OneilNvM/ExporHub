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
import fetchFavourite from '~/fetch/fetchFavourite'
import fetchFollowedUsers from '~/fetch/fetchFollowedUsers'
import { logOut } from '../actions/auth'
import { redirect } from 'next/navigation'

export default async function Account() {
  try {
    const cookieStore = await cookies()
    const session = cookieStore.get('session')
    const result = await decrypt(session?.value)
  
    if (result) {
      const user = await fetchUser(result.userId)
      const followings = await fetchFollowedUsers(result.userId)
      const projectNotifications = await fetchUpdatedProjects(result.userId)
      const projects = await fetchUserProjects(result.userId)
      const favouriteProjects = await fetchFavouriteProjects(result.userId)
    
      let favouritesArr = []
      if (user && projects) {
        for (const project of projects) {
          const favouriteObj = await fetchFavourite(user.user_id, project.project_id)
    
          favouriteObj && favouritesArr.push(true)
        }
      }
  
      console.log(result)
      console.log(followings)
  
      return (
        <div className='grid auto-rows-auto size-full overflow-auto'>
          <NavBarComponent />
          <main className='flex flex-col md:flex-row items-center row-span-1'>
            <AccountTab sessionUserId={result.userId} followings={followings} favouritesArr={favouritesArr} favouriteProjects={favouriteProjects} user={user} projects={projects} projectNotifications={projectNotifications} />
          </main>
          <FooterComponent />
        </div>
      )
    } else {
      await logOut()
  
      redirect('/login')
    }
  } catch (error) {
    console.error()
  }

}
