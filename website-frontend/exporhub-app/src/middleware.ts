import { MiddlewareConfig, NextRequest } from "next/server";

export function middleware(request: NextRequest) {
    
}

export const config: MiddlewareConfig = {
    matcher: '/'
}