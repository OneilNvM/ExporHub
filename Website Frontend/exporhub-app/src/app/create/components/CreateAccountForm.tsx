'use client'

import Link from 'next/link'
import React, { FormEvent, useRef, useState } from 'react'
import { AccountCreateStatus, User } from '~/types/types'

export default function CreateAccountForm({ setError }: { setError: React.Dispatch<React.SetStateAction<string>> }) {
  const [username, setUsername] = useState("")
  const [email, setEmail] = useState("")
  const [password, setPassword] = useState("")
  const usernameInput = useRef<HTMLInputElement | null>(null)
  const usernameContainer = useRef<HTMLSpanElement | null>(null)
  const emailInput = useRef<HTMLInputElement | null>(null)
  const emailContainer = useRef<HTMLSpanElement | null>(null)
  const passwordInput = useRef<HTMLInputElement | null>(null)
  const passwordContainer = useRef<HTMLSpanElement | null>(null)

  const resetInputs = () => {
    setUsername("")
    setPassword("")
    setEmail("")
  }

  const resetContainers = () => {
    if (usernameInput.current && usernameContainer.current && emailInput.current && emailContainer.current && passwordInput.current && passwordContainer.current) {
      if (!usernameContainer.current.classList.contains("after:transparent")) {
        usernameContainer.current.classList.replace("after:bg-green-400", "after:transparent")
        usernameContainer.current.classList.replace("after:bg-red-500", "after:transparent")
        usernameInput.current.classList.replace("dark:bg-transparent", "dark:bg-pink-950/10")
        usernameInput.current.classList.replace("dark:border-transparent", "dark:border")
      }
      if (!emailContainer.current.classList.contains("after:transparent")) {
        emailContainer.current.classList.replace("after:bg-green-400", "after:transparent")
        emailContainer.current.classList.replace("after:bg-red-500", "after:transparent")
        emailInput.current.classList.replace("dark:bg-transparent", "dark:bg-pink-950/10")
        emailInput.current.classList.replace("dark:border-transparent", "dark:border")
      }
      if (!passwordContainer.current.classList.contains("after:transparent")) {
        passwordContainer.current.classList.replace("after:bg-green-400", "after:transparent")
        passwordContainer.current.classList.replace("after:bg-red-500", "after:transparent")
        passwordInput.current.classList.replace("dark:bg-transparent", "dark:bg-pink-950/10")
        passwordInput.current.classList.replace("dark:border-transparent", "dark:border")
      }
    }
  }

  const inputBlur = async (input: React.RefObject<HTMLInputElement | null>, container: React.RefObject<HTMLSpanElement | null>) => {
    if (input.current && container.current) {
      if (input.current.value) {
        if (input.current.id === "username") {
          try {
            let encoded = Buffer.from("OneilNvM:authorized").toString("base64")

            const response = await fetch(`https://api.exporhub.com:9000/api/user/${input.current.value}`, {
              method: "get",
              headers: {
                "Authorization": `Basic ${encoded}`
              },
              credentials: "include"
            })

            if (response.ok) {
              throw new Error(`Username is already taken`)
            }

            if (input.current.value.length < 3 || input.current.value.includes("@")) {
              container.current.classList.contains("after:transparent") ? container.current.classList.replace("after:transparent", "after:bg-red-500") : container.current.classList.replace("after:bg-green-400", "after:bg-red-500")
            } else {
              container.current.classList.contains("after:transparent") ? container.current.classList.replace("after:transparent", "after:bg-green-400") : container.current.classList.replace("after:bg-red-500", "after:bg-green-400")
            }

          } catch (error) {
            const message = `${error}`

            container.current.classList.contains("after:transparent") ? container.current.classList.replace("after:transparent", "after:bg-red-500") : container.current.classList.replace("after:bg-green-400", "after:bg-red-500")

            setError(message.replace("Error: ", ""))
          }
        } else if (input.current.id === "email") {
          if (!input.current.value.includes("@")) {
            container.current.classList.contains("after:transparent") ? container.current.classList.replace("after:transparent", "after:bg-red-500") : container.current.classList.replace("after:bg-green-400", "after:bg-red-500")
          } else {
            container.current.classList.contains("after:transparent") ? container.current.classList.replace("after:transparent", "after:bg-green-400") : container.current.classList.replace("after:bg-red-500", "after:bg-green-400")
          }
        } else if (input.current.type === "password") {
          if (input.current.value.length < 8) {
            container.current.classList.contains("after:transparent") ? container.current.classList.replace("after:transparent", "after:bg-red-500") : container.current.classList.replace("after:bg-green-400", "after:bg-red-500")
          } else {
            container.current.classList.contains("after:transparent") ? container.current.classList.replace("after:transparent", "after:bg-green-400") : container.current.classList.replace("after:bg-red-500", "after:bg-green-400")
          }
        }

        input.current.classList.replace("dark:bg-pink-950/10", "dark:bg-transparent")
        input.current.classList.replace("dark:border", "dark:border-transparent")
      } else {
        if (!input.current.classList.contains("dark:bg-pink-950/10")) {
          input.current.classList.replace("dark:bg-transparent", "dark:bg-pink-950/10")
          input.current.classList.replace("dark:border-transparent", "dark:border")
        }
        container.current.classList.replace("after:bg-green-400", "after:transparent")
        container.current.classList.replace("after:bg-red-500", "after:transparent")
      }
    }
  }

  const handleAccountCreation = async (e: FormEvent) => {
    e.preventDefault()

    try {
      const response = await fetch("https://api.exporhub.com:9000/account/create-account", {
        method: "post",
        headers: {
          "Content-Type": "application/json"
        },
        body: JSON.stringify({ username: username, email: email, password: password })
      })

      if (!response.ok) {
        const error = await response.json() as AccountCreateStatus;

        throw new Error(`${error.message}`)
      }

      let json = await response.json() as User

      console.dir(json)

    } catch (error) {
      const message = `${error}`
      setError(message.replace("Error: ", ""))

      resetInputs()
      resetContainers()
    }
  }

  return (
    <div className='flex flex-1 mt-24'>
      <form onSubmit={handleAccountCreation} className='flex flex-col gap-5'>
        <div className='flex flex-col gap-3'>
          <label className='text-2xl' htmlFor="username">Username</label>
          <span ref={usernameContainer} className='relative after:size-6 after:block after:transparent after:rounded-full after:absolute after:top-5 after:right-6 after:transition-all after:duration-1000 after:ease-out'>
            <input value={username} onChange={e => setUsername(e.target.value)} onBlur={() => inputBlur(usernameInput, usernameContainer)} ref={usernameInput} className='w-full px-5 py-4 text-xl rounded-3xl shadow-lg shadow-pink-300/40  bg-gradient-to-b from-transparent to-pink-300/30 dark:bg-none dark:bg-pink-950/10 dark:border dark:border-pink-800 dark:shadow-none transition-all duration-1000 ease-in-out' required type="text" id='username' spellCheck='false' />
          </span>
        </div>
        <div className='flex flex-col gap-3'>
          <label className='text-2xl' htmlFor="email">Email Address</label>
          <span ref={emailContainer} className='relative after:size-6 after:block after:transparent after:rounded-full after:absolute after:top-5 after:right-6 after:transition-all after:duration-1000 after:ease-out'>
            <input value={email} onChange={e => setEmail(e.target.value)} onBlur={() => inputBlur(emailInput, emailContainer)} ref={emailInput} className='w-full px-5 py-4 text-xl rounded-3xl shadow-lg shadow-pink-300/40 bg-gradient-to-b from-transparent to-pink-300/30 dark:bg-none dark:bg-pink-950/10 dark:border dark:border-pink-800 dark:shadow-none transition-all duration-1000 ease-in-out' required type="text" id='email' spellCheck='false' />
          </span>
        </div>
        <div className='flex flex-col gap-3'>
          <label className='text-2xl' htmlFor="password">Password</label>
          <span ref={passwordContainer} className='relative after:size-6 after:block after:transparent after:rounded-full after:absolute after:top-5 after:right-6 after:transition-all after:duration-1000 after:ease-out'>
            <input value={password} onChange={e => setPassword(e.target.value)} onBlur={() => inputBlur(passwordInput, passwordContainer)} ref={passwordInput} className='w-full px-5 py-4 text-pink-600 text-xl rounded-3xl shadow-lg shadow-pink-300/40 bg-gradient-to-b from-transparent to-pink-300/30 dark:bg-none dark:bg-pink-950/10 dark:border dark:border-pink-800 dark:shadow-none transition-all duration-1000 ease-in-out' required type="password" name="" id="password" spellCheck='false' />
          </span>
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
