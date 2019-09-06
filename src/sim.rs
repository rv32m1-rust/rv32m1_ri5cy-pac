#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
  _reserved0: [u8; 4usize],
  #[doc = "0x04 - Chip Control Register"]
  pub chipctrl: CHIPCTRL,
  _reserved1: [u8; 28usize],
  #[doc = "0x24 - System Device Identification Register"]
  pub sdid: SDID,
  _reserved2: [u8; 36usize],
  #[doc = "0x4c - Flash Configuration Register 1"]
  pub fcfg1: FCFG1,
  #[doc = "0x50 - Flash Configuration Register 2"]
  pub fcfg2: FCFG2,
  _reserved4: [u8; 4usize],
  #[doc = "0x58 - Unique Identification Register High"]
  pub uidh: UIDH,
  #[doc = "0x5c - Unique Identification Register Mid Middle"]
  pub uidm: UIDM,
  #[doc = "0x60 - Unique Identification Register Mid Low"]
  pub uidl: UIDL,
  #[doc = "0x64 - RF Mac Address Low"]
  pub rfaddrl: RFADDRL,
  #[doc = "0x68 - RF MAC Address High"]
  pub rfaddrh: RFADDRH,
  _reserved9: [u8; 4usize],
  #[doc = "0x70 - MISC2 Register"]
  pub misc2: MISC2,
}
#[doc = "Chip Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [chipctrl](chipctrl) module"]
pub type CHIPCTRL = crate::Reg<u32, _CHIPCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHIPCTRL;
#[doc = "`read()` method returns [chipctrl::R](chipctrl::R) reader structure"]
impl crate::Readable for CHIPCTRL {}
#[doc = "`write(|w| ..)` method takes [chipctrl::W](chipctrl::W) writer structure"]
impl crate::Writable for CHIPCTRL {}
#[doc = "Chip Control Register"]
pub mod chipctrl;
#[doc = "System Device Identification Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sdid](sdid) module"]
pub type SDID = crate::Reg<u32, _SDID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDID;
#[doc = "`read()` method returns [sdid::R](sdid::R) reader structure"]
impl crate::Readable for SDID {}
#[doc = "System Device Identification Register"]
pub mod sdid;
#[doc = "Flash Configuration Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fcfg1](fcfg1) module"]
pub type FCFG1 = crate::Reg<u32, _FCFG1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FCFG1;
#[doc = "`read()` method returns [fcfg1::R](fcfg1::R) reader structure"]
impl crate::Readable for FCFG1 {}
#[doc = "`write(|w| ..)` method takes [fcfg1::W](fcfg1::W) writer structure"]
impl crate::Writable for FCFG1 {}
#[doc = "Flash Configuration Register 1"]
pub mod fcfg1;
#[doc = "Flash Configuration Register 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fcfg2](fcfg2) module"]
pub type FCFG2 = crate::Reg<u32, _FCFG2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FCFG2;
#[doc = "`read()` method returns [fcfg2::R](fcfg2::R) reader structure"]
impl crate::Readable for FCFG2 {}
#[doc = "Flash Configuration Register 2"]
pub mod fcfg2;
#[doc = "Unique Identification Register High\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uidh](uidh) module"]
pub type UIDH = crate::Reg<u32, _UIDH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UIDH;
#[doc = "`read()` method returns [uidh::R](uidh::R) reader structure"]
impl crate::Readable for UIDH {}
#[doc = "Unique Identification Register High"]
pub mod uidh;
#[doc = "Unique Identification Register Mid Middle\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uidm](uidm) module"]
pub type UIDM = crate::Reg<u32, _UIDM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UIDM;
#[doc = "`read()` method returns [uidm::R](uidm::R) reader structure"]
impl crate::Readable for UIDM {}
#[doc = "Unique Identification Register Mid Middle"]
pub mod uidm;
#[doc = "Unique Identification Register Mid Low\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uidl](uidl) module"]
pub type UIDL = crate::Reg<u32, _UIDL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UIDL;
#[doc = "`read()` method returns [uidl::R](uidl::R) reader structure"]
impl crate::Readable for UIDL {}
#[doc = "Unique Identification Register Mid Low"]
pub mod uidl;
#[doc = "RF Mac Address Low\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rfaddrl](rfaddrl) module"]
pub type RFADDRL = crate::Reg<u32, _RFADDRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RFADDRL;
#[doc = "`read()` method returns [rfaddrl::R](rfaddrl::R) reader structure"]
impl crate::Readable for RFADDRL {}
#[doc = "RF Mac Address Low"]
pub mod rfaddrl;
#[doc = "RF MAC Address High\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rfaddrh](rfaddrh) module"]
pub type RFADDRH = crate::Reg<u32, _RFADDRH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RFADDRH;
#[doc = "`read()` method returns [rfaddrh::R](rfaddrh::R) reader structure"]
impl crate::Readable for RFADDRH {}
#[doc = "RF MAC Address High"]
pub mod rfaddrh;
#[doc = "MISC2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [misc2](misc2) module"]
pub type MISC2 = crate::Reg<u32, _MISC2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MISC2;
#[doc = "`read()` method returns [misc2::R](misc2::R) reader structure"]
impl crate::Readable for MISC2 {}
#[doc = "`write(|w| ..)` method takes [misc2::W](misc2::W) writer structure"]
impl crate::Writable for MISC2 {}
#[doc = "MISC2 Register"]
pub mod misc2;
