'use client'

import { Star } from 'lucide-react'
import React, { useEffect, useId } from 'react'
import { Project, User } from '~/types/types'

export default function FavouriteButton({ user, isFavourited, project }: { user: User | null, isFavourited: boolean, project: Project }) {
    const uniqueId = useId()

    useEffect(() => {
        const favouriteButton = document.getElementById(uniqueId)

        if (favouriteButton) {
            if (isFavourited) {
                favouriteButton.classList.add("favourited")
                favouriteButton.classList.remove("unfavourited")
            } else {
                favouriteButton.classList.add("unfavourited")
                favouriteButton.classList.remove("favourited")
            }
        }
    }, [])

    const handleFavourite = async () => {
        const favouriteButton = document.getElementById(uniqueId)

        try {
            const favouriteResponse = await fetch(`https://api.exporhub.com:9000/api/favourite/u-p-id?user_id=${user?.user_id}&project_id=${project.project_id}`)

            if (!favouriteResponse.ok) {
                throw new Error(`Failed to find favourite`)
            }

            const json = await favouriteResponse.json()

            if (json.code) {
                const newFavourite = await fetch(`https://api.exporhub.com:9000/api/favourite/new?user_id=${user?.user_id}&project_id=${project.project_id}`, {
                    method: "post"
                })

                if (!newFavourite.ok) {
                    throw new Error(`Failed to create favourite`)
                }

                console.log("Successful favourite")

                if (favouriteButton) {
                    favouriteButton.classList.add("favourited")
                    favouriteButton.classList.remove("unfavourited")
                }
            } else {
                const deleteFavourite = await fetch(`https://api.exporhub.com:9000/api/favourite/unfavourite?user_id=${user?.user_id}&project_id=${project.project_id}`, {
                    method: "post"
                })
            
                if (!deleteFavourite.ok) {
                    throw new Error(`Failed to delete favourite`)
                }
    
                console.log("Successful delete")
    
                if (favouriteButton) {
                    favouriteButton.classList.add("unfavourited")
                    favouriteButton.classList.remove("favourited")
                }
            }
        } catch (error) {
            console.error(error)
        }
    }

    return (
        <div id={uniqueId} className='w-fit border rounded-md transition-colors duration-200 ease-in-out border-pink-300 dark:border-pink-950'>
            <button onClick={handleFavourite} className='flex px-4 items-center gap-4 text-pink-400 dark:text-pink-950'>
                <Star size={16} absoluteStrokeWidth={true} />
                <span>{project.favourites !== 1 ? project.favourites + " favourites" : "1 favourite"}</span>
            </button>
        </div>
    )
}
