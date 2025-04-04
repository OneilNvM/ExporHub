import React from 'react'
import NavBarComponent from '../components/general/NavBarComponent'
import FooterComponent from '../components/general/FooterComponent'
import ProfileTabs from './components/ProfileTabs'

export default function Profile() {
  return (
    <div className='grid auto-rows-auto size-full overflow-auto'>
      <NavBarComponent />
      <main className='flex flex-col md:flex-row items-center row-span-1'>
        <ProfileTabs />
      </main>
      <FooterComponent />
    </div>
  )
}
