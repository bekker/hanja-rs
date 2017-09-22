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
