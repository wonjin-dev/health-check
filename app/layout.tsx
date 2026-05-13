import type { Metadata } from 'next';

export const metadata: Metadata = {
	title: 'Health Check',
};

export default function RootLayout({
	children,
}: Readonly<{
	children: React.ReactNode;
}>) {
	return (
		<html lang='ko'>
			<body>{children}</body>
		</html>
	);
}
