import { MiddlewareConfig, NextRequest, NextResponse } from "next/server";

export function middleware(request: NextRequest) {
    const headers = new Headers(request.headers)

    headers.set("x-search-params", request.nextUrl.searchParams.toString())

    return NextResponse.next({ request: {headers: headers} })
}

export const config: MiddlewareConfig = {
    matcher: ['/account', '/profile/:path']
}