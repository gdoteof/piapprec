use sha2::{Sha256, Digest};

type PubKey = [u8; 32];

/*
    Generates a reveal chain with `length` epochs.  This includes the genesis epoch made of `initial_public` and `initial_secret`

    Epochs should be revealed from the beginning of the Vec<RevealEpoch> to the end.
    Tuple is (Public,Secret)
*/
pub fn gen_reveal_chain(initial_public : &PubKey, initial_secret : &PubKey, length: usize) -> Vec<(PubKey,PubKey)> {
    let (mut prev_public, mut prev_secret) = (*initial_public, *initial_secret);

    let mut pairs: Vec<(PubKey,PubKey)> = vec!();
    pairs.push((prev_public, prev_secret));

    for _ in 0..length - 1{
        let to_hash = mk_hash_frame(prev_public, prev_secret);

        let next_public = Sha256::digest(to_hash);
        let next_secret = Sha256::digest(prev_secret);

        pairs.push((next_public.into(),next_secret.into()));

        prev_public = next_public.into();
        prev_secret = next_secret.into();
    }
    pairs.reverse();
    pairs
}

pub fn verify(prev_hash: &PubKey, new_hash: &PubKey, new_secret: &PubKey) -> bool{
        *prev_hash == Sha256::digest( 
                mk_hash_frame(*new_hash, *new_secret)
            ).as_slice()
}

fn mk_hash_frame(public: PubKey, secret: PubKey) -> Vec<u8> {
        let mut to_hash: Vec<u8> = secret.into();
        let back_half: Vec<u8> = public.into();
        to_hash.extend(back_half);
        to_hash
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let inner_secret = b"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA";
        let inner_hash = b"00000000000000000000000000000000";
        let reveal_epochs =  gen_reveal_chain(&inner_hash, &inner_secret, 420);
        let mut epoch_iter = reveal_epochs.iter();

        let (mut prev_hash, _) = epoch_iter.next().unwrap();

        while let Some((new_hash, new_secret)) = epoch_iter.next() {
            assert_eq!(true, verify(&prev_hash, new_hash, new_secret));
            prev_hash = *new_hash;
        }
    }

    #[test]
    fn gen_reveal_chain_returns_expected_number_of_epochs() {
        let inner_secret = b"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA";
        let inner_hash = b"00000000000000000000000000000000";
        let reveal_epochs =  gen_reveal_chain(&inner_hash, &inner_secret, 10);
        assert_eq!(reveal_epochs.len(),10 )
    }
}
