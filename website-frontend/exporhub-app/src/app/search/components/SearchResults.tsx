'use client'

import ProjectItem from '@/app/components/project/ProjectItem'
import ProfileItem from '@/app/components/user/ProfileItem'
import { useSearchParams } from 'next/navigation'
import React, { useEffect, useState } from 'react'
import fetchFavourite from '~/fetch/fetchFavourite'
import { fetchUser } from '~/fetch/fetchUser'
import { Project, User } from '~/types/types'

export default function SearchResults({ sessionUserId }: {
    sessionUserId: number,
}) {
    const [results, setResults] = useState<Array<{ Users: User[], Projects: Project[] }> | null>(null)
    const [projects, setProjects] = useState<Project[] | null>(null)
    const [users, setUsers] = useState<User[] | null>(null)
    const [favouritesArr, setFavouritesArr] = useState<boolean[]>([])
    const queryParams = useSearchParams()

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

    useEffect(() => {
        const setStates = async () => {
            if (results) {
                const arr: boolean[] = []

                for (const project of results[1].Projects) {
                    const favouriteObj = await fetchFavourite(sessionUserId, project.project_id)

                    favouriteObj && arr.push(true)
                }

                setFavouritesArr(arr)
                setUsers(results[0].Users)
                setProjects(results[1].Projects)
            }
        }

        setStates()
    }, [results, sessionUserId])
    return (
        <>
            <section className='flex w-full justify-evenly'>
                <p className='text-2xl'>Search results</p>
                <div className='bg-black h-fit rounded-full'>
                    <button className='text-lg px-6 py-1'>Filter</button>
                </div>
            </section>
            <section className='flex w-full items-center flex-col gap-8'>
                {users ? users.map(user => {
                    return <ProfileItem key={user.user_id} user={user} followedUsers={null} sessionUserId={sessionUserId} />
                }) : null}
                {projects ? projects.map(async (project, index) => {
                    const user = await fetchUser(project.user_id)
                    return <ProjectItem key={project.project_id} user={user} project={project} sessionUserId={sessionUserId} isFavourited={favouritesArr[index]} />
                }) : null}
                {projects && users ? users.length === 0 && projects.length === 0 && <p>No Results</p> : null}
            </section>
        </>
    )
}
