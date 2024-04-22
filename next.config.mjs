import BundleAnalyzer from '@next/bundle-analyzer';

const withBundleAnalyzer = BundleAnalyzer({
  enabled: process.env.ANALYZE === 'true',
});

/** @type {import('next').NextConfig} */
const nextConfig = {
  reactStrictMode: true,
  swcMinify: true,
  trailingSlash: true,
  output: 'export',
  compiler: {
    removeConsole:
      process.env.NODE_ENV === 'production' ? { exclude: ['error'] } : false,
  },
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

export default withBundleAnalyzer(nextConfig);
