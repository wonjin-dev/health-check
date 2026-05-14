import { NextRequest, NextResponse } from 'next/server';

export async function GET(request: NextRequest) {
	const searchParams = request.nextUrl.searchParams;
	const targetUrl = searchParams.get('url');

	if (!targetUrl) {
		return NextResponse.json({ error: 'URL is required' }, { status: 400 });
	}

	try {
		const response = await fetch(targetUrl);
		const data = await response.text();

		return new NextResponse(data, {
			status: response.status,
			headers: { 'Content-Type': 'text/plain' },
		});
	} catch (error) {
		return NextResponse.json({ error: 'Failed to fetch' }, { status: 500 });
	}
}
