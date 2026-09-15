import subprocess

IMG_HASH = "0x03297a176cc58f2647876304c35a3146c559cfbb0e5c26592cf88d3d93593919"
FAUCET_WORD = "0x000000000000000000000000000000000055830500881218516d141fcb18b2d2"
RECIPIENT_WORD = "0x000000000000000000000000000000000086341565987080016e478676445d61"

print(">> Flame Towers NFT mintləmə əməliyyatı hazırlanır...")

# v0.16 rəsmi 'use miden::...' sintaksisi
masm_script = f"""
use miden::standards::components::faucets::non_fungible_faucet

@transaction_script
pub proc main
    # 1. Alıcı cüzdan
    push.{RECIPIENT_WORD}

    # 2. Not növü (1 = Public) və Tag (0)
    push.1
    push.0

    # 3. Flame Towers ZK Şəkil Hash-i
    push.{IMG_HASH}

    # 4. NFT Darxana ID-si
    push.{FAUCET_WORD}

    # 5. Prosedur çağırışı
    call.non_fungible_faucet::mint_and_send
    dropw dropw dropw dropw
end
"""

with open("mint_flame_nft.masm", "w") as f:
    f.write(masm_script.strip())

print(">> 'mint_flame_nft.masm' v0.16 boşluqlu 'use' sintaksisi ilə yeniləndi.")
print(">> Miden VM üzərində NFT mint icra olunur...")

cmd = [
    "miden-client", "exec",
    "--account", "0x55830500881218516d141fcb18b2d2",
    "--script-path", "mint_flame_nft.masm"
]

subprocess.run(cmd)
