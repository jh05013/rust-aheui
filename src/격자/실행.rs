use crate::{
    가명::*,
    격자::한글명령::{속도변환, 입출력인자},
    한글::종성,
};

use super::{
    한글명령::{속도, 집어넣기인자, 한글명령},
    해독::아희격자,
};
use 벡 as 통로;

use crate::입력기::아희입력기;

type 종료코드 = i32;

pub trait 저장공간: 디버그 {
    fn 종료_코드(&self) -> 종료코드;
    fn 뽑기(&mut self) -> 옵션<i64>;
    fn 집어넣기(&mut self, 값: i64);
    fn 덧셈(&mut self) -> 부울;
    fn 곱셈(&mut self) -> 부울;
    fn 뺄셈(&mut self) -> 부울;
    fn 나눗셈(&mut self) -> 부울;
    fn 나머지(&mut self) -> 부울;
    fn 중복(&mut self) -> 부울;
    fn 바꿔치기(&mut self) -> 부울;
    fn 비교(&mut self) -> 부울;
}

impl 저장공간 for 벡<i64> {
    fn 종료_코드(&self) -> 종료코드 {
        self.last()
            .map(|&수| 수.try_into().unwrap_or(255))
            .unwrap_or(0)
    }

    fn 뽑기(&mut self) -> 옵션<i64> {
        self.pop()
    }

    fn 집어넣기(&mut self, 값: i64) {
        self.push(값)
    }

    fn 덧셈(&mut self) -> 부울 {
        if self.len() < 2 {
            return false;
        }
        let 첫 = self.pop().unwrap();
        *self.last_mut().unwrap() += 첫;
        true
    }

    fn 곱셈(&mut self) -> bool {
        if self.len() < 2 {
            return false;
        }
        let 첫 = self.pop().unwrap();
        *self.last_mut().unwrap() *= 첫;
        true
    }

    fn 뺄셈(&mut self) -> bool {
        if self.len() < 2 {
            return false;
        }
        let 첫 = self.pop().unwrap();
        *self.last_mut().unwrap() -= 첫;
        true
    }

    fn 나눗셈(&mut self) -> bool {
        if self.len() < 2 {
            return false;
        }
        let 첫 = self.pop().unwrap();
        *self.last_mut().unwrap() /= 첫;
        true
    }

    fn 나머지(&mut self) -> bool {
        if self.len() < 2 {
            return false;
        }
        let 첫 = self.pop().unwrap();
        *self.last_mut().unwrap() %= 첫;
        true
    }

    fn 중복(&mut self) -> bool {
        if let 있음(값) = self.last() {
            self.집어넣기(*값);
            true
        } else {
            false
        }
    }

    fn 바꿔치기(&mut self) -> bool {
        let 길이 = self.len();
        if 길이 < 2 {
            return false;
        }
        self.swap(길이 - 2, 길이 - 1);
        true
    }

    fn 비교(&mut self) -> bool {
        if self.len() < 2 {
            return false;
        }
        let 첫 = self.pop().unwrap();
        let 두 = self.last_mut().unwrap();
        *두 = (*두 >= 첫) as i64;
        true
    }
}

impl 저장공간 for 벡데크<i64> {
    fn 종료_코드(&self) -> 종료코드 {
        self.back()
            .map(|&수| 수.try_into().unwrap_or(255))
            .unwrap_or(0)
    }

    fn 뽑기(&mut self) -> 옵션<i64> {
        self.pop_back()
    }

    fn 집어넣기(&mut self, 값: i64) {
        self.push_front(값)
    }

    fn 덧셈(&mut self) -> 부울 {
        if self.len() < 2 {
            return false;
        }
        let 첫 = self.pop_back().unwrap();
        *self.back_mut().unwrap() += 첫;
        true
    }

    fn 곱셈(&mut self) -> bool {
        if self.len() < 2 {
            return false;
        }
        let 첫 = self.pop_back().unwrap();
        *self.back_mut().unwrap() *= 첫;
        true
    }

    fn 뺄셈(&mut self) -> bool {
        if self.len() < 2 {
            return false;
        }
        let 첫 = self.pop_back().unwrap();
        *self.back_mut().unwrap() -= 첫;
        true
    }

    fn 나눗셈(&mut self) -> bool {
        if self.len() < 2 {
            return false;
        }
        let 첫 = self.pop_back().unwrap();
        *self.back_mut().unwrap() /= 첫;
        true
    }

    fn 나머지(&mut self) -> bool {
        if self.len() < 2 {
            return false;
        }
        let 첫 = self.pop_back().unwrap();
        *self.back_mut().unwrap() %= 첫;
        true
    }

    fn 중복(&mut self) -> bool {
        if let 있음(값) = self.back() {
            self.집어넣기(*값);
            true
        } else {
            false
        }
    }

