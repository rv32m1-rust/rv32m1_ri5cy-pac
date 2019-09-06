#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
  #[doc = "0x00 - Version ID Register"]
  pub verid: VERID,
  #[doc = "0x04 - Parameter Register"]
  pub param: PARAM,
  #[doc = "0x08 - Module Control Register"]
  pub mcr: MCR,
  #[doc = "0x0c - Module Status Register"]
  pub msr: MSR,
  #[doc = "0x10 - Module Interrupt Enable Register"]
  pub mier: MIER,
  #[doc = "0x14 - Set Timer Enable Register"]
  pub setten: SETTEN,
  #[doc = "0x18 - Clear Timer Enable Register"]
  pub clrten: CLRTEN,
  _reserved7: [u8; 4usize],
  #[doc = "0x20 - no description available"]
  pub channel0: CHANNEL,
  _reserved8: [u8; 4usize],
  #[doc = "0x30 - no description available"]
  pub channel1: CHANNEL,
  _reserved9: [u8; 4usize],
  #[doc = "0x40 - no description available"]
  pub channel2: CHANNEL,
  _reserved10: [u8; 4usize],
  #[doc = "0x50 - no description available"]
  pub channel3: CHANNEL,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct CHANNEL {
  #[doc = "0x00 - Timer Value Register"]
  pub tval: self::channel::TVAL,
  #[doc = "0x04 - Current Timer Value"]
  pub cval: self::channel::CVAL,
  #[doc = "0x08 - Timer Control Register"]
  pub tctrl: self::channel::TCTRL,
}
#[doc = r"Register block"]
#[doc = "no description available"]
pub mod channel;
#[doc = "Version ID Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [verid](verid) module"]
pub type VERID = crate::Reg<u32, _VERID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VERID;
#[doc = "`read()` method returns [verid::R](verid::R) reader structure"]
impl crate::Readable for VERID {}
#[doc = "Version ID Register"]
pub mod verid;
#[doc = "Parameter Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [param](param) module"]
pub type PARAM = crate::Reg<u32, _PARAM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PARAM;
#[doc = "`read()` method returns [param::R](param::R) reader structure"]
impl crate::Readable for PARAM {}
#[doc = "Parameter Register"]
pub mod param;
#[doc = "Module Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mcr](mcr) module"]
pub type MCR = crate::Reg<u32, _MCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCR;
#[doc = "`read()` method returns [mcr::R](mcr::R) reader structure"]
impl crate::Readable for MCR {}
#[doc = "`write(|w| ..)` method takes [mcr::W](mcr::W) writer structure"]
impl crate::Writable for MCR {}
#[doc = "Module Control Register"]
pub mod mcr;
#[doc = "Module Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [msr](msr) module"]
pub type MSR = crate::Reg<u32, _MSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MSR;
#[doc = "`read()` method returns [msr::R](msr::R) reader structure"]
impl crate::Readable for MSR {}
#[doc = "`write(|w| ..)` method takes [msr::W](msr::W) writer structure"]
impl crate::Writable for MSR {}
#[doc = "Module Status Register"]
pub mod msr;
#[doc = "Module Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mier](mier) module"]
pub type MIER = crate::Reg<u32, _MIER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MIER;
#[doc = "`read()` method returns [mier::R](mier::R) reader structure"]
impl crate::Readable for MIER {}
#[doc = "`write(|w| ..)` method takes [mier::W](mier::W) writer structure"]
impl crate::Writable for MIER {}
#[doc = "Module Interrupt Enable Register"]
pub mod mier;
#[doc = "Set Timer Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [setten](setten) module"]
pub type SETTEN = crate::Reg<u32, _SETTEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SETTEN;
#[doc = "`read()` method returns [setten::R](setten::R) reader structure"]
impl crate::Readable for SETTEN {}
#[doc = "`write(|w| ..)` method takes [setten::W](setten::W) writer structure"]
impl crate::Writable for SETTEN {}
#[doc = "Set Timer Enable Register"]
pub mod setten;
#[doc = "Clear Timer Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [clrten](clrten) module"]
pub type CLRTEN = crate::Reg<u32, _CLRTEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLRTEN;
#[doc = "`read()` method returns [clrten::R](clrten::R) reader structure"]
impl crate::Readable for CLRTEN {}
#[doc = "`write(|w| ..)` method takes [clrten::W](clrten::W) writer structure"]
impl crate::Writable for CLRTEN {}
#[doc = "Clear Timer Enable Register"]
pub mod clrten;
