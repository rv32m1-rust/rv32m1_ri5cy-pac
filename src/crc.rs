#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
  #[doc = "0x00 - CRC Data register"]
  pub data: DATA,
  #[doc = "0x04 - CRC Polynomial register"]
  pub gpoly: GPOLY,
  #[doc = "0x08 - CRC Control register"]
  pub ctrl: CTRL,
}
#[doc = "CRC Data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [data](data) module"]
pub type DATA = crate::Reg<u32, _DATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA;
#[doc = "`read()` method returns [data::R](data::R) reader structure"]
impl crate::Readable for DATA {}
#[doc = "`write(|w| ..)` method takes [data::W](data::W) writer structure"]
impl crate::Writable for DATA {}
#[doc = "CRC Data register"]
pub mod data;
#[doc = "CRC Polynomial register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpoly](gpoly) module"]
pub type GPOLY = crate::Reg<u32, _GPOLY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPOLY;
#[doc = "`read()` method returns [gpoly::R](gpoly::R) reader structure"]
impl crate::Readable for GPOLY {}
#[doc = "`write(|w| ..)` method takes [gpoly::W](gpoly::W) writer structure"]
impl crate::Writable for GPOLY {}
#[doc = "CRC Polynomial register"]
pub mod gpoly;
#[doc = "CRC Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ctrl](ctrl) module"]
pub type CTRL = crate::Reg<u32, _CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL;
#[doc = "`read()` method returns [ctrl::R](ctrl::R) reader structure"]
impl crate::Readable for CTRL {}
#[doc = "`write(|w| ..)` method takes [ctrl::W](ctrl::W) writer structure"]
impl crate::Writable for CTRL {}
#[doc = "CRC Control register"]
pub mod ctrl;
