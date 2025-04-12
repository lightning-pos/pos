/** @type {import('next').NextConfig} */
const nextConfig = {
    sassOptions: {
        quietDeps: true // This will suppress warnings from dependencies
    },
    async redirects() {
        return [
            {
                source: '/',
                destination: '/login',
                permanent: true,
            },
        ]
    }
};

export default nextConfig;
