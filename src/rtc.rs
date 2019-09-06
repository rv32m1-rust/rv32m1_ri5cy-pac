#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
  #[doc = "0x00 - RTC Time Seconds Register"]
  pub tsr: TSR,
  #[doc = "0x04 - RTC Time Prescaler Register"]
  pub tpr: TPR,
  #[doc = "0x08 - RTC Time Alarm Register"]
  pub tar: TAR,
  #[doc = "0x0c - RTC Time Compensation Register"]
  pub tcr: TCR,
  #[doc = "0x10 - RTC Control Register"]
  pub cr: CR,
  #[doc = "0x14 - RTC Status Register"]
  pub sr: SR,
  #[doc = "0x18 - RTC Lock Register"]
  pub lr: LR,
  #[doc = "0x1c - RTC Interrupt Enable Register"]
  pub ier: IER,
  #[doc = "0x20 - RTC Tamper Time Seconds Register"]
  pub ttsr: TTSR,
  #[doc = "0x24 - RTC Monotonic Enable Register"]
  pub mer: MER,
  #[doc = "0x28 - RTC Monotonic Counter Low Register"]
  pub mclr: MCLR,
  #[doc = "0x2c - RTC Monotonic Counter High Register"]
  pub mchr: MCHR,
  _reserved12: [u8; 4usize],
  #[doc = "0x34 - RTC Tamper Detect Register"]
  pub tdr: TDR,
  _reserved13: [u8; 4usize],
  #[doc = "0x3c - RTC Tamper Interrupt Register"]
  pub tir: TIR,
  #[doc = "0x40 - RTC Pin Configuration Register"]
  pub pcr: [PCR; 4],
  _reserved15: [u8; 1968usize],
  #[doc = "0x800 - RTC Write Access Register"]
  pub war: WAR,
  #[doc = "0x804 - RTC Read Access Register"]
  pub rar: RAR,
}
#[doc = "RTC Time Seconds Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tsr](tsr) module"]
pub type TSR = crate::Reg<u32, _TSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TSR;
#[doc = "`read()` method returns [tsr::R](tsr::R) reader structure"]
impl crate::Readable for TSR {}
#[doc = "`write(|w| ..)` method takes [tsr::W](tsr::W) writer structure"]
impl crate::Writable for TSR {}
#[doc = "RTC Time Seconds Register"]
pub mod tsr;
#[doc = "RTC Time Prescaler Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tpr](tpr) module"]
pub type TPR = crate::Reg<u32, _TPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TPR;
#[doc = "`read()` method returns [tpr::R](tpr::R) reader structure"]
impl crate::Readable for TPR {}
#[doc = "`write(|w| ..)` method takes [tpr::W](tpr::W) writer structure"]
impl crate::Writable for TPR {}
#[doc = "RTC Time Prescaler Register"]
pub mod tpr;
#[doc = "RTC Time Alarm Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tar](tar) module"]
pub type TAR = crate::Reg<u32, _TAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TAR;
#[doc = "`read()` method returns [tar::R](tar::R) reader structure"]
impl crate::Readable for TAR {}
#[doc = "`write(|w| ..)` method takes [tar::W](tar::W) writer structure"]
impl crate::Writable for TAR {}
#[doc = "RTC Time Alarm Register"]
pub mod tar;
#[doc = "RTC Time Compensation Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tcr](tcr) module"]
pub type TCR = crate::Reg<u32, _TCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCR;
#[doc = "`read()` method returns [tcr::R](tcr::R) reader structure"]
impl crate::Readable for TCR {}
#[doc = "`write(|w| ..)` method takes [tcr::W](tcr::W) writer structure"]
impl crate::Writable for TCR {}
#[doc = "RTC Time Compensation Register"]
pub mod tcr;
#[doc = "RTC Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cr](cr) module"]
pub type CR = crate::Reg<u32, _CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR;
#[doc = "`read()` method returns [cr::R](cr::R) reader structure"]
impl crate::Readable for CR {}
#[doc = "`write(|w| ..)` method takes [cr::W](cr::W) writer structure"]
impl crate::Writable for CR {}
#[doc = "RTC Control Register"]
pub mod cr;
#[doc = "RTC Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sr](sr) module"]
pub type SR = crate::Reg<u32, _SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SR;
#[doc = "`read()` method returns [sr::R](sr::R) reader structure"]
impl crate::Readable for SR {}
#[doc = "`write(|w| ..)` method takes [sr::W](sr::W) writer structure"]
impl crate::Writable for SR {}
#[doc = "RTC Status Register"]
pub mod sr;
#[doc = "RTC Lock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lr](lr) module"]
pub type LR = crate::Reg<u32, _LR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LR;
#[doc = "`read()` method returns [lr::R](lr::R) reader structure"]
impl crate::Readable for LR {}
#[doc = "`write(|w| ..)` method takes [lr::W](lr::W) writer structure"]
impl crate::Writable for LR {}
#[doc = "RTC Lock Register"]
pub mod lr;
#[doc = "RTC Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ier](ier) module"]
pub type IER = crate::Reg<u32, _IER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IER;
#[doc = "`read()` method returns [ier::R](ier::R) reader structure"]
impl crate::Readable for IER {}
#[doc = "`write(|w| ..)` method takes [ier::W](ier::W) writer structure"]
impl crate::Writable for IER {}
#[doc = "RTC Interrupt Enable Register"]
pub mod ier;
#[doc = "RTC Tamper Time Seconds Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ttsr](ttsr) module"]
pub type TTSR = crate::Reg<u32, _TTSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TTSR;
#[doc = "`read()` method returns [ttsr::R](ttsr::R) reader structure"]
impl crate::Readable for TTSR {}
#[doc = "RTC Tamper Time Seconds Register"]
pub mod ttsr;
#[doc = "RTC Monotonic Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mer](mer) module"]
pub type MER = crate::Reg<u32, _MER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MER;
#[doc = "`read()` method returns [mer::R](mer::R) reader structure"]
impl crate::Readable for MER {}
#[doc = "`write(|w| ..)` method takes [mer::W](mer::W) writer structure"]
impl crate::Writable for MER {}
#[doc = "RTC Monotonic Enable Register"]
pub mod mer;
#[doc = "RTC Monotonic Counter Low Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mclr](mclr) module"]
pub type MCLR = crate::Reg<u32, _MCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCLR;
#[doc = "`read()` method returns [mclr::R](mclr::R) reader structure"]
impl crate::Readable for MCLR {}
#[doc = "`write(|w| ..)` method takes [mclr::W](mclr::W) writer structure"]
impl crate::Writable for MCLR {}
#[doc = "RTC Monotonic Counter Low Register"]
pub mod mclr;
#[doc = "RTC Monotonic Counter High Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mchr](mchr) module"]
pub type MCHR = crate::Reg<u32, _MCHR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCHR;
#[doc = "`read()` method returns [mchr::R](mchr::R) reader structure"]
impl crate::Readable for MCHR {}
#[doc = "`write(|w| ..)` method takes [mchr::W](mchr::W) writer structure"]
impl crate::Writable for MCHR {}
#[doc = "RTC Monotonic Counter High Register"]
pub mod mchr;
#[doc = "RTC Tamper Detect Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tdr](tdr) module"]
pub type TDR = crate::Reg<u32, _TDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TDR;
#[doc = "`read()` method returns [tdr::R](tdr::R) reader structure"]
impl crate::Readable for TDR {}
#[doc = "`write(|w| ..)` method takes [tdr::W](tdr::W) writer structure"]
impl crate::Writable for TDR {}
#[doc = "RTC Tamper Detect Register"]
pub mod tdr;
#[doc = "RTC Tamper Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tir](tir) module"]
pub type TIR = crate::Reg<u32, _TIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIR;
#[doc = "`read()` method returns [tir::R](tir::R) reader structure"]
impl crate::Readable for TIR {}
#[doc = "`write(|w| ..)` method takes [tir::W](tir::W) writer structure"]
impl crate::Writable for TIR {}
#[doc = "RTC Tamper Interrupt Register"]
pub mod tir;
#[doc = "RTC Pin Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcr](pcr) module"]
pub type PCR = crate::Reg<u32, _PCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCR;
#[doc = "`read()` method returns [pcr::R](pcr::R) reader structure"]
impl crate::Readable for PCR {}
#[doc = "`write(|w| ..)` method takes [pcr::W](pcr::W) writer structure"]
impl crate::Writable for PCR {}
#[doc = "RTC Pin Configuration Register"]
pub mod pcr;
#[doc = "RTC Write Access Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [war](war) module"]
pub type WAR = crate::Reg<u32, _WAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WAR;
#[doc = "`read()` method returns [war::R](war::R) reader structure"]
impl crate::Readable for WAR {}
#[doc = "`write(|w| ..)` method takes [war::W](war::W) writer structure"]
impl crate::Writable for WAR {}
#[doc = "RTC Write Access Register"]
pub mod war;
#[doc = "RTC Read Access Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rar](rar) module"]
pub type RAR = crate::Reg<u32, _RAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RAR;
#[doc = "`read()` method returns [rar::R](rar::R) reader structure"]
impl crate::Readable for RAR {}
#[doc = "`write(|w| ..)` method takes [rar::W](rar::W) writer structure"]
impl crate::Writable for RAR {}
#[doc = "RTC Read Access Register"]
pub mod rar;
