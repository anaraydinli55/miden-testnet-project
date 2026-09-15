#!/bin/bash

# Əsas cüzdanı varsayılan təyin et
miden-client account --default 0x86341565987080016e478676445d61

TARGETS_FILE="top_active_targets.txt"

if [ ! -f "$TARGETS_FILE" ]; then
  echo "Xəta: $TARGETS_FILE faylı tapılmadı!"
  exit 1
fi

TOTAL=$(wc -l < "$TARGETS_FILE")
COUNT=0
SUCCESS=0
FAILED=0

echo "==============================================================="
echo "🔥 Toplam $TOTAL aktiv cüzdana paylanma başladı..."
echo "==============================================================="

while IFS= read -r TARGET || [ -n "$TARGET" ]; do
  [ -z "$TARGET" ] && continue

  COUNT=$((COUNT + 1))
  echo ""
  echo "[$COUNT/$TOTAL] ==============================================="
  echo ">> Sync olunur və göndərilir -> $TARGET"

  # Blok budanması xətasına qarşı ani sync
  miden-client sync >/dev/null 2>&1

  # Otomatik 'y' ilə transfer
  if echo "y" | miden-client transfer --target "$TARGET" --asset 1::SKS --note-type public; then
    echo ">> [UĞURLU] Göndərildi -> $TARGET"
    SUCCESS=$((SUCCESS + 1))
  else
    echo ">> [XƏTA] $TARGET"
    FAILED=$((FAILED + 1))
  fi

  sleep 1
done < "$TARGETS_FILE"

echo ""
echo "==============================================================="
echo "🔥 Bütün cüzdanlara paylanma tamamlandı!"
echo "Uğurlu: $SUCCESS | Uğursuz: $FAILED"
echo "Son sinxronizasiya edilir və gələn notlar qəbul edilir..."
miden-client sync
miden-client consume-notes
miden-client sync

echo ""
echo "Yenilənmiş Cüzdan Durumu:"
miden-client account -s 0x86341565987080016e478676445d61
