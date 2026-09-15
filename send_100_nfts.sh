#!/bin/bash

cd ~/midenup/miden-testnet-project
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
echo "🔥 Toplam $TOTAL aktiv cüzdana Flame Towers paylanması başladı..."
echo "==============================================================="

while IFS= read -r TARGET || [ -n "$TARGET" ]; do
  [ -z "$TARGET" ] && continue

  COUNT=$((COUNT + 1))
  echo ""
  echo "[$COUNT/$TOTAL] ==============================================="
  echo ">> Sync olunur və Flame Towers #$COUNT göndərilir -> $TARGET"

  # Blok budanması xətasına qarşı ani sync
  miden-client sync >/dev/null 2>&1

  # Avtomatik 'y' ilə transfer
  if echo "y" | miden-client transfer --target "$TARGET" --asset 1::SKS --note-type public; then
    echo ">> [UĞURLU] Flame Towers #$COUNT -> $TARGET"
    SUCCESS=$((SUCCESS + 1))
  else
    echo ">> [XƏTA] $TARGET"
    FAILED=$((FAILED + 1))
  fi

  sleep 1
done < "$TARGETS_FILE"

echo ""
echo "==============================================================="
echo "🔥 100 Cüzdana Paylanma Uğurla Tamamlandı!"
echo "Uğurlu: $SUCCESS | Uğursuz: $FAILED"
echo "Son sinxronizasiya edilir..."
miden-client sync

echo ""
echo "Yenilənmiş Cüzdan Vəziyyətin:"
miden-client account -s 0x86341565987080016e478676445d61
