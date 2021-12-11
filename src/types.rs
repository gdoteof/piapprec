use arrayref::array_ref;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct PublicHash([u8; 32]);

impl From<PublicHash> for Vec<u8> {
    fn from(item: PublicHash) -> Self{
       item.0.to_vec() 
    }
}

impl PublicHash {
    pub fn new(bytes: &[u8]) -> PublicHash{
        let arr = array_ref![bytes,0,32];
        return PublicHash(*arr);
    }
}



#[derive(Copy, Clone, Debug)]
pub struct SecretHash([u8; 32]);

impl From<SecretHash> for Vec<u8> {
    fn from(item: SecretHash) -> Self{
       item.0.to_vec() 
    }
}

impl SecretHash {
    pub fn new(bytes: &[u8]) -> SecretHash{
        let arr = array_ref![bytes,0,32];
        return SecretHash(*arr);
    }

    pub fn to_vec(&self) -> Vec<u8> {
        self.0.to_vec()
    }
}

impl AsRef<[u8]> for SecretHash {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

pub type RevealEpoch = (PublicHash, SecretHash);


