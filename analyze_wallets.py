import sqlite3
import os

db_path = os.path.expanduser("~/.miden/store.sqlite3")

if not os.path.exists(db_path):
    # Yerel proje dizinine de bak
    db_path = ".miden/store.sqlite3"

if not os.path.exists(db_path):
    print("Veritabanı dosyası bulunamadı!")
    exit(1)

conn = sqlite3.connect(db_path)
cursor = conn.cursor()

print("=== Veritabanındaki Tablolar ===")
cursor.execute("SELECT name FROM sqlite_master WHERE type='table';")
tables = [t[0] for t in cursor.fetchall()]
print(tables)

# Hesaplar ve işlem kayıtlarını listele
for table in tables:
    cursor.execute(f"PRAGMA table_info({table});")
    columns = [c[1] for c in cursor.fetchall()]
    print(f"\nTablo: {table} -> Sütunlar: {columns}")

conn.close()