    fn 바꿔치기(&mut self) -> bool {
        if self.len() < 2 {
            return false;
        }
        let 첫 = self.pop_back().unwrap();
        let 두 = self.pop_back().unwrap();
        self.push_back(두);
        self.push_back(첫);
        true
    }

    fn 비교(&mut self) -> bool {
        if self.len() < 2 {
            return false;
        }
        let 첫 = self.pop_back().unwrap();
        let 두 = self.back_mut().unwrap();
        *두 = (*두 >= 첫) as i64;
        true
    }
}

// impl 저장공간 for 통로<i64> {}

#[derive(복제, 디버그, 기본, 부분같음, 같음)]
pub struct 저장공간묶음 {
    없음: 벡<i64>,
    ㄱ: 벡<i64>,
    ㄲ: 벡<i64>,
    ㄳ: 벡<i64>,
    ㄴ: 벡<i64>,
    ㄵ: 벡<i64>,
    ㄶ: 벡<i64>,
    ㄷ: 벡<i64>,
    ㄹ: 벡<i64>,
    ㄺ: 벡<i64>,
    ㄻ: 벡<i64>,
    ㄼ: 벡<i64>,
    ㄽ: 벡<i64>,
    ㄾ: 벡<i64>,
    ㄿ: 벡<i64>,
    ㅀ: 벡<i64>,
    ㅁ: 벡<i64>,
    ㅂ: 벡<i64>,
    ㅄ: 벡<i64>,
    ㅅ: 벡<i64>,
    ㅆ: 벡<i64>,
    ㅇ: 벡데크<i64>,
    ㅈ: 벡<i64>,
    ㅊ: 벡<i64>,
    ㅋ: 벡<i64>,
    ㅌ: 벡<i64>,
    ㅍ: 벡<i64>,
    ㅎ: 통로<i64>,
}

impl 저장공간묶음 {
    pub fn 고르기(&mut self, 종: 종성) -> 상자<&mut dyn 저장공간> {
        match 종 {
            종성::없음 => 상자::new(&mut self.없음),
            종성::ㄱ => 상자::new(&mut self.ㄱ),
            종성::ㄲ => 상자::new(&mut self.ㄲ),
            종성::ㄳ => 상자::new(&mut self.ㄳ),
            종성::ㄴ => 상자::new(&mut self.ㄴ),
            종성::ㄵ => 상자::new(&mut self.ㄵ),
            종성::ㄶ => 상자::new(&mut self.ㄶ),
            종성::ㄷ => 상자::new(&mut self.ㄷ),
            종성::ㄹ => 상자::new(&mut self.ㄹ),
            종성::ㄺ => 상자::new(&mut self.ㄺ),
            종성::ㄻ => 상자::new(&mut self.ㄻ),
            종성::ㄼ => 상자::new(&mut self.ㄼ),
            종성::ㄽ => 상자::new(&mut self.ㄽ),
            종성::ㄾ => 상자::new(&mut self.ㄾ),
            종성::ㄿ => 상자::new(&mut self.ㄿ),
            종성::ㅀ => 상자::new(&mut self.ㅀ),
            종성::ㅁ => 상자::new(&mut self.ㅁ),
            종성::ㅂ => 상자::new(&mut self.ㅂ),
            종성::ㅄ => 상자::new(&mut self.ㅄ),
            종성::ㅅ => 상자::new(&mut self.ㅅ),
            종성::ㅆ => 상자::new(&mut self.ㅆ),
            종성::ㅇ => 상자::new(&mut self.ㅇ),
            종성::ㅈ => 상자::new(&mut self.ㅈ),
            종성::ㅊ => 상자::new(&mut self.ㅊ),
            종성::ㅋ => 상자::new(&mut self.ㅋ),
            종성::ㅌ => 상자::new(&mut self.ㅌ),
            종성::ㅍ => 상자::new(&mut self.ㅍ),
            종성::ㅎ => 상자::new(&mut self.ㅎ),
        }
    }
}

#[derive(복제)]
pub struct 아희실행기<ㅇ: 읽음, ㅆ: 씀> {
    행: i크기,
    열: i크기,
    속도: 속도,

    지금: 종성,
    저장공간묶음: 저장공간묶음,

    행수: i크기,
    열수: i크기,
    입력기: 아희입력기<ㅇ>,
    출력기: ㅆ,
    종료: 옵션<종료코드>,
}

impl<ㅇ: 읽음, ㅆ: 씀> 디버그 for 아희실행기<ㅇ, ㅆ> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("아희실행기")
            .field("행", &self.행)
            .field("열", &self.열)
            .field("속도", &self.속도)
            .field("지금", &self.지금)
            .field("저장공간묶음", &self.저장공간묶음)
            .field("종료", &self.종료)
            .finish()
    }
}

