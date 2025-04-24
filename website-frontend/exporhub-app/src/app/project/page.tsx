import React from 'react'
import NavBarComponent from '../components/general/NavBarComponent'
import FooterComponent from '../components/general/FooterComponent'

export default function Project() {
  return (
    <div className='grid auto-rows-auto size-full overflow-auto'>
      <NavBarComponent />
      <main className='flex flex-col md:flex-row items-center row-span-1'>
        <div>hi</div>
      </main>
      <FooterComponent />
    </div>
  )
}
