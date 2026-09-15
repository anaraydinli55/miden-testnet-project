import urllib.request
import json
import re
import time
from collections import Counter

print("===============================================================")
print(">> Miden Testnet ağındaki aktif 1.000 cüzdan taranıyor...")
print("===============================================================")

HEADERS = {'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64)'}
wallets = []

# Daha önce bildiğimiz doğrulanmış testnet adreslerini taban havuz olarak ekle
known_base = [
  "mtst1azrrg9t9npcgqqtwg7r8v3zavygw4h5t", "mtst1azrf4dga3ztqxy20rl6gj85yn5l8ugud",
  "mtst1apgrw530c2wkfyt34askwpjv5swyg8rl", "mtst1arl88vck8dfsqyfxzq770h29v5ste7ra",
  "mtst1apwgn32fyzdklyg4kg0kaheafyxpg9tx", "mtst1aq03vjuwzfj765fjlk9wqg3xevfkzpnf",
  "mtst1arn2gyl5hmcle522zm8rmj67zs8n05vf", "mtst1aqu3dl6rtrh0h52ypadh3gatsv5vmx23",
  "mtst1artrgqc7mlp2v5gljeteest7lgakawe8", "mtst1arxzfura892maytw94qj58ejfgmkpnla",
  "mtst1azspzac049p34y2348e4e7kh85xk0vs8", "mtst1aqzjdk6z4ryd55g0har3tpkh8yff9avm",
  "mtst1ar8t0ewtnrw7ryg9sc2mvhw00sfvwn0j", "mtst1aqskts2yspu48qfjanm9ytv3wu7qwd2l",
  "mtst1az5a76djngnzp52s4kgclttfnsvxegt5", "mtst1arvdwvzllvg3s5fzjle7nkljeuhkcufr",
  "mtst1apqk2y2uky2mkyfcjv95fjm5zgnrwk6x", "mtst1arcf9xpxfrc7wygpv744ytgr6cw2df6h",
  "mtst1arqxg9er3xclayt95nud82jnpggl9azj", "mtst1ardp776ey96mjqfdqalxm00gmvnatkre",
  "mtst1aq39uk9nslcsvq2732h9naperc20nyt6", "mtst1apfrparjrupvvqtz3skgcs3l8cjkxlpn",
  "mtst1apgzfjdyn3u4us25jxwms8u3wcex9src", "mtst1aqd9xsca4zu08q20rhke84t3evszp3rk"
]
wallets.extend(known_base)

def fetch_json(url):
    try:
        req = urllib.request.Request(url, headers=HEADERS)
        with urllib.request.urlopen(req, timeout=5) as resp:
            return json.loads(resp.read().decode())
    except Exception:
        return None

# 1. MidenScan Sayfalarını Tara (Son İşlemler & Bloklar)
print(">> Midenscan gezgini taranıyor...")
for page in range(1, 30):
    url = f"https://testnet.midenscan.com/api/transactions?page={page}&limit=50"
    data = fetch_json(url)
    if not data:
        break
    
    items = data.get('items', data if isinstance(data, list) else [])
    for item in items:
        if isinstance(item, dict):
            for k in ['account_id', 'target', 'sender', 'recipient', 'address']:
                v = item.get(k)
                if v and isinstance(v, str) and (v.startswith('mtst1') or v.startswith('0x')):
                    wallets.append(v)
    
    if len(set(wallets)) >= 1000:
        break
    time.sleep(0.1)

# 2. MidenScan Hesap Listesi ve Blok Başlıkları
print(">> Blok ve hesap verileri taranıyor...")
for page in range(1, 20):
    url = f"https://testnet.midenscan.com/api/accounts?page={page}&limit=50"
    data = fetch_json(url)
    if not data:
        break
    items = data.get('items', data if isinstance(data, list) else [])
    for item in items:
        if isinstance(item, dict):
            acc = item.get('id') or item.get('address') or item.get('account_id')
            if acc and isinstance(acc, str):
                wallets.append(acc)

# 3. Faucet API İşlem Kayıtları
faucet_info = fetch_json("https://faucet-api.testnet.miden.io/info")
if faucet_info and isinstance(faucet_info, dict):
    for k, v in faucet_info.items():
        if isinstance(v, str) and (v.startswith('mtst1') or v.startswith('0x')):
            wallets.append(v)

# Tekrarları temizle ve frekansa göre sırala
counter = Counter(wallets)
unique_active = [addr for addr, _ in counter.most_common()]

print(f">> Ağdan toplanan benzersiz aktif cüzdan sayısı: {len(unique_active)}")

# 1000 adede tamamla
final_targets = unique_active[:1000]

# Dosyaya kaydet
with open("active_1000_targets.txt", "w") as f:
    for target in final_targets:
        f.write(f"{target}\n")

print(f"===============================================================")
print(f"Başarılı! {len(final_targets)} adet aktif cüzdan 'active_1000_targets.txt' dosyasına kaydedildi.")
print("===============================================================")
