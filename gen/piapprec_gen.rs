use piapprec::gen_reveal_chain;
use std::io::{Error, Write}; // bring trait into scope
use std::fs::{File};

fn main() -> Result<(), std::io::Error> {

    let inner_secret = b"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA";
    let inner_hash = b"00000000000000000000000000000000";
    let reveal_epochs =  gen_reveal_chain(&inner_hash, &inner_secret, 1000000);

    if let Ok(mut file)  = File::create("piapprec.bin")
        {
            for (hash, secret) in reveal_epochs {
                let bytes = file.write(&[hash,secret].concat())?;
                assert_eq!(bytes, 64);
            }
            file.flush()?; 
            Ok(())
        } else {
            Err(Error::last_os_error())
        }
}


