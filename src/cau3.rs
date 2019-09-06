#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
  #[doc = "0x00 - Processor Core Type"]
  pub pct: PCT,
  #[doc = "0x04 - Memory Configuration"]
  pub mcfg: MCFG,
  _reserved2: [u8; 8usize],
  #[doc = "0x10 - Control Register"]
  pub cr: CR,
  #[doc = "0x14 - Status Register"]
  pub sr: SR,
  _reserved4: [u8; 8usize],
  #[doc = "0x20 - Debug Control/Status Register"]
  pub dbgcsr: DBGCSR,
  #[doc = "0x24 - Debug PC Breakpoint Register"]
  pub dbgpbr: DBGPBR,
  _reserved6: [u8; 8usize],
  #[doc = "0x30 - Debug Memory Command Register"]
  pub dbgmcmd: DBGMCMD,
  #[doc = "0x34 - Debug Memory Address Register"]
  pub dbgmadr: DBGMADR,
  #[doc = "0x38 - Debug Memory Data Register"]
  pub dbgmdr: DBGMDR,
  _reserved9: [u8; 180usize],
  #[doc = "0xf0 - Semaphore Register"]
  pub sema4: SEMA4,
  #[doc = "0xf4 - Semaphore Ownership Register"]
  pub smownr: SMOWNR,
  _reserved11: [u8; 4usize],
  #[doc = "0xfc - Address Remap Register"]
  pub arr: ARR,
  _reserved12: [u8; 128usize],
  #[doc = "0x180 - CryptoCore General Purpose Registers"]
  pub cc_r0: CC_R0,
  #[doc = "0x184 - CryptoCore General Purpose Registers"]
  pub cc_r1: CC_R1,
  #[doc = "0x188 - CryptoCore General Purpose Registers"]
  pub cc_r2: CC_R2,
  #[doc = "0x18c - CryptoCore General Purpose Registers"]
  pub cc_r3: CC_R3,
  #[doc = "0x190 - CryptoCore General Purpose Registers"]
  pub cc_r4: CC_R4,
  #[doc = "0x194 - CryptoCore General Purpose Registers"]
  pub cc_r5: CC_R5,
  #[doc = "0x198 - CryptoCore General Purpose Registers"]
  pub cc_r6: CC_R6,
  #[doc = "0x19c - CryptoCore General Purpose Registers"]
  pub cc_r7: CC_R7,
  #[doc = "0x1a0 - CryptoCore General Purpose Registers"]
  pub cc_r8: CC_R8,
  #[doc = "0x1a4 - CryptoCore General Purpose Registers"]
  pub cc_r9: CC_R9,
  #[doc = "0x1a8 - CryptoCore General Purpose Registers"]
  pub cc_r10: CC_R10,
  #[doc = "0x1ac - CryptoCore General Purpose Registers"]
  pub cc_r11: CC_R11,
  #[doc = "0x1b0 - CryptoCore General Purpose Registers"]
  pub cc_r12: CC_R12,
  #[doc = "0x1b4 - CryptoCore General Purpose Registers"]
  pub cc_r13: CC_R13,
  #[doc = "0x1b8 - CryptoCore General Purpose Registers"]
  pub cc_r14: CC_R14,
  #[doc = "0x1bc - CryptoCore General Purpose Registers"]
  pub cc_r15: CC_R15,
  #[doc = "0x1c0 - CryptoCore General Purpose Registers"]
  pub cc_r16: CC_R16,
  #[doc = "0x1c4 - CryptoCore General Purpose Registers"]
  pub cc_r17: CC_R17,
  #[doc = "0x1c8 - CryptoCore General Purpose Registers"]
  pub cc_r18: CC_R18,
  #[doc = "0x1cc - CryptoCore General Purpose Registers"]
  pub cc_r19: CC_R19,
  #[doc = "0x1d0 - CryptoCore General Purpose Registers"]
  pub cc_r20: CC_R20,
  #[doc = "0x1d4 - CryptoCore General Purpose Registers"]
  pub cc_r21: CC_R21,
  #[doc = "0x1d8 - CryptoCore General Purpose Registers"]
  pub cc_r22: CC_R22,
  #[doc = "0x1dc - CryptoCore General Purpose Registers"]
  pub cc_r23: CC_R23,
  #[doc = "0x1e0 - CryptoCore General Purpose Registers"]
  pub cc_r24: CC_R24,
  #[doc = "0x1e4 - CryptoCore General Purpose Registers"]
  pub cc_r25: CC_R25,
  #[doc = "0x1e8 - CryptoCore General Purpose Registers"]
  pub cc_r26: CC_R26,
  #[doc = "0x1ec - CryptoCore General Purpose Registers"]
  pub cc_r27: CC_R27,
  #[doc = "0x1f0 - CryptoCore General Purpose Registers"]
  pub cc_r28: CC_R28,
  #[doc = "0x1f4 - CryptoCore General Purpose Registers"]
  pub cc_r29: CC_R29,
  #[doc = "0x1f8 - General Purpose R30"]
  pub cc_r30: CC_R30,
  #[doc = "0x1fc - General Purpose R31"]
  pub cc_r31: CC_R31,
  #[doc = "0x200 - Program Counter"]
  pub cc_pc: CC_PC,
  #[doc = "0x204 - Start Command Register"]
  pub cc_cmd: CC_CMD,
  #[doc = "0x208 - Condition Flag"]
  pub cc_cf: CC_CF,
  _reserved47: [u8; 500usize],
  #[doc = "0x400 - Mode Register (PublicKey)"]
  pub mdpk: MDPK,
  _reserved48: [u8; 44usize],
  #[doc = "0x430 - Command Register"]
  pub com: COM,
  #[doc = "0x434 - Control Register"]
  pub ctl: CTL,
  _reserved50: [u8; 8usize],
  #[doc = "0x440 - Clear Written Register"]
  pub cw: CW,
  _reserved51: [u8; 4usize],
  #[doc = "0x448 - Status Register"]
  pub sta: STA,
  #[doc = "0x44c - Error Status Register"]
  pub esta: ESTA,
  _reserved53: [u8; 48usize],
  #[doc = "0x480 - PKHA A Size Register"]
  pub pkasz: PKASZ,
  _reserved54: [u8; 4usize],
  #[doc = "0x488 - PKHA B Size Register"]
  pub pkbsz: PKBSZ,
  _reserved55: [u8; 4usize],
  #[doc = "0x490 - PKHA N Size Register"]
  pub pknsz: PKNSZ,
  _reserved56: [u8; 4usize],
  #[doc = "0x498 - PKHA E Size Register"]
  pub pkesz: PKESZ,
  _reserved57: [u8; 84usize],
  #[doc = "0x4f0 - PKHA Revision ID 1"]
  pub pkha_vid1: PKHA_VID1,
  #[doc = "0x4f4 - PKHA Revision ID 2"]
  pub pkha_vid2: PKHA_VID2,
  #[doc = "0x4f8 - CHA Revision ID"]
  pub cha_vid: CHA_VID,
  _reserved60: [u8; 260usize],
  #[doc = "0x600 - PKHA Clock Control Register"]
  pub pkha_ccr: PKHA_CCR,
  #[doc = "0x604 - Global Status Register"]
  pub gsr: GSR,
  #[doc = "0x608 - Clock Linear Feedback Shift Register"]
  pub cklfsr: CKLFSR,
  _reserved63: [u8; 500usize],
  #[doc = "0x800 - PKHA A0 Register"]
  pub pka0_: [PKA0_; 32],
  #[doc = "0x880 - PKHA A1 Register"]
  pub pka1_: [PKA1_; 32],
  #[doc = "0x900 - PKHA A2 Register"]
  pub pka2_: [PKA2_; 32],
  #[doc = "0x980 - PKHA A3 Register"]
  pub pka3_: [PKA3_; 32],
  #[doc = "0xa00 - PKHA B0 Register"]
  pub pkb0_: [PKB0_; 32],
  #[doc = "0xa80 - PKHA B1 Register"]
  pub pkb1_: [PKB1_; 32],
  #[doc = "0xb00 - PKHA B2 Register"]
  pub pkb2_: [PKB2_; 32],
  #[doc = "0xb80 - PKHA B3 Register"]
  pub pkb3_: [PKB3_; 32],
  #[doc = "0xc00 - PKHA N0 Register"]
  pub pkn0_: [PKN0_; 32],
  #[doc = "0xc80 - PKHA N1 Register"]
  pub pkn1_: [PKN1_; 32],
  #[doc = "0xd00 - PKHA N2 Register"]
  pub pkn2_: [PKN2_; 32],
  #[doc = "0xd80 - PKHA N3 Register"]
  pub pkn3_: [PKN3_; 32],
  #[doc = "0xe00 - PKHA E Register"]
  pub pke_: [PKE_; 128],
}
#[doc = "Processor Core Type\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pct](pct) module"]
pub type PCT = crate::Reg<u32, _PCT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCT;
#[doc = "`read()` method returns [pct::R](pct::R) reader structure"]
impl crate::Readable for PCT {}
#[doc = "Processor Core Type"]
pub mod pct;
#[doc = "Memory Configuration\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mcfg](mcfg) module"]
pub type MCFG = crate::Reg<u32, _MCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCFG;
#[doc = "`read()` method returns [mcfg::R](mcfg::R) reader structure"]
impl crate::Readable for MCFG {}
#[doc = "Memory Configuration"]
pub mod mcfg;
#[doc = "Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cr](cr) module"]
pub type CR = crate::Reg<u32, _CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR;
#[doc = "`read()` method returns [cr::R](cr::R) reader structure"]
impl crate::Readable for CR {}
#[doc = "`write(|w| ..)` method takes [cr::W](cr::W) writer structure"]
impl crate::Writable for CR {}
#[doc = "Control Register"]
pub mod cr;
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sr](sr) module"]
pub type SR = crate::Reg<u32, _SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SR;
#[doc = "`read()` method returns [sr::R](sr::R) reader structure"]
impl crate::Readable for SR {}
#[doc = "`write(|w| ..)` method takes [sr::W](sr::W) writer structure"]
impl crate::Writable for SR {}
#[doc = "Status Register"]
pub mod sr;
#[doc = "Debug Control/Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dbgcsr](dbgcsr) module"]
pub type DBGCSR = crate::Reg<u32, _DBGCSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DBGCSR;
#[doc = "`read()` method returns [dbgcsr::R](dbgcsr::R) reader structure"]
impl crate::Readable for DBGCSR {}
#[doc = "`write(|w| ..)` method takes [dbgcsr::W](dbgcsr::W) writer structure"]
impl crate::Writable for DBGCSR {}
#[doc = "Debug Control/Status Register"]
pub mod dbgcsr;
#[doc = "Debug PC Breakpoint Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dbgpbr](dbgpbr) module"]
pub type DBGPBR = crate::Reg<u32, _DBGPBR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DBGPBR;
#[doc = "`read()` method returns [dbgpbr::R](dbgpbr::R) reader structure"]
impl crate::Readable for DBGPBR {}
#[doc = "`write(|w| ..)` method takes [dbgpbr::W](dbgpbr::W) writer structure"]
impl crate::Writable for DBGPBR {}
#[doc = "Debug PC Breakpoint Register"]
pub mod dbgpbr;
#[doc = "Debug Memory Command Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dbgmcmd](dbgmcmd) module"]
pub type DBGMCMD = crate::Reg<u32, _DBGMCMD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DBGMCMD;
#[doc = "`read()` method returns [dbgmcmd::R](dbgmcmd::R) reader structure"]
impl crate::Readable for DBGMCMD {}
#[doc = "`write(|w| ..)` method takes [dbgmcmd::W](dbgmcmd::W) writer structure"]
impl crate::Writable for DBGMCMD {}
#[doc = "Debug Memory Command Register"]
pub mod dbgmcmd;
#[doc = "Debug Memory Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dbgmadr](dbgmadr) module"]
pub type DBGMADR = crate::Reg<u32, _DBGMADR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DBGMADR;
#[doc = "`read()` method returns [dbgmadr::R](dbgmadr::R) reader structure"]
impl crate::Readable for DBGMADR {}
#[doc = "`write(|w| ..)` method takes [dbgmadr::W](dbgmadr::W) writer structure"]
impl crate::Writable for DBGMADR {}
#[doc = "Debug Memory Address Register"]
pub mod dbgmadr;
#[doc = "Debug Memory Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dbgmdr](dbgmdr) module"]
pub type DBGMDR = crate::Reg<u32, _DBGMDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DBGMDR;
#[doc = "`read()` method returns [dbgmdr::R](dbgmdr::R) reader structure"]
impl crate::Readable for DBGMDR {}
#[doc = "`write(|w| ..)` method takes [dbgmdr::W](dbgmdr::W) writer structure"]
impl crate::Writable for DBGMDR {}
#[doc = "Debug Memory Data Register"]
pub mod dbgmdr;
#[doc = "Semaphore Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sema4](sema4) module"]
pub type SEMA4 = crate::Reg<u32, _SEMA4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEMA4;
#[doc = "`read()` method returns [sema4::R](sema4::R) reader structure"]
impl crate::Readable for SEMA4 {}
#[doc = "`write(|w| ..)` method takes [sema4::W](sema4::W) writer structure"]
impl crate::Writable for SEMA4 {}
#[doc = "Semaphore Register"]
pub mod sema4;
#[doc = "Semaphore Ownership Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [smownr](smownr) module"]
pub type SMOWNR = crate::Reg<u32, _SMOWNR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SMOWNR;
#[doc = "`read()` method returns [smownr::R](smownr::R) reader structure"]
impl crate::Readable for SMOWNR {}
#[doc = "Semaphore Ownership Register"]
pub mod smownr;
#[doc = "Address Remap Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [arr](arr) module"]
pub type ARR = crate::Reg<u32, _ARR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ARR;
#[doc = "`read()` method returns [arr::R](arr::R) reader structure"]
impl crate::Readable for ARR {}
#[doc = "`write(|w| ..)` method takes [arr::W](arr::W) writer structure"]
impl crate::Writable for ARR {}
#[doc = "Address Remap Register"]
pub mod arr;
#[doc = "CryptoCore General Purpose Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cc_r0](cc_r0) module"]
pub type CC_R0 = crate::Reg<u32, _CC_R0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CC_R0;
#[doc = "`read()` method returns [cc_r0::R](cc_r0::R) reader structure"]
impl crate::Readable for CC_R0 {}
#[doc = "`write(|w| ..)` method takes [cc_r0::W](cc_r0::W) writer structure"]
impl crate::Writable for CC_R0 {}
#[doc = "CryptoCore General Purpose Registers"]
pub mod cc_r0;
#[doc = "CryptoCore General Purpose Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cc_r1](cc_r1) module"]
pub type CC_R1 = crate::Reg<u32, _CC_R1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CC_R1;
#[doc = "`read()` method returns [cc_r1::R](cc_r1::R) reader structure"]
impl crate::Readable for CC_R1 {}
#[doc = "`write(|w| ..)` method takes [cc_r1::W](cc_r1::W) writer structure"]
impl crate::Writable for CC_R1 {}
#[doc = "CryptoCore General Purpose Registers"]
pub mod cc_r1;
#[doc = "CryptoCore General Purpose Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cc_r2](cc_r2) module"]
pub type CC_R2 = crate::Reg<u32, _CC_R2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CC_R2;
#[doc = "`read()` method returns [cc_r2::R](cc_r2::R) reader structure"]
impl crate::Readable for CC_R2 {}
#[doc = "`write(|w| ..)` method takes [cc_r2::W](cc_r2::W) writer structure"]
impl crate::Writable for CC_R2 {}
#[doc = "CryptoCore General Purpose Registers"]
pub mod cc_r2;
#[doc = "CryptoCore General Purpose Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cc_r3](cc_r3) module"]
pub type CC_R3 = crate::Reg<u32, _CC_R3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CC_R3;
#[doc = "`read()` method returns [cc_r3::R](cc_r3::R) reader structure"]
impl crate::Readable for CC_R3 {}
#[doc = "`write(|w| ..)` method takes [cc_r3::W](cc_r3::W) writer structure"]
impl crate::Writable for CC_R3 {}
#[doc = "CryptoCore General Purpose Registers"]
pub mod cc_r3;
#[doc = "CryptoCore General Purpose Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cc_r4](cc_r4) module"]
pub type CC_R4 = crate::Reg<u32, _CC_R4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CC_R4;
#[doc = "`read()` method returns [cc_r4::R](cc_r4::R) reader structure"]
impl crate::Readable for CC_R4 {}
#[doc = "`write(|w| ..)` method takes [cc_r4::W](cc_r4::W) writer structure"]
impl crate::Writable for CC_R4 {}
#[doc = "CryptoCore General Purpose Registers"]
pub mod cc_r4;
#[doc = "CryptoCore General Purpose Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cc_r5](cc_r5) module"]
pub type CC_R5 = crate::Reg<u32, _CC_R5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CC_R5;
#[doc = "`read()` method returns [cc_r5::R](cc_r5::R) reader structure"]
impl crate::Readable for CC_R5 {}
#[doc = "`write(|w| ..)` method takes [cc_r5::W](cc_r5::W) writer structure"]
impl crate::Writable for CC_R5 {}
#[doc = "CryptoCore General Purpose Registers"]
pub mod cc_r5;
#[doc = "CryptoCore General Purpose Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cc_r6](cc_r6) module"]
pub type CC_R6 = crate::Reg<u32, _CC_R6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CC_R6;
#[doc = "`read()` method returns [cc_r6::R](cc_r6::R) reader structure"]
impl crate::Readable for CC_R6 {}
#[doc = "`write(|w| ..)` method takes [cc_r6::W](cc_r6::W) writer structure"]
impl crate::Writable for CC_R6 {}
#[doc = "CryptoCore General Purpose Registers"]
pub mod cc_r6;
#[doc = "CryptoCore General Purpose Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cc_r7](cc_r7) module"]
pub type CC_R7 = crate::Reg<u32, _CC_R7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CC_R7;
#[doc = "`read()` method returns [cc_r7::R](cc_r7::R) reader structure"]
impl crate::Readable for CC_R7 {}
#[doc = "`write(|w| ..)` method takes [cc_r7::W](cc_r7::W) writer structure"]
impl crate::Writable for CC_R7 {}
#[doc = "CryptoCore General Purpose Registers"]
pub mod cc_r7;
#[doc = "CryptoCore General Purpose Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cc_r8](cc_r8) module"]
pub type CC_R8 = crate::Reg<u32, _CC_R8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CC_R8;
#[doc = "`read()` method returns [cc_r8::R](cc_r8::R) reader structure"]
impl crate::Readable for CC_R8 {}
#[doc = "`write(|w| ..)` method takes [cc_r8::W](cc_r8::W) writer structure"]
impl crate::Writable for CC_R8 {}
#[doc = "CryptoCore General Purpose Registers"]
pub mod cc_r8;
#[doc = "CryptoCore General Purpose Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cc_r9](cc_r9) module"]
pub type CC_R9 = crate::Reg<u32, _CC_R9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CC_R9;
#[doc = "`read()` method returns [cc_r9::R](cc_r9::R) reader structure"]
impl crate::Readable for CC_R9 {}
#[doc = "`write(|w| ..)` method takes [cc_r9::W](cc_r9::W) writer structure"]
impl crate::Writable for CC_R9 {}
#[doc = "CryptoCore General Purpose Registers"]
pub mod cc_r9;
#[doc = "CryptoCore General Purpose Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cc_r10](cc_r10) module"]
pub type CC_R10 = crate::Reg<u32, _CC_R10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CC_R10;
#[doc = "`read()` method returns [cc_r10::R](cc_r10::R) reader structure"]
impl crate::Readable for CC_R10 {}
#[doc = "`write(|w| ..)` method takes [cc_r10::W](cc_r10::W) writer structure"]
impl crate::Writable for CC_R10 {}
#[doc = "CryptoCore General Purpose Registers"]
pub mod cc_r10;
#[doc = "CryptoCore General Purpose Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cc_r11](cc_r11) module"]
pub type CC_R11 = crate::Reg<u32, _CC_R11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CC_R11;
#[doc = "`read()` method returns [cc_r11::R](cc_r11::R) reader structure"]
impl crate::Readable for CC_R11 {}
#[doc = "`write(|w| ..)` method takes [cc_r11::W](cc_r11::W) writer structure"]
impl crate::Writable for CC_R11 {}
#[doc = "CryptoCore General Purpose Registers"]
pub mod cc_r11;
#[doc = "CryptoCore General Purpose Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cc_r12](cc_r12) module"]
pub type CC_R12 = crate::Reg<u32, _CC_R12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CC_R12;
#[doc = "`read()` method returns [cc_r12::R](cc_r12::R) reader structure"]
impl crate::Readable for CC_R12 {}
#[doc = "`write(|w| ..)` method takes [cc_r12::W](cc_r12::W) writer structure"]
impl crate::Writable for CC_R12 {}
#[doc = "CryptoCore General Purpose Registers"]
pub mod cc_r12;
#[doc = "CryptoCore General Purpose Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cc_r13](cc_r13) module"]
pub type CC_R13 = crate::Reg<u32, _CC_R13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CC_R13;
#[doc = "`read()` method returns [cc_r13::R](cc_r13::R) reader structure"]
impl crate::Readable for CC_R13 {}
#[doc = "`write(|w| ..)` method takes [cc_r13::W](cc_r13::W) writer structure"]
impl crate::Writable for CC_R13 {}
#[doc = "CryptoCore General Purpose Registers"]
pub mod cc_r13;
#[doc = "CryptoCore General Purpose Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cc_r14](cc_r14) module"]
pub type CC_R14 = crate::Reg<u32, _CC_R14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CC_R14;
#[doc = "`read()` method returns [cc_r14::R](cc_r14::R) reader structure"]
impl crate::Readable for CC_R14 {}
#[doc = "`write(|w| ..)` method takes [cc_r14::W](cc_r14::W) writer structure"]
impl crate::Writable for CC_R14 {}
#[doc = "CryptoCore General Purpose Registers"]
pub mod cc_r14;
#[doc = "CryptoCore General Purpose Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cc_r15](cc_r15) module"]
pub type CC_R15 = crate::Reg<u32, _CC_R15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CC_R15;
#[doc = "`read()` method returns [cc_r15::R](cc_r15::R) reader structure"]
impl crate::Readable for CC_R15 {}
#[doc = "`write(|w| ..)` method takes [cc_r15::W](cc_r15::W) writer structure"]
impl crate::Writable for CC_R15 {}
#[doc = "CryptoCore General Purpose Registers"]
pub mod cc_r15;
#[doc = "CryptoCore General Purpose Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cc_r16](cc_r16) module"]
pub type CC_R16 = crate::Reg<u32, _CC_R16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CC_R16;
#[doc = "`read()` method returns [cc_r16::R](cc_r16::R) reader structure"]
impl crate::Readable for CC_R16 {}
#[doc = "`write(|w| ..)` method takes [cc_r16::W](cc_r16::W) writer structure"]
impl crate::Writable for CC_R16 {}
#[doc = "CryptoCore General Purpose Registers"]
pub mod cc_r16;
#[doc = "CryptoCore General Purpose Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cc_r17](cc_r17) module"]
pub type CC_R17 = crate::Reg<u32, _CC_R17>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CC_R17;
#[doc = "`read()` method returns [cc_r17::R](cc_r17::R) reader structure"]
impl crate::Readable for CC_R17 {}
#[doc = "`write(|w| ..)` method takes [cc_r17::W](cc_r17::W) writer structure"]
impl crate::Writable for CC_R17 {}
#[doc = "CryptoCore General Purpose Registers"]
pub mod cc_r17;
#[doc = "CryptoCore General Purpose Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cc_r18](cc_r18) module"]
pub type CC_R18 = crate::Reg<u32, _CC_R18>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CC_R18;
#[doc = "`read()` method returns [cc_r18::R](cc_r18::R) reader structure"]
impl crate::Readable for CC_R18 {}
#[doc = "`write(|w| ..)` method takes [cc_r18::W](cc_r18::W) writer structure"]
impl crate::Writable for CC_R18 {}
#[doc = "CryptoCore General Purpose Registers"]
pub mod cc_r18;
#[doc = "CryptoCore General Purpose Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cc_r19](cc_r19) module"]
pub type CC_R19 = crate::Reg<u32, _CC_R19>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CC_R19;
#[doc = "`read()` method returns [cc_r19::R](cc_r19::R) reader structure"]
impl crate::Readable for CC_R19 {}
#[doc = "`write(|w| ..)` method takes [cc_r19::W](cc_r19::W) writer structure"]
impl crate::Writable for CC_R19 {}
#[doc = "CryptoCore General Purpose Registers"]
pub mod cc_r19;
#[doc = "CryptoCore General Purpose Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cc_r20](cc_r20) module"]
pub type CC_R20 = crate::Reg<u32, _CC_R20>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CC_R20;
#[doc = "`read()` method returns [cc_r20::R](cc_r20::R) reader structure"]
impl crate::Readable for CC_R20 {}
#[doc = "`write(|w| ..)` method takes [cc_r20::W](cc_r20::W) writer structure"]
impl crate::Writable for CC_R20 {}
#[doc = "CryptoCore General Purpose Registers"]
pub mod cc_r20;
#[doc = "CryptoCore General Purpose Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cc_r21](cc_r21) module"]
pub type CC_R21 = crate::Reg<u32, _CC_R21>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CC_R21;
#[doc = "`read()` method returns [cc_r21::R](cc_r21::R) reader structure"]
impl crate::Readable for CC_R21 {}
#[doc = "`write(|w| ..)` method takes [cc_r21::W](cc_r21::W) writer structure"]
impl crate::Writable for CC_R21 {}
#[doc = "CryptoCore General Purpose Registers"]
pub mod cc_r21;
#[doc = "CryptoCore General Purpose Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cc_r22](cc_r22) module"]
pub type CC_R22 = crate::Reg<u32, _CC_R22>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CC_R22;
#[doc = "`read()` method returns [cc_r22::R](cc_r22::R) reader structure"]
impl crate::Readable for CC_R22 {}
#[doc = "`write(|w| ..)` method takes [cc_r22::W](cc_r22::W) writer structure"]
impl crate::Writable for CC_R22 {}
#[doc = "CryptoCore General Purpose Registers"]
pub mod cc_r22;
#[doc = "CryptoCore General Purpose Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cc_r23](cc_r23) module"]
pub type CC_R23 = crate::Reg<u32, _CC_R23>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CC_R23;
#[doc = "`read()` method returns [cc_r23::R](cc_r23::R) reader structure"]
impl crate::Readable for CC_R23 {}
#[doc = "`write(|w| ..)` method takes [cc_r23::W](cc_r23::W) writer structure"]
impl crate::Writable for CC_R23 {}
#[doc = "CryptoCore General Purpose Registers"]
pub mod cc_r23;
#[doc = "CryptoCore General Purpose Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cc_r24](cc_r24) module"]
pub type CC_R24 = crate::Reg<u32, _CC_R24>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CC_R24;
#[doc = "`read()` method returns [cc_r24::R](cc_r24::R) reader structure"]
impl crate::Readable for CC_R24 {}
#[doc = "`write(|w| ..)` method takes [cc_r24::W](cc_r24::W) writer structure"]
impl crate::Writable for CC_R24 {}
#[doc = "CryptoCore General Purpose Registers"]
pub mod cc_r24;
#[doc = "CryptoCore General Purpose Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cc_r25](cc_r25) module"]
pub type CC_R25 = crate::Reg<u32, _CC_R25>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CC_R25;
#[doc = "`read()` method returns [cc_r25::R](cc_r25::R) reader structure"]
impl crate::Readable for CC_R25 {}
#[doc = "`write(|w| ..)` method takes [cc_r25::W](cc_r25::W) writer structure"]
impl crate::Writable for CC_R25 {}
#[doc = "CryptoCore General Purpose Registers"]
pub mod cc_r25;
#[doc = "CryptoCore General Purpose Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cc_r26](cc_r26) module"]
pub type CC_R26 = crate::Reg<u32, _CC_R26>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CC_R26;
#[doc = "`read()` method returns [cc_r26::R](cc_r26::R) reader structure"]
impl crate::Readable for CC_R26 {}
#[doc = "`write(|w| ..)` method takes [cc_r26::W](cc_r26::W) writer structure"]
impl crate::Writable for CC_R26 {}
#[doc = "CryptoCore General Purpose Registers"]
pub mod cc_r26;
#[doc = "CryptoCore General Purpose Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cc_r27](cc_r27) module"]
pub type CC_R27 = crate::Reg<u32, _CC_R27>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CC_R27;
#[doc = "`read()` method returns [cc_r27::R](cc_r27::R) reader structure"]
impl crate::Readable for CC_R27 {}
#[doc = "`write(|w| ..)` method takes [cc_r27::W](cc_r27::W) writer structure"]
impl crate::Writable for CC_R27 {}
#[doc = "CryptoCore General Purpose Registers"]
pub mod cc_r27;
#[doc = "CryptoCore General Purpose Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cc_r28](cc_r28) module"]
pub type CC_R28 = crate::Reg<u32, _CC_R28>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CC_R28;
#[doc = "`read()` method returns [cc_r28::R](cc_r28::R) reader structure"]
impl crate::Readable for CC_R28 {}
#[doc = "`write(|w| ..)` method takes [cc_r28::W](cc_r28::W) writer structure"]
impl crate::Writable for CC_R28 {}
#[doc = "CryptoCore General Purpose Registers"]
pub mod cc_r28;
#[doc = "CryptoCore General Purpose Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cc_r29](cc_r29) module"]
pub type CC_R29 = crate::Reg<u32, _CC_R29>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CC_R29;
#[doc = "`read()` method returns [cc_r29::R](cc_r29::R) reader structure"]
impl crate::Readable for CC_R29 {}
#[doc = "`write(|w| ..)` method takes [cc_r29::W](cc_r29::W) writer structure"]
impl crate::Writable for CC_R29 {}
#[doc = "CryptoCore General Purpose Registers"]
pub mod cc_r29;
#[doc = "General Purpose R30\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cc_r30](cc_r30) module"]
pub type CC_R30 = crate::Reg<u32, _CC_R30>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CC_R30;
#[doc = "`read()` method returns [cc_r30::R](cc_r30::R) reader structure"]
impl crate::Readable for CC_R30 {}
#[doc = "`write(|w| ..)` method takes [cc_r30::W](cc_r30::W) writer structure"]
impl crate::Writable for CC_R30 {}
#[doc = "General Purpose R30"]
pub mod cc_r30;
#[doc = "General Purpose R31\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cc_r31](cc_r31) module"]
pub type CC_R31 = crate::Reg<u32, _CC_R31>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CC_R31;
#[doc = "`read()` method returns [cc_r31::R](cc_r31::R) reader structure"]
impl crate::Readable for CC_R31 {}
#[doc = "`write(|w| ..)` method takes [cc_r31::W](cc_r31::W) writer structure"]
impl crate::Writable for CC_R31 {}
#[doc = "General Purpose R31"]
pub mod cc_r31;
#[doc = "Program Counter\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cc_pc](cc_pc) module"]
pub type CC_PC = crate::Reg<u32, _CC_PC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CC_PC;
#[doc = "`read()` method returns [cc_pc::R](cc_pc::R) reader structure"]
impl crate::Readable for CC_PC {}
#[doc = "`write(|w| ..)` method takes [cc_pc::W](cc_pc::W) writer structure"]
impl crate::Writable for CC_PC {}
#[doc = "Program Counter"]
pub mod cc_pc;
#[doc = "Start Command Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cc_cmd](cc_cmd) module"]
pub type CC_CMD = crate::Reg<u32, _CC_CMD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CC_CMD;
#[doc = "`read()` method returns [cc_cmd::R](cc_cmd::R) reader structure"]
impl crate::Readable for CC_CMD {}
#[doc = "`write(|w| ..)` method takes [cc_cmd::W](cc_cmd::W) writer structure"]
impl crate::Writable for CC_CMD {}
#[doc = "Start Command Register"]
pub mod cc_cmd;
#[doc = "Condition Flag\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cc_cf](cc_cf) module"]
pub type CC_CF = crate::Reg<u32, _CC_CF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CC_CF;
#[doc = "`read()` method returns [cc_cf::R](cc_cf::R) reader structure"]
impl crate::Readable for CC_CF {}
#[doc = "Condition Flag"]
pub mod cc_cf;
#[doc = "Mode Register (PublicKey)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mdpk](mdpk) module"]
pub type MDPK = crate::Reg<u32, _MDPK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDPK;
#[doc = "`read()` method returns [mdpk::R](mdpk::R) reader structure"]
impl crate::Readable for MDPK {}
#[doc = "`write(|w| ..)` method takes [mdpk::W](mdpk::W) writer structure"]
impl crate::Writable for MDPK {}
#[doc = "Mode Register (PublicKey)"]
pub mod mdpk;
#[doc = "Command Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [com](com) module"]
pub type COM = crate::Reg<u32, _COM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COM;
#[doc = "`read()` method returns [com::R](com::R) reader structure"]
impl crate::Readable for COM {}
#[doc = "`write(|w| ..)` method takes [com::W](com::W) writer structure"]
impl crate::Writable for COM {}
#[doc = "Command Register"]
pub mod com;
#[doc = "Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ctl](ctl) module"]
pub type CTL = crate::Reg<u32, _CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTL;
#[doc = "`read()` method returns [ctl::R](ctl::R) reader structure"]
impl crate::Readable for CTL {}
#[doc = "`write(|w| ..)` method takes [ctl::W](ctl::W) writer structure"]
impl crate::Writable for CTL {}
#[doc = "Control Register"]
pub mod ctl;
#[doc = "Clear Written Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cw](cw) module"]
pub type CW = crate::Reg<u32, _CW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CW;
#[doc = "`read()` method returns [cw::R](cw::R) reader structure"]
impl crate::Readable for CW {}
#[doc = "`write(|w| ..)` method takes [cw::W](cw::W) writer structure"]
impl crate::Writable for CW {}
#[doc = "Clear Written Register"]
pub mod cw;
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sta](sta) module"]
pub type STA = crate::Reg<u32, _STA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STA;
#[doc = "`read()` method returns [sta::R](sta::R) reader structure"]
impl crate::Readable for STA {}
#[doc = "`write(|w| ..)` method takes [sta::W](sta::W) writer structure"]
impl crate::Writable for STA {}
#[doc = "Status Register"]
pub mod sta;
#[doc = "Error Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [esta](esta) module"]
pub type ESTA = crate::Reg<u32, _ESTA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ESTA;
#[doc = "`read()` method returns [esta::R](esta::R) reader structure"]
impl crate::Readable for ESTA {}
#[doc = "Error Status Register"]
pub mod esta;
#[doc = "PKHA A Size Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pkasz](pkasz) module"]
pub type PKASZ = crate::Reg<u32, _PKASZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PKASZ;
#[doc = "`read()` method returns [pkasz::R](pkasz::R) reader structure"]
impl crate::Readable for PKASZ {}
#[doc = "`write(|w| ..)` method takes [pkasz::W](pkasz::W) writer structure"]
impl crate::Writable for PKASZ {}
#[doc = "PKHA A Size Register"]
pub mod pkasz;
#[doc = "PKHA B Size Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pkbsz](pkbsz) module"]
pub type PKBSZ = crate::Reg<u32, _PKBSZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PKBSZ;
#[doc = "`read()` method returns [pkbsz::R](pkbsz::R) reader structure"]
impl crate::Readable for PKBSZ {}
#[doc = "`write(|w| ..)` method takes [pkbsz::W](pkbsz::W) writer structure"]
impl crate::Writable for PKBSZ {}
#[doc = "PKHA B Size Register"]
pub mod pkbsz;
#[doc = "PKHA N Size Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pknsz](pknsz) module"]
pub type PKNSZ = crate::Reg<u32, _PKNSZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PKNSZ;
#[doc = "`read()` method returns [pknsz::R](pknsz::R) reader structure"]
impl crate::Readable for PKNSZ {}
#[doc = "`write(|w| ..)` method takes [pknsz::W](pknsz::W) writer structure"]
impl crate::Writable for PKNSZ {}
#[doc = "PKHA N Size Register"]
pub mod pknsz;
#[doc = "PKHA E Size Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pkesz](pkesz) module"]
pub type PKESZ = crate::Reg<u32, _PKESZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PKESZ;
#[doc = "`read()` method returns [pkesz::R](pkesz::R) reader structure"]
impl crate::Readable for PKESZ {}
#[doc = "`write(|w| ..)` method takes [pkesz::W](pkesz::W) writer structure"]
impl crate::Writable for PKESZ {}
#[doc = "PKHA E Size Register"]
pub mod pkesz;
#[doc = "PKHA Revision ID 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pkha_vid1](pkha_vid1) module"]
pub type PKHA_VID1 = crate::Reg<u32, _PKHA_VID1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PKHA_VID1;
#[doc = "`read()` method returns [pkha_vid1::R](pkha_vid1::R) reader structure"]
impl crate::Readable for PKHA_VID1 {}
#[doc = "PKHA Revision ID 1"]
pub mod pkha_vid1;
#[doc = "PKHA Revision ID 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pkha_vid2](pkha_vid2) module"]
pub type PKHA_VID2 = crate::Reg<u32, _PKHA_VID2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PKHA_VID2;
#[doc = "`read()` method returns [pkha_vid2::R](pkha_vid2::R) reader structure"]
impl crate::Readable for PKHA_VID2 {}
#[doc = "PKHA Revision ID 2"]
pub mod pkha_vid2;
#[doc = "CHA Revision ID\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cha_vid](cha_vid) module"]
pub type CHA_VID = crate::Reg<u32, _CHA_VID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHA_VID;
#[doc = "`read()` method returns [cha_vid::R](cha_vid::R) reader structure"]
impl crate::Readable for CHA_VID {}
#[doc = "CHA Revision ID"]
pub mod cha_vid;
#[doc = "PKHA Clock Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pkha_ccr](pkha_ccr) module"]
pub type PKHA_CCR = crate::Reg<u32, _PKHA_CCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PKHA_CCR;
#[doc = "`read()` method returns [pkha_ccr::R](pkha_ccr::R) reader structure"]
impl crate::Readable for PKHA_CCR {}
#[doc = "`write(|w| ..)` method takes [pkha_ccr::W](pkha_ccr::W) writer structure"]
impl crate::Writable for PKHA_CCR {}
#[doc = "PKHA Clock Control Register"]
pub mod pkha_ccr;
#[doc = "Global Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gsr](gsr) module"]
pub type GSR = crate::Reg<u32, _GSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GSR;
#[doc = "`read()` method returns [gsr::R](gsr::R) reader structure"]
impl crate::Readable for GSR {}
#[doc = "Global Status Register"]
pub mod gsr;
#[doc = "Clock Linear Feedback Shift Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cklfsr](cklfsr) module"]
pub type CKLFSR = crate::Reg<u32, _CKLFSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CKLFSR;
#[doc = "`read()` method returns [cklfsr::R](cklfsr::R) reader structure"]
impl crate::Readable for CKLFSR {}
#[doc = "`write(|w| ..)` method takes [cklfsr::W](cklfsr::W) writer structure"]
impl crate::Writable for CKLFSR {}
#[doc = "Clock Linear Feedback Shift Register"]
pub mod cklfsr;
#[doc = "PKHA A0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pka0_](pka0_) module"]
pub type PKA0_ = crate::Reg<u32, _PKA0_>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PKA0_;
#[doc = "`read()` method returns [pka0_::R](pka0_::R) reader structure"]
impl crate::Readable for PKA0_ {}
#[doc = "`write(|w| ..)` method takes [pka0_::W](pka0_::W) writer structure"]
impl crate::Writable for PKA0_ {}
#[doc = "PKHA A0 Register"]
pub mod pka0_;
#[doc = "PKHA A1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pka1_](pka1_) module"]
pub type PKA1_ = crate::Reg<u32, _PKA1_>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PKA1_;
#[doc = "`read()` method returns [pka1_::R](pka1_::R) reader structure"]
impl crate::Readable for PKA1_ {}
#[doc = "`write(|w| ..)` method takes [pka1_::W](pka1_::W) writer structure"]
impl crate::Writable for PKA1_ {}
#[doc = "PKHA A1 Register"]
pub mod pka1_;
#[doc = "PKHA A2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pka2_](pka2_) module"]
pub type PKA2_ = crate::Reg<u32, _PKA2_>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PKA2_;
#[doc = "`read()` method returns [pka2_::R](pka2_::R) reader structure"]
impl crate::Readable for PKA2_ {}
#[doc = "`write(|w| ..)` method takes [pka2_::W](pka2_::W) writer structure"]
impl crate::Writable for PKA2_ {}
#[doc = "PKHA A2 Register"]
pub mod pka2_;
#[doc = "PKHA A3 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pka3_](pka3_) module"]
pub type PKA3_ = crate::Reg<u32, _PKA3_>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PKA3_;
#[doc = "`read()` method returns [pka3_::R](pka3_::R) reader structure"]
impl crate::Readable for PKA3_ {}
#[doc = "`write(|w| ..)` method takes [pka3_::W](pka3_::W) writer structure"]
impl crate::Writable for PKA3_ {}
#[doc = "PKHA A3 Register"]
pub mod pka3_;
#[doc = "PKHA B0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pkb0_](pkb0_) module"]
pub type PKB0_ = crate::Reg<u32, _PKB0_>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PKB0_;
#[doc = "`read()` method returns [pkb0_::R](pkb0_::R) reader structure"]
impl crate::Readable for PKB0_ {}
#[doc = "`write(|w| ..)` method takes [pkb0_::W](pkb0_::W) writer structure"]
impl crate::Writable for PKB0_ {}
#[doc = "PKHA B0 Register"]
pub mod pkb0_;
#[doc = "PKHA B1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pkb1_](pkb1_) module"]
pub type PKB1_ = crate::Reg<u32, _PKB1_>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PKB1_;
#[doc = "`read()` method returns [pkb1_::R](pkb1_::R) reader structure"]
impl crate::Readable for PKB1_ {}
#[doc = "`write(|w| ..)` method takes [pkb1_::W](pkb1_::W) writer structure"]
impl crate::Writable for PKB1_ {}
#[doc = "PKHA B1 Register"]
pub mod pkb1_;
#[doc = "PKHA B2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pkb2_](pkb2_) module"]
pub type PKB2_ = crate::Reg<u32, _PKB2_>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PKB2_;
#[doc = "`read()` method returns [pkb2_::R](pkb2_::R) reader structure"]
impl crate::Readable for PKB2_ {}
#[doc = "`write(|w| ..)` method takes [pkb2_::W](pkb2_::W) writer structure"]
impl crate::Writable for PKB2_ {}
#[doc = "PKHA B2 Register"]
pub mod pkb2_;
#[doc = "PKHA B3 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pkb3_](pkb3_) module"]
pub type PKB3_ = crate::Reg<u32, _PKB3_>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PKB3_;
#[doc = "`read()` method returns [pkb3_::R](pkb3_::R) reader structure"]
impl crate::Readable for PKB3_ {}
#[doc = "`write(|w| ..)` method takes [pkb3_::W](pkb3_::W) writer structure"]
impl crate::Writable for PKB3_ {}
#[doc = "PKHA B3 Register"]
pub mod pkb3_;
#[doc = "PKHA N0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pkn0_](pkn0_) module"]
pub type PKN0_ = crate::Reg<u32, _PKN0_>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PKN0_;
#[doc = "`read()` method returns [pkn0_::R](pkn0_::R) reader structure"]
impl crate::Readable for PKN0_ {}
#[doc = "`write(|w| ..)` method takes [pkn0_::W](pkn0_::W) writer structure"]
impl crate::Writable for PKN0_ {}
#[doc = "PKHA N0 Register"]
pub mod pkn0_;
#[doc = "PKHA N1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pkn1_](pkn1_) module"]
pub type PKN1_ = crate::Reg<u32, _PKN1_>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PKN1_;
#[doc = "`read()` method returns [pkn1_::R](pkn1_::R) reader structure"]
impl crate::Readable for PKN1_ {}
#[doc = "`write(|w| ..)` method takes [pkn1_::W](pkn1_::W) writer structure"]
impl crate::Writable for PKN1_ {}
#[doc = "PKHA N1 Register"]
pub mod pkn1_;
#[doc = "PKHA N2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pkn2_](pkn2_) module"]
pub type PKN2_ = crate::Reg<u32, _PKN2_>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PKN2_;
#[doc = "`read()` method returns [pkn2_::R](pkn2_::R) reader structure"]
impl crate::Readable for PKN2_ {}
#[doc = "`write(|w| ..)` method takes [pkn2_::W](pkn2_::W) writer structure"]
impl crate::Writable for PKN2_ {}
#[doc = "PKHA N2 Register"]
pub mod pkn2_;
#[doc = "PKHA N3 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pkn3_](pkn3_) module"]
pub type PKN3_ = crate::Reg<u32, _PKN3_>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PKN3_;
#[doc = "`read()` method returns [pkn3_::R](pkn3_::R) reader structure"]
impl crate::Readable for PKN3_ {}
#[doc = "`write(|w| ..)` method takes [pkn3_::W](pkn3_::W) writer structure"]
impl crate::Writable for PKN3_ {}
#[doc = "PKHA N3 Register"]
pub mod pkn3_;
#[doc = "PKHA E Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pke_](pke_) module"]
pub type PKE_ = crate::Reg<u32, _PKE_>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PKE_;
#[doc = "`write(|w| ..)` method takes [pke_::W](pke_::W) writer structure"]
impl crate::Writable for PKE_ {}
#[doc = "PKHA E Register"]
pub mod pke_;
