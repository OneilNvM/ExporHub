import React from 'react'
import NavBarComponent from '../components/NavBarComponent'
import FooterComponent from '../components/FooterComponent'

export default function Home() {
  return (
    <div className='flex flex-col justify-between flex-1'>
      <NavBarComponent />
      <main className='flex-1'>
        <div></div>
      </main>
      <FooterComponent />
    </div>
  )
}
