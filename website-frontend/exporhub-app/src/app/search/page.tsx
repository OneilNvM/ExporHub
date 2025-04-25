import React from 'react'
import NavBarComponent from '../components/general/NavBarComponent'
import FooterComponent from '../components/general/FooterComponent'
import SearchResults from './components/SearchResults'
import { cookies } from 'next/headers'
import { decrypt } from '../lib/session'

export default async function Search() {
  try {
    const cookieStore = await cookies()
    const session = cookieStore.get('session')
    const result = await decrypt(session?.value)

    if (result) {
      return (
        <div className='grid auto-rows-auto size-full overflow-auto'>
          <NavBarComponent />
          <main className='flex flex-col items-center gap-16 mt-8'>
            <SearchResults sessionUserId={result.userId} />
          </main>
          <FooterComponent />
        </div>
      )
    } else {
      throw new Error(`Not logged in`)
    }
  } catch (error) {
    console.error(error)
  }
}
