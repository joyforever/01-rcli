use rand::seq::SliceRandom;
use zxcvbn::zxcvbn;

const UPEER: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZ";
const LOWER: &[u8] = b"abcdefghijkmnopqrstuvwxyz";
const NUMBER: &[u8] = b"123456789";
const SYMBOL: &[u8] = b"!@#$%^&*_";

pub fn process_genpass(
    length: u8,
    no_uppercase: bool,
    no_lowercase: bool,
    no_number: bool,
    no_symbol: bool,
) -> anyhow::Result<()> {
    let mut rng = rand::thread_rng();
    let mut password = Vec::with_capacity(length as usize);

    let mut chars = Vec::new();
    if !no_uppercase {
        chars.extend_from_slice(UPEER);
        password.push(*UPEER.choose(&mut rng).unwrap());
    }
    if !no_lowercase {
        chars.extend_from_slice(LOWER);
        password.push(*LOWER.choose(&mut rng).unwrap());
    }
    if !no_number {
        chars.extend_from_slice(NUMBER);
        password.push(*NUMBER.choose(&mut rng).unwrap());
    }
    if !no_symbol {
        chars.extend_from_slice(SYMBOL);
        password.push(*SYMBOL.choose(&mut rng).unwrap());
    }

    let n = length - password.len() as u8;
    for _ in 0..n {
        password.push(*chars.choose(&mut rng).unwrap());
    }

    password.shuffle(&mut rng);

    let password = String::from_utf8(password)?;
    println!("{}", password);

    let estimate = zxcvbn(&password, &[])?;
    eprintln!("password strength: {}", estimate.score());

    Ok(())
}
