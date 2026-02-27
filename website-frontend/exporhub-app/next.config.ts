import type { NextConfig } from "next";
import { URL } from "url";

const nextConfig: NextConfig = {
  images: {
    remotePatterns: [{
      protocol: 'https',
      hostname: 'mwbiyrhmklzoczsbtblp.supabase.co',
      port: '',
      pathname: '/storage/v1/object/sign/**',
    },],
  },
};

export default nextConfig;
