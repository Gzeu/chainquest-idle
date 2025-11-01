import Link from 'next/link';

export default function Page() {
  return (
    <main style={{ padding: 24, fontFamily: 'system-ui, sans-serif' }}>
      <h1>ChainQuest Idle</h1>
      <p>Frontend Next.js integrat. Alege un demo:</p>
      <ul>
        <li><Link href="/wallet">Conectează xPortal Wallet</Link></li>
        <li><Link href="/wasm">Rulează clientul WASM</Link></li>
        <li><Link href="/status">Status & deployment tips</Link></li>
      </ul>
    </main>
  );
}
