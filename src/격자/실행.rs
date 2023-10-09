use crate::{가명::*, 한글};

#[derive(복제, 디버그, 기본, 부분같음, 같음)]
pub struct 통로<T>(벡<T>);

pub trait 저장공간 {}

impl 저장공간 for 벡<i64> {}

impl 저장공간 for 벡데크<i64> {}

impl 저장공간 for 통로<i64> {}

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
    pub fn 고르기(&mut self, 종: 한글::종성) -> Box<&mut dyn 저장공간> {
        match 종 {
            한글::종성::없음 => Box::new(&mut self.없음),
            한글::종성::ㄱ => Box::new(&mut self.ㄱ),
            한글::종성::ㄲ => Box::new(&mut self.ㄲ),
            한글::종성::ㄳ => Box::new(&mut self.ㄳ),
            한글::종성::ㄴ => Box::new(&mut self.ㄴ),
            한글::종성::ㄵ => Box::new(&mut self.ㄵ),
            한글::종성::ㄶ => Box::new(&mut self.ㄶ),
            한글::종성::ㄷ => Box::new(&mut self.ㄷ),
            한글::종성::ㄹ => Box::new(&mut self.ㄹ),
            한글::종성::ㄺ => Box::new(&mut self.ㄺ),
            한글::종성::ㄻ => Box::new(&mut self.ㄻ),
            한글::종성::ㄼ => Box::new(&mut self.ㄼ),
            한글::종성::ㄽ => Box::new(&mut self.ㄽ),
            한글::종성::ㄾ => Box::new(&mut self.ㄾ),
            한글::종성::ㄿ => Box::new(&mut self.ㄿ),
            한글::종성::ㅀ => Box::new(&mut self.ㅀ),
            한글::종성::ㅁ => Box::new(&mut self.ㅁ),
            한글::종성::ㅂ => Box::new(&mut self.ㅂ),
            한글::종성::ㅄ => Box::new(&mut self.ㅄ),
            한글::종성::ㅅ => Box::new(&mut self.ㅅ),
            한글::종성::ㅆ => Box::new(&mut self.ㅆ),
            한글::종성::ㅇ => Box::new(&mut self.ㅇ),
            한글::종성::ㅈ => Box::new(&mut self.ㅈ),
            한글::종성::ㅊ => Box::new(&mut self.ㅊ),
            한글::종성::ㅋ => Box::new(&mut self.ㅋ),
            한글::종성::ㅌ => Box::new(&mut self.ㅌ),
            한글::종성::ㅍ => Box::new(&mut self.ㅍ),
            한글::종성::ㅎ => Box::new(&mut self.ㅎ),
        }
    }
}


