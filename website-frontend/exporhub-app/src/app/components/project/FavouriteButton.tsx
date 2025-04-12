'use client'

import { Star } from 'lucide-react'
import React, { useId } from 'react'

export default function FavouriteButton() {
    const uniqueId = useId()
    const handleFavourite = () => {
        const favouriteButton = document.getElementById(uniqueId)

        if (favouriteButton) {
            if (favouriteButton.classList.contains("favourited")) {
                favouriteButton.classList.add("unfavourited")
                favouriteButton.classList.remove("favourited")
            } else {
                favouriteButton.classList.add("favourited")
                favouriteButton.classList.remove("unfavourited")
            }
        }
    }

    return (
        <div id={uniqueId} className='w-fit border rounded-md transition-colors duration-200 text-pink-400 ease-in-out border-pink-300 dark:border-pink-950'>
            <button onClick={handleFavourite} className='flex px-4 items-center gap-4 dark:text-pink-950'>
                <Star size={16} absoluteStrokeWidth={true} />
                <span>Favourite</span>
            </button>
        </div>
    )
}
