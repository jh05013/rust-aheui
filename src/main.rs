use aheui::격자::실행::아희실행기;
use clap::Parser as 해독기;

use aheui::가명::*;
use aheui::격자::해독;
use aheui::시험;

#[derive(해독기, 디버그)]
#[command(author, version, about)]
struct 실행인자 {
    프로그램_위치: 문자열,

    /// 참일 경우, 예상 출력과 실제 출력을 비교합니다.
    #[clap(short, long, default_value_t = false)]
    test: 부울,
}

fn main() {
    let 인자 = 실행인자::parse();
    let 프로그램_위치 = &인자.프로그램_위치;

    if 인자.test {
        시험::프로그램_시험(프로그램_위치);
        return;
    }

    let 프로그램 = std::fs::read_to_string(프로그램_위치).expect("프로그램을 불러올 수 없습니다.");
    let 격자 = 해독::격자로_읽기(프로그램);
    let 종료코드 = 아희실행기::새로(표준입력(), 버퍼출력::new(&표준출력())).실행(&격자);
    std::process::exit(종료코드);
}
