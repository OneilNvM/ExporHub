import React from 'react'
import NavBarComponent from '../components/general/NavBarComponent'
import FooterComponent from '../components/general/FooterComponent'
import AccountTab from './components/AccountTab'
import { cookies } from 'next/headers'
import { decrypt } from '../lib/session'
import { fetchUser } from '~/fetch/fetchUser'

export default async function Account() {
  const cookieStore = await cookies()

  const session = cookieStore.get('session')

  const result = await decrypt(session?.value)

  console.log(result)

  const user = await fetchUser(result ? result.userId : null)

  return (
    <div className='grid auto-rows-auto size-full overflow-auto'>
      <NavBarComponent />
      <main className='flex flex-col md:flex-row items-center row-span-1'>
        <AccountTab user={user} />
      </main>
      <FooterComponent />
    </div>
  )
}