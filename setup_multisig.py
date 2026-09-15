import subprocess

# Anahtar indekslerini [<felt>, 0, 0, 0] kuralına göre ilk sıraya alıyoruz
toml_content = """
"miden::standards::auth::multisig::threshold_config.threshold" = "2"
"miden::standards::auth::multisig::threshold_config.num_approvers" = "3"

"miden::standards::auth::multisig::approver_public_keys" = [
    { key = ["0", "0", "0", "0"], value = "0x0000000000000000000000000000000000000000000000000000000000000001" },
    { key = ["1", "0", "0", "0"], value = "0x0000000000000000000000000000000000000000000000000000000000000002" },
    { key = ["2", "0", "0", "0"], value = "0x0000000000000000000000000000000000000000000000000000000000000003" }
]
"""

with open("multisig_init_data.toml", "w") as f:
    f.write(toml_content.strip())

print(">> multisig_init_data.toml u32 uyumlu olarak güncellendi.")
print(">> 2-of-3 Multi-Sig cüzdanı oluşturuluyor...")

cmd = [
    "miden-client", "new-account",
    "-t", "public",
    "-p", "/home/anaraydinli/.miden/packages/auth/multisig-auth.masp",
    "-p", "/home/anaraydinli/.miden/packages/basic-wallet.masp",
    "-i", "multisig_init_data.toml"
]

subprocess.run(cmd)
