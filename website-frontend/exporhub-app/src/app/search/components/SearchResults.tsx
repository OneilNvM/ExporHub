'use client'

import ProjectItem from '@/app/components/project/ProjectItem'
import ProfileItem from '@/app/components/user/ProfileItem'
import { useSearchParams } from 'next/navigation'
import React, { useEffect, useState } from 'react'
import { Project, User } from '~/types/types'

export default function SearchResults({ sessionUserId }: {
    sessionUserId: number,
}) {
    const [results, setResults] = useState<Array<{Users: User[], Projects: Project[]}> | null>(null)
    const queryParams = useSearchParams()

    const test = (results: Array<{Users: User[], Projects: Project[]}>) => {
        let arr = []

        if (results[0].Users) {
            for (let i = 0; i < results[0].Users.length; i++) {
                arr.push(results[0].Users[i])
            }
        } else {
            for (let i = 0; i < results[1].Projects.length; i++) {
                arr.push(results[1].Projects[i])
            }
        }
        console.log(arr)
        return null
    }

    useEffect(() => {
        fetch(`https://api.exporhub.com:9000/api/search/query?q=${queryParams.get("q")}`)
            .then(res => {
                if (!res.ok) {
                    throw new Error(`Failed to process search`)
                }

                return res.json()
            })
            .then(vals => {
                setResults(vals)

                console.log(vals)
            })
            .catch(error => {
                console.error(error)
            })
    }, [queryParams])
    return (
        <>
            <section className='flex w-full justify-evenly'>
                <p className='text-2xl'>Search results</p>
                <div className='bg-black h-fit rounded-full'>
                    <button className='text-lg px-6 py-1'>Filter</button>
                </div>
            </section>
            <section className='flex flex-col gap-8'>
                {results ? test(results) : null}
            </section>
        </>
    )
}
