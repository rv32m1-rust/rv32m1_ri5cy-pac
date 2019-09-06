#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
  #[doc = "0x00 - Version ID Register"]
  pub verid: VERID,
  #[doc = "0x04 - Parameter Register"]
  pub param: PARAM,
  #[doc = "0x08 - Pin Enable 1 register"]
  pub pe1: PE1,
  #[doc = "0x0c - Pin Enable 2 register"]
  pub pe2: PE2,
  _reserved4: [u8; 8usize],
  #[doc = "0x18 - Module Interrupt Enable register"]
  pub me: ME,
  #[doc = "0x1c - Module DMA/Trigger Enable register"]
  pub de: DE,
  #[doc = "0x20 - Pin Flag register"]
  pub pf: PF,
  _reserved7: [u8; 12usize],
  #[doc = "0x30 - Pin Filter register"]
  pub filt: FILT,
  _reserved8: [u8; 4usize],
  #[doc = "0x38 - Pin DMA/Trigger Configuration 1 register"]
  pub pdc1: PDC1,
  #[doc = "0x3c - Pin DMA/Trigger Configuration 2 register"]
  pub pdc2: PDC2,
  _reserved10: [u8; 8usize],
  #[doc = "0x48 - Pin Filter DMA/Trigger Configuration register"]
  pub fdc: FDC,
  _reserved11: [u8; 4usize],
  #[doc = "0x50 - Pin Mode Configuration register"]
  pub pmc: PMC,
  _reserved12: [u8; 4usize],
  #[doc = "0x58 - Pin Filter Mode Configuration register"]
  pub fmc: FMC,
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
#[doc = "Pin Enable 1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pe1](pe1) module"]
pub type PE1 = crate::Reg<u32, _PE1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PE1;
#[doc = "`read()` method returns [pe1::R](pe1::R) reader structure"]
impl crate::Readable for PE1 {}
#[doc = "`write(|w| ..)` method takes [pe1::W](pe1::W) writer structure"]
impl crate::Writable for PE1 {}
#[doc = "Pin Enable 1 register"]
pub mod pe1;
#[doc = "Pin Enable 2 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pe2](pe2) module"]
pub type PE2 = crate::Reg<u32, _PE2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PE2;
#[doc = "`read()` method returns [pe2::R](pe2::R) reader structure"]
impl crate::Readable for PE2 {}
#[doc = "`write(|w| ..)` method takes [pe2::W](pe2::W) writer structure"]
impl crate::Writable for PE2 {}
#[doc = "Pin Enable 2 register"]
pub mod pe2;
#[doc = "Module Interrupt Enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [me](me) module"]
pub type ME = crate::Reg<u32, _ME>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ME;
#[doc = "`read()` method returns [me::R](me::R) reader structure"]
impl crate::Readable for ME {}
#[doc = "`write(|w| ..)` method takes [me::W](me::W) writer structure"]
impl crate::Writable for ME {}
#[doc = "Module Interrupt Enable register"]
pub mod me;
#[doc = "Module DMA/Trigger Enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [de](de) module"]
pub type DE = crate::Reg<u32, _DE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DE;
#[doc = "`read()` method returns [de::R](de::R) reader structure"]
impl crate::Readable for DE {}
#[doc = "`write(|w| ..)` method takes [de::W](de::W) writer structure"]
impl crate::Writable for DE {}
#[doc = "Module DMA/Trigger Enable register"]
pub mod de;
#[doc = "Pin Flag register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pf](pf) module"]
pub type PF = crate::Reg<u32, _PF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PF;
#[doc = "`read()` method returns [pf::R](pf::R) reader structure"]
impl crate::Readable for PF {}
#[doc = "`write(|w| ..)` method takes [pf::W](pf::W) writer structure"]
impl crate::Writable for PF {}
#[doc = "Pin Flag register"]
pub mod pf;
#[doc = "Pin Filter register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [filt](filt) module"]
pub type FILT = crate::Reg<u32, _FILT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FILT;
#[doc = "`read()` method returns [filt::R](filt::R) reader structure"]
impl crate::Readable for FILT {}
#[doc = "`write(|w| ..)` method takes [filt::W](filt::W) writer structure"]
impl crate::Writable for FILT {}
#[doc = "Pin Filter register"]
pub mod filt;
#[doc = "Pin DMA/Trigger Configuration 1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdc1](pdc1) module"]
pub type PDC1 = crate::Reg<u32, _PDC1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDC1;
#[doc = "`read()` method returns [pdc1::R](pdc1::R) reader structure"]
impl crate::Readable for PDC1 {}
#[doc = "`write(|w| ..)` method takes [pdc1::W](pdc1::W) writer structure"]
impl crate::Writable for PDC1 {}
#[doc = "Pin DMA/Trigger Configuration 1 register"]
pub mod pdc1;
#[doc = "Pin DMA/Trigger Configuration 2 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdc2](pdc2) module"]
pub type PDC2 = crate::Reg<u32, _PDC2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDC2;
#[doc = "`read()` method returns [pdc2::R](pdc2::R) reader structure"]
impl crate::Readable for PDC2 {}
#[doc = "`write(|w| ..)` method takes [pdc2::W](pdc2::W) writer structure"]
impl crate::Writable for PDC2 {}
#[doc = "Pin DMA/Trigger Configuration 2 register"]
pub mod pdc2;
#[doc = "Pin Filter DMA/Trigger Configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fdc](fdc) module"]
pub type FDC = crate::Reg<u32, _FDC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FDC;
#[doc = "`read()` method returns [fdc::R](fdc::R) reader structure"]
impl crate::Readable for FDC {}
#[doc = "`write(|w| ..)` method takes [fdc::W](fdc::W) writer structure"]
impl crate::Writable for FDC {}
#[doc = "Pin Filter DMA/Trigger Configuration register"]
pub mod fdc;
#[doc = "Pin Mode Configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pmc](pmc) module"]
pub type PMC = crate::Reg<u32, _PMC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMC;
#[doc = "`read()` method returns [pmc::R](pmc::R) reader structure"]
impl crate::Readable for PMC {}
#[doc = "`write(|w| ..)` method takes [pmc::W](pmc::W) writer structure"]
impl crate::Writable for PMC {}
#[doc = "Pin Mode Configuration register"]
pub mod pmc;
#[doc = "Pin Filter Mode Configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fmc](fmc) module"]
pub type FMC = crate::Reg<u32, _FMC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMC;
#[doc = "`read()` method returns [fmc::R](fmc::R) reader structure"]
impl crate::Readable for FMC {}
#[doc = "`write(|w| ..)` method takes [fmc::W](fmc::W) writer structure"]
impl crate::Writable for FMC {}
#[doc = "Pin Filter Mode Configuration register"]
pub mod fmc;
