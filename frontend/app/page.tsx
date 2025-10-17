export default function Page() {
  const repo = "https://github.com/Gzeu/chainquest-idle";
  const renderHint = "Set RENDER_SERVICE_ID/RENDER_API_KEY in GitHub Secrets and run the Render workflow.";
  const vercelHint = "Connect this repo to Vercel or use VERCEL_TOKEN in CI to deploy.";

  return (
    <main>
      <h1>ChainQuest Idle - Status</h1>
      <p>Welcome! This is the minimal frontend for demo, status and links.</p>
      <ul>
        <li><b>Repository:</b> <a href={repo} target="_blank" rel="noreferrer">{repo}</a></li>
        <li><b>Render:</b> {renderHint}</li>
        <li><b>Vercel:</b> {vercelHint}</li>
      </ul>
      <h2>Local commands</h2>
      <pre>
        <code>cargo run --bin server</code>
      </pre>
      <pre>
        <code>cargo run --bin client</code>
      </pre>
    </main>
  );
}
