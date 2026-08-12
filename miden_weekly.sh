#!/bin/bash

# Miden Çoklu Varlık Haftalık Aktivite Otomasyonu
PROJECT_DIR="$HOME/midenup/miden-testnet-project"
SENDER_ACCOUNT="0x17aa90766d756b81398a9bf0bd485e"
TARGET_ACCOUNT="0x2458e5446128e6b150b75b8ebd9ce1"

echo "========================================="
echo "  Miden Coklu Varlik Aktivite Otomasyonu"
echo "========================================="

# Proje dizinine geçiş
cd "$PROJECT_DIR" || { echo "Hata: Proje klasörü bulunamadı!"; exit 1; }

# Sistem yollarını doğrula
export PATH="$HOME/.cargo/bin:$PATH"

echo "Adım 1: İstemci canlı ağla senkronize ediliyor..."
miden client sync

echo ""
echo "Adım 2: 1 MIDEN hedef hesaba gönderiliyor..."
miden client send --target "$TARGET_ACCOUNT" --asset 1::MIDEN --note-type public --force

echo ""
echo "Adım 3: 1 SKS (SAKASENA) hedef hesaba gönderiliyor..."
miden client send --target "$TARGET_ACCOUNT" --asset 1::SKS --note-type public --force

echo ""
echo "Adım 4: İşlemleri onaylamak için tekrar senkronize ediliyor..."
miden client sync

echo ""
echo "Adım 5: Güncel bakiye ve hesap detayları listeleniyor..."
miden client account -s "$SENDER_ACCOUNT"

echo "========================================="
echo "  Çoklu Varlık Aktivitesi Başarıyla Bitti!"
echo "========================================="
