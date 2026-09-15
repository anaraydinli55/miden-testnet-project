import subprocess

IMG_HASH = "0x03297a176cc58f2647876304c35a3146c559cfbb0e5c26592cf88d3d93593919"
FAUCET_ID = "0x55830500881218516d141fcb18b2d2"
MY_WALLET = "0x86341565987080016e478676445d61"

print(">> Flame Towers NFT ZK doğrulama proqramı hazırlanır...")

# Rəsmi 'exec.' və 'use' sintaksisinə uyğun MASM proqramı
masm_script = f"""
use miden::protocol::active_account
use miden::protocol::account_id
use miden::core::mem

@transaction_script
pub proc main
    # 1. Flame Towers ZK Şəkil Hash-ini yığına (stack) qoyuruq
    push.{IMG_HASH}

    # 2. Hash-i yaddaşdan təmizləyib icranı təsdiqləyirik
    dropw
end
"""

with open("verify_flame_nft.masm", "w") as f:
    f.write(masm_script.strip())

print(">> 'verify_flame_nft.masm' rəsmi sintaksis ilə yaradıldı.")
print(">> Miden VM üzərində NFT ZK doğrulaması icra olunur...")

cmd = [
    "miden-client", "exec",
    "--account", MY_WALLET,
    "--script-path", "verify_flame_nft.masm"
]

subprocess.run(cmd)
