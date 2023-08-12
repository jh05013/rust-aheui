use hangul::HangulExt;

#[derive(Debug, Clone)]
pub enum ArgType {
    IoInt,
    IoChar,
    ConstInt(i32),
}

fn jong_arg_for_pop(jong: Option<char>) -> Option<ArgType> {
    use ArgType::*;
    match jong {
        Some('ㅇ') => Some(IoInt),
        Some('ㅎ') => Some(IoChar),
        _ => None,
    }
}

fn jong_arg_for_push(jong: Option<char>) -> ArgType {
    use ArgType::*;
    if jong.is_none() {
        return ConstInt(0);
    }
    let jong = jong.unwrap();
    match jong {
        'ㅇ' => IoInt,
        'ㅎ' => IoChar,
        'ㄱ' | 'ㄴ' | 'ㅅ' => ConstInt(2),
        'ㄷ' | 'ㅈ' | 'ㅋ' => ConstInt(3),
        'ㅁ' | 'ㅂ' | 'ㅊ' | 'ㅌ' | 'ㅍ' | 'ㄲ' | 'ㄳ' | 'ㅆ' => ConstInt(4),
        'ㄹ' | 'ㄵ' | 'ㄶ' => ConstInt(5),
        'ㅄ' => ConstInt(6),
        'ㄺ' | 'ㄽ' => ConstInt(7),
        'ㅀ' => ConstInt(8),
        'ㄻ' | 'ㄼ' | 'ㄾ' | 'ㄿ' => ConstInt(9),
        _ => unreachable!(),
    }
}

#[derive(Debug, Clone)]
pub enum StorageArgument {
    Stack(usize),
    Queue,
    ExtensionProtocol,
}

impl TryFrom<Option<char>> for StorageArgument {
    type Error = char;

