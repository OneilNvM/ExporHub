import { Metadata } from "next";
import React from "react";

export const metadata: Metadata = {
    title: "Login To Account",
    description: "Account login page for ExporHub"
}

export default function LoginLayout({ children }: Readonly<{ children: React.ReactNode }>) {
    return (
        <div className="size-full">
            {children}
        </div>
    )
}