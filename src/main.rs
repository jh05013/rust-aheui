use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct 실행인자 {
    프로그램_위치: String,
}

fn main() {
    let 인자 = 실행인자::parse();
    let 프로그램 =
        std::fs::read_to_string(인자.프로그램_위치).expect("프로그램을 불러올 수 없습니다.");
    println!("{프로그램:?}");
}
