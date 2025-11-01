"use client";

import { useEffect, useRef, useState } from 'react';

export default function WasmPage() {
  const [status, setStatus] = useState('WASM client soon. Build coming next.');
  const canvasRef = useRef<HTMLCanvasElement>(null);

  useEffect(() => {
    // Placeholder for future wasm-bindgen init
    setStatus('WASM bootstrap placeholder - integrare in lucru.');
  }, []);

  return (
    <main style={{ padding: 24 }}>
      <h1>WASM Client</h1>
      <p>{status}</p>
      <canvas ref={canvasRef} width={800} height={600} style={{ border: '1px solid #ccc' }} />
    </main>
  );
}
