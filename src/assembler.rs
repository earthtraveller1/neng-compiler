// This may turn out to be unnecessary but I've decided that it will probably make things much easier

pub enum Instruction {
    MoveRight,
    MoveLeft,
    Increment,
    Decrement,
    Print,
    Read,
    JumpForwardIf0,
    JumpBackwardIfNot0,
}

impl Instruction {
    pub fn assemble(self) -> char {
        match self {
            Self::MoveRight => '>',
            Self::MoveLeft => '<',
            Self::Increment => '+',
            Self::Decrement => '-',
            Self::Print => '.',
            Self::Read => ',',
            Self::JumpForwardIf0 => '[',
            Self::JumpBackwardIfNot0 => ']'
        }
    }
}
