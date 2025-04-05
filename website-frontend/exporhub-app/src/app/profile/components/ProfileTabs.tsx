'use client'

import FavouritesTab from '@/app/components/profile-tabs/FavouritesTab'
import FollowingTab from '@/app/components/profile-tabs/FollowingTab'
import ProfileTab from '@/app/components/profile-tabs/ProfileTab'
import ProjectsTab from '@/app/components/profile-tabs/ProjectsTab'
import { useSearchParams } from 'next/navigation'
import React from 'react'

export default function ProfileTabs() {
    const searchParams = useSearchParams()

    switch (searchParams.get('tab')) {
        case 'projects':
            return (
                <ProjectsTab />
            )
        case 'favourites':
            return (
                <FavouritesTab />
            )
        case 'following':
            return (
                <FollowingTab />
            )
        default:
            return (
                <ProfileTab />
            )
    }
}
