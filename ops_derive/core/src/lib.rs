use syn::Ident;
use quote::format_ident;

pub type OpsBitflag = u32;
pub const ADD:          OpsBitflag = 1 << 0;
pub const ADD_ASSIGN:   OpsBitflag = 1 << 1;
pub const SUB:          OpsBitflag = 1 << 2;
pub const SUB_ASSIGN:   OpsBitflag = 1 << 3;
pub const MUL:          OpsBitflag = 1 << 4;
pub const MUL_ASSIGN:   OpsBitflag = 1 << 5;
pub const DIV:          OpsBitflag = 1 << 6;
pub const DIV_ASSIGN:   OpsBitflag = 1 << 7;
pub const REM:          OpsBitflag = 1 << 8;
pub const REM_ASSIGN:   OpsBitflag = 1 << 9;
pub const NEG:          OpsBitflag = 1 << 10;

pub const ADD_OPS:      OpsBitflag = ADD | ADD_ASSIGN;
pub const SUB_OPS:      OpsBitflag = SUB | SUB_ASSIGN;
pub const MUL_OPS:      OpsBitflag = MUL | MUL_ASSIGN;
pub const DIV_OPS:      OpsBitflag = DIV | DIV_ASSIGN;
pub const REM_OPS:      OpsBitflag = REM | REM_ASSIGN;
pub const ARITH_OPS:    OpsBitflag = ADD_OPS | SUB_OPS | MUL_OPS | DIV_OPS | REM_OPS | NEG;

#[cfg(feature="bit_ops")]
pub mod bit_ops {
    use crate::OpsBitflag;

    pub const NOT:              OpsBitflag = 1 << 11;
    pub const BIT_AND:          OpsBitflag = 1 << 12;
    pub const BIT_AND_ASSIGN:   OpsBitflag = 1 << 13;
    pub const BIT_OR:           OpsBitflag = 1 << 12;
    pub const BIT_OR_ASSIGN:    OpsBitflag = 1 << 13;
    pub const BIT_XOR:          OpsBitflag = 1 << 12;
    pub const BIT_XOR_ASSIGN:   OpsBitflag = 1 << 13;

    pub const BIT_AND_OPS:      OpsBitflag = BIT_AND | BIT_AND_ASSIGN;
    pub const BIT_OR_OPS:       OpsBitflag = BIT_OR | BIT_OR_ASSIGN;
    pub const BIT_XOR_OPS:      OpsBitflag = BIT_XOR | BIT_XOR_ASSIGN;
    pub const BIT_BASIC_OPS:    OpsBitflag = NOT | BIT_AND_OPS | BIT_OR_OPS | BIT_XOR_OPS;

    pub const SHL:              OpsBitflag = 1 << 14;
    pub const SHL_ASSIGN:       OpsBitflag = 1 << 15;
    pub const SHR:              OpsBitflag = 1 << 16;
    pub const SHR_ASSIGN:       OpsBitflag = 1 << 17;

    pub const SHL_OPS:          OpsBitflag = SHL | SHL_ASSIGN;
    pub const SHR_OPS:          OpsBitflag = SHR | SHR_ASSIGN;
    pub const SHIFT_OPS:        OpsBitflag = SHL_OPS | SHR_OPS;

    pub const BIT_OPS:          OpsBitflag = BIT_BASIC_OPS | SHIFT_OPS;
}
#[cfg(feature="bit_ops")]
pub use bit_ops::*;

#[cfg(feature="iter_ops")]
pub mod iter_ops {
    use crate::OpsBitflag;

    pub const SUM:      OpsBitflag = 1 << 18;
    pub const PRODUCT:  OpsBitflag = 1 << 19;

    pub const ITER_OPS: OpsBitflag = SUM | PRODUCT;
}
#[cfg(feature="iter_ops")]
pub use iter_ops::*;

