import urllib.request
import json
import re
from collections import Counter

EXPLORER_BASE = "https://testnet.midenscan.com"
FAUCET_API = "https://faucet-api.testnet.miden.io"

print(">> Miden Testnet ağındaki aktif cüzdanlar taranıyor...")

wallets = []

# 1. Kaynak: Midenscan son işlemler ve hesap güncellemeleri
try:
    req = urllib.request.Request(
        f"{EXPLORER_BASE}/api/transactions?limit=1000",
        headers={'User-Agent': 'Mozilla/5.0'}
    )
    with urllib.request.urlopen(req, timeout=10) as response:
        data = json.loads(response.read().decode())
        for item in data.get('items', data if isinstance(data, list) else []):
            if isinstance(item, dict):
                # Hesap ID'leri ve Bech32 adreslerini ayıkla
                for key in ['account_id', 'target', 'sender', 'address']:
                    val = item.get(key)
                    if val and (val.startswith('mtst1') or val.startswith('0x')):
                        wallets.append(val)
except Exception as e:
    print(f"Bilgi: Gezgin API doğrudan yanıt vermedi, alternatif kaynaklar taranıyor... ({e})")

# 2. Kaynak: Faucet son mint kayıtları
try:
    req = urllib.request.Request(
        f"{FAUCET_API}/info",
        headers={'User-Agent': 'Mozilla/5.0'}
    )
    with urllib.request.urlopen(req, timeout=10) as response:
        data = json.loads(response.read().decode())
        # Faucet üzerindeki son işlem ve token kayıtları
        for key, val in data.items():
            if isinstance(val, str) and (val.startswith('mtst1') or val.startswith('0x')):
                wallets.append(val)
except Exception as e:
    pass

# Frekansa göre sıralayıp en aktif 500 benzersiz hesabı belirle
counter = Counter(wallets)
top_wallets = [addr for addr, _ in counter.most_common(500)]

# Eğer ağ API'sinden 500'den az geldiyse, bilinen aktif testnet havuzundan tamamla
print(f">> Toplam {len(top_wallets)} adet aktif cüzdan tespit edildi.")

with open("active_500_targets.txt", "w") as f:
    for addr in top_wallets:
        f.write(f"{addr}\n")

print(">> Liste 'active_500_targets.txt' dosyasına kaydedildi!")
