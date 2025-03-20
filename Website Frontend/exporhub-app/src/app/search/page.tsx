import React from 'react'
import NavBarComponent from '../components/general/NavBarComponent'
import FooterComponent from '../components/general/FooterComponent'
import SearchResults from './components/SearchResults'

export default function Search() {
  return (
    <div className='grid auto-rows-auto size-full overflow-auto'>
      <NavBarComponent />
      <main className='flex flex-col items-center gap-16 mt-8'>
        <SearchResults />
      </main>
      <FooterComponent />
    </div>
  )
}
