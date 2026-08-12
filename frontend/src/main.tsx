import { StrictMode } from 'react';
import { createRoot } from 'react-dom/client';
import App from './App.tsx';
import './index.css';

// Miden Wallet Adapter kütüphanelerini içe aktarıyoruz
import { WalletProvider, MidenWalletAdapter } from '@miden-sdk/miden-wallet-adapter';

// SAKASENA Portal uygulamamız için adaptörü tanımlıyoruz
const wallets = [
  new MidenWalletAdapter({ appName: 'SAKASENA SKS Portal' }),
];

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    {/* Uygulamamızı Miden Wallet sağlayıcısı ile sarmalıyoruz */}
    <WalletProvider wallets={wallets} autoConnect={true}>
      <App />
    </WalletProvider>
  </StrictMode>
);