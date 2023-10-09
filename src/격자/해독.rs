use super::한글명령::한글명령;
use crate::가명::*;

pub fn 격자로_읽기(프로그램: 문자열) -> 벡<벡<한글명령>> {
    프로그램
        .lines()
        .map(|줄| 줄.chars().map(|글자| 글자.into()).collect())
        .collect()
}
