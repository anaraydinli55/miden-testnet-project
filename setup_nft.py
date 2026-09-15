import subprocess

toml_content = """
# Sembol (SKNFT -> Hex formatında felt)
"miden::standards::faucets::non_fungible::symbol.symbol" = "0x534b4e4654"

# İsim Word Alanları (token_name_0 ve token_name_1)
"miden::standards::faucets::token_name_0.data_0" = "0"
"miden::standards::faucets::token_name_0.data_1" = "0"
"miden::standards::faucets::token_name_0.data_2" = "0"
"miden::standards::faucets::token_name_0.data_3" = "0"
"miden::standards::faucets::token_name_1.data_0" = "0"
"miden::standards::faucets::token_name_1.data_1" = "0"
"miden::standards::faucets::token_name_1.data_2" = "0"
"miden::standards::faucets::token_name_1.data_3" = "0"

# Mutability Konfigürasyonu (7 boolean alan)
"miden::standards::faucets::mutability_config.is_description_mutable" = "false"
"miden::standards::faucets::mutability_config.is_logo_uri_mutable" = "false"
"miden::standards::faucets::mutability_config.is_contract_uri_mutable" = "false"
"miden::standards::faucets::mutability_config.is_external_link_mutable" = "false"
"miden::standards::faucets::mutability_config.is_max_supply_mutable" = "false"
"miden::standards::faucets::mutability_config.is_token_name_mutable" = "false"
"miden::standards::faucets::mutability_config.is_symbol_mutable" = "false"
"""

# Boş metadata ve link alanları
for i in range(7):
    for d in range(4):
        toml_content += f'"miden::standards::faucets::external_link_{i}.data_{d}" = "0"\n'
        toml_content += f'"miden::standards::faucets::logo_uri_{i}.data_{d}" = "0"\n'
        toml_content += f'"miden::standards::faucets::token_description_{i}.data_{d}" = "0"\n'

with open("nft_init_data.toml", "w") as f:
    f.write(toml_content.strip())

print(">> nft_init_data.toml felt uyumlu olarak hazırlandı.")
print(">> NFT Faucet hesabı oluşturuluyor...")

cmd = [
    "miden-client", "new-account",
    "-t", "public",
    "-p", "/home/anaraydinli/.miden/packages/basic-wallet.masp",
    "-p", "/home/anaraydinli/.miden/packages/basic-non-fungible-faucet.masp",
    "-i", "nft_init_data.toml"
]

subprocess.run(cmd)
