'use client'

import { Star } from 'lucide-react'
import React, { useEffect, useId, useState } from 'react'
import { Project } from '~/types/types'

export default function FavouriteButton({ sessionUserId, isFavourited, project }: { sessionUserId: number, isFavourited: boolean, project: Project }) {
    const [favourited, setFavourited] = useState<boolean>(isFavourited)
    const uniqueId = useId()

    useEffect(() => {
        const favouriteButton = document.getElementById(uniqueId)

        if (favouriteButton) {
            if (favourited) {
                favouriteButton.classList.add("favourited")
                favouriteButton.classList.remove("unfavourited")
            } else {
                favouriteButton.classList.add("unfavourited")
                favouriteButton.classList.remove("favourited")
            }
        }
    }, [favourited, uniqueId])

    const handleFavourite = async () => {
        const favouriteButton = document.getElementById(uniqueId)

        try {
            if (!favourited) {
                const newFavourite = await fetch(`https://api.exporhub.com:9000/api/favourite/new?user_id=${sessionUserId}&project_id=${project.project_id}`, {
                    method: "post",
                    headers: {
                        "Content-Type": "application/json",
                        "Authorization": `Basic ${process.env.NEXT_PUBLIC_EXPORHUB_API_KEY}`
                    }
                })

                if (!newFavourite.ok) {
                    throw new Error(`Failed to create favourite`)
                }

                console.log("Successful favourite")

                if (favouriteButton) {
                    favouriteButton.classList.add("favourited")
                    favouriteButton.classList.remove("unfavourited")
                }

                setFavourited(true)
            } else {
                const deleteFavourite = await fetch(`https://api.exporhub.com:9000/api/favourite/unfavourite?user_id=${sessionUserId}&project_id=${project.project_id}`, {
                    method: "post",
                    headers: {
                        "Content-Type": "application/json",
                        "Authorization": `Basic ${process.env.NEXT_PUBLIC_EXPORHUB_API_KEY}`
                    }
                })

                if (!deleteFavourite.ok) {
                    throw new Error(`Failed to delete favourite`)
                }

                console.log("Successful delete")

                if (favouriteButton) {
                    favouriteButton.classList.add("unfavourited")
                    favouriteButton.classList.remove("favourited")
                }

                setFavourited(false)
            }
        } catch (error) {
            console.error(error)
        }
    }

    return (
        <div id={uniqueId} className='w-fit border rounded-md transition-colors duration-200 ease-in-out text-pink-400 dark:text-pink-950 border-pink-300 dark:border-pink-950'>
            <button onClick={handleFavourite} className='flex px-4 items-center gap-4'>
                <Star size={16} absoluteStrokeWidth={true} />
                <span>{project.favourites !== 1 ? project.favourites + " favourites" : "1 favourite"}</span>
            </button>
        </div>
    )
}
