import React from 'react'
import LoginFormComponent from './components/LoginFormComponent'

export default function Login() {
  return (
    <div className='flex flex-col flex-1'>
      <main className='flex flex-1 items-center'>
        <div className='flex flex-[0_0_100%] justify-center'>
          <LoginFormComponent />
        </div>
      </main>
      <footer className='m-10 text-pink-950/70'>
        <div className='flex gap-24 justify-center items-center'>
          <div>
            <p>image</p>
          </div>
          <div className='flex flex-col'>
            <p>Github</p>
            <p>X</p>
            <p>Linkedin</p>
          </div>
          <div className='flex flex-col'>
            <p>About us</p>
            <p>Contact us</p>
          </div>
          <div className='flex flex-col'>
            <p>Cookies</p>
            <p>Terms</p>
            <p>Privacy</p>
          </div>
        </div>
      </footer>
    </div>
  )
}
