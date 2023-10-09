use char as 문자;
use Option as 옵션;
use Option::None as 없음;
use Option::Some as 있음;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum 초성 {
    ㄱ,
    ㄲ,
    ㄴ,
    ㄷ,
    ㄸ,
    ㄹ,
    ㅁ,
    ㅂ,
    ㅃ,
    ㅅ,
    ㅆ,
    ㅇ,
    ㅈ,
    ㅉ,
    ㅊ,
    ㅋ,
    ㅌ,
    ㅍ,
    ㅎ,
}

impl From<u32> for 초성 {
    fn from(value: u32) -> Self {
        use 초성::*;
        match value {
            0 => ㄱ,
            1 => ㄲ,
            2 => ㄴ,
            3 => ㄷ,
            4 => ㄸ,
            5 => ㄹ,
            6 => ㅁ,
            7 => ㅂ,
            8 => ㅃ,
            9 => ㅅ,
            10 => ㅆ,
            11 => ㅇ,
            12 => ㅈ,
            13 => ㅉ,
            14 => ㅊ,
            15 => ㅋ,
            16 => ㅌ,
            17 => ㅍ,
            18 => ㅎ,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum 중성 {
    ㅏ,
    ㅐ,
    ㅑ,
    ㅒ,
    ㅓ,
    ㅔ,
    ㅕ,
    ㅖ,
    ㅗ,
    ㅘ,
    ㅙ,
    ㅚ,
    ㅛ,
    ㅜ,
    ㅝ,
    ㅞ,
    ㅟ,
    ㅠ,
    ㅡ,
    ㅢ,
    ㅣ,
}

impl From<u32> for 중성 {
    fn from(value: u32) -> Self {
        use 중성::*;
        match value {
            0 => ㅏ,
            1 => ㅐ,
            2 => ㅑ,
            3 => ㅒ,
            4 => ㅓ,
            5 => ㅔ,
            6 => ㅕ,
            7 => ㅖ,
            8 => ㅗ,
            9 => ㅘ,
            10 => ㅙ,
            11 => ㅚ,
            12 => ㅛ,
            13 => ㅜ,
            14 => ㅝ,
            15 => ㅞ,
            16 => ㅟ,
            17 => ㅠ,
            18 => ㅡ,
            19 => ㅢ,
            20 => ㅣ,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum 종성 {
    없음,
    ㄱ,
    ㄲ,
    ㄳ,
    ㄴ,
    ㄵ,
    ㄶ,
    ㄷ,
    ㄹ,
    ㄺ,
    ㄻ,
    ㄼ,
    ㄽ,
    ㄾ,
    ㄿ,
    ㅀ,
    ㅁ,
    ㅂ,
    ㅄ,
    ㅅ,
    ㅆ,
    ㅇ,
    ㅈ,
    ㅊ,
    ㅋ,
    ㅌ,
    ㅍ,
    ㅎ,
}

impl From<u32> for 종성 {
    fn from(value: u32) -> Self {
        use 종성::*;
        match value {
            0 => 없음,
            1 => ㄱ,
            2 => ㄲ,
            3 => ㄳ,
            4 => ㄴ,
            5 => ㄵ,
            6 => ㄶ,
            7 => ㄷ,
            8 => ㄹ,
            9 => ㄺ,
            10 => ㄻ,
            11 => ㄼ,
            12 => ㄽ,
            13 => ㄾ,
            14 => ㄿ,
            15 => ㅀ,
            16 => ㅁ,
            17 => ㅂ,
            18 => ㅄ,
            19 => ㅅ,
            20 => ㅆ,
            21 => ㅇ,
            22 => ㅈ,
            23 => ㅊ,
            24 => ㅋ,
            25 => ㅌ,
            26 => ㅍ,
            27 => ㅎ,
            _ => unreachable!(),
        }
    }
}

pub fn 한글_분해하기(글자: 문자) -> 옵션<(초성, 중성, 종성)> {
    if !('가'..='힣').contains(&글자) {
        return 없음;
    }
    let 번 = 글자 as u32;
    let 번 = 번 - 44032;
    let 초 = 번 / 588;
    let 중 = (번 / 28) % 21;
    let 종 = 번 % 28;
    있음((초.into(), 중.into(), 종.into()))
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct 한글문자 {
    초성: 초성,
    중성: 중성,
    종성: 종성,
}

impl TryFrom<문자> for 한글문자 {
    type Error = char;

    fn try_from(글자: 문자) -> Result<Self, Self::Error> {
        match 한글_분해하기(글자) {
            있음((초성, 중성, 종성)) => Ok(한글문자 {
                초성, 중성, 종성
            }),
            없음 => Err(글자),
        }
    }
}

#[cfg(test)]
mod 테스트 {
    use crate::한글::{종성, 중성, 초성, 한글_분해하기};
    use assert_eq as 주장_같음;
    use Option::None as 없음;
    use Option::Some as 있음;

    #[test]
    fn 한글_분해_테스트() {
        주장_같음!(한글_분해하기('가'), 있음((초성::ㄱ, 중성::ㅏ, 종성::없음)));
        주장_같음!(한글_분해하기('뛙'), 있음((초성::ㄸ, 중성::ㅞ, 종성::ㄵ)));
        주장_같음!(한글_분해하기('힣'), 있음((초성::ㅎ, 중성::ㅣ, 종성::ㅎ)));

        주장_같음!(한글_분해하기('ㄱ'), 없음);
        주장_같음!(한글_분해하기('ㅏ'), 없음);
        주장_같음!(한글_분해하기('🦀'), 없음);
    }
}