impl<ㅇ: 읽음, ㅆ: 씀> 아희실행기<ㅇ, ㅆ> {
    pub fn 새로(입력기: ㅇ, 출력기: ㅆ) -> Self {
        Self {
            행: 0,
            열: 0,
            속도: 속도::ㅜ,

            지금: 종성::없음,
            저장공간묶음: 저장공간묶음::default(),

            행수: 0,
            열수: 0,
            입력기: 아희입력기::새(8192, 입력기),
            출력기,
            종료: 없음,
        }
    }

    pub fn 저장공간(&mut self) -> 상자<&mut dyn 저장공간> {
        self.저장공간묶음.고르기(self.지금)
    }

    pub fn 이동(&mut self) {
        let (행변화, 열변화) = self.속도.변화량();
        self.행 += 행변화;
        self.열 += 열변화;
    }

    pub fn 처리(&mut self, 글자: &한글명령) {
        use super::한글명령::명령::*;
        let 있음(명령) = &글자.명령 else {
            self.속도.처리(글자.속도변환);
            self.이동();
            return;
        };

        let 성공 = match 명령 {
            끝냄 => {
                self.종료 = 있음(self.저장공간().종료_코드());
                return;
            }
            덧셈 => self.저장공간().덧셈(),
            곱셈 => self.저장공간().곱셈(),
            뺄셈 => self.저장공간().뺄셈(),
            나눗셈 => self.저장공간().나눗셈(),
            나머지 => self.저장공간().나머지(),
            뽑기(인자) => {
                if let 있음(값) = self.저장공간().뽑기() {
                    match 인자 {
                        없음 => {}
                        있음(입출력인자::십진수) => {
                            let _ = write!(self.출력기, "{값}");
                        }
                        있음(입출력인자::유니코드) => {
                            if let 있음(문자) = char::from_u32(값 as u32) {
                                let _ = write!(self.출력기, "{문자}");
                            }
                        }
                    }
                    true
                } else {
                    false
                }
            }
            집어넣기(인자) => {
                let 아마값 = match 인자 {
                    집어넣기인자::입력(입출력인자::십진수) => {
                        self.입력기.정수_읽기()
                    }
                    집어넣기인자::입력(입출력인자::유니코드) => {
                        self.입력기.문자_읽기().map(|글자| 글자 as u32 as i64)
                    }
                    집어넣기인자::상수(값) => 좋음(*값),
                };
                if let 좋음(값) = 아마값 {
                    self.저장공간().집어넣기(값);
                    true
                } else {
                    false
                }
            }
            중복 => self.저장공간().중복(),
            바꿔치기 => self.저장공간().바꿔치기(),
            선택(종성) => {
                self.지금 = *종성;
                true
            }
            이동(종성) => {
                if let 있음(값) = self.저장공간().뽑기() {
                    self.저장공간묶음.고르기(*종성).집어넣기(값);
                    true
                } else {
                    false
                }
            }
            비교 => self.저장공간().비교(),
            조건 => {
                if let 있음(값) = self.저장공간().뽑기() {
                    값 != 0
                } else {
                    false
                }
            }
        };

        self.속도.처리(글자.속도변환);
        if !성공 {
            self.속도.처리(속도변환::모두뒤집기);
        }
        self.이동();
    }

    fn 명령_읽기<'a>(&mut self, 격자: &'a 아희격자) -> &'a 한글명령 {
        loop {
            let 있음(줄) = 격자.get(self.행 as usize)
            else {
                self.이동();
                if self.행 < 0 { self.행 += self.행수; }
                if self.행 >= self.행수 { self.행 -= self.행수; }
                continue;
            };
            let 있음(명) = 줄.get(self.열 as usize)
            else {
                self.이동();
                if self.열 < 0 { self.열 += self.열수; }
                if self.열 >= self.열수 { self.열 -= self.열수; }
                continue;
            };
            return 명;
        }
    }

    pub fn 실행(&mut self, 격자: &아희격자) -> 종료코드 {
        self.행수 = 격자.len() as isize;
        self.열수 = 격자.iter().map(|줄| 줄.len()).max().unwrap_or(1) as isize;
        #[cfg(debug_assert)]
        println!("{} x {}", self.행수, self.열수);
        loop {
            let 명 = self.명령_읽기(격자);
            #[cfg(debug_assert)]
            println!("{self:?}");
            #[cfg(debug_assert)]
            println!("{명:?}");
            self.처리(명);
            if self.행 < 0 {
                self.행 += self.행수;
            }
            if self.행 >= self.행수 {
                self.행 -= self.행수;
            }
            if self.열 < 0 {
                self.열 += self.열수;
            }
            if self.열 >= self.열수 {
                self.열 -= self.열수;
            }
            if let 있음(종료_코드) = self.종료 {
                return 종료_코드;
            }
        }
    }
}
