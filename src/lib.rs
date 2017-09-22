#![doc(include = "README.md")]

extern crate phf;

#[cfg(test)]
mod tests {
    use super::lookup_hangeul;
    #[test]
    fn it_works() {
        match lookup_hangeul('가').unwrap()[0] {
            (hanja, _) => assert_eq!(hanja, '可')
        }
    }
}

include!(concat!(env!("OUT_DIR"), "/data.rs"));

pub fn get(hangeul: char) -> Option<&'static [(char, &'static str)]> {
    ENTRIES.get(&hangeul).cloned()
}
