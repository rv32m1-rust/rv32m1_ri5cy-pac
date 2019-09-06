#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
  #[doc = "0x00 - Pin Control Register 0"]
  pub pcr0: PCR0,
  #[doc = "0x04 - Pin Control Register 1"]
  pub pcr1: PCR1,
  _reserved2: [u8; 20usize],
  #[doc = "0x1c - Pin Control Register 7"]
  pub pcr7: PCR7,
  #[doc = "0x20 - Pin Control Register 8"]
  pub pcr8: PCR8,
  #[doc = "0x24 - Pin Control Register 9"]
  pub pcr9: PCR9,
  #[doc = "0x28 - Pin Control Register 10"]
  pub pcr10: PCR10,
  #[doc = "0x2c - Pin Control Register 11"]
  pub pcr11: PCR11,
  #[doc = "0x30 - Pin Control Register 12"]
  pub pcr12: PCR12,
  _reserved8: [u8; 52usize],
  #[doc = "0x68 - Pin Control Register 26"]
  pub pcr26: PCR26,
  #[doc = "0x6c - Pin Control Register 27"]
  pub pcr27: PCR27,
  #[doc = "0x70 - Pin Control Register 28"]
  pub pcr28: PCR28,
  #[doc = "0x74 - Pin Control Register 29"]
  pub pcr29: PCR29,
  #[doc = "0x78 - Pin Control Register 30"]
  pub pcr30: PCR30,
  _reserved13: [u8; 4usize],
  #[doc = "0x80 - Global Pin Control Low Register"]
  pub gpclr: GPCLR,
  #[doc = "0x84 - Global Pin Control High Register"]
  pub gpchr: GPCHR,
  #[doc = "0x88 - Global Interrupt Control Low Register"]
  pub giclr: GICLR,
  #[doc = "0x8c - Global Interrupt Control High Register"]
  pub gichr: GICHR,
  _reserved17: [u8; 16usize],
  #[doc = "0xa0 - Interrupt Status Flag Register"]
  pub isfr: ISFR,
}
#[doc = "Pin Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcr0](pcr0) module"]
pub type PCR0 = crate::Reg<u32, _PCR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCR0;
#[doc = "`read()` method returns [pcr0::R](pcr0::R) reader structure"]
impl crate::Readable for PCR0 {}
#[doc = "`write(|w| ..)` method takes [pcr0::W](pcr0::W) writer structure"]
impl crate::Writable for PCR0 {}
#[doc = "Pin Control Register 0"]
pub mod pcr0;
#[doc = "Pin Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcr1](pcr1) module"]
pub type PCR1 = crate::Reg<u32, _PCR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCR1;
#[doc = "`read()` method returns [pcr1::R](pcr1::R) reader structure"]
impl crate::Readable for PCR1 {}
#[doc = "`write(|w| ..)` method takes [pcr1::W](pcr1::W) writer structure"]
impl crate::Writable for PCR1 {}
#[doc = "Pin Control Register 1"]
pub mod pcr1;
#[doc = "Pin Control Register 7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcr7](pcr7) module"]
pub type PCR7 = crate::Reg<u32, _PCR7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCR7;
#[doc = "`read()` method returns [pcr7::R](pcr7::R) reader structure"]
impl crate::Readable for PCR7 {}
#[doc = "`write(|w| ..)` method takes [pcr7::W](pcr7::W) writer structure"]
impl crate::Writable for PCR7 {}
#[doc = "Pin Control Register 7"]
pub mod pcr7;
#[doc = "Pin Control Register 8\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcr8](pcr8) module"]
pub type PCR8 = crate::Reg<u32, _PCR8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCR8;
#[doc = "`read()` method returns [pcr8::R](pcr8::R) reader structure"]
impl crate::Readable for PCR8 {}
#[doc = "`write(|w| ..)` method takes [pcr8::W](pcr8::W) writer structure"]
impl crate::Writable for PCR8 {}
#[doc = "Pin Control Register 8"]
pub mod pcr8;
#[doc = "Pin Control Register 9\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcr9](pcr9) module"]
pub type PCR9 = crate::Reg<u32, _PCR9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCR9;
#[doc = "`read()` method returns [pcr9::R](pcr9::R) reader structure"]
impl crate::Readable for PCR9 {}
#[doc = "`write(|w| ..)` method takes [pcr9::W](pcr9::W) writer structure"]
impl crate::Writable for PCR9 {}
#[doc = "Pin Control Register 9"]
pub mod pcr9;
#[doc = "Pin Control Register 10\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcr10](pcr10) module"]
pub type PCR10 = crate::Reg<u32, _PCR10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCR10;
#[doc = "`read()` method returns [pcr10::R](pcr10::R) reader structure"]
impl crate::Readable for PCR10 {}
#[doc = "`write(|w| ..)` method takes [pcr10::W](pcr10::W) writer structure"]
impl crate::Writable for PCR10 {}
#[doc = "Pin Control Register 10"]
pub mod pcr10;
#[doc = "Pin Control Register 11\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcr11](pcr11) module"]
pub type PCR11 = crate::Reg<u32, _PCR11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCR11;
#[doc = "`read()` method returns [pcr11::R](pcr11::R) reader structure"]
impl crate::Readable for PCR11 {}
#[doc = "`write(|w| ..)` method takes [pcr11::W](pcr11::W) writer structure"]
impl crate::Writable for PCR11 {}
#[doc = "Pin Control Register 11"]
pub mod pcr11;
#[doc = "Pin Control Register 12\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcr12](pcr12) module"]
pub type PCR12 = crate::Reg<u32, _PCR12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCR12;
#[doc = "`read()` method returns [pcr12::R](pcr12::R) reader structure"]
impl crate::Readable for PCR12 {}
#[doc = "`write(|w| ..)` method takes [pcr12::W](pcr12::W) writer structure"]
impl crate::Writable for PCR12 {}
#[doc = "Pin Control Register 12"]
pub mod pcr12;
#[doc = "Pin Control Register 26\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcr26](pcr26) module"]
pub type PCR26 = crate::Reg<u32, _PCR26>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCR26;
#[doc = "`read()` method returns [pcr26::R](pcr26::R) reader structure"]
impl crate::Readable for PCR26 {}
#[doc = "`write(|w| ..)` method takes [pcr26::W](pcr26::W) writer structure"]
impl crate::Writable for PCR26 {}
#[doc = "Pin Control Register 26"]
pub mod pcr26;
#[doc = "Pin Control Register 27\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcr27](pcr27) module"]
pub type PCR27 = crate::Reg<u32, _PCR27>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCR27;
#[doc = "`read()` method returns [pcr27::R](pcr27::R) reader structure"]
impl crate::Readable for PCR27 {}
#[doc = "`write(|w| ..)` method takes [pcr27::W](pcr27::W) writer structure"]
impl crate::Writable for PCR27 {}
#[doc = "Pin Control Register 27"]
pub mod pcr27;
#[doc = "Pin Control Register 28\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcr28](pcr28) module"]
pub type PCR28 = crate::Reg<u32, _PCR28>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCR28;
#[doc = "`read()` method returns [pcr28::R](pcr28::R) reader structure"]
impl crate::Readable for PCR28 {}
#[doc = "`write(|w| ..)` method takes [pcr28::W](pcr28::W) writer structure"]
impl crate::Writable for PCR28 {}
#[doc = "Pin Control Register 28"]
pub mod pcr28;
#[doc = "Pin Control Register 29\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcr29](pcr29) module"]
pub type PCR29 = crate::Reg<u32, _PCR29>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCR29;
#[doc = "`read()` method returns [pcr29::R](pcr29::R) reader structure"]
impl crate::Readable for PCR29 {}
#[doc = "`write(|w| ..)` method takes [pcr29::W](pcr29::W) writer structure"]
impl crate::Writable for PCR29 {}
#[doc = "Pin Control Register 29"]
pub mod pcr29;
#[doc = "Pin Control Register 30\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcr30](pcr30) module"]
pub type PCR30 = crate::Reg<u32, _PCR30>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCR30;
#[doc = "`read()` method returns [pcr30::R](pcr30::R) reader structure"]
impl crate::Readable for PCR30 {}
#[doc = "`write(|w| ..)` method takes [pcr30::W](pcr30::W) writer structure"]
impl crate::Writable for PCR30 {}
#[doc = "Pin Control Register 30"]
pub mod pcr30;
#[doc = "Global Pin Control Low Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpclr](gpclr) module"]
pub type GPCLR = crate::Reg<u32, _GPCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPCLR;
#[doc = "`write(|w| ..)` method takes [gpclr::W](gpclr::W) writer structure"]
impl crate::Writable for GPCLR {}
#[doc = "Global Pin Control Low Register"]
pub mod gpclr;
#[doc = "Global Pin Control High Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpchr](gpchr) module"]
pub type GPCHR = crate::Reg<u32, _GPCHR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPCHR;
#[doc = "`write(|w| ..)` method takes [gpchr::W](gpchr::W) writer structure"]
impl crate::Writable for GPCHR {}
#[doc = "Global Pin Control High Register"]
pub mod gpchr;
#[doc = "Global Interrupt Control Low Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [giclr](giclr) module"]
pub type GICLR = crate::Reg<u32, _GICLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICLR;
#[doc = "`write(|w| ..)` method takes [giclr::W](giclr::W) writer structure"]
impl crate::Writable for GICLR {}
#[doc = "Global Interrupt Control Low Register"]
pub mod giclr;
#[doc = "Global Interrupt Control High Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gichr](gichr) module"]
pub type GICHR = crate::Reg<u32, _GICHR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICHR;
#[doc = "`write(|w| ..)` method takes [gichr::W](gichr::W) writer structure"]
impl crate::Writable for GICHR {}
#[doc = "Global Interrupt Control High Register"]
pub mod gichr;
#[doc = "Interrupt Status Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [isfr](isfr) module"]
pub type ISFR = crate::Reg<u32, _ISFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISFR;
#[doc = "`read()` method returns [isfr::R](isfr::R) reader structure"]
impl crate::Readable for ISFR {}
#[doc = "`write(|w| ..)` method takes [isfr::W](isfr::W) writer structure"]
impl crate::Writable for ISFR {}
#[doc = "Interrupt Status Flag Register"]
pub mod isfr;
