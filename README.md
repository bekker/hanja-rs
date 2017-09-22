# hangeul-rs

[![Build Status](https://travis-ci.org/bekker/hanja-rs.svg?branch=master)](https://travis-ci.org/bekker/hanja-rs)

[![](http://meritbadge.herokuapp.com/hanja)](https://crates.io/crates/hanja)

Korean-Hanja(Chinese character for Korean language) dictionary for Rust.

You can lookup Hanja for a given Hangeul syllable, like most of Korean IMEs do.

All Hanja entries are sorted by the use frequency of each.

Only syllables supported, not words(Hanjaeo).

Hashmap is precompiled fast using [phf](https://github.com/sfackler/rust-phf), taking only a few seconds.

Hanja dictionary and frequency data credited to [libhangul](https://github.com/choehwanjin/libhangul) and its contributors.

```toml
[dependencies]
hanja = "0.1.0"
```

## Usage

```rust
extern crate hanja;

fn main() {
    for &(hanja, desc) in hanja::get('화').unwrap().iter().take(10) {
        println!("{}: {}", hanja, desc);
    }

    let korea_in_hangeul = "대한민국";
    let korea_in_hanja = korea_in_hangeul.chars()
            .map(|c| hanja::get(c).unwrap()[0].0)
            .collect::<String>();
    println!("{}", korea_in_hanja); // 大韓民國
}
```
```
火: 불 화
花: 꽃 화
化: 될 화
話: 말할 화, 이야기 화
和: 화할 화
畵: 그림 화
華: 꽃 화, 빛날 화
貨: 재화 화, 재물 화
禍: 재화 화
禾: 벼 화
大韓民國
```

## Documentation
[Docs.rs](https://docs.rs/hanja/)

## License
Distributed under MIT License
