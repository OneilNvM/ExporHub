import React from 'react'
import NavBarComponent from '../components/NavBarComponent'
import FooterComponent from '../components/FooterComponent'
import AccountTab from './components/AccountTab'

export default function Account() {
  return (
    <div className='grid auto-rows-auto size-full overflow-auto overflow-x-hidden'>
      <NavBarComponent />
      <main className='flex flex-col md:flex-row items-center row-span-1'>
        <AccountTab />
      </main>
      <FooterComponent />
    </div>
  )
}