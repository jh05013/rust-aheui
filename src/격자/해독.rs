use crate::한글::한글문자;

use Option as 옵션;
use String as 문자열;
use Vec as 벡;

use super::한글명령::한글명령;

pub fn 격자로_읽기(프로그램: 문자열) -> 벡<벡<옵션<한글명령>>> {
    프로그램
        .lines()
        .map(|줄| {
            줄.chars()
                .map(|글자| {
                    글자
                        .try_into()
                        .ok()
                        .map(|글자: 한글문자| 한글명령::from(글자))
                })
                .collect()
        })
        .collect()
}
