#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub enum OpCode {
    Return,
    Branch(usize),
    ConditionalBranch(usize, usize),
    SIAdd(usize, usize),
    SISub(usize, usize),
    SIMul(usize, usize),
    SIDiv(usize, usize),
    SIMod(usize, usize),
    UIAdd(usize, usize),
    UISub(usize, usize),
    UIMul(usize, usize),
    UIDiv(usize, usize),
    UIMod(usize, usize),
    FAdd(usize, usize),
    FSub(usize, usize),
    FMul(usize, usize),
    FDiv(usize, usize),
    FMod(usize, usize),
    Shl(usize, usize),
    Shr(usize, usize),
    BitAnd(usize, usize),
    BitOr(usize, usize),
    Xor(usize, usize),
    LogicalNot(usize),
    BitNot(usize),
    SILt(usize, usize),
    SILe(usize, usize),
    SIGe(usize, usize),
    SIGt(usize, usize),
    UILt(usize, usize),
    UILe(usize, usize),
    UIGe(usize, usize),
    UIGt(usize, usize),
    FLt(usize, usize),
    FLe(usize, usize),
    FGe(usize, usize),
    FGt(usize, usize),
    Eq(usize, usize),
    Ne(usize, usize),
    SIConst8(usize, i8),
    SIConst16(usize, i16),
    SIConst32(usize, i32),
    SIConst64(usize, i64),
    UIConst8(usize, u8),
    UIConst16(usize, u16),
    UIConst32(usize, u32),
    UIConst64(usize, u64),
    FConst64(usize, f64),
    Load8(usize, usize),
    Load16(usize, usize),
    Load32(usize, usize),
    Load64(usize, usize),
    Store8(usize, usize),
    Store16(usize, usize),
    Store32(usize, usize),
    Store64(usize, usize),
    Mov(usize, usize),
    LoadGlobal(usize, usize),
    StoreGlobal(usize, usize),
    Call(usize),
    CallIndirect(usize),
    CallNative(usize),
    CallNativeIndirect(usize)
}
