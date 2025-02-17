'use client'

import Link from 'next/link'
import React, { useRef } from 'react'

export default function CreateAccountForm() {
  const usernameInput = useRef<HTMLInputElement | null>(null)
  const emailInput = useRef<HTMLInputElement | null>(null)
  const passwordInput = useRef<HTMLInputElement | null>(null)

  const handleBlur = (input: React.RefObject<HTMLInputElement | null>) => {
    if (input.current?.value) {
      input.current.classList.replace("dark:bg-pink-950/10", "dark:bg-transparent")
      input.current.classList.replace("dark:border", "dark:border-transparent")
    } else {
      if (!input.current?.classList.contains("dark:bg-pink-950/10")) {
        input.current?.classList.replace("dark:bg-transparent", "dark:bg-pink-950/10")
        input.current?.classList.replace("dark:border-transparent", "dark:border")
      }
    }
  }

  return (
    <div className='flex flex-1 mt-24'>
      <form className='flex flex-col gap-5' action="">
        <div className='flex flex-col gap-3'>
          <label className='text-2xl' htmlFor="username">Username</label>
          <span className='after:size-8 after:block after:bg-white after:relative after:left-12'>
            <input onBlur={() => handleBlur(usernameInput)} ref={usernameInput} className='w-full px-5 py-4 text-xl rounded-3xl shadow-lg shadow-pink-300/40  bg-gradient-to-b from-transparent to-pink-300/30 dark:bg-none dark:bg-pink-950/10 dark:border dark:border-pink-800 dark:shadow-none  transition-all duration-1000 ease-in-out' required type="text" id='username' spellCheck='false' />
          </span>
        </div>
        <div className='flex flex-col gap-3'>
          <label className='text-2xl' htmlFor="email">Email Address</label>
          <input onBlur={() => handleBlur(emailInput)} ref={emailInput} className='px-5 py-4 text-xl rounded-3xl shadow-lg shadow-pink-300/40 bg-gradient-to-b from-transparent to-pink-300/30 dark:bg-none dark:bg-pink-950/10 dark:border dark:border-pink-800 dark:shadow-none transition-all duration-1000 ease-in-out' required type="text" id='email' spellCheck='false' />
        </div>
        <div className='flex flex-col gap-3'>
          <label className='text-2xl' htmlFor="password">Password</label>
          <input onBlur={() => handleBlur(passwordInput)} ref={passwordInput} className='px-5 py-4 text-pink-600 text-xl rounded-3xl shadow-lg shadow-pink-300/40 bg-gradient-to-b from-transparent to-pink-300/30 dark:bg-none dark:bg-pink-950/10 dark:border dark:border-pink-800 dark:shadow-none transition-all duration-1000 ease-in-out' required type="password" name="" id="password" spellCheck='false' />
        </div>
        <div className='flex flex-row-reverse items-center gap-4'>
          <label htmlFor="terms-privacy">By ticking this checkbox, you confirm that you agree to the <Link className='text-blue-400 underline visited:text-purple-500' href={'/terms-and-conditions'}>Terms and Conditions</Link> and the <Link className='text-blue-400 underline visited:text-purple-500' href={'/privacy-policy'}>Privacy Policy</Link></label>
          <input className='min-w-20 h-9 appearance-none transition-all before:transition-all before:block before:size-7 before:relative before:top-1 before:left-1 before:bg-gray-200 before:rounded-full before:checked:translate-x-[2.75rem] rounded-full bg-gray-300 checked:bg-pink-600 dark:bg-pink-950/30' required type="checkbox" name="" id="terms-privacy" />
        </div>

        <input className='mt-12 px-6 py-2 text-lg rounded-3xl self-center cursor-pointer shadow-lg transition-all hover:translate-y-[-5px] hover:bg-pink-400 hover:shadow-pink-300/90 hover:shadow-xl bg-pink-200 dark:bg-[#030303] dark:hover:shadow-pink-900/20 dark:hover:shadow-lg dark:hover:bg-pink-900' type="submit" value="Create Account" />
      </form>
    </div>
  )
}