pub fn bit_to_traits(bit: OpsBitflag) -> Vec<Ident> {
    let mut res = Vec::new();
    if bit & ADD != 0 {
        res.push(format_ident!("Add"));
    }
    pub const ADD:          OpsBitflag = 1 << 0;
    if bit & ADD_ASSIGN != 0 {
        res.push(format_ident!("AddAssign"));
    }
    if bit & SUB != 0 {
        res.push(format_ident!("Sub"));
    }
    if bit & SUB_ASSIGN != 0 {
        res.push(format_ident!("SubAssign"));
    }
    if bit & MUL != 0 {
        res.push(format_ident!("Mul"));
    }
    if bit & MUL_ASSIGN != 0 {
        res.push(format_ident!("MulAssign"));
    }
    if bit & DIV != 0 {
        res.push(format_ident!("Div"));
    }
    if bit & DIV_ASSIGN != 0 {
        res.push(format_ident!("DivAssign"));
    }
    if bit & REM != 0 {
        res.push(format_ident!("Rem"));
    }
    if bit & REM_ASSIGN != 0 {
        res.push(format_ident!("RemAssign"));
    }
    if bit & NEG != 0 {
        res.push(format_ident!("Neg"));
    }
    #[cfg(feature="bit_ops")] {
        if bit & NOT != 0 {
            res.push(format_ident!("Not"))
        }
        if bit & BIT_AND != 0 {
            res.push(format_ident!("BitAnd"))
        }
        if bit & BIT_AND_ASSIGN != 0 {
            res.push(format_ident!("BitAndAssign"))
        }
        if bit & BIT_OR != 0 {
            res.push(format_ident!("BitOr"))
        }
        if bit & BIT_OR_ASSIGN != 0 {
            res.push(format_ident!("BitOrAssign"))
        }
        if bit & BIT_XOR != 0 {
            res.push(format_ident!("BitXor"))
        }
        if bit & BIT_XOR_ASSIGN != 0 {
            res.push(format_ident!("BitXorAssign"))
        }
        if bit & SHL != 0 {
            res.push(format_ident!("Shl"))
        }
        if bit & SHL_ASSIGN != 0 {
            res.push(format_ident!("ShlAssign"))
        }
        if bit & SHR != 0 {
            res.push(format_ident!("Shr"))
        }
        if bit & SHR_ASSIGN != 0 {
            res.push(format_ident!("ShrAssign"))
        }
    }
    #[cfg(feature="iter_ops")]
    {
        if bit & SUM != 0 {
            res.push(format_ident!("Sum"));
        }
        if bit & PRODUCT != 0 {
            res.push(format_ident!("Product"));
        }
    }
    res
}



pub type TargetBiflag = u16;
pub const I8:       TargetBiflag = 1 << 0;
pub const I16:      TargetBiflag = 1 << 1;
pub const I32:      TargetBiflag = 1 << 2;
pub const I64:      TargetBiflag = 1 << 3;
pub const I128:     TargetBiflag = 1 << 4;
pub const ISIZE:    TargetBiflag = 1 << 5;
pub const U8:       TargetBiflag = 1 << 6;
pub const U16:      TargetBiflag = 1 << 7;
pub const U32:      TargetBiflag = 1 << 8;
pub const U64:      TargetBiflag = 1 << 9;
pub const U128:     TargetBiflag = 1 << 10;
pub const USIZE:    TargetBiflag = 1 << 11;

pub const SIGNED:   TargetBiflag = I8 | I16 | I32 | I64 | I128 | ISIZE;
pub const UNSIGNED: TargetBiflag = U8 | U16 | U32 | U64 | U128 | USIZE;
pub const INTEGER:  TargetBiflag = SIGNED | UNSIGNED;

pub fn bit_to_types(bit: TargetBiflag) -> Vec<Ident> {
    let mut res = Vec::new();
    if bit & I8 != 0 {
        res.push(format_ident!("i8"));
    }
    if bit & I16 != 0 {
        res.push(format_ident!("i16"));
    }
    if bit & I32 != 0 {
        res.push(format_ident!("i32"));
    }
    if bit & I64 != 0 {
        res.push(format_ident!("i64"));
    }
    if bit & I128 != 0 {
        res.push(format_ident!("i128"));
    }
    if bit & ISIZE != 0 {
        res.push(format_ident!("isize"));
    }
    if bit & U8 != 0 {
        res.push(format_ident!("u8"));
    }
    if bit & U16 != 0 {
        res.push(format_ident!("u16"));
    }
    if bit & U32 != 0 {
        res.push(format_ident!("u32"));
    }
    if bit & U64 != 0 {
        res.push(format_ident!("u64"));
    }
    if bit & U128 != 0 {
        res.push(format_ident!("u128"));
    }
    if bit & USIZE != 0 {
        res.push(format_ident!("usize"));
    }
    res
}