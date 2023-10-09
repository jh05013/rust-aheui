use crate::{가명::*, 격자::한글명령::속도변환, 한글::종성};

use super::{
    한글명령::{속도, 한글명령},
    해독::아희격자,
};
use 벡 as 통로;

pub trait 저장공간: std::fmt::Debug {
    fn 종료_코드(&self) -> 종료코드;
    fn 덧셈(&mut self) -> 부울;
}

impl 저장공간 for 벡<i64> {
    fn 종료_코드(&self) -> 종료코드 {
        let 코드 = self
            .last()
            .map(|&수| 수.try_into().unwrap_or(255))
            .unwrap_or(0);
        종료코드::from(코드)
    }

    fn 덧셈(&mut self) -> 부울 {
        if self.len() < 2 {
            return false;
        }
        let 첫 = self.pop().unwrap();
        *self.last_mut().unwrap() += 첫;
        true
    }
}

impl 저장공간 for 벡데크<i64> {
    fn 종료_코드(&self) -> 종료코드 {
        let 코드 = self
            .back()
            .map(|&수| 수.try_into().unwrap_or(255))
            .unwrap_or(0);
        종료코드::from(코드)
    }

    fn 덧셈(&mut self) -> 부울 {
        if self.len() < 2 {
            return false;
        }
        let 첫 = self.pop_back().unwrap();
        *self.back_mut().unwrap() += 첫;
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

#[derive(복제, 디버그)]
pub struct 아희실행기 {
    행: i크기,
    열: i크기,
    속도: 속도,
    지금: 종성,
    저장공간묶음: 저장공간묶음,
    종료: 옵션<종료코드>,
}

impl 아희실행기 {
    pub fn 새로() -> Self {
        Self {
            행: 0,
            열: 0,
            속도: 속도::ㅜ,
            지금: 종성::없음,
            저장공간묶음: 저장공간묶음::default(),
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
            곱셈 => todo!(),
            뺄셈 => todo!(),
            나눗셈 => todo!(),
            나머지 => todo!(),
            뽑기(인자) => todo!(),
            집어넣기(인자) => todo!(),
            중복 => todo!(),
            바꿔치기 => todo!(),
            선택(종성) => todo!(),
            이동(종성) => todo!(),
            비교 => todo!(),
            조건 => todo!(),
        };

        self.속도.처리(글자.속도변환);
        if !성공 {
            self.속도.처리(속도변환::모두뒤집기);
        }
        self.이동();
    }

    fn 명령_읽기<'ㄱ>(&mut self, 격자: &'ㄱ 아희격자) -> &'ㄱ 한글명령 {
        loop {
            let 행 = self.행 as usize;
            let 있음(줄) = 격자.get(행)
			else {
				println!("{self:?}에서 읽기 실패!");
				let 행수 = 격자.len() as i크기;
				self.이동();
				if self.행 < 0 { self.행 += 행수; }
				if self.행 >= 행수 { self.행 -= 행수; }
				continue;
			};
            let 있음(명) = 줄.get(self.열 as usize)
			else {
				println!("{self:?}에서 읽기 실패!");
				let 행수 = 격자.len() as i크기;
				let 열수 = 줄.len() as i크기;
				self.이동();
				if self.행 < 0 { self.행 += 행수; }
				if self.행 >= 행수 { self.행 -= 행수; }
				if self.열 < 0 { self.열 += 열수; }
				if self.열 >= 열수 { self.열 -= 열수; }
				continue;
			};
            return 명;
        }
    }

    pub fn 실행(&mut self, 격자: &아희격자) -> 종료코드 {
        loop {
            let 명 = self.명령_읽기(격자);
            self.처리(명);

            println!("{self:?}");

            if let 있음(종료_코드) = self.종료 {
                return 종료_코드;
            }
        }
    }
}
