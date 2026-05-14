'use client';

import { useEffect, useState, useCallback } from 'react';

export const WasmComponent = () => {
	const [result, setResult] = useState<unknown>(null);
	const [loading, setLoading] = useState<boolean>(false);
	const [wasmModule, setWasmModule] = useState<{
		check_site_wasm: (url: string) => Promise<unknown>;
	} | null>(null);

	useEffect(() => {
		const initWasm = async () => {
			try {
				const wasm = await import('../../pkg/health_check');
				await wasm.default();
				setWasmModule(wasm);
			} catch (error) {
				console.error('WASM 로드 실패:', error);
				setResult('WASM 로드 에러');
			}
		};
		initWasm();
	}, []);

	const performCheck = useCallback(
		async (checkFn: (url: string) => Promise<unknown>, targetUrl: string) => {
			setLoading(true);
			try {
				const encodedTarget = encodeURIComponent(targetUrl);
				const proxyUrl = `${window.location.origin}/api/proxy?url=${encodedTarget}`;

				const data = await checkFn(proxyUrl);
				setResult(data);
			} catch (error) {
				console.error('체크 중 에러:', error);
				setResult({ error: 'Failed to fetch via proxy' });
			} finally {
				setLoading(false);
			}
		},
		[]
	);

	const handleButtonClick = () => {
		if (wasmModule?.check_site_wasm) {
			performCheck(wasmModule.check_site_wasm, 'https://www.google.com');
		} else {
			alert('WASM 모듈이 아직 로드되지 않았습니다.');
		}
	};

	return (
		<div>
			<button onClick={handleButtonClick} disabled={loading || !wasmModule}>
				{loading ? '체크 중...' : '상태 확인'}
			</button>
			<div>
				<strong>결과</strong>
				<pre>
					{result
						? JSON.stringify(result, null, 2)
						: '결과가 여기에 표시됩니다.'}
				</pre>
			</div>
		</div>
	);
};
