use bip39::{Language, Mnemonic, Seed};
use bitcoin::util::bip32::{ExtendedPubKey, IntoDerivationPath, DerivationPath};
use bitcoin::{
    network::constants::Network,
    util::bip32::ExtendedPrivKey,
};
use secp256k1::Secp256k1;
use dotenv::dotenv;

fn get_extended_keypair(
    seed: &[u8],
    hd_path: DerivationPath,
) -> (ExtendedPrivKey, ExtendedPubKey) {
    let secp = Secp256k1::new();
    let pk = ExtendedPrivKey::new_master(Network::Bitcoin, seed)
            .unwrap()
            .derive_priv(&secp, &hd_path)
            .unwrap();
    let pubk = ExtendedPubKey::from_private(&secp, &pk);
    (pk, pubk)
}

fn main() {
    dotenv().ok();

    let hd_path = "m/44'/118'/0'/0/0".into_derivation_path().unwrap();

    let mnemonic_str: String = std::env::var("MNEMONIC").expect("MNEMONIC does not exist");
    let mnemonic_reference: &str = &mnemonic_str;

    let mnemonic = Mnemonic::from_phrase(mnemonic_reference, Language::English).unwrap();

    let seed = Seed::new(&mnemonic, ""); //128 hex chars = 512 bits
    let seed_bytes: &[u8] = seed.as_bytes();

    // let hd_path = CustomHDPath::try_from("m/44'/118'/0'/0/0").unwrap();
    let (_pk, _) = get_extended_keypair(&seed_bytes, hd_path);


    let path = home::home_dir().unwrap().join(".orga-wallet").join("privkey");

    std::fs::write(&path, _pk.private_key.to_bytes()).unwrap();
}


