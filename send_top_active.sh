#!/bin/bash

cd ~/midenup/miden-testnet-project
miden-client account --default 0x86341565987080016e478676445d61

TARGETS_FILE="top_active_targets.txt"
TOTAL=$(wc -l < "$TARGETS_FILE")
COUNT=0
SUCCESS=0
FAILED=0

echo "==============================================================="
echo "Toplam $TOTAL ədəd TOP aktiv cüzdana 1'er SKS göndərilir..."
echo "==============================================================="

while IFS= read -r TARGET || [ -n "$TARGET" ]; do
  [ -z "$TARGET" ] && continue

  COUNT=$((COUNT + 1))
  echo ""
  echo "[$COUNT/$TOTAL] ==============================================="
  echo ">> Sync olunur və 1 SKS göndərilir -> $TARGET"

  miden-client sync >/dev/null 2>&1

  if echo "y" | miden-client transfer --target "$TARGET" --asset 1::SKS --note-type public; then
    echo ">> [UĞURLU] 1 SKS -> $TARGET"
    SUCCESS=$((SUCCESS + 1))
  else
    echo ">> [XƏTA] $TARGET"
    FAILED=$((FAILED + 1))
  fi

  sleep 1
done < "$TARGETS_FILE"

echo ""
echo "==============================================================="
echo "Bütün göndərişlər tamamlandı!"
echo "Uğurlu: $SUCCESS | Uğursuz: $FAILED"
miden-client sync
