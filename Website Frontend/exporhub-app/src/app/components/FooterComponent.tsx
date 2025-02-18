import React from 'react'

export default function FooterComponent() {
    return (
        <footer className='flex gap-24 justify-center items-center m-10 text-pink-950/70'>
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
        </footer>
    )
}
