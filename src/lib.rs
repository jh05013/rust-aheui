use hangul::HangulExt;

#[derive(Debug, Clone)]
pub enum IoArgument {
	Discard,
	Int,
	Char,
}

impl From<Option<char>> for IoArgument {
    fn from(jong: Option<char>) -> Self {
		use IoArgument::*;
		match jong {
			Some('ㅇ') => Int,
			Some('ㅎ') => Char,
			_ => Discard
		}
    }
}

#[derive(Debug, Clone)]
pub enum PushArgument {
	Io(IoArgument),
	Const(i32),
}

impl TryFrom<Option<char>> for PushArgument {
    type Error = char;

    fn try_from(jong: Option<char>) -> Result<Self, Self::Error> {
		use PushArgument::*;
		if jong.is_none() {
			return Ok(Const(0));
		}
		let jong = jong.unwrap();
		match jong {
			'ㅇ' => Ok(Io(IoArgument::Int)),
			'ㅎ' => Ok(Io(IoArgument::Char)),
			'ㄱ' | 'ㄴ' | 'ㅅ' => Ok(Const(2)),
			'ㄷ' | 'ㅈ' | 'ㅋ' => Ok(Const(3)),
			'ㅁ' | 'ㅂ' | 'ㅊ' | 'ㅌ' | 'ㅍ' | 'ㄲ' | 'ㄳ' | 'ㅆ' => Ok(Const(4)),
			'ㄹ' | 'ㄵ' | 'ㄶ' => Ok(Const(5)),
			'ㅄ' => Ok(Const(6)),
			'ㄺ' | 'ㄽ' => Ok(Const(7)),
			'ㅀ' => Ok(Const(8)),
			'ㄻ' | 'ㄼ' | 'ㄾ' | 'ㄿ' => Ok(Const(9)),
			_ => Err(jong)
		}
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
			_ => Err(jong)
		}
    }
}

#[derive(Debug, Clone)]
pub enum Instruction {
	Nop, // ㅇ
	Halt, // ㅎ
	Add, // ㄷ
	Mul, // ㄸ
	Sub, // ㅌ
	Div, // ㄴ
	Mod, // ㄹ
	Pop(IoArgument), // ㅁ
	Push(PushArgument), // ㅂ
	Dup, // ㅃ
	Swap, // ㅍ
	Choose(StorageArgument), // ㅅ
	Move(StorageArgument), // ㅆ
	Compare, // ㅈ
	Conditional, // ㅊ
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
			'ㅁ' => Ok(Pop(jong.into())),
			'ㅂ' => Ok(Push(jong.try_into()?)),
			'ㅃ' => Ok(Dup),
			'ㅍ' => Ok(Swap),
			'ㅅ' => Ok(Choose(jong.try_into()?)),
			'ㅆ' => Ok(Move(jong.try_into()?)),
			'ㅈ' => Ok(Compare),
			'ㅊ' => Ok(Conditional),
			_ => Err(cho)
		}
    }
}

#[derive(Debug, Clone)]
pub enum Momentum {
	Retain,
	Up, // ㅗ
	UpUp, // ㅛ
	Left, // ㅓ
	LeftLeft, // ㅕ
	Right, // ㅏ
	RightRight, // ㅑ
	Down, // ㅜ
	DownDown, // ㅠ
	FlipVertical, // ㅡ
	FlipHorizontal, // ㅣ
	FlipBoth, // ㅢ
}

impl From<char> for Momentum {
    fn from(c: char) -> Self {
		use Momentum::*;
		match c {
			'ㅗ' => Up,
			'ㅛ' => UpUp,
			'ㅓ' => Left,
			'ㅕ' => LeftLeft,
			'ㅏ' => Right,
			'ㅑ' => RightRight,
			'ㅜ' => Down,
			'ㅠ' => DownDown,
			'ㅡ' => FlipVertical,
			'ㅣ' => FlipHorizontal,
			'ㅢ' => FlipBoth,
			_ => Retain
		}
    }
}

#[derive(Debug, Clone)]
pub struct AheuiSyllable {
	instruction: Instruction,
	momentum: Momentum,
}

pub enum AheuiSyllableError {
	NotSyllable(hangul::ParseSyllableError),
	Todo
}

impl TryFrom<char> for AheuiSyllable {
    type Error = AheuiSyllableError;

    fn try_from(c: char) -> Result<Self, Self::Error> {
		use AheuiSyllableError::*;
		let (cho, jung, jong) = c.to_jamo()
			.map_err(|e| NotSyllable(e))?;
		let instruction: Instruction = (cho, jong).try_into().unwrap();
		let momentum: Momentum = jung.into();
		Ok(Self { instruction, momentum })
    }
}

#[derive(Debug, Clone)]
pub struct AheuiGrid {
	grid: Vec<Vec<Option<AheuiSyllable>>>,
}

impl AheuiGrid {
	pub fn new(source: String) -> Self {
		let grid = source.lines()
			.map(|line|
				line.chars()
				.map(|c| c.try_into().ok())
				.collect::<Vec<_>>()
			)
			.collect::<Vec<_>>();
		Self { grid }
	}
}
