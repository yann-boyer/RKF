#[derive(Clone, Copy)]
pub enum Instructions {
    IncrementPointer,
    DecrementPointer,
    IncrementByte,
    DecrementByte,
    WriteByte,
    ReadByte,
    JumpForward,
    JumpBackward
}
