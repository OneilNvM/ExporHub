import Link from 'next/link'
import React from 'react'

export default function FooterComponent() {
    return (
        <footer className='flex row-span-1 gap-24 justify-center items-center m-10 text-pink-300 dark:text-pink-950'>
            <div>
                <p>&copy; 2025 ExporHub</p>
            </div>
            <div className='flex flex-col'>
                <Link className='hover:underline' target='blank' href={'https://github.com/OneilNvM/ExporHub'}>Github</Link>
                <Link className='hover:underline' target='blank' href={'https://x.com/OneilNvMs'}>X</Link>
                <Link className='hover:underline' target='blank' href={'/'}>Linkedin</Link>
            </div>
            <div className='flex flex-col'>
                <Link className='hover:underline' href={'/about-us'}>About Us</Link>
                <Link className='hover:underline' href={'/contact-us'}>Contact Us</Link>
            </div>
            <div className='flex flex-col'>
                <Link className='hover:underline' href={'/settings?tab=cookies'}>Manage Cookies</Link>
                <Link className='hover:underline' href={'/terms-and-conditions'}>Terms and Conditions</Link>
                <Link className='hover:underline' href={'/privacy-policy'}>Privacy Policy</Link>
            </div>
        </footer>
    )
}
