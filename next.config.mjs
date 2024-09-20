/** @type {import('next').NextConfig} */
const nextConfig = {
  output: 'export',
  transpilePackages: [
    "@electric-sql/pglite"
  ]
};

export default nextConfig;
