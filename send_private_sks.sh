#!/bin/bash

cd ~/midenup/miden-testnet-project

# Varsayılan hesap
miden-client account --default 0x86341565987080016e478676445d61

# MidenScan-dan seçilən aktiv testnet cüzdanları
TARGETS=(
  "mtst1arpcmvvf9y99r5gd34pyynynnynx2790"
  "mtst1aqvpq8a9ytqhfvt9al20wzsrs56g83ec"
  "mtst1azm83hvhskp9jy2fk6k67q8gf5gzhjkh"
  "mtst1apnc56sdll470yg6ewujua9nncsxpncv"
  "mtst1aqpj69hppmzzcs2f0psmcdevwcck5qj5"
  "mtst1apt8ea6x3vhddsf8q8wlrnpe2yru3v2m"
  "mtst1azrrg9t9npcgqqtwg7r8v3zavygw4h5t"
  "mtst1azmzve9f30xmrqgexw2hh3phnyka3l8k"
  "mtst1aznyl6hgy87n452sm0p5nscwxu7vra94"
  "mtst1apu9jlmj26xndstn6fjdplvqjqj653a0"
  "mtst1azmtjkr573atkqgd2mc6acsvyyypqpkr"
  "mtst1arwzlnwkuk8z4sf302t2wsu9kukcdxxa"
  "mtst1azca3svqjavnaq2kjxht9l8y4vrzjx3j"
  "mtst1arj73z7u8h993q2sq7ursc3y6gvh27ja"
  "mtst1azm64e0d23cguqf7dxm7we8hgu65wu25"
  "mtst1azs3gxlsdupf05gan5c4sj5xu5f6q9qx"
  "mtst1apvpadhyl3q925t4cnm5q2lufgunc6qe"
  "mtst1azlhrru8x3pl4sg32nz0g7dcgcnjxkln"
  "mtst1ar5v8zyzaenuq5tptcl5ax74jv7d66qn"
)

TOTAL=${#TARGETS[@]}
COUNT=0
SUCCESS=0
FAILED=0

echo "==============================================================="
echo "Toplam $TOTAL adet cüzdana 1'er adet PRIVATE SKS gönderimi başlatılıyor..."
echo "==============================================================="

for TARGET in "${TARGETS[@]}"; do
  COUNT=$((COUNT + 1))

  echo ""
  echo "[$COUNT/$TOTAL] ==============================================="
  echo ">> Sync yapılıyor ve ZK Gizli Not üretiliyor -> $TARGET"

  # Güncel blok durumunu al
  miden-client sync >/dev/null 2>&1

  # Private SKS transfer
  if echo "y" | miden-client transfer \
      --target "$TARGET" \
      --asset 1::SKS \
      --note-type private; then

    echo ">> [BAŞARILI - PRIVATE] 1 SKS -> $TARGET"
    SUCCESS=$((SUCCESS + 1))

  else

    echo ">> [HATA ALINDI] $TARGET"
    FAILED=$((FAILED + 1))

  fi

  sleep 1
done

echo ""
echo "==============================================================="
echo "Toplu Private SKS Gönderimi Tamamlandı!"
echo "Başarılı: $SUCCESS | Başarısız: $FAILED"
echo "==============================================================="

echo "Son senkronizasyon yapılıyor..."
miden-client sync

echo ""
echo "Güncel Cüzdan Durumun:"
miden-client account -s 0x86341565987080016e478676445d61
