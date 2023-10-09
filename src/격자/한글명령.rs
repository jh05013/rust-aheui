use crate::한글::{종성, 중성, 초성, 한글문자};
use char as 문자;
use Option as 옵션;
use Option::None as 없음;
use Option::Some as 있음;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum 입출력인자 {
    십진수,
    유니코드,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum 집어넣기인자 {
    입력(입출력인자),
    상수(i64),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum 명령 {
    /// ㅎ
    끝냄,

    /// ㄷ
    덧셈,
    /// ㄸ
    곱셈,
    /// ㅌ
    뺄셈,
    /// ㄴ
    나눗셈,
    /// ㄹ
    나머지,

    /// ㅁ
    뽑기(옵션<입출력인자>),
    /// ㅂ
    집어넣기(집어넣기인자),
    /// ㅃ
    중복,
    /// ㅍ
    바꿔치기,

    /// ㅅ
    선택(종성),
    /// ㅆ
    이동(종성),
    /// ㅈ
    비교,
    /// ㅊ
    조건,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum 속도 {
    ㅏ,
    ㅑ,
    ㅓ,
    ㅕ,
    ㅗ,
    ㅛ,
    ㅜ,
    ㅠ,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum 속도변환 {
    유지,
    설정하기(속도),
    위아래뒤집기,
    좌우뒤집기,
    모두뒤집기,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct 한글명령 {
    pub 명령: 옵션<명령>,
    pub 속도변환: 속도변환,
}

impl From<한글문자> for 한글명령 {
    fn from(글자: 한글문자) -> Self {
        let (초, 중, 종) = (글자.초성, 글자.중성, 글자.종성);
        let 명령 = match 초 {
            초성::ㄱ => 없음,
            초성::ㄲ => 없음,
            초성::ㄴ => 있음(명령::나눗셈),
            초성::ㄷ => 있음(명령::덧셈),
            초성::ㄸ => 있음(명령::곱셈),
            초성::ㄹ => 있음(명령::나머지),
            초성::ㅁ => {
                let 인자 = match 종 {
                    종성::ㅇ => 있음(입출력인자::십진수),
                    종성::ㅎ => 있음(입출력인자::유니코드),
                    _ => 없음,
                };
                있음(명령::뽑기(인자))
            }
            초성::ㅂ => {
                let 인자 = match 종 {
                    종성::없음 => 집어넣기인자::상수(0),
                    종성::ㄱ => 집어넣기인자::상수(2),
                    종성::ㄲ => 집어넣기인자::상수(4),
                    종성::ㄳ => 집어넣기인자::상수(4),
                    종성::ㄴ => 집어넣기인자::상수(2),
                    종성::ㄵ => 집어넣기인자::상수(5),
                    종성::ㄶ => 집어넣기인자::상수(5),
                    종성::ㄷ => 집어넣기인자::상수(3),
                    종성::ㄹ => 집어넣기인자::상수(5),
                    종성::ㄺ => 집어넣기인자::상수(7),
                    종성::ㄻ => 집어넣기인자::상수(9),
                    종성::ㄼ => 집어넣기인자::상수(9),
                    종성::ㄽ => 집어넣기인자::상수(7),
                    종성::ㄾ => 집어넣기인자::상수(9),
                    종성::ㄿ => 집어넣기인자::상수(9),
                    종성::ㅀ => 집어넣기인자::상수(8),
                    종성::ㅁ => 집어넣기인자::상수(4),
                    종성::ㅂ => 집어넣기인자::상수(4),
                    종성::ㅄ => 집어넣기인자::상수(6),
                    종성::ㅅ => 집어넣기인자::상수(2),
                    종성::ㅆ => 집어넣기인자::상수(4),
                    종성::ㅇ => 집어넣기인자::입력(입출력인자::십진수),
                    종성::ㅈ => 집어넣기인자::상수(3),
                    종성::ㅊ => 집어넣기인자::상수(4),
                    종성::ㅋ => 집어넣기인자::상수(3),
                    종성::ㅌ => 집어넣기인자::상수(4),
                    종성::ㅍ => 집어넣기인자::상수(4),
                    종성::ㅎ => 집어넣기인자::입력(입출력인자::유니코드),
                };
                있음(명령::집어넣기(인자))
            }
            초성::ㅃ => 있음(명령::중복),
            초성::ㅅ => 있음(명령::선택(종)),
            초성::ㅆ => 있음(명령::이동(종)),
            초성::ㅇ => 없음,
            초성::ㅈ => 있음(명령::비교),
            초성::ㅉ => 없음,
            초성::ㅊ => 있음(명령::조건),
            초성::ㅋ => 없음,
            초성::ㅌ => 있음(명령::뺄셈),
            초성::ㅍ => 있음(명령::바꿔치기),
            초성::ㅎ => 있음(명령::끝냄),
        };
        let 속도변환 = match 중 {
            중성::ㅏ => 속도변환::설정하기(속도::ㅏ),
            중성::ㅑ => 속도변환::설정하기(속도::ㅑ),
            중성::ㅓ => 속도변환::설정하기(속도::ㅓ),
            중성::ㅕ => 속도변환::설정하기(속도::ㅕ),
            중성::ㅗ => 속도변환::설정하기(속도::ㅗ),
            중성::ㅛ => 속도변환::설정하기(속도::ㅛ),
            중성::ㅜ => 속도변환::설정하기(속도::ㅜ),
            중성::ㅠ => 속도변환::설정하기(속도::ㅠ),
            중성::ㅡ => 속도변환::위아래뒤집기,
            중성::ㅢ => 속도변환::모두뒤집기,
            중성::ㅣ => 속도변환::좌우뒤집기,
            _ => 속도변환::유지,
        };
        한글명령 {
            명령, 속도변환
        }
    }
}

impl TryFrom<문자> for 한글명령 {
    type Error = char;

    fn try_from(글자: 문자) -> Result<Self, Self::Error> {
        let 글자 = 한글문자::try_from(글자)?;
        Ok(글자.into())
    }
}

#[cfg(test)]
mod 테스트 {
    use super::*;
    use assert as 주장;
    use assert_eq as 주장_같음;
    use Option::None as 없음;
    use Option::Some as 있음;
    use Result::Ok as 좋아;

    #[test]
    fn 한글_분해_테스트() {
        주장_같음!(
            한글명령::try_from('가'),
            좋아(한글명령 {
                명령: 없음,
                속도변환: 속도변환::설정하기(속도::ㅏ)
            })
        );
        주장_같음!(
            한글명령::try_from('든'),
            좋아(한글명령 {
                명령: 있음(명령::덧셈),
                속도변환: 속도변환::위아래뒤집기
            })
        );
        주장_같음!(
            한글명령::try_from('봐'),
            좋아(한글명령 {
                명령: 있음(명령::집어넣기(집어넣기인자::상수(0))),
                속도변환: 속도변환::유지
            })
        );
        주장_같음!(
            한글명령::try_from('힣'),
            좋아(한글명령 {
                명령: 있음(명령::끝냄),
                속도변환: 속도변환::좌우뒤집기
            })
        );

        주장!(한글명령::try_from('ㄱ').is_err());
        주장!(한글명령::try_from('ㅏ').is_err());
        주장!(한글명령::try_from('🦀').is_err());
    }
}
