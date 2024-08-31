/** @type {import('next').NextConfig} */
const nextConfig = {
  output: 'export',
  async redirects() {
    return [
      {
        source: '/dash',
        destination: '/dash/pos',
        permanent: true,
      },
    ];
  }
};

export default nextConfig;
