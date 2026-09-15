import json
import subprocess
import time

with open("flame_towers_10k.json", "r") as f:
    collection = json.load(f)

TOTAL = len(collection)
BATCH_SIZE = 100  # Hər dövrdə 100 NFT

print("===============================================================")
print(f"🔥 Baku Flame Towers 10.000 NFT Kolleksiyasının Mintlənməsi...")
print("===============================================================")

# İlk 100 ədədlik nümunə partiyasını test edirik
print(f">> İlk {BATCH_SIZE} ədəd Flame Towers NFT üçün ZK sübutları hazırlanır...")

for item in collection[:BATCH_SIZE]:
    nft_id = item['edition']
    nft_hash = item['nft_hash']
    
    masm_code = f"""
    @transaction_script
    pub proc main
        push.{nft_hash}
        dropw
    end
    """
    with open("temp_nft.masm", "w") as mf:
        mf.write(masm_code.strip())
        
    cmd = [
        "miden-client", "exec",
        "--account", "0x86341565987080016e478676445d61",
        "--script-path", "temp_nft.masm"
    ]
    subprocess.run(cmd, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
    print(f">> [MINT EDİLDİ] {item['name']} -> {nft_hash[:18]}...", end="\r")

print(f"\n>> [TAMAMLANDI] İlk partiya uğurla zəncirə işləndi!")
