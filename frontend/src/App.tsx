// @ts-nocheck
import React, { useState, useEffect } from 'react'
import { ethers } from 'ethers'
import './App.css'

const EVM_BRIDGE = '0x90cbAe500C2c008B58656f474d4e35F5B7A7996a'
const MIDEN_BRIDGE = '0x5eb65e512ab979911ec04e6798ead0'
const BANK_CONTRACT_ID = "0xa4a6062a3e32ef311d57f9f00ca71b"
const TIMELOCK_VAULT_ID = "0xb7245ee36bb8a9d1516d7b153f22d9"
const ESCROW_CONTRACT_ID = "0x794d75d9138f2af126b9ebd7d455eb"

const FAUCET_COOLDOWN_MS = 0  // Cooldown devre disi (test icin)

const BRIDGE_ABI = [
  "function mint(bytes32 midenAccountId, uint256 amount, uint256 nonce, address recipient) external",
  "function burn(uint256 amount, bytes32 destMidenAccount) external",
  "function balanceOf(address) view returns (uint256)",
  "function decimals() view returns (uint8)",
  "function name() view returns (string)",
  "function symbol() view returns (string)",
  "event Burn(bytes32 indexed midenAccountId, uint256 amount, uint256 nonce, bytes32 destMidenAccount)"
]

const getMidenWallet = () => window.midenWallet || window.miden || null

function toHexString(val) {
  if (!val) return ''
  if (typeof val === 'string') return val
  if (Array.isArray(val) || val instanceof Uint8Array) {
    return '0x' + Array.from(val).map(b => b.toString(16).padStart(2, '0')).join('')
  }
  return String(val)
}

function formatTimeLeft(ms) {
  const hours = Math.floor(ms / (1000 * 60 * 60))
  const mins = Math.floor((ms % (1000 * 60 * 60)) / (1000 * 60))
  return `${hours}s ${mins}d`
}

function safeGetItem(key) {
  try { return localStorage.getItem(key) } catch (e) { return null }
}
function safeSetItem(key, val) {
  try { localStorage.setItem(key, val) } catch (e) {}
}

