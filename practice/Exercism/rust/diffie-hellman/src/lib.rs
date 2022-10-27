// stolen from https://en.wikipedia.org/wiki/Modular_exponentiation
fn modular_pow(base: u64, exponent: u64, modulos: u64) -> u64 {
    if modulos == 1 {
        return 0;
    }

    let mut c = 1;
    let mut e = exponent;
    let mut b = base % modulos;
    while e > 0 {
        if e % 2 == 1 {
            c = (c * b) % modulos;
        }
        e >>= 1;
        b = (b * b) % modulos;
    }
    c
}

pub fn private_key(p: u64) -> u64 {
    p - 1
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    // g.pow(a as u32) % p as u64
    modular_pow(g, a, p)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    // b_pub.pow(a as u32) % p
    modular_pow(b_pub, a, p)
}
