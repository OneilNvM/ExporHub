import React from 'react'

export default async function Project({ params }: { params: Promise<{ projectName: string }> }) {
    return (
        <div>{(await params).projectName.replaceAll("%20", " ")}</div>
    )
}
