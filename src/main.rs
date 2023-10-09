use clap::Parser as 해독기;

use aheui::가명::*;
use aheui::격자::해독;

#[derive(해독기, 디버그)]
#[command(author, version, about)]
struct 실행인자 {
    프로그램_위치: 문자열,
}

fn main() {
    let 인자 = 실행인자::parse();
    let 프로그램 =
        std::fs::read_to_string(인자.프로그램_위치).expect("프로그램을 불러올 수 없습니다.");
    println!("{:?}", 해독::격자로_읽기(프로그램));
}
