"use client";

import { DappProvider, EnvironmentsEnum, LoginButton } from '@multiversx/sdk-dapp';

export default function WalletPage() {
  return (
    <DappProvider
      environment={EnvironmentsEnum.testnet}
      customNetworkConfig={{
        walletConnectV2ProjectId: process.env.NEXT_PUBLIC_WC_PROJECT_ID || '',
      }}
    >
      <main style={{ padding: 24 }}>
        <h1>xPortal Wallet Connect (Testnet)</h1>
        <p>Conectează-te pentru a vedea SFT-urile ChainQuest Idle.</p>
        <LoginButton loginToken={'chainquest-demo'} text={'Conectează xPortal'} />
      </main>
    </DappProvider>
  );
}
