//! Cryptanalysis of the Engima in Rust.

pub mod enigma;
pub mod plugboard;
pub mod reflector;
pub mod rotor;


trait CharIndex {
    fn index(&self) -> usize;
}

impl CharIndex for char {
    fn index(&self) -> usize {
        *self as usize - 65
    }
}


trait ToChar {
    fn to_char(&self) -> char;
}

impl ToChar for usize {
    fn to_char(&self) -> char {
        (*self as u8 + 65) as char
    }
}
