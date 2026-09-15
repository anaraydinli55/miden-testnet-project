#!/bin/bash

# Varsayılan hesabın ana cüzdanın olduğundan emin ol
miden-client account --default 0x86341565987080016e478676445d61

TARGETS_FILE="active_1000_targets.txt"

if [ ! -f "$TARGETS_FILE" ]; then
  echo "Hata: $TARGETS_FILE bulunamadı!"
  exit 1
fi

TOTAL=$(wc -l < "$TARGETS_FILE")
COUNT=0
SUCCESS=0
FAILED=0

echo "==============================================================="
echo "Toplam $TOTAL adet cüzdana 1'er SKS dağıtımı başlatılıyor..."
echo "==============================================================="

while IFS= read -r TARGET || [ -n "$TARGET" ]; do
  # Boş satırları atla
  [ -z "$TARGET" ] && continue

  COUNT=$((COUNT + 1))
  echo ""
  echo "[$COUNT/$TOTAL] ==============================================="
  echo ">> Sync yapılıyor ve 1 SKS gönderiliyor -> $TARGET"

  # Blok yenileme
  miden-client sync >/dev/null 2>&1

  # Otomatik 'y' onayı ile 1 SKS transferi
  if echo "y" | miden-client transfer --target "$TARGET" --asset 1::SKS --note-type public; then
    echo ">> [BAŞARILI] 1 SKS -> $TARGET"
    SUCCESS=$((SUCCESS + 1))
  else
    echo ">> [HATA ALINDI] $TARGET"
    FAILED=$((FAILED + 1))
  fi

  sleep 1
done < "$TARGETS_FILE"

echo ""
echo "==============================================================="
echo "Tüm 1.000 Transfer Tamamlandı!"
echo "Başarılı: $SUCCESS | Başarısız: $FAILED"
echo "Son senkronizasyon yapılıyor..."
miden-client sync

echo ""
echo "Güncel Cüzdan Durumun:"
miden-client account -s 0x86341565987080016e478676445d61
