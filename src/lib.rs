#![feature(generic_const_exprs)]

mod macros;
pub(crate) mod utils;

use core::ops;

use crate::utils::*;

ifxp!(IFxp8, i8);
ifxp!(IFxp16, i16);
ifxp!(IFxp32, i32);
ifxp!(IFxp64, i64);
ifxp!(IFxp128, i128);

ufxp!(UFxp8, u8);
ufxp!(UFxp16, u16);
ufxp!(UFxp32, u32);
ufxp!(UFxp64, u64);
ufxp!(UFxp128, u128);
