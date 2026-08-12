import { StrictMode } from 'react';
import { createRoot } from 'react-dom/client';
import App from './App.tsx';
import './index.css';

// Miden Wallet Adapter ve Resmi Stil dosyasını içe aktarıyoruz
import { WalletProvider, WalletModalProvider, MidenWalletAdapter } from '@miden-sdk/miden-wallet-adapter';
import '@miden-sdk/miden-wallet-adapter/styles.css'; // <-- Stil dosyası eklendi!

const wallets = [
  new MidenWalletAdapter({ appName: 'SAKASENA SKS Portal' }),
];

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <WalletProvider wallets={wallets} autoConnect={true}>
      {/* Bağlantı penceresi sağlayıcısını ekledik */}
      <WalletModalProvider>
        <App />
      </WalletModalProvider>
    </WalletProvider>
  </StrictMode>
);