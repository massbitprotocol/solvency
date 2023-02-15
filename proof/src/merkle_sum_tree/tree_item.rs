use crate::Field;

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug)]
pub struct TreeItem<F: Field> {
    pub hash: F,
    pub sum: u64,
}
impl<F: Field> Default for TreeItem<F> {
    fn default() -> Self {
        Self {
            hash: F::zero(),
            sum: 0u64,
        }
    }
}
impl<F: Field> TreeItem<F> {
    pub fn new(hash: F, sum: u64) -> Self {
        Self { hash, sum }
    }
    pub fn from_hash(hash: F) -> Self {
        Self { hash, sum: 0u64 }
    }
}

impl<F: Field> Into<Vec<u8>> for TreeItem<F> {
    fn into(self) -> Vec<u8> {
        let mut buffer = vec![];
        buffer.extend_from_slice(&self.sum.to_le_bytes());
        buffer.extend_from_slice(&self.hash.to_repr());
        buffer
    }
}
impl<F: Field> TryFrom<Vec<u8>> for TreeItem<F> {
    type Error = super::Error;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        if let (Ok(sum), Ok(hash)) = (
            value[0..8]
                .try_into()
                .map(|bytes| u64::from_le_bytes(bytes)),
            value[8..]
                .try_into()
                .map(|bytes| F::from_repr(bytes).unwrap()),
        ) {
            Ok(Self { hash, sum })
        } else {
            Err(Self::Error::new(
                super::ErrorKind::HashConversionError,
                String::from("Data size is not matched"),
            ))
        }
    }
}

// impl TreeItem {
//     pub fn from_bytes(bytes: [u8; SIZE]) -> Self {
//         Self(bytes)
//     }
// }
// impl Into<Fp> for TreeItem {
//     fn into(self) -> Fp {
//         let mut values = [0u64; 4];
//         let mut cur_ind = 0;
//         for i in 0..4 {
//             let mut bytes = [0u8; 8];
//             for j in 0..8 {
//                 bytes[j] = self.0[cur_ind];
//                 cur_ind += 1;
//             }
//             values[i] = u64::from_le_bytes(bytes);
//         }
//         Fp::from_raw(values)
//     }
// }

// impl TryFrom<[u8; SIZE]> for TreeItem {
//     type Error = Error;

//     fn try_from(value: [u8; SIZE]) -> Result<Self, Self::Error> {
//         Ok(Self(value))
//     }
// }
// impl From<&Account> for TreeItem {
//     fn from(account: &Account) -> TreeItem {
//         let bytes = account.as_bytes();
//         let mut buffers = [0; SIZE];
//         for i in 0..bytes.len() {
//             if i < SIZE {
//                 buffers[i] = bytes[i]
//             }
//         }
//         TreeItem(buffers)
//     }
// }
// impl From<&Address> for TreeItem {
//     fn from(address: &Address) -> TreeItem {
//         let bytes = address.as_bytes();
//         let mut buffers = [0; SIZE];
//         for i in 0..bytes.len() {
//             if i < SIZE {
//                 buffers[i] = bytes[i]
//             }
//         }
//         TreeItem(buffers)
//     }
// }
// impl AsRef<[u8]> for TreeItem {
//     fn as_ref(&self) -> &[u8] {
//         &self.0
//     }
// }
