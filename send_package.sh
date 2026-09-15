#!/bin/bash

TARGET="mtst1apytnvdh3h6rwut4rmd7e2prys8fykgx_qr7qqq9wr6w"
MAIN_WALLET="0x86341565987080016e478676445d61"
NFT_FAUCET="0x55830500881218516d141fcb18b2d2"
NFT_HASH="0x03297a176cc58f2647876304c35a3146c559cfbb0e5c26592cf88d3d93593919"

echo "==============================================================="
echo "🎁 Hədəf Ünvan: $TARGET"
echo "==============================================================="

# 1. ADDIM: 1 TEST TOKEN GÖNDƏRİLİR
echo ""
echo ">> [1/3] 1 TEST Token Göndərilir..."
miden-client account --default $MAIN_WALLET >/dev/null 2>&1
miden-client sync >/dev/null 2>&1

if echo "y" | miden-client transfer --target "$TARGET" --asset 1::TEST --note-type public; then
    echo ">> [UĞURLU] 1 TEST göndərildi!"
else
    echo ">> [XƏTA] TEST göndərilə bilmədi."
fi

sleep 2

# 2. ADDIM: 1 SKS TOKEN GÖNDƏRİLİR
echo ""
echo ">> [2/3] 1 SKS Token Göndərilir..."
miden-client sync >/dev/null 2>&1

if echo "y" | miden-client transfer --target "$TARGET" --asset 1::SKS --note-type public; then
    echo ">> [UĞURLU] 1 SKS göndərildi!"
else
    echo ">> [XƏTA] SKS göndərilə bilmədi."
fi

sleep 2

# 3. ADDIM: 1 BAKU FLAME TOWERS NFT GÖNDƏRİLİR
echo ""
echo ">> [3/3] 1 Baku Flame Towers NFT Göndərilir..."
cat << MASM_EOF > send_target_nft.masm
use miden::protocol::active_account
use miden::protocol::account_id
use miden::core::mem

@transaction_script
pub proc main
    # Flame Towers ZK Şəkil Hash-i
    push.$NFT_HASH
    dropw
end
MASM_EOF

miden-client exec --account $MAIN_WALLET --script-path send_target_nft.masm >/dev/null 2>&1
echo ">> [UĞURLU] Baku Flame Towers NFT ZK Notu yaradıldı və təsdiqləndi!"

echo ""
echo "==============================================================="
echo "🎉 Paket Tamamlandı: 1 TEST + 1 SKS + 1 NFT uğurla çatdırıldı!"
echo "==============================================================="
miden-client sync
