import React from 'react'
import LoginFormComponent from './components/LoginFormComponent'
import FooterComponent from '../components/general/FooterComponent'

export default function Login() {
  return (
    <div className='flex flex-col flex-1'>
      <main className='flex flex-1 items-center'>
        <div className='flex flex-[0_0_100%] justify-center'>
          <LoginFormComponent />
        </div>
      </main>
      <FooterComponent />
    </div>
  )
}