    fn try_from(jong: Option<char>) -> Result<Self, Self::Error> {
        use StorageArgument::*;
        if jong.is_none() {
            return Ok(Stack(0));
        }
        let jong = jong.unwrap();
        match jong {
            'ㅇ' => Ok(Queue),
            'ㅎ' => Ok(ExtensionProtocol),
            'ㄱ' => Ok(Stack(1)),
            'ㄴ' => Ok(Stack(2)),
            'ㄷ' => Ok(Stack(3)),
            'ㄹ' => Ok(Stack(4)),
            'ㅁ' => Ok(Stack(5)),
            'ㅂ' => Ok(Stack(6)),
            'ㅅ' => Ok(Stack(7)),
            'ㅈ' => Ok(Stack(8)),
            'ㅊ' => Ok(Stack(9)),
            'ㅋ' => Ok(Stack(10)),
            'ㅌ' => Ok(Stack(11)),
            'ㅍ' => Ok(Stack(12)),
            'ㄲ' => Ok(Stack(13)),
            'ㄳ' => Ok(Stack(14)),
            'ㄵ' => Ok(Stack(15)),
            'ㄶ' => Ok(Stack(16)),
            'ㄺ' => Ok(Stack(17)),
            'ㄻ' => Ok(Stack(18)),
            'ㄼ' => Ok(Stack(19)),
            'ㄽ' => Ok(Stack(20)),
            'ㄾ' => Ok(Stack(21)),
            'ㄿ' => Ok(Stack(22)),
            'ㅀ' => Ok(Stack(23)),
            'ㅄ' => Ok(Stack(24)),
            'ㅆ' => Ok(Stack(25)),
            _ => Err(jong),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Instruction {
    Nop,                     // ㅇ
    Halt,                    // ㅎ
    Add,                     // ㄷ
    Mul,                     // ㄸ
    Sub,                     // ㅌ
    Div,                     // ㄴ
    Mod,                     // ㄹ
    Pop(Option<ArgType>),    // ㅁ
    Push(ArgType),           // ㅂ
    Dup,                     // ㅃ
    Swap,                    // ㅍ
    Choose(StorageArgument), // ㅅ
    Move(StorageArgument),   // ㅆ
    Compare,                 // ㅈ
    Conditional,             // ㅊ
}

impl TryFrom<(char, Option<char>)> for Instruction {
    type Error = char;

    fn try_from((cho, jong): (char, Option<char>)) -> Result<Self, Self::Error> {
        use Instruction::*;
        match cho {
            'ㄱ' | 'ㄲ' | 'ㅇ' => Ok(Nop),
            'ㅎ' => Ok(Halt),
            'ㄷ' => Ok(Add),
            'ㄸ' => Ok(Mul),
            'ㅌ' => Ok(Sub),
            'ㄴ' => Ok(Div),
            'ㄹ' => Ok(Mod),
            'ㅁ' => Ok(Pop(jong_arg_for_pop(jong))),
            'ㅂ' => Ok(Push(jong_arg_for_push(jong))),
            'ㅃ' => Ok(Dup),
            'ㅍ' => Ok(Swap),
            'ㅅ' => Ok(Choose(jong.try_into()?)),
            'ㅆ' => Ok(Move(jong.try_into()?)),
            'ㅈ' => Ok(Compare),
            'ㅊ' => Ok(Conditional),
            _ => Err(cho),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Momentum {
    Up,         // ㅗ
    UpUp,       // ㅛ
    Left,       // ㅓ
    LeftLeft,   // ㅕ
    Right,      // ㅏ
    RightRight, // ㅑ
    Down,       // ㅜ
    DownDown,   // ㅠ
}

#[derive(Debug, Copy, Clone)]
pub enum MomentumChange {
    Retain,
    Set(Momentum),  // ㅗㅛㅏㅑㅓㅕㅜㅠ
    FlipVertical,   // ㅡ
    FlipHorizontal, // ㅣ
    FlipBoth,       // ㅢ
}

impl From<char> for MomentumChange {
    fn from(c: char) -> Self {
        use Momentum::*;
        use MomentumChange::*;
        match c {
            'ㅗ' => Set(Up),
            'ㅛ' => Set(UpUp),
            'ㅓ' => Set(Left),
            'ㅕ' => Set(LeftLeft),
            'ㅏ' => Set(Right),
            'ㅑ' => Set(RightRight),
            'ㅜ' => Set(Down),
            'ㅠ' => Set(DownDown),
            'ㅡ' => FlipVertical,
            'ㅣ' => FlipHorizontal,
            'ㅢ' => FlipBoth,
            _ => Retain,
        }
    }
}

impl Momentum {
    pub fn flip_vertical(&self) -> Momentum {
        use Momentum::*;
        match *self {
            Up => Down,
            UpUp => DownDown,
            Down => Up,
            DownDown => UpUp,
            _ => *self,
        }
    }

    pub fn flip_horizontal(&self) -> Momentum {
        use Momentum::*;
        match *self {
            Left => Right,
            LeftLeft => RightRight,
            Right => Left,
            RightRight => LeftLeft,
            _ => *self,
        }
    }

    pub fn apply_change(&self, mc: MomentumChange) -> Momentum {
        use MomentumChange::*;
        match mc {
            Retain => *self,
            Set(mnew) => mnew,
            FlipVertical => self.flip_vertical(),
            FlipHorizontal => self.flip_horizontal(),
            FlipBoth => self.flip_vertical().flip_horizontal(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct AheuiSyllable {
    instruction: Instruction,
    momentum_change: MomentumChange,
}

pub enum AheuiSyllableError {
    NotSyllable(hangul::ParseSyllableError),
    Todo,
}

impl TryFrom<char> for AheuiSyllable {
    type Error = AheuiSyllableError;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        use AheuiSyllableError::*;
        let (cho, jung, jong) = c.to_jamo().map_err(|e| NotSyllable(e))?;
        let instruction: Instruction = (cho, jong).try_into().unwrap();
        let momentum_change: MomentumChange = jung.into();
        Ok(Self {
            instruction,
            momentum_change,
        })
    }
}

#[derive(Debug, Clone)]
pub struct AheuiGrid {
    grid: Vec<Vec<Option<AheuiSyllable>>>,
}

impl AheuiGrid {
    pub fn new(source: String) -> Self {
        let grid = source
            .lines()
            .map(|line| line.chars().map(|c| c.try_into().ok()).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        Self { grid }
    }
}
