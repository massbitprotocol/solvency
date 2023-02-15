use ethers_core::types::Address;
//use merkletree::merkle::{Element, MerkleTree};
pub const SIZE: usize = 0x10;
/// Account of the Ethereum State Trie, which contains an in-memory key-value
/// database that represents the Account Storage Trie.
#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
pub struct Account {
    pub address: Address,
}

impl Account {
    pub fn as_bytes(&self) -> &[u8] {
        self.address.as_bytes()
    }
}
// impl AsRef<[u8]> for Account {
//     fn as_ref(&self) -> &[u8] {

//     }
// }

// impl Element for Account {
//     fn byte_len() -> usize {
//         SIZE
//     }
//     fn from_slice(bytes: &[u8]) -> Self {
//         assert_eq!(bytes.len(), Self::byte_len());
//         let mut el = [0u8; SIZE];
//         el[..].copy_from_slice(bytes);
//         Account(el)
//     }
//     fn copy_to_slice(&self, bytes: &mut [u8]) {
//         bytes.copy_from_slice(&self.0);
//     }
// }
