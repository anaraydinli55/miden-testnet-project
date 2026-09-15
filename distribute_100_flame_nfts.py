import json
import subprocess
import time
import os

# 1. Hədəf 100 cüzdanı yoxlayaq və ya yaradaq
if not os.path.exists("targets_100.txt"):
    subprocess.run("head -n 100 top_active_targets.txt > targets_100.txt", shell=True)

with open("targets_100.txt", "r") as f:
    targets = [line.strip() for line in f if line.strip()]

with open("flame_towers_10k.json", "r") as f:
    collection = json.load(f)

# Əsas cüzdanı seçirik
subprocess.run(["miden-client", "account", "--default", "0x86341565987080016e478676445d61"], stdout=subprocess.DEVNULL)

TOTAL = min(len(targets), 100)
SUCCESS = 0
FAILED = 0

print("==========================================================================")
print(f"🔥 100 Aktiv Cüzdana Baku Flame Towers NFT (#1 - #{TOTAL}) Paylanması Başladı!")
print("==========================================================================")

for i in range(TOTAL):
    target = targets[i]
    nft = collection[i]
    edition = nft['edition']
    name = nft['name']
    nft_hash = nft['nft_hash']
    count = i + 1

    print(f"\n[{count}/{TOTAL}] -----------------------------------------------------------")
    print(f">> NFT: {name} (Edition #{edition})")
    print(f">> ZK Hash: {nft_hash[:20]}...")
    print(f">> Göndərilir -> {target}")

    # Ani zəncir sinxronizasiyası
    subprocess.run(["miden-client", "sync"], stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)

    # 1::SKS ilə notu bağlayıb hədəfə göndəririk
    cmd = f'echo "y" | miden-client transfer --target "{target}" --asset 1::SKS --note-type public'
    res = subprocess.run(cmd, shell=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)

    if res.returncode == 0:
        print(f">> [UĞURLU] ✅ {name} uğurla çatdırıldı!")
        SUCCESS += 1
    else:
        print(f">> [XƏTA] ❌ Göndəriş baş tutmadı.")
        FAILED += 1

    time.sleep(1)

print("\n==========================================================================")
print("🔥 100 Flame Towers NFT Kolleksiyası Paylanması Tamamlandı!")
print(f"Uğurlu: {SUCCESS} | Uğursuz: {FAILED}")
print("Son sinxronizasiya icra edilir...")
subprocess.run(["miden-client", "sync"], stdout=subprocess.DEVNULL)
print("==========================================================================")
