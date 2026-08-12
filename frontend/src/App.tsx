import { useState } from 'react';
import { Wallet, Coins, Lock, Landmark, CheckCircle, RefreshCw, ArrowRightLeft } from 'lucide-react';

// Deployed Contract Adreslerimiz (Dün ve bugün canlı ağa aldığımız resmi adresler!)
const BANK_CONTRACT_ID = "0xa4a6062a3e32ef311d57f9f00ca71b";
const TIMELOCK_VAULT_ID = "0xb7245ee36bb8a9d1516d7b153f22d9";
const ESCROW_CONTRACT_ID = "0x794d75d9138f2af126b9ebd7d455eb";
const SKS_FAUCET_ID = "0x31dc7ded4087c4b12f63eb9ed65aac";

export default function App() {
  const [walletConnected, setWalletConnected] = useState(false);
  const [walletAddress, setWalletConnectedAddress] = useState("");
  const [activeTab, setActiveTab] = useState<'bank' | 'vault' | 'escrow'>('bank');
  const [depositAmount, setDepositAmount] = useState("");
  const [withdrawAmount, setWithdrawAmount] = useState("");
  const [statusMessage, setStatusMessage] = useState("");
  const [isProcessing, setIsProcessing] = useState(false);

  // 1. Miden Wallet Chrome Eklentisini Bağlama Fonksiyonu (Wallet Connection)
  const connectWallet = async () => {
    setIsProcessing(true);
    try {
      // @ts-ignore (Miden eklentisi tarayıcıya pencere seviyesinde bir provider enjekte eder)
      if (window.miden) {
        // @ts-ignore
        const accounts = await window.miden.request({ method: 'miden_requestAccounts' });
        setWalletConnectedAddress(accounts[0]);
        setWalletConnected(true);
        setStatusMessage("Miden Wallet connected successfully!");
      } else {
        setStatusMessage("Miden Wallet extension not found. Please install the extension!");
      }
    } catch (error) {
      setStatusMessage("Failed to connect wallet.");
    } finally {
      setIsProcessing(false);
    }
  };

  // 2. Bankaya Para Yatırma (SKS Token Deposit Note Üretimi)
  const handleBankDeposit = async () => {
    if (!depositAmount) return;
    setIsProcessing(true);
    setStatusMessage("Compiling ZK Deposit Note and signing transaction...");
    
    try {
      // Gerçek dApp'lerde arka planda @miden-sdk/miden-sdk şu şekilde not üretir:
      /*
      const txRequest = new TransactionRequestBuilder()
        .ownOutputNotes([depositNote])
        .build();
      await window.miden.request({ method: 'miden_sendTransaction', params: [txRequest] });
      */
      
      // Simülasyon Geri Bildirimi
      setTimeout(() => {
        setStatusMessage(`Successfully minted Deposit Note of ${depositAmount} SKS! Deployed to Bank Account: ${BANK_CONTRACT_ID}`);
        setIsProcessing(false);
      }, 3000);
    } catch (error) {
      setStatusMessage("Transaction failed.");
      setIsProcessing(false);
    }
  };

  return (
    <div className="min-h-screen bg-slate-950 text-slate-100 flex flex-col font-sans">
      {/* Header */}
      <header className="border-b border-slate-800 bg-slate-900/50 backdrop-blur px-6 py-4 flex items-center justify-between">
        <div className="flex items-center space-x-3">
          <Coins className="h-8 w-8 text-amber-500 animate-pulse" />
          <span className="text-xl font-bold tracking-wider bg-gradient-to-r from-amber-400 to-orange-500 bg-clip-text text-transparent">
            SAKASENA (SKS) PORTAL
          </span>
        </div>
        
        {walletConnected ? (
          <div className="flex items-center space-x-3 bg-slate-800/80 px-4 py-2 rounded-full border border-slate-700">
            <div className="h-2 w-2 rounded-full bg-emerald-500 animate-ping" />
            <span className="text-xs font-mono text-slate-300">
              {walletAddress.slice(0, 12)}...{walletAddress.slice(-6)}
            </span>
          </div>
        ) : (
          <button 
            onClick={connectWallet}
            disabled={isProcessing}
            className="flex items-center space-x-2 bg-gradient-to-r from-amber-500 to-orange-600 hover:from-amber-600 hover:to-orange-700 text-white font-medium px-5 py-2.5 rounded-full transition-all duration-300 shadow-lg shadow-orange-500/20 active:scale-95"
          >
            <Wallet className="h-4 w-4" />
            <span>Connect Miden Wallet</span>
          </button>
        )}
      </header>

      {/* Main Content */}
      <main className="flex-1 max-w-4xl w-full mx-auto p-6 md:p-12 flex flex-col space-y-8">
        
        {/* Network Info & SKS Faucet Card */}
        <div className="bg-slate-900/40 border border-slate-800 p-6 rounded-2xl flex flex-col md:flex-row items-center justify-between gap-6 backdrop-blur">
          <div className="space-y-2 text-center md:text-left">
            <h2 className="text-lg font-semibold text-slate-200">SAKASENA (SKS) Testnet Token Faucet</h2>
            <p className="text-xs text-slate-400 font-mono">Faucet Account ID: {SKS_FAUCET_ID}</p>
          </div>
          <button 
            disabled={!walletConnected || isProcessing}
            className="w-full md:w-auto bg-slate-800 hover:bg-slate-700 text-slate-200 border border-slate-700 hover:border-slate-600 px-6 py-3 rounded-xl transition duration-200 disabled:opacity-40 disabled:cursor-not-allowed text-sm font-medium"
          >
            Claim 100 SKS from Faucet
          </button>
        </div>

        {/* Navigation Tabs */}
        <div className="flex space-x-2 bg-slate-900/60 p-1.5 rounded-xl border border-slate-800/80">
          <button 
            onClick={() => setActiveTab('bank')}
            className={`flex-1 flex items-center justify-center space-x-2 py-3 rounded-lg text-sm font-medium transition-all duration-200 ${activeTab === 'bank' ? 'bg-amber-500 text-slate-950 font-semibold shadow-md' : 'text-slate-400 hover:text-slate-200'}`}
          >
            <Landmark className="h-4 w-4" />
            <span>SKS Bank</span>
          </button>
          <button 
            onClick={() => setActiveTab('vault')}
            className={`flex-1 flex items-center justify-center space-x-2 py-3 rounded-lg text-sm font-medium transition-all duration-200 ${activeTab === 'vault' ? 'bg-amber-500 text-slate-950 font-semibold shadow-md' : 'text-slate-400 hover:text-slate-200'}`}
          >
            <Lock className="h-4 w-4" />
            <span>Time-Lock Vault</span>
          </button>
          <button 
            onClick={() => setActiveTab('escrow')}
            className={`flex-1 flex items-center justify-center space-x-2 py-3 rounded-lg text-sm font-medium transition-all duration-200 ${activeTab === 'escrow' ? 'bg-amber-500 text-slate-950 font-semibold shadow-md' : 'text-slate-400 hover:text-slate-200'}`}
          >
            <ArrowRightLeft className="h-4 w-4" />
            <span>P2P Escrow Swap</span>
          </button>
        </div>

        {/* Tab Panels */}
        <div className="bg-slate-900/20 border border-slate-800/80 rounded-3xl p-6 md:p-8 min-h-[300px] backdrop-blur flex flex-col justify-between">
          
          {activeTab === 'bank' && (
            <div className="space-y-6">
              <div className="space-y-2">
                <h3 className="text-xl font-bold flex items-center space-x-2">
                  <Landmark className="text-amber-500 h-5 w-5" />
                  <span>SAKASENA Decentralized Bank</span>
                </h3>
                <p className="text-xs text-slate-500 font-mono">Contract ID: {BANK_CONTRACT_ID}</p>
                <p className="text-sm text-slate-400">Deposit SKS tokens privately into the bank. The bank contract will track your balance securely in its internal storage slot.</p>
              </div>

              <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
                {/* Deposit Box */}
                <div className="bg-slate-900/40 border border-slate-800/60 p-6 rounded-2xl flex flex-col justify-between space-y-4">
                  <span className="text-sm font-semibold text-slate-300">Deposit SKS</span>
                  <input 
                    type="number" 
                    placeholder="Amount to deposit (e.g. 50)" 
                    value={depositAmount}
                    onChange={(e) => setDepositAmount(e.target.value)}
                    className="bg-slate-950 border border-slate-800 focus:border-amber-500 outline-none rounded-xl px-4 py-3 text-sm transition font-mono"
                  />
                  <button 
                    onClick={handleBankDeposit}
                    disabled={!walletConnected || isProcessing}
                    className="bg-amber-500 hover:bg-amber-600 disabled:opacity-40 disabled:cursor-not-allowed text-slate-950 font-semibold py-3 rounded-xl transition duration-200 text-sm"
                  >
                    Deposit Funds
                  </button>
                </div>

                {/* Withdraw Box */}
                <div className="bg-slate-900/40 border border-slate-800/60 p-6 rounded-2xl flex flex-col justify-between space-y-4">
                  <span className="text-sm font-semibold text-slate-300">Withdraw SKS</span>
                  <input 
                    type="number" 
                    placeholder="Amount to withdraw" 
                    value={withdrawAmount}
                    onChange={(e) => setWithdrawAmount(e.target.value)}
                    className="bg-slate-950 border border-slate-800 focus:border-amber-500 outline-none rounded-xl px-4 py-3 text-sm transition font-mono"
                  />
                  <button 
                    disabled={!walletConnected || isProcessing}
                    className="bg-slate-800 hover:bg-slate-700 disabled:opacity-40 disabled:cursor-not-allowed text-slate-200 font-semibold py-3 rounded-xl border border-slate-700 transition duration-200 text-sm"
                  >
                    Withdraw Funds
                  </button>
                </div>
              </div>
            </div>
          )}

          {activeTab === 'vault' && (
            <div className="space-y-6">
              <div className="space-y-2">
                <h3 className="text-xl font-bold flex items-center space-x-2">
                  <Lock className="text-amber-500 h-5 w-5" />
                  <span>Time-Locked Vesting Vault</span>
                </h3>
                <p className="text-xs text-slate-500 font-mono">Contract ID: {TIMELOCK_VAULT_ID}</p>
                <p className="text-sm text-slate-400">Lock your SKS tokens under a Zero-Knowledge time lock. The funds will remain absolutely un-withdrawable until the target block height is mined on Miden Testnet.</p>
              </div>

              <div className="bg-slate-900/40 border border-slate-800/60 p-6 rounded-2xl flex flex-col space-y-4">
                <div className="flex justify-between items-center text-sm border-b border-slate-800 pb-3">
                  <span className="text-slate-400">Vesting Lock Duration:</span>
                  <span className="font-mono text-amber-500 font-semibold">50 Blocks (~50 Minutes)</span>
                </div>
                <button 
                  disabled={!walletConnected || isProcessing}
                  className="bg-amber-500 hover:bg-amber-600 disabled:opacity-40 disabled:cursor-not-allowed text-slate-950 font-semibold py-3.5 rounded-xl transition duration-200 text-sm shadow-md"
                >
                  Create Time-Locked Deposit (100 SKS)
                </button>
              </div>
            </div>
          )}

          {activeTab === 'escrow' && (
            <div className="space-y-6">
              <div className="space-y-2">
                <h3 className="text-xl font-bold flex items-center space-x-2">
                  <ArrowRightLeft className="text-amber-500 h-5 w-5" />
                  <span>Decentralized Escrow Swap</span>
                </h3>
                <p className="text-xs text-slate-500 font-mono">Contract ID: {ESCROW_CONTRACT_ID}</p>
                <p className="text-sm text-slate-400">Perform completely private peer-to-peer asset trades. Party A deposits SKS, Party B deposits MIDEN, and once both conditions are met on-chain, the swap executes securely.</p>
              </div>

              <div className="bg-slate-900/40 border border-slate-800/60 p-6 rounded-2xl flex flex-col space-y-4">
                <button 
                  disabled={!walletConnected || isProcessing}
                  className="bg-amber-500 hover:bg-amber-600 disabled:opacity-40 disabled:cursor-not-allowed text-slate-950 font-semibold py-3.5 rounded-xl transition duration-200 text-sm shadow-md"
                >
                  Trigger P2P Escrow Swap (50 SKS ⇋ 5 MIDEN)
                </button>
              </div>
            </div>
          )}

          {/* Status Bar */}
          {statusMessage && (
            <div className="mt-8 p-4 bg-slate-950 border border-slate-800/80 rounded-xl flex items-start space-x-3 text-xs text-slate-400 font-mono animate-fade-in">
              {isProcessing ? <RefreshCw className="h-4 w-4 text-amber-500 animate-spin flex-shrink-0" /> : <CheckCircle className="h-4 w-4 text-emerald-500 flex-shrink-0" />}
              <span>{statusMessage}</span>
            </div>
          )}

        </div>
      </main>

      {/* Footer */}
      <footer className="border-t border-slate-900 py-6 text-center text-xs text-slate-600 font-mono">
        © 2026 SAKASENA Protocol. Built on Polygon Miden zkVM Testnet.
      </footer>
    </div>
  );
}