/** @type {import('next').NextConfig} */
const nextConfig = {
  reactStrictMode: true,
  swcMinify: true,
  trailingSlash: true,
  output: 'export',
  distDir: 'dist',
  images: {
    unoptimized: true,
  },
  transpilePackages: [
    'ahooks',
    'antd',
    'rc-util',
    '@ant-design/icons',
    '@ant-design/icons-svg',
    'rc-pagination',
    'rc-picker',
    'rc-tree',
    'rc-table',
  ],
};

module.exports = nextConfig;
