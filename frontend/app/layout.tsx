export default function RootLayout({ children }: { children: React.ReactNode }) {
  return (
    <html lang="en">
      <body style={{fontFamily:'Inter, system-ui, sans-serif', margin:0, padding:20, background:'#0b0f1a', color:'#e6e8ef'}}>
        {children}
      </body>
    </html>
  );
}
