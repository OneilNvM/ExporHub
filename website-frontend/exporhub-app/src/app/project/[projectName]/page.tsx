import Image from 'next/image'
import React from 'react'
import { fetchProjectImages } from '~/fetch/fetchProjectImages'
import { fetchProjects } from '~/fetch/fetchProjects'
import { downloadImages } from '~/supabase/functions/downloadImages'
import { Project } from '~/types/types'

export default async function ProjectPage({ params }: { params: Promise<{ projectName: string }> }) {
    const projectName = (await params).projectName

    const project = await fetchProjects(projectName) as Project

    if (project) {
        let projectImages = null
        const images = await fetchProjectImages(project.user_id, project.project_id)

        console.dir("Images", images)

        console.dir("Project", project)

        if (images) {
            projectImages = await downloadImages(images)
        }

        console.dir("ProjectImages", projectImages)

        return (
            <div>
                {(await params).projectName.replaceAll("%20", " ")}
                {
                    projectImages ?
                        projectImages.length !== 0 ?
                            projectImages.map((url, index) => {
                                return <Image key={index} src={url.signedUrl} width={300} height={300} priority alt='Project image' />
                            }) :
                            null :
                        null
                }
            </div>
        )
    } else {
        console.error("Something went wrong")
    }
}
