use bip39::{Language, Mnemonic, Seed};
use bitcoin::util::bip32::ExtendedPubKey;
use bitcoin::{
    network::constants::Network,
    util::bip32::{ExtendedPrivKey, ChildNumber},
};
use hdpath::CustomHDPath;
use secp256k1::Secp256k1;
use dotenv::dotenv;

fn get_extended_keypair(
    seed: &[u8],
    hd_path: &CustomHDPath,
) -> (ExtendedPrivKey, ExtendedPubKey) {
    let secp = Secp256k1::new();
    let pk = ExtendedPrivKey::new_master(Network::Bitcoin, seed)
        // we convert HD Path to bitcoin lib format (DerivationPath)
        .and_then(|k| k.derive_priv(&secp, &convert_to_vec_child_number(hd_path)))
        .unwrap();
    let pubk = ExtendedPubKey::from_private(&secp, &pk);

    (pk, pubk)
}

fn convert_to_vec_child_number(value: &CustomHDPath) -> Vec<ChildNumber> {
    let mut result: Vec<ChildNumber> = Vec::with_capacity(value.0.len());
    for item in value.0.iter() {
        result.push(ChildNumber::from(item.to_raw()))
    }
    return result;
}

fn main() {
    dotenv().ok();

    let mnemonic_str: String = std::env::var("MNEMONIC").expect("MNEMONIC does not exist");
    let mnemonic_reference: &str = &mnemonic_str;

    // ----------------------------------------------------------------------------- 1 mnemonic
    let mnemonic = Mnemonic::from_phrase(mnemonic_reference, Language::English).unwrap();

    // ----------------------------------------------------------------------------- 3 derived addr
    // get the HD wallet seed
    let seed = Seed::new(&mnemonic, ""); //128 hex chars = 512 bits
    let seed_bytes: &[u8] = seed.as_bytes();

    let hd_path = CustomHDPath::try_from("m/44'/118'/0'/0/0").unwrap();
    let (_pk, _) = get_extended_keypair(&seed_bytes, &hd_path);


    let path = home::home_dir().unwrap().join(".orga-wallet").join("privkey");

    std::fs::write(&path, _pk.private_key.to_bytes()).unwrap();
}


