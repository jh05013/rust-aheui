use super::한글명령::한글명령;
use crate::가명::*;

type 아희격자 = 벡<벡<한글명령>>;

pub fn 격자로_읽기(프로그램: 문자열) -> 아희격자 {
    프로그램
        .lines()
        .map(|줄| 줄.chars().map(|글자| 글자.into()).collect())
        .collect()
}
