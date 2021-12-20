use piapprec::gen_reveal_chain;
use solana_program::pubkey::Pubkey;
use std::io::{Error, Write}; // bring trait into scope
use std::fs::{File, self};

fn main() -> Result<(), std::io::Error> {

    let inner_secret = b"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA";
    let inner_hash = b"00000000000000000000000000000000";
    let reveal_epochs =  gen_reveal_chain(&inner_hash, &inner_secret, 1000000);

    let len = reveal_epochs.len();
    let mut count =0;

    let mut first_five = vec!();
    let mut last_five = vec!();

    if let Ok(mut file)  = File::create("piapprec.bin")
        {
            for (hash, secret) in reveal_epochs {
                let bytes = file.write(&[hash,secret].concat())?;
                assert_eq!(bytes, 64);

                if count < 5 {
                    println!("count: {}, hash: {}", count, Pubkey::new(&hash).to_string());
                    first_five.push((hash, secret));
                } else if count >= len - 5 {
                    last_five.push((hash,secret));
                }

                count += 1;
            }
            file.flush()?; 
        } else {
            return Err(Error::last_os_error());
        }

        let first_pairs = first_five.into_iter().map( |
            (h,s) |  
                [
                    Pubkey::new(&h).to_string(), 
                    Pubkey::new(&s).to_string(),
                ].join(" ")
        ).reduce(|cur, nxt| cur+ &"\n" + &nxt)
        .unwrap();

        let last_pairs = last_five.into_iter().map( |
            (h,s) |  
                [
                    Pubkey::new(&h).to_string(), 
                    Pubkey::new(&s).to_string(),
                ].join(" ")
        ).reduce(|cur, nxt| cur+ &"\n" + &nxt)
        .unwrap();
        

        let debug_info = format!("Debug info:\nFirstFive:\n{}\nLastFive:\n{}", first_pairs, last_pairs);
        fs::write("piapprec.debug.txt", debug_info.clone())?;
        println!("{}", debug_info);
        println!("Success!");
        Ok(())


}


