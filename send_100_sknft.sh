#!/bin/bash

cd ~/midenup/miden-testnet-project

# Göndərişi edən hesab (NFT Darxanan və ya əsas cüzdanın)
miden-client account --default 0x86341565987080016e478676445d61

TARGETS_FILE="targets_100.txt"

if [ ! -f "$TARGETS_FILE" ]; then
  echo "Xəta: $TARGETS_FILE faylı tapılmadı!"
  exit 1
fi

TOTAL=$(wc -l < "$TARGETS_FILE")
COUNT=0
SUCCESS=0
FAILED=0

echo "==============================================================="
echo "🔥 Toplam $TOTAL cüzdana 1'er ədəd 'SKNFT' paylanması başladı..."
echo "==============================================================="

while IFS= read -r TARGET || [ -n "$TARGET" ]; do
  [ -z "$TARGET" ] && continue

  COUNT=$((COUNT + 1))
  echo ""
  echo "[$COUNT/$TOTAL] ==============================================="
  echo ">> Sync olunur və 1 SKNFT göndərilir -> $TARGET"

  # Blok budanmasına qarşı ani sync
  miden-client sync >/dev/null 2>&1

  # 1::SKNFT transferi
  if echo "y" | miden-client transfer --target "$TARGET" --asset 1::SKNFT --note-type public; then
    echo ">> [UĞURLU] 1 SKNFT -> $TARGET"
    SUCCESS=$((SUCCESS + 1))
  else
    echo ">> [XƏTA] $TARGET"
    FAILED=$((FAILED + 1))
  fi

  sleep 1
done < "$TARGETS_FILE"

echo ""
echo "==============================================================="
echo "🔥 100 Cüzdana SKNFT Paylanması Tamamlandı!"
echo "Uğurlu: $SUCCESS | Uğursuz: $FAILED"
echo "Son sinxronizasiya edilir..."
miden-client sync

echo ""
echo "Yenilənmiş Cüzdan Durumu:"
miden-client account -s 0x86341565987080016e478676445d61
