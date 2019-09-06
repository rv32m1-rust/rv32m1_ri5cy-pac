#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
  _reserved0: [u8; 8usize],
  #[doc = "0x08 - Crossbar Switch (AXBS) Slave Configuration"]
  pub plasc: PLASC,
  #[doc = "0x0a - Crossbar Switch (AXBS) Master Configuration"]
  pub plamc: PLAMC,
  #[doc = "0x0c - Core Platform Control Register"]
  pub cpcr: CPCR,
  #[doc = "0x10 - Interrupt Status and Control Register"]
  pub iscr: ISCR,
  _reserved4: [u8; 32usize],
  #[doc = "0x34 - Core Platform Control Register 2"]
  pub cpcr2: CPCR2,
  _reserved5: [u8; 8usize],
  #[doc = "0x40 - Compute Operation Control Register"]
  pub cpo: CPO,
}
#[doc = "Crossbar Switch (AXBS) Slave Configuration\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [plasc](plasc) module"]
pub type PLASC = crate::Reg<u16, _PLASC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLASC;
#[doc = "`read()` method returns [plasc::R](plasc::R) reader structure"]
impl crate::Readable for PLASC {}
#[doc = "Crossbar Switch (AXBS) Slave Configuration"]
pub mod plasc;
#[doc = "Crossbar Switch (AXBS) Master Configuration\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [plamc](plamc) module"]
pub type PLAMC = crate::Reg<u16, _PLAMC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLAMC;
#[doc = "`read()` method returns [plamc::R](plamc::R) reader structure"]
impl crate::Readable for PLAMC {}
#[doc = "Crossbar Switch (AXBS) Master Configuration"]
pub mod plamc;
#[doc = "Core Platform Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cpcr](cpcr) module"]
pub type CPCR = crate::Reg<u32, _CPCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPCR;
#[doc = "`read()` method returns [cpcr::R](cpcr::R) reader structure"]
impl crate::Readable for CPCR {}
#[doc = "`write(|w| ..)` method takes [cpcr::W](cpcr::W) writer structure"]
impl crate::Writable for CPCR {}
#[doc = "Core Platform Control Register"]
pub mod cpcr;
#[doc = "Interrupt Status and Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [iscr](iscr) module"]
pub type ISCR = crate::Reg<u32, _ISCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISCR;
#[doc = "`read()` method returns [iscr::R](iscr::R) reader structure"]
impl crate::Readable for ISCR {}
#[doc = "`write(|w| ..)` method takes [iscr::W](iscr::W) writer structure"]
impl crate::Writable for ISCR {}
#[doc = "Interrupt Status and Control Register"]
pub mod iscr;
#[doc = "Core Platform Control Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cpcr2](cpcr2) module"]
pub type CPCR2 = crate::Reg<u32, _CPCR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPCR2;
#[doc = "`read()` method returns [cpcr2::R](cpcr2::R) reader structure"]
impl crate::Readable for CPCR2 {}
#[doc = "`write(|w| ..)` method takes [cpcr2::W](cpcr2::W) writer structure"]
impl crate::Writable for CPCR2 {}
#[doc = "Core Platform Control Register 2"]
pub mod cpcr2;
#[doc = "Compute Operation Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cpo](cpo) module"]
pub type CPO = crate::Reg<u32, _CPO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPO;
#[doc = "`read()` method returns [cpo::R](cpo::R) reader structure"]
impl crate::Readable for CPO {}
#[doc = "`write(|w| ..)` method takes [cpo::W](cpo::W) writer structure"]
impl crate::Writable for CPO {}
#[doc = "Compute Operation Control Register"]
pub mod cpo;
