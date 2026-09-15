import hashlib
import json

BASE_IMG_HASH = "0x03297a176cc58f2647876304c35a3146c559cfbb0e5c26592cf88d3d93593919"
TOTAL_SUPPLY = 10000

print(f">> Baku Flame Towers {TOTAL_SUPPLY} ədədlik kolleksiyası hazırlanır...")

collection = []

for i in range(1, TOTAL_SUPPLY + 1):
    # Əsas şəkil hash-i ilə seriya nömrəsini kriptoqrafik olaraq bağlayırıq
    data = f"{BASE_IMG_HASH}:{i}".encode('utf-8')
    edition_hash = "0x" + hashlib.sha256(data).hexdigest()
    
    collection.append({
        "edition": i,
        "name": f"Baku Flame Towers #{i}",
        "base_image": BASE_IMG_HASH,
        "nft_hash": edition_hash
    })

# Bütün kolleksiyanı JSON faylına yazırıq
with open("flame_towers_10k.json", "w") as f:
    json.dump(collection, f, indent=2)

print(f">> [UĞURLU] flame_towers_10k.json faylında {TOTAL_SUPPLY} unikal NFT yaradıldı!")
print(f">> Nümunə #1: {collection[0]['nft_hash']}")
print(f">> Nümunə #10000: {collection[-1]['nft_hash']}")