function App() {
  const [midenConnected, setMidenConnected] = useState(false)
  const [midenAccount, setMidenAccount] = useState('')
  const [midenError, setMidenError] = useState('')
  const [midenAssets, setMidenAssets] = useState([])

  const [account, setAccount] = useState(null)
  const [balance, setBalance] = useState('0')
  const [signer, setSigner] = useState(null)

  const [activeTab, setActiveTab] = useState('bridge')
  const [loading, setLoading] = useState(false)
  const [txHash, setTxHash] = useState(null)
  const [midenTxHash, setMidenTxHash] = useState(null)
  const [statusMessage, setStatusMessage] = useState('')

  const [burnAmount, setBurnAmount] = useState('1')
  const [depositAmount, setDepositAmount] = useState('')
  const [withdrawAmount, setWithdrawAmount] = useState('')
  const [vaultAmount, setVaultAmount] = useState('100')
  const [escrowAmount, setEscrowAmount] = useState('50')

  const [burnEvents, setBurnEvents] = useState([])
  const [eventsLoading, setEventsLoading] = useState(false)
  const [faucetCooldown, setFaucetCooldown] = useState(0)

  // Miden wallet dinamik faucetId (bech32)
  const defaultFaucetId = 'mtst1arut8ltmq8yxzu2az9x2nsgl0qmrjh86_qr7qqq9wr6w'  // SKS Faucet ID (calisiyor)

  useEffect(() => {
    const mw = getMidenWallet()
    if (!mw) return
    const onConnect = (data) => {
      console.log('Miden connected:', data)
      setMidenConnected(true)
      setMidenAccount(toHexString(data?.address || data?.publicKey || mw.address || mw.publicKey || ''))
      if (mw.requestAssets) {
        mw.requestAssets().then(a => {
          const assets = a?.assets || a || []
          setMidenAssets(assets)
          console.log('Miden assets loaded:', assets)
        }).catch(() => {})
      }
    }
    const onDisconnect = () => {
      setMidenConnected(false)
      setMidenAccount('')
      setMidenAssets([])
    }
    if (mw.on) {
      mw.on('connect', onConnect)
      mw.on('disconnect', onDisconnect)
      mw.on('accountChanged', onConnect)
    }
    if (mw.address || mw.publicKey || mw.isAvailable) {
      setMidenConnected(true)
      setMidenAccount(toHexString(mw.address || mw.publicKey || ''))
      if (mw.requestAssets) {
        mw.requestAssets().then(a => {
          const assets = a?.assets || a || []
          setMidenAssets(assets)
        }).catch(() => {})
      }
    }
    return () => {
      if (mw.off) {
        mw.off('connect', onConnect)
        mw.off('disconnect', onDisconnect)
        mw.off('accountChanged', onConnect)
      }
    }
  }, [])

  useEffect(() => {
    const checkCooldown = () => {
      const last = safeGetItem('sakasena_faucet_last_claim')
      if (!last) { setFaucetCooldown(0); return }
      const elapsed = Date.now() - parseInt(last)
      if (elapsed >= FAUCET_COOLDOWN_MS) { setFaucetCooldown(0) }
      else { setFaucetCooldown(FAUCET_COOLDOWN_MS - elapsed) }
    }
    checkCooldown()
    const interval = setInterval(checkCooldown, 60000)
    return () => clearInterval(interval)
  }, [])

  const loadBurnEvents = async () => {
    if (!window.ethereum) return
    setEventsLoading(true)
    try {
      const provider = new ethers.BrowserProvider(window.ethereum)
      const c = new ethers.Contract(EVM_BRIDGE, BRIDGE_ABI, provider)
      const currentBlock = await provider.getBlockNumber()
      const fromBlock = Math.max(0, currentBlock - 5000)
      const filter = c.filters.Burn()
      const events = await c.queryFilter(filter, fromBlock, currentBlock) || []
      const formatted = (events || []).reverse().map((e, i) => ({
        id: i,
        tx: e.transactionHash,
        amount: ethers.formatUnits(e.args.amount, 18),
        dest: toHexString(e.args.destMidenAccount),
        midenId: toHexString(e.args.midenAccountId),
        nonce: e.args.nonce.toString()
      }))
      setBurnEvents(formatted)
    } catch (err) {
      console.error('Events load error:', err)
    } finally {
      setEventsLoading(false)
    }
  }

  useEffect(() => {
    if (account) loadBurnEvents()
    const interval = setInterval(() => { if (account) loadBurnEvents() }, 30000)
    return () => clearInterval(interval)
  }, [account])

  const handleMidenConnect = async () => {
    setMidenError('')
    const mw = getMidenWallet()
    if (!mw) {
      setMidenError('Miden Wallet extension bulunamadi! Lutfen kurun.')
      return
    }
    try {
      if (mw.connect) await mw.connect({ appName: 'SAKASENA Finance' })
      console.log('Miden connected, address:', mw.address)
      setMidenConnected(true)
      setMidenAccount(toHexString(mw.address || mw.publicKey || ''))
      if (mw.requestAssets) {
        const assetsRes = await mw.requestAssets()
        const assets = assetsRes?.assets || assetsRes || []
        setMidenAssets(assets)
      }
    } catch (err) {
      setMidenError('Miden baglanti hatasi: ' + (err.message || err))
    }
  }

  const handleMidenDisconnect = async () => {
    const mw = getMidenWallet()
    if (mw && mw.disconnect) {
      try { await mw.disconnect() } catch(e){}
    }
    setMidenConnected(false)
    setMidenAccount('')
    setMidenAssets([])
  }

  const connectMetaMask = async () => {
    if (!window.ethereum) { alert('MetaMask yok!'); return }
    try {
      const provider = new ethers.BrowserProvider(window.ethereum)
      await provider.send("eth_requestAccounts", [])
      const s = await provider.getSigner()
      const addr = await s.getAddress()
      setSigner(s)
      setAccount(addr)
      try {
        const c = new ethers.Contract(EVM_BRIDGE, BRIDGE_ABI, provider)
        const bal = await c.balanceOf(addr)
        setBalance(ethers.formatUnits(bal, 18))
      } catch (e) { setBalance('0') }
    } catch (err) { alert('Hata: ' + err.message) }
  }

  const burnTokens = async () => {
    if (!signer) { alert('Once MetaMask bagla!'); return }
    if (!burnAmount || parseFloat(burnAmount) <= 0) { alert('Gecerli miktar gir!'); return }
    setLoading(true)
    setMidenTxHash(null)
    setStatusMessage('MetaMask onayi bekleniyor...')
    try {
      const c = new ethers.Contract(EVM_BRIDGE, BRIDGE_ABI, signer)
      const tx = await c.burn(
        ethers.parseUnits(burnAmount, 18),
        '0x000000000000000000000000000000000078674c6cf5e6a0b109599029c2105f'
      )
      setTxHash(tx.hash)
      setStatusMessage('TX gonderildi: ' + tx.hash.slice(0, 20) + '... Bekleniyor...')
      await tx.wait()
      setStatusMessage('✅ Burn basarili! TX: ' + tx.hash)
      try {
        const provider = new ethers.BrowserProvider(window.ethereum)
        const c2 = new ethers.Contract(EVM_BRIDGE, BRIDGE_ABI, provider)
        const bal = await c2.balanceOf(account)
        setBalance(ethers.formatUnits(bal, 18))
      } catch (e) { setBalance('0') }
      await loadBurnEvents()
    } catch (err) {
      setStatusMessage('❌ Hata: ' + (err.reason || err.message))
    } finally { setLoading(false) }
  }

  const sendMidenTx = async (type, params, description) => {
    const mw = getMidenWallet()
    if (!mw) {
      setStatusMessage('❌ Miden Wallet bulunamadi!')
      return null
    }
    // Eger connect yapilmamissa, once connect et
    if (!mw.address && mw.connect) {
      setStatusMessage('Miden Wallet baglaniyor...')
      await mw.connect({ appName: 'SAKASENA Finance' })
    }
    setTxHash(null)
    setMidenTxHash(null)
    setStatusMessage(description + ' - Miden Wallet onayi bekleniyor...')
    try {
      let result
      if (type === 'send' && mw.requestSend) {
        const sendParams = {
          recipient: params.recipient || params.to || params.address || mw.address,  // bech32 format
          amount: String(params.amount),
          faucetId: params.faucetId || defaultFaucetId
        }
        console.log('requestSend params:', JSON.stringify(sendParams, null, 2))
        result = await mw.requestSend(sendParams)
      } else if (type === 'consume' && mw.requestConsume) {
        // Not ID'si lazim — once consumable notes'lari kontrol et
        let noteId = params.noteId
        if (!noteId && mw.requestConsumableNotes) {
          try {
            const notesRes = await mw.requestConsumableNotes()
            const notes = notesRes?.consumableNotes || []
            console.log('Consumable notes:', notes)
            if (notes.length > 0) {
              noteId = notes[0].id || notes[0].noteId || notes[0].hash || notes[0]
            }
          } catch (e) {
            console.log('requestConsumableNotes failed:', e)
          }
        }
        if (!noteId) {
          setStatusMessage('❌ Tuketilecek note bulunamadi. Once deposit yapin.')
          setLoading(false)
          return null
        }
        const consumeParams = { noteId: String(noteId), amount: String(params.amount || '1') }
        console.log('requestConsume params:', JSON.stringify(consumeParams, null, 2))
        result = await mw.requestConsume(consumeParams)
      } else {
        setStatusMessage('❌ Miden Wallet transaction API bulunamadi.')
        return null
      }

      console.log('=== Miden TX result ===', result)
      if (typeof result === 'object' && result !== null) {
        console.log('Keys:', Object.keys(result))
        for (const k of Object.keys(result)) {
          console.log(`  ${k}:`, result[k], `(type: ${typeof result[k]})`)
        }
      }

      // waitForTransaction Miden Wallet'ta bug'li, kullanma
      let finalTxId = null
      if (result) {
        if (typeof result === 'string') {
          finalTxId = result
        } else if (typeof result === 'object') {
          finalTxId = result.txId || result.transactionId || result.id || result.hash || result.transactionHash
        }
      }

      // Hex hash mi yoksa UUID mi?
      if (finalTxId && typeof finalTxId === 'string' && finalTxId.startsWith('0x') && finalTxId.length > 40) {
        setMidenTxHash(finalTxId)
        setStatusMessage('✅ ' + description + ' basarili!')
      } else if (finalTxId) {
        // UUID - hex hash alinamadi (Miden Wallet bug)
        setMidenTxHash(null)
        setStatusMessage('✅ ' + description + ' basarili! (TX ID: ' + finalTxId + ')')
      } else {
        setStatusMessage('✅ ' + description + ' basarili!')
      }

      // Asset'leri yenile
      if (mw.requestAssets) {
        const assetsRes = await mw.requestAssets()
        const assets = assetsRes?.assets || assetsRes || []
        setMidenAssets(assets)
      }
      return result
    } catch (err) {
      console.error('Miden TX hatasi:', err)
      setStatusMessage('❌ ' + description + ' hatasi: ' + (err.message || err))
      throw err
    }
  }

  const handleDeposit = async () => {
    if (!midenConnected) { alert('Once Miden Wallet bagla!'); return }
    setLoading(true)
    try {
      await sendMidenTx('send', {
        recipient: MIDEN_BRIDGE,
        amount: '1',
        faucetId: defaultFaucetId
      }, 'Bridge Deposit (Miden -> EVM)')
    } finally { setLoading(false) }
  }

  const handleClaimFaucet = async () => {
    if (!midenConnected) { alert('Once Miden Wallet bagla!'); return }
    // Cooldown devre disi (test icin)
    // const last = safeGetItem('sakasena_faucet_last_claim')
    // if (last) {
    //   const elapsed = Date.now() - parseInt(last)
    //   if (elapsed < FAUCET_COOLDOWN_MS) {
    //     setStatusMessage('⏳ Faucet cooldown aktif. Kalan: ' + formatTimeLeft(FAUCET_COOLDOWN_MS - elapsed))
    //     return
    //   }
    // }
    setLoading(true)
    try {
      await sendMidenTx('send', {
        recipient: midenAccount,  // DİKKAT: Bu hex, bech32 olmalı
        amount: '100',
        faucetId: defaultFaucetId
      }, 'Faucet Claim (100 SKS)')
      safeSetItem('sakasena_faucet_last_claim', Date.now().toString())
      setFaucetCooldown(FAUCET_COOLDOWN_MS)
    } finally { setLoading(false) }
  }

  const handleBankDeposit = async () => {
    if (!midenConnected) { alert('Once Miden Wallet bagla!'); return }
    if (!depositAmount) { alert('Miktar gir!'); return }
    setLoading(true)
    try {
      await sendMidenTx('send', {
        recipient: BANK_CONTRACT_ID,
        amount: depositAmount,
        faucetId: defaultFaucetId
      }, 'Bank Deposit (' + depositAmount + ' SKS)')
    } finally { setLoading(false) }
  }

  const handleBankWithdraw = async () => {
    if (!midenConnected) { alert('Once Miden Wallet bagla!'); return }
    if (!withdrawAmount) { alert('Miktar gir!'); return }
    setLoading(true)
    try {
      await sendMidenTx('consume', {
        notes: [],
        account: BANK_CONTRACT_ID,
        amount: withdrawAmount
      }, 'Bank Withdraw (' + withdrawAmount + ' SKS)')
    } finally { setLoading(false) }
  }

  const handleCreateTimeLock = async () => {
    if (!midenConnected) { alert('Once Miden Wallet bagla!'); return }
    setLoading(true)
    try {
      await sendMidenTx('send', {
        recipient: TIMELOCK_VAULT_ID,
        amount: vaultAmount,
        faucetId: defaultFaucetId
      }, 'Time-Lock Vault (' + vaultAmount + ' SKS)')
    } finally { setLoading(false) }
  }

  const handleTriggerEscrow = async () => {
    if (!midenConnected) { alert('Once Miden Wallet bagla!'); return }
    setLoading(true)
    try {
      await sendMidenTx('send', {
        recipient: ESCROW_CONTRACT_ID,
        amount: escrowAmount,
        faucetId: defaultFaucetId
      }, 'P2P Escrow Swap (' + escrowAmount + ' SKS)')
    } finally { setLoading(false) }
  }

  return (
    <div className="app">
      <header>
        <h1>SAKASENA Finance</h1>
        <p>Miden ↔ EVM Sepolia</p>
      </header>

      <div className="wallet">
        <div style={{ display: 'flex', gap: '12px', alignItems: 'center', justifyContent: 'center', flexWrap: 'wrap' }}>
          {!midenConnected ? (
            <button className="btn btn-connect" onClick={handleMidenConnect}>🔗 Select Wallet (Miden)</button>
          ) : (
            <div className="wallet-info">
              <span className="addr">{midenAccount?.slice(0,10)}...{midenAccount?.slice(-4)}</span>
              <button className="btn btn-connect" onClick={handleMidenDisconnect} style={{marginLeft:'8px',fontSize:'12px'}}>Disconnect</button>
            </div>
          )}
          {!account ? (
            <button className="btn btn-connect" onClick={connectMetaMask}>🔗 MetaMask Bagla</button>
          ) : (
            <div className="wallet-info">
              <span className="addr">{account.slice(0,6)}...{account.slice(-4)}</span>
              <span className="bal">{balance} wSKS</span>
            </div>
          )}
        </div>
        {midenError && <p style={{color:'#ef4444',fontSize:'12px',marginTop:'8px'}}>{midenError}</p>}
        {midenAssets.length > 0 && (
          <div style={{ marginTop: '8px', fontSize: '12px', color: '#94a3b8' }}>
            Miden Assets: {midenAssets.map((a, i) => `${a.symbol || a.faucetId?.slice(0,10) || 'Token'}: ${a.amount || a.balance || 0}`).join(', ')}
          </div>
        )}
      </div>

      <div className="cards">
        <div className="card cm">
          <h3>⛓️ Miden</h3>
          <code>{MIDEN_BRIDGE}</code>
          <span className="ok">● {midenConnected ? 'Online' : 'Offline'}</span>
        </div>
        <div className="arr">⟷</div>
        <div className="card ce">
          <h3>⛓️ EVM</h3>
          <code>{EVM_BRIDGE}</code>
          <span className="ok">● {account ? 'Online' : 'Offline'}</span>
        </div>
      </div>

      <div style={{ display: 'flex', gap: '8px', justifyContent: 'center', marginBottom: '20px', flexWrap: 'wrap' }}>
        {['bridge','bank','vault','escrow'].map(tab => (
          <button key={tab} onClick={() => setActiveTab(tab)} style={{
            padding: '10px 20px', borderRadius: '10px', border: 'none', cursor: 'pointer',
            background: activeTab === tab ? '#f59e0b' : '#1e293b',
            color: activeTab === tab ? '#0f172a' : '#94a3b8',
            fontWeight: activeTab === tab ? 'bold' : 'normal',
            textTransform: 'capitalize'
          }}>{tab}</button>
        ))}
      </div>

      {activeTab === 'bridge' && (
        <div style={{ maxWidth: '600px', margin: '0 auto' }}>
          <div style={{ background: '#1e293b', padding: '20px', borderRadius: '16px', marginBottom: '16px' }}>
            <h3 style={{ marginBottom: '12px' }}>🌉 Bridge</h3>
            <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: '16px' }}>
              <div>
                <p style={{ fontSize: '12px', color: '#64748b', marginBottom: '8px' }}>Miden → EVM Deposit</p>
                <button className="btn btn-dep" onClick={handleDeposit} disabled={!midenConnected || loading} style={{ width: '100%' }}>
                  {loading ? '⏳...' : '📥 Deposit'}
                </button>
              </div>
              <div>
                <p style={{ fontSize: '12px', color: '#64748b', marginBottom: '8px' }}>EVM → Miden Withdraw</p>
                <input type="number" placeholder="Burn miktar (wSKS)" value={burnAmount} onChange={e => setBurnAmount(e.target.value)}
                  style={{ width: '100%', padding: '10px', borderRadius: '8px', border: '1px solid #334155', background: '#0f172a', color: '#e2e8f0', marginBottom: '8px' }} />
                <button className="btn btn-wit" onClick={burnTokens} disabled={!account || loading} style={{ width: '100%' }}>
                  {loading ? '⏳...' : '📤 Withdraw'}
                </button>
              </div>
            </div>
          </div>
        </div>
      )}

      {activeTab === 'bank' && (
        <div style={{ maxWidth: '600px', margin: '0 auto' }}>
          <div style={{ background: '#1e293b', padding: '20px', borderRadius: '16px', marginBottom: '16px' }}>
            <h3 style={{ marginBottom: '12px' }}>🏦 SKS Bank</h3>
            <p style={{ fontSize: '12px', color: '#64748b', marginBottom: '16px' }}>Contract: {BANK_CONTRACT_ID}</p>
            <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: '16px' }}>
              <div>
                <input type="number" placeholder="Amount to deposit" value={depositAmount} onChange={e => setDepositAmount(e.target.value)}
                  style={{ width: '100%', padding: '10px', borderRadius: '8px', border: '1px solid #334155', background: '#0f172a', color: '#e2e8f0', marginBottom: '8px' }} />
                <button className="btn btn-dep" onClick={handleBankDeposit} disabled={!midenConnected || loading} style={{ width: '100%' }}>
                  {loading ? '⏳...' : 'Deposit Funds'}
                </button>
              </div>
              <div>
                <input type="number" placeholder="Amount to withdraw" value={withdrawAmount} onChange={e => setWithdrawAmount(e.target.value)}
                  style={{ width: '100%', padding: '10px', borderRadius: '8px', border: '1px solid #334155', background: '#0f172a', color: '#e2e8f0', marginBottom: '8px' }} />
                <button className="btn btn-wit" onClick={handleBankWithdraw} disabled={!midenConnected || loading} style={{ width: '100%' }}>
                  {loading ? '⏳...' : 'Withdraw Funds'}
                </button>
              </div>
            </div>
          </div>
          <div style={{ background: '#1e293b', padding: '20px', borderRadius: '16px' }}>
            <h3 style={{ marginBottom: '12px' }}>🚰 SKS Faucet</h3>
            <p style={{ fontSize: '12px', color: '#64748b', marginBottom: '16px' }}>Faucet (SKS): {defaultFaucetId}</p>
            {/* Cooldown devre disi */}
            <button className="btn btn-dep" onClick={handleClaimFaucet} disabled={!midenConnected || loading} style={{ width: '100%' }}>
              {loading ? '⏳...' : 'Claim 100 SKS from Faucet'}
            </button>
          </div>
        </div>
      )}

      {activeTab === 'vault' && (
        <div style={{ maxWidth: '600px', margin: '0 auto', background: '#1e293b', padding: '20px', borderRadius: '16px' }}>
          <h3 style={{ marginBottom: '12px' }}>🔒 Time-Locked Vesting Vault</h3>
          <p style={{ fontSize: '12px', color: '#64748b', marginBottom: '16px' }}>Contract: {TIMELOCK_VAULT_ID}</p>
          <div style={{ display: 'flex', justifyContent: 'space-between', fontSize: '14px', marginBottom: '16px', padding: '12px', background: '#0f172a', borderRadius: '8px' }}>
            <span style={{ color: '#94a3b8' }}>Vesting Lock Duration:</span>
            <span style={{ color: '#f59e0b', fontWeight: 'bold' }}>50 Blocks (~50 Minutes)</span>
          </div>
          <input type="number" placeholder="Miktar (SKS)" value={vaultAmount} onChange={e => setVaultAmount(e.target.value)}
            style={{ width: '100%', padding: '10px', borderRadius: '8px', border: '1px solid #334155', background: '#0f172a', color: '#e2e8f0', marginBottom: '12px' }} />
          <button className="btn btn-dep" onClick={handleCreateTimeLock} disabled={!midenConnected || loading} style={{ width: '100%' }}>
            {loading ? '⏳...' : 'Create Time-Locked Deposit'}
          </button>
        </div>
      )}

      {activeTab === 'escrow' && (
        <div style={{ maxWidth: '600px', margin: '0 auto', background: '#1e293b', padding: '20px', borderRadius: '16px' }}>
          <h3 style={{ marginBottom: '12px' }}>🤝 Decentralized Escrow Swap</h3>
          <p style={{ fontSize: '12px', color: '#64748b', marginBottom: '16px' }}>Contract: {ESCROW_CONTRACT_ID}</p>
          <input type="number" placeholder="SKS miktar" value={escrowAmount} onChange={e => setEscrowAmount(e.target.value)}
            style={{ width: '100%', padding: '10px', borderRadius: '8px', border: '1px solid #334155', background: '#0f172a', color: '#e2e8f0', marginBottom: '12px' }} />
          <button className="btn btn-dep" onClick={handleTriggerEscrow} disabled={!midenConnected || loading} style={{ width: '100%' }}>
            {loading ? '⏳...' : 'Trigger P2P Escrow Swap'}
          </button>
        </div>
      )}

      {txHash && (
        <div className="txok">
          <p>✅ EVM TX: <a href={`https://sepolia.etherscan.io/tx/${txHash}`} target="_blank" rel="noreferrer">{txHash.slice(0,20)}...</a></p>
        </div>
      )}

      {midenTxHash && midenTxHash.startsWith('0x') && midenTxHash.length > 40 ? (
        <div className="txok" style={{ borderColor: '#8b5cf6', background: 'rgba(139,92,246,0.1)' }}>
          <p>✅ Miden TX: <a href={`https://testnet.midenscan.com/tx/${midenTxHash}`} target="_blank" rel="noreferrer" style={{ color: '#8b5cf6' }}>{midenTxHash.slice(0,20)}...</a></p>
        </div>
      ) : midenTxHash ? (
        <div className="txok" style={{ borderColor: '#f59e0b', background: 'rgba(245,158,11,0.1)' }}>
          <p>⏳ Miden TX: <span style={{ color: '#f59e0b' }}>{midenTxHash}</span> (Onay bekleniyor...)</p>
        </div>
      ) : null}

      {statusMessage && (
        <div style={{ maxWidth: '600px', margin: '20px auto', padding: '16px', background: '#0f172a', border: '1px solid #334155', borderRadius: '12px', fontSize: '13px', color: '#94a3b8' }}>
          {statusMessage}
        </div>
      )}

      <div className="events">
        <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: '12px' }}>
          <h2>🔥 Burn Events</h2>
          <button onClick={loadBurnEvents} disabled={eventsLoading} style={{ background: '#1e293b', border: '1px solid #334155', color: '#94a3b8', padding: '6px 12px', borderRadius: '6px', cursor: 'pointer', fontSize: '12px' }}>
            {eventsLoading ? '⏳...' : '🔄 Refresh'}
          </button>
        </div>
        <table>
          <thead><tr><th>TX</th><th>Amount</th><th>Dest</th><th>Status</th></tr></thead>
          <tbody>
            {burnEvents.length === 0 ? (
              <tr><td colSpan={4} style={{textAlign:'center',color:'#64748b'}}>{eventsLoading ? 'Yukleniyor...' : 'Heniz burn event yok'}</td></tr>
            ) : (
              burnEvents.map((e) => (
                <tr key={e.id}>
                  <td className="m">{e.tx.slice(0,6)}...{e.tx.slice(-4)}</td>
                  <td>{e.amount} wSKS</td>
                  <td className="m">{e.dest.slice(0,6)}...{e.dest.slice(-4)}</td>
                  <td><span className="ok">✅ Confirmed</span></td>
                </tr>
              ))
            )}
          </tbody>
        </table>
      </div>
    </div>
  )
}

export default App
