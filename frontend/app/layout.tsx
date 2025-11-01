export const metadata = {
  title: 'ChainQuest Idle',
  description: 'Idle RPG cu SFT-uri MultiversX, ENet multiplayer, AI maps',
};

import './globals.css';

export default function RootLayout({ children }: { children: React.ReactNode }) {
  return (
    <html lang="en">
      <body>{children}</body>
    </html>
  );
}
