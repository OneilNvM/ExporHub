'use client'

import React, { useEffect, useState } from 'react'
import CreateAccountForm from './CreateAccountForm'

export default function FormContainer() {
    const [error, setError] = useState("")

    useEffect(() => {
        const errorContainer = document.getElementById('errorContainer')

        if (errorContainer) {
            if (error.length > 0) {
                errorContainer.style.display = "block"
            }
        }
    }, [error])

    return (
        <section className='relative flex flex-[2_1_0%] flex-col items-start gap-24 m-5 lg:items-center'>
            <div className='text-5xl font-[700]'>Create your new account</div>
            <div id='errorContainer' className='hidden absolute top-32 border border-red-900 rounded-xl px-16 py-8'>
                <p className='text-red-500'>{error}</p>
            </div>
            <CreateAccountForm setError={setError}/>
        </section>
    )
}
