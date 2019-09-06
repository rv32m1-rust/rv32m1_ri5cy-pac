#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
  #[doc = "0x00 - Pin Control Register 0"]
  pub pcr0: PCR0,
  #[doc = "0x04 - Pin Control Register 1"]
  pub pcr1: PCR1,
  #[doc = "0x08 - Pin Control Register 2"]
  pub pcr2: PCR2,
  #[doc = "0x0c - Pin Control Register 3"]
  pub pcr3: PCR3,
  #[doc = "0x10 - Pin Control Register 4"]
  pub pcr4: PCR4,
  #[doc = "0x14 - Pin Control Register 5"]
  pub pcr5: PCR5,
  #[doc = "0x18 - Pin Control Register 6"]
  pub pcr6: PCR6,
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
  _reserved12: [u8; 80usize],
  #[doc = "0x80 - Global Pin Control Low Register"]
  pub gpclr: GPCLR,
  #[doc = "0x84 - Global Pin Control High Register"]
  pub gpchr: GPCHR,
  #[doc = "0x88 - Global Interrupt Control Low Register"]
  pub giclr: GICLR,
  #[doc = "0x8c - Global Interrupt Control High Register"]
  pub gichr: GICHR,
  _reserved16: [u8; 16usize],
  #[doc = "0xa0 - Interrupt Status Flag Register"]
  pub isfr: ISFR,
  _reserved17: [u8; 28usize],
  #[doc = "0xc0 - Digital Filter Enable Register"]
  pub dfer: DFER,
  #[doc = "0xc4 - Digital Filter Clock Register"]
  pub dfcr: DFCR,
  #[doc = "0xc8 - Digital Filter Width Register"]
  pub dfwr: DFWR,
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
#[doc = "Pin Control Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcr2](pcr2) module"]
pub type PCR2 = crate::Reg<u32, _PCR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCR2;
#[doc = "`read()` method returns [pcr2::R](pcr2::R) reader structure"]
impl crate::Readable for PCR2 {}
#[doc = "`write(|w| ..)` method takes [pcr2::W](pcr2::W) writer structure"]
impl crate::Writable for PCR2 {}
#[doc = "Pin Control Register 2"]
pub mod pcr2;
#[doc = "Pin Control Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcr3](pcr3) module"]
pub type PCR3 = crate::Reg<u32, _PCR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCR3;
#[doc = "`read()` method returns [pcr3::R](pcr3::R) reader structure"]
impl crate::Readable for PCR3 {}
#[doc = "`write(|w| ..)` method takes [pcr3::W](pcr3::W) writer structure"]
impl crate::Writable for PCR3 {}
#[doc = "Pin Control Register 3"]
pub mod pcr3;
#[doc = "Pin Control Register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcr4](pcr4) module"]
pub type PCR4 = crate::Reg<u32, _PCR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCR4;
#[doc = "`read()` method returns [pcr4::R](pcr4::R) reader structure"]
impl crate::Readable for PCR4 {}
#[doc = "`write(|w| ..)` method takes [pcr4::W](pcr4::W) writer structure"]
impl crate::Writable for PCR4 {}
#[doc = "Pin Control Register 4"]
pub mod pcr4;
#[doc = "Pin Control Register 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcr5](pcr5) module"]
pub type PCR5 = crate::Reg<u32, _PCR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCR5;
#[doc = "`read()` method returns [pcr5::R](pcr5::R) reader structure"]
impl crate::Readable for PCR5 {}
#[doc = "`write(|w| ..)` method takes [pcr5::W](pcr5::W) writer structure"]
impl crate::Writable for PCR5 {}
#[doc = "Pin Control Register 5"]
pub mod pcr5;
#[doc = "Pin Control Register 6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcr6](pcr6) module"]
pub type PCR6 = crate::Reg<u32, _PCR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCR6;
#[doc = "`read()` method returns [pcr6::R](pcr6::R) reader structure"]
impl crate::Readable for PCR6 {}
#[doc = "`write(|w| ..)` method takes [pcr6::W](pcr6::W) writer structure"]
impl crate::Writable for PCR6 {}
#[doc = "Pin Control Register 6"]
pub mod pcr6;
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
#[doc = "Digital Filter Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dfer](dfer) module"]
pub type DFER = crate::Reg<u32, _DFER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFER;
#[doc = "`read()` method returns [dfer::R](dfer::R) reader structure"]
impl crate::Readable for DFER {}
#[doc = "`write(|w| ..)` method takes [dfer::W](dfer::W) writer structure"]
impl crate::Writable for DFER {}
#[doc = "Digital Filter Enable Register"]
pub mod dfer;
#[doc = "Digital Filter Clock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dfcr](dfcr) module"]
pub type DFCR = crate::Reg<u32, _DFCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFCR;
#[doc = "`read()` method returns [dfcr::R](dfcr::R) reader structure"]
impl crate::Readable for DFCR {}
#[doc = "`write(|w| ..)` method takes [dfcr::W](dfcr::W) writer structure"]
impl crate::Writable for DFCR {}
#[doc = "Digital Filter Clock Register"]
pub mod dfcr;
#[doc = "Digital Filter Width Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dfwr](dfwr) module"]
pub type DFWR = crate::Reg<u32, _DFWR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFWR;
#[doc = "`read()` method returns [dfwr::R](dfwr::R) reader structure"]
impl crate::Readable for DFWR {}
#[doc = "`write(|w| ..)` method takes [dfwr::W](dfwr::W) writer structure"]
impl crate::Writable for DFWR {}
#[doc = "Digital Filter Width Register"]
pub mod dfwr;
