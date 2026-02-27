import FooterComponent from '@/app/components/general/FooterComponent'
import NavBarComponent from '@/app/components/general/NavBarComponent'
import React from 'react'
import ProfileTabs from '../components/ProfileTabs'
import { fetchUser } from '~/fetch/fetchUser'
import fetchFollowedUsers from '~/fetch/fetchFollowedUsers'
import fetchUpdatedProjects from '~/fetch/fetchUpdatedProjects'
import fetchUserProjects from '~/fetch/fetchUserProjects'
import fetchFavouriteProjects from '~/fetch/fetchFavouriteProjects'
import { cookies } from 'next/headers'
import { decrypt } from '@/app/lib/session'
import fetchFollowedUser from '~/fetch/fetchFollowedUser'

export default async function Profile({ params }: { params: Promise<{ username: string }> }) {
  try {
    const username = (await params).username
    const user = await fetchUser(username)

    const cookieStore = await cookies()
    const session = cookieStore.get('session')
    const result = await decrypt(session?.value)

    console.log(result)

    if (user && result) {
      console.log(user)
      const isFollowed = await fetchFollowedUser(result.userId, user.user_id)
      const followings = await fetchFollowedUsers(user.user_id)
      const projectNotifications = await fetchUpdatedProjects(user.user_id)
      const projects = await fetchUserProjects(user.user_id)
      const favouriteProjects = await fetchFavouriteProjects(user.user_id)

      return (
        <div className='grid auto-rows-auto size-full overflow-auto'>
          <NavBarComponent />
          <main className='flex flex-col md:flex-row items-center row-span-1'>
            <ProfileTabs isFollowed={isFollowed} user={user} profileUserId={user.user_id} sessionUserId={result.userId} followings={followings} projects={projects} projectNotifications={projectNotifications} favouriteProjects={favouriteProjects} />
          </main>
          <FooterComponent />
        </div>
      )
    } else {
      throw new Error(`User does not exist`)
    }
  } catch (error) {
    console.error(error)
  }
}
