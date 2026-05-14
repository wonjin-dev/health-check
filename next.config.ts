import type { NextConfig } from 'next';

const nextConfig: NextConfig = {
	async rewrites() {
		return [
			{
				source: '/proxy/:all*',
				destination: 'https://:all*',
			},
		];
	},
	webpack(config) {
		config.experiments = {
			...config.experiments,
			asyncWebAssembly: true,
			layers: true,
		};
		return config;
	},
};

export default nextConfig;
