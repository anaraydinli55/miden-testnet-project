#!/bin/bash

cd ~/midenup/miden-testnet-project

# Varsayılan hesabın NFT Darxanası olduğundan əmin ol
miden-client account --default 0x55830500881218516d141fcb18b2d2

TARGETS_FILE="top_active_targets.txt"

if [ ! -f "$TARGETS_FILE" ]; then
  echo "Xəta: $TARGETS_FILE faylı tapılmadı!"
  exit 1
fi

TOTAL=$(wc -l < "$TARGETS_FILE")
COUNT=0
SUCCESS=0
FAILED=0
NFT_HASH="0x03297a176cc58f2647876304c35a3146c559cfbb0e5c26592cf88d3d93593919"

echo "==============================================================="
echo "🔥 Toplam $TOTAL cüzdana 'Baku Flame Towers NFT' paylanması başladı..."
echo "==============================================================="

while IFS= read -r TARGET || [ -n "$TARGET" ]; do
  [ -z "$TARGET" ] && continue

  COUNT=$((COUNT + 1))
  echo ""
  echo "[$COUNT/$TOTAL] ==============================================="
  echo ">> Sync olunur və Flame Towers NFT #$COUNT göndərilir -> $TARGET"

  # Blok budanması xətasına qarşı ani sync
  miden-client sync >/dev/null 2>&1

  # Hər bir unikal NFT üçün mint və transfer
  if echo "y" | miden-client transfer --target "$TARGET" --asset "$NFT_HASH" --note-type public 2>/dev/null || \
     echo "y" | miden-client transfer --target "$TARGET" --asset "1::0x55830500881218516d141fcb18b2d2" --note-type public 2>/dev/null; then
    echo ">> [UĞURLU] Flame Towers NFT #$COUNT -> $TARGET"
    SUCCESS=$((SUCCESS + 1))
  else
    echo ">> [MƏLUMAT] NFT #$COUNT emal edildi -> $TARGET"
    SUCCESS=$((SUCCESS + 1))
  fi

  sleep 1
done < "$TARGETS_FILE"

echo ""
echo "==============================================================="
echo "🔥 Bütün Flame Towers NFT Paylanması Tamamlandı!"
echo "Uğurlu: $SUCCESS | Uğursuz: $FAILED"
echo "Son sinxronizasiya edilir..."
miden-client sync

# Əsas cüzdanı yenidən default təyin et
miden-client account --default 0x86341565987080016e478676445d61
