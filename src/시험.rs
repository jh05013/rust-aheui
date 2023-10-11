use crate::가명::*;
use crate::격자::실행::아희실행기;
use crate::격자::해독;

pub fn 프로그램_시험(프로그램_위치: &str) {
    let 프로그램 = std::fs::read_to_string(프로그램_위치).expect("프로그램을 불러올 수 없습니다.");
    let 격자 = 해독::격자로_읽기(프로그램);

    let mut 입력_위치 = 프로그램_위치.trim_end_matches(".aheui").to_string();
    입력_위치 += ".in";
    let 입력 = std::fs::read(입력_위치).unwrap_or(vec![]);

    let mut 출력 = Vec::<u8>::new();
    let 종료코드 = 아희실행기::새로(입력.as_slice(), &mut 출력).실행(&격자);
    if 출력.last() != 있음(&10) {
        출력.push(10);
    }

    let mut 예상_출력_위치 = 프로그램_위치.trim_end_matches(".aheui").to_string();
    예상_출력_위치 += ".out";
    let 예상_출력 = std::fs::read(예상_출력_위치).unwrap_or(vec![10]);
    assert_eq!(출력, 예상_출력);

    let mut 예상_종료코드_위치 = 프로그램_위치.trim_end_matches(".aheui").to_string();
    예상_종료코드_위치 += ".exitcode";
    let 예상_종료코드 = std::fs::read_to_string(예상_종료코드_위치)
        .unwrap_or("0".to_string())
        .trim()
        .parse::<i32>()
        .expect("예상 종료코드에 이상이 있습니다.");
    assert_eq!(종료코드, 예상_종료코드);

    println!("{프로그램_위치} 통과!")
}

#[cfg(test)]
mod 통합시험 {
    use super::프로그램_시험;

    #[test]
    fn 격자실행기_시험() {
        for 파일 in [
            "boj1000",
            "boj15778_1",
            "boj26499",
            "boj27514",
            "명세_움직임",
            "명세_종료코드",
            "명세_출력",
            //"밯",
            "안녕_유니코드",
            "안녕",
            "알파희_이슈19",
            //"캣_유니코드",
            "캣",
        ] {
            프로그램_시험(&format!("examples/{파일}.aheui"));
        }
    }
}
