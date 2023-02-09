use crate::string::String;
use crate::string::ToString;

#[derive(Debug, PartialEq, Clone, Eq, Display)]
pub enum Error {
    #[display("asm error: {}", _0)]
    Asm(u8),
    #[display("cycles error: max cycles exceeded")]
    CyclesExceeded,
    #[display("cycles error: overflow")]
    CyclesOverflow,
    #[display("elf error: bits")]
    ElfBits,
    #[display("elf error: {}", "_0")]
    ElfParseError(String),
    #[display("elf error: segment is unreadable")]
    ElfSegmentUnreadable,
    #[display("elf error: segment is writable and executable")]
    ElfSegmentWritableAndExecutable,
    #[display("elf error: segment addr or size is wrong")]
    ElfSegmentAddrOrSizeError,
    // External error type is for the debugging tool of CKB-VM, it should not be
    // used in this project.
    #[display("external error: {}", "_0")]
    External(String),
    #[display("invalid syscall {}", "_0")]
    InvalidEcall(u64),
    #[display(
        "invalid instruction pc=0x{:x} instruction=0x{:x}",
        pc,
        instruction
    )]
    InvalidInstruction { pc: u64, instruction: u32 },
    #[display("invalid operand {}", "_0")]
    InvalidOp(u16),
    #[display("invalid version")]
    InvalidVersion,
    #[cfg(feature = "std")]
    #[display("I/O error: {:?} {}", "kind", "data")]
    IO {
        kind: std::io::ErrorKind,
        data: String,
    },
    #[display("memory error: out of bound")]
    MemOutOfBound,
    #[display("memory error: out of stack")]
    MemOutOfStack,
    #[display("memory error: unaligned page access")]
    MemPageUnalignedAccess,
    #[display("memory error: write on executable page")]
    MemWriteOnExecutablePage,
    #[display("memory error: write on freezed page")]
    MemWriteOnFreezedPage,
    #[display("unexpected error")]
    Unexpected(String),
    #[display("unimplemented")]
    Unimplemented,
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

#[cfg(feature = "std")]
impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::IO {
            kind: error.kind(),
            data: error.to_string(),
        }
    }
}

impl From<goblin_v023::error::Error> for Error {
    fn from(error: goblin_v023::error::Error) -> Self {
        Error::ElfParseError(error.to_string())
    }
}

impl From<goblin_v040::error::Error> for Error {
    fn from(error: goblin_v040::error::Error) -> Self {
        Error::ElfParseError(error.to_string())
    }
}
