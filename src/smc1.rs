#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
  #[doc = "0x00 - Version ID Register"]
  pub verid: VERID,
  #[doc = "0x04 - Parameter Register"]
  pub param: PARAM,
  #[doc = "0x08 - Power Mode Protection register"]
  pub pmprot: PMPROT,
  _reserved3: [u8; 4usize],
  #[doc = "0x10 - Power Mode Control register"]
  pub pmctrl: PMCTRL,
  _reserved4: [u8; 4usize],
  #[doc = "0x18 - Power Mode Status register"]
  pub pmstat: PMSTAT,
  _reserved5: [u8; 4usize],
  #[doc = "0x20 - System Reset Status"]
  pub srs: SRS,
  _reserved6: [u8; 4usize],
  #[doc = "0x28 - Sticky System Reset Status"]
  pub ssrs: SSRS,
  #[doc = "0x2c - System Reset Interrupt Enable"]
  pub srie: SRIE,
  #[doc = "0x30 - System Reset Interrupt Flag"]
  pub srif: SRIF,
  _reserved9: [u8; 12usize],
  #[doc = "0x40 - Mode Register"]
  pub mr: MR,
  _reserved10: [u8; 12usize],
  #[doc = "0x50 - Force Mode Register"]
  pub fm: FM,
  _reserved11: [u8; 12usize],
  #[doc = "0x60 - SRAM Low Power Register"]
  pub sramlpr: SRAMLPR,
  #[doc = "0x64 - SRAM Deep Sleep Register"]
  pub sramdsr: SRAMDSR,
}
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
#[doc = "Power Mode Protection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pmprot](pmprot) module"]
pub type PMPROT = crate::Reg<u32, _PMPROT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMPROT;
#[doc = "`read()` method returns [pmprot::R](pmprot::R) reader structure"]
impl crate::Readable for PMPROT {}
#[doc = "`write(|w| ..)` method takes [pmprot::W](pmprot::W) writer structure"]
impl crate::Writable for PMPROT {}
#[doc = "Power Mode Protection register"]
pub mod pmprot;
#[doc = "Power Mode Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pmctrl](pmctrl) module"]
pub type PMCTRL = crate::Reg<u32, _PMCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMCTRL;
#[doc = "`read()` method returns [pmctrl::R](pmctrl::R) reader structure"]
impl crate::Readable for PMCTRL {}
#[doc = "`write(|w| ..)` method takes [pmctrl::W](pmctrl::W) writer structure"]
impl crate::Writable for PMCTRL {}
#[doc = "Power Mode Control register"]
pub mod pmctrl;
#[doc = "Power Mode Status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pmstat](pmstat) module"]
pub type PMSTAT = crate::Reg<u32, _PMSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMSTAT;
#[doc = "`read()` method returns [pmstat::R](pmstat::R) reader structure"]
impl crate::Readable for PMSTAT {}
#[doc = "`write(|w| ..)` method takes [pmstat::W](pmstat::W) writer structure"]
impl crate::Writable for PMSTAT {}
#[doc = "Power Mode Status register"]
pub mod pmstat;
#[doc = "System Reset Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [srs](srs) module"]
pub type SRS = crate::Reg<u32, _SRS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRS;
#[doc = "`read()` method returns [srs::R](srs::R) reader structure"]
impl crate::Readable for SRS {}
#[doc = "System Reset Status"]
pub mod srs;
#[doc = "Sticky System Reset Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ssrs](ssrs) module"]
pub type SSRS = crate::Reg<u32, _SSRS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSRS;
#[doc = "`read()` method returns [ssrs::R](ssrs::R) reader structure"]
impl crate::Readable for SSRS {}
#[doc = "`write(|w| ..)` method takes [ssrs::W](ssrs::W) writer structure"]
impl crate::Writable for SSRS {}
#[doc = "Sticky System Reset Status"]
pub mod ssrs;
#[doc = "System Reset Interrupt Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [srie](srie) module"]
pub type SRIE = crate::Reg<u32, _SRIE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRIE;
#[doc = "`read()` method returns [srie::R](srie::R) reader structure"]
impl crate::Readable for SRIE {}
#[doc = "`write(|w| ..)` method takes [srie::W](srie::W) writer structure"]
impl crate::Writable for SRIE {}
#[doc = "System Reset Interrupt Enable"]
pub mod srie;
#[doc = "System Reset Interrupt Flag\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [srif](srif) module"]
pub type SRIF = crate::Reg<u32, _SRIF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRIF;
#[doc = "`read()` method returns [srif::R](srif::R) reader structure"]
impl crate::Readable for SRIF {}
#[doc = "`write(|w| ..)` method takes [srif::W](srif::W) writer structure"]
impl crate::Writable for SRIF {}
#[doc = "System Reset Interrupt Flag"]
pub mod srif;
#[doc = "Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mr](mr) module"]
pub type MR = crate::Reg<u32, _MR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MR;
#[doc = "`read()` method returns [mr::R](mr::R) reader structure"]
impl crate::Readable for MR {}
#[doc = "`write(|w| ..)` method takes [mr::W](mr::W) writer structure"]
impl crate::Writable for MR {}
#[doc = "Mode Register"]
pub mod mr;
#[doc = "Force Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fm](fm) module"]
pub type FM = crate::Reg<u32, _FM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FM;
#[doc = "`read()` method returns [fm::R](fm::R) reader structure"]
impl crate::Readable for FM {}
#[doc = "`write(|w| ..)` method takes [fm::W](fm::W) writer structure"]
impl crate::Writable for FM {}
#[doc = "Force Mode Register"]
pub mod fm;
#[doc = "SRAM Low Power Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sramlpr](sramlpr) module"]
pub type SRAMLPR = crate::Reg<u32, _SRAMLPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRAMLPR;
#[doc = "`read()` method returns [sramlpr::R](sramlpr::R) reader structure"]
impl crate::Readable for SRAMLPR {}
#[doc = "`write(|w| ..)` method takes [sramlpr::W](sramlpr::W) writer structure"]
impl crate::Writable for SRAMLPR {}
#[doc = "SRAM Low Power Register"]
pub mod sramlpr;
#[doc = "SRAM Deep Sleep Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sramdsr](sramdsr) module"]
pub type SRAMDSR = crate::Reg<u32, _SRAMDSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRAMDSR;
#[doc = "`read()` method returns [sramdsr::R](sramdsr::R) reader structure"]
impl crate::Readable for SRAMDSR {}
#[doc = "`write(|w| ..)` method takes [sramdsr::W](sramdsr::W) writer structure"]
impl crate::Writable for SRAMDSR {}
#[doc = "SRAM Deep Sleep Register"]
pub mod sramdsr;
