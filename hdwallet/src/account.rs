use chain_addr::{Address, Discrimination, Kind};
use chain_crypto::PublicKey;
use cryptoxide::ed25519::{self, PUBLIC_KEY_LENGTH, SEED_LENGTH};

pub struct Account {
    seed: [u8; SEED_LENGTH],
}

impl Account {
    pub fn new(entropy: &bip39::Entropy, password: &[u8]) -> Self {
        let mut account = Self::from_seed([0; SEED_LENGTH]);
        crate::keygen::generate_seed(entropy, password, &mut account.seed);
        account
    }

    pub fn from_seed(seed: [u8; SEED_LENGTH]) -> Self {
        Account { seed }
    }

    pub fn public(&self) -> [u8; PUBLIC_KEY_LENGTH] {
        let (_, pk) = ed25519::keypair(&self.seed);
        pk
    }

    pub fn address(&self, discrimination: Discrimination) -> Address {
        let pk = if let Ok(pk) = PublicKey::from_binary(&self.public()) {
            pk
        } else {
            unsafe { std::hint::unreachable_unchecked() }
        };
        let kind = Kind::Account(pk);

        Address(discrimination, kind)
    }
}

impl Drop for Account {
    fn drop(&mut self) {
        cryptoxide::util::secure_memset(&mut self.seed, 0)
    }
}

impl From<[u8; SEED_LENGTH]> for Account {
    fn from(seed: [u8; SEED_LENGTH]) -> Self {
        Self { seed }
    }
}
