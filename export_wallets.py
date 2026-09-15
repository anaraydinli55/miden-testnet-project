import sqlite3
import os

db_path = os.path.expanduser("~/.miden/store.sqlite3")
if not os.path.exists(db_path):
    db_path = ".miden/store.sqlite3"

conn = sqlite3.connect(db_path)
cursor = conn.cursor()

print("=== 1. Kayıtlı Tüm Bech32 (mtst1...) Adresleri ===")
cursor.execute("SELECT DISTINCT address FROM addresses WHERE address IS NOT NULL AND address != '';")
addresses = [row[0] for row in cursor.fetchall()]
print(f"Toplam bulunan adres sayısı: {len(addresses)}")

for idx, addr in enumerate(addresses, 1):
    print(f"{idx}. {addr}")

# Adresleri bir dosyaya da kaydedelim
with open("active_targets.txt", "w") as f:
    for addr in addresses:
        f.write(f"{addr}\n")

print("\nTüm adresler 'active_targets.txt' dosyasına kaydedildi!")
conn.close()
