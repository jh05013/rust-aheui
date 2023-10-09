use crate::한글::한글문자;

use Option as 옵션;
use String as 문자열;
use Vec as 벡;

pub fn 격자로_읽기(프로그램: 문자열) -> 벡<벡<옵션<한글문자>>> {
    프로그램
        .lines()
        .map(|줄| 줄.chars().map(|글자| 글자.try_into().ok()).collect())
        .collect()
}
