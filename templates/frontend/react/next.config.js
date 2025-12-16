/** @type {import('next').NextConfig} */
const nextConfig = {
  reactStrictMode: true,
  experimental: {
    serverComponentsExternalPackages: ["mongoose"],
  },
  async rewrites() {
    return [
      {
        source: '/api/:path*',
        destination: 'http://localhost:8080/api/:path*', // Proxy to Backend
      },
      {
        source: '/management/:path*',
        destination: 'http://localhost:8080/management/:path*',
      },
    ]
  },
}

module.exports = nextConfig
