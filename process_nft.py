import hashlib
import json
import os
import glob

# 1. Windows Masaüstü və Yükləmələr qovluğunda şəkli axtarırıq
search_patterns = [
    "/mnt/c/Users/*/Desktop/*flame*.png",
    "/mnt/c/Users/*/Desktop/*flame*.jpg",
    "/mnt/c/Users/*/Downloads/*flame*.png",
    "/mnt/c/Users/*/OneDrive/Desktop/*flame*.png"
]

found_file = None
for pattern in search_patterns:
    matches = glob.glob(pattern)
    if matches:
        found_file = matches[0]
        break

if found_file:
    print(f">> Şəkil Windows-da tapıldı: {found_file}")
    with open(found_file, "rb") as src, open("flame-towers-miden-nft.png", "wb") as dst:
        dst.write(src.read())
    print(">> Şəkil layihə qovluğuna kopyalandı.")

if not os.path.exists("flame-towers-miden-nft.png"):
    print("Xəbərdarlıq: 'flame-towers-miden-nft.png' hələ tapılmadı, zəhmət olmasa fayl adını dəqiqləşdirin.")
    exit(1)

# 2. Şəklin SHA-256 ZK Hash-ini hesablayırıq
with open("flame-towers-miden-nft.png", "rb") as f:
    img_bytes = f.read()

sha256_hash = hashlib.sha256(img_bytes).hexdigest()

print("\n=======================================================")
print(f"🔥 Kolleksiya: Baku Flame Towers Miden NFT")
print(f"📦 Şəkil Ölçüsü: {len(img_bytes) / 1024:.2f} KB")
print(f"🔑 Dəyişməz ZK Şəkil Hash-i: 0x{sha256_hash}")
print("=======================================================\n")

# 3. Kolleksiya metadata faylını yaradırıq
metadata = {
    "name": "Baku Flame Towers Miden NFT Collection",
    "symbol": "SKNFT",
    "description": "Exclusive Miden ZK Testnet NFT representing Baku Flame Towers",
    "image_hash": f"0x{sha256_hash}",
    "faucet_id": "0x55830500881218516d141fcb18b2d2"
}

with open("nft_collection_metadata.json", "w") as f:
    json.dump(metadata, f, indent=2)

print(">> 'nft_collection_metadata.json' faylı uğurla yaradıldı.")
