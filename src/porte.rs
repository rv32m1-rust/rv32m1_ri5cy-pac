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
  _reserved6: [u8; 8usize],
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
  #[doc = "0x34 - Pin Control Register 13"]
  pub pcr13: PCR13,
  #[doc = "0x38 - Pin Control Register 14"]
  pub pcr14: PCR14,
  #[doc = "0x3c - Pin Control Register 15"]
  pub pcr15: PCR15,
  #[doc = "0x40 - Pin Control Register 16"]
  pub pcr16: PCR16,
  #[doc = "0x44 - Pin Control Register 17"]
  pub pcr17: PCR17,
  #[doc = "0x48 - Pin Control Register 18"]
  pub pcr18: PCR18,
  #[doc = "0x4c - Pin Control Register 19"]
  pub pcr19: PCR19,
  _reserved18: [u8; 4usize],
  #[doc = "0x54 - Pin Control Register 21"]
  pub pcr21: PCR21,
  #[doc = "0x58 - Pin Control Register 22"]
  pub pcr22: PCR22,
  _reserved20: [u8; 16usize],
  #[doc = "0x6c - Pin Control Register 27"]
  pub pcr27: PCR27,
  #[doc = "0x70 - Pin Control Register 28"]
  pub pcr28: PCR28,
  #[doc = "0x74 - Pin Control Register 29"]
  pub pcr29: PCR29,
  #[doc = "0x78 - Pin Control Register 30"]
  pub pcr30: PCR30,
  _reserved24: [u8; 4usize],
  #[doc = "0x80 - Global Pin Control Low Register"]
  pub gpclr: GPCLR,
  #[doc = "0x84 - Global Pin Control High Register"]
  pub gpchr: GPCHR,
  #[doc = "0x88 - Global Interrupt Control Low Register"]
  pub giclr: GICLR,
  #[doc = "0x8c - Global Interrupt Control High Register"]
  pub gichr: GICHR,
  _reserved28: [u8; 16usize],
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
#[doc = "Pin Control Register 13\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcr13](pcr13) module"]
pub type PCR13 = crate::Reg<u32, _PCR13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCR13;
#[doc = "`read()` method returns [pcr13::R](pcr13::R) reader structure"]
impl crate::Readable for PCR13 {}
#[doc = "`write(|w| ..)` method takes [pcr13::W](pcr13::W) writer structure"]
impl crate::Writable for PCR13 {}
#[doc = "Pin Control Register 13"]
pub mod pcr13;
#[doc = "Pin Control Register 14\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcr14](pcr14) module"]
pub type PCR14 = crate::Reg<u32, _PCR14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCR14;
#[doc = "`read()` method returns [pcr14::R](pcr14::R) reader structure"]
impl crate::Readable for PCR14 {}
#[doc = "`write(|w| ..)` method takes [pcr14::W](pcr14::W) writer structure"]
impl crate::Writable for PCR14 {}
#[doc = "Pin Control Register 14"]
pub mod pcr14;
#[doc = "Pin Control Register 15\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcr15](pcr15) module"]
pub type PCR15 = crate::Reg<u32, _PCR15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCR15;
#[doc = "`read()` method returns [pcr15::R](pcr15::R) reader structure"]
impl crate::Readable for PCR15 {}
#[doc = "`write(|w| ..)` method takes [pcr15::W](pcr15::W) writer structure"]
impl crate::Writable for PCR15 {}
#[doc = "Pin Control Register 15"]
pub mod pcr15;
#[doc = "Pin Control Register 16\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcr16](pcr16) module"]
pub type PCR16 = crate::Reg<u32, _PCR16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCR16;
#[doc = "`read()` method returns [pcr16::R](pcr16::R) reader structure"]
impl crate::Readable for PCR16 {}
#[doc = "`write(|w| ..)` method takes [pcr16::W](pcr16::W) writer structure"]
impl crate::Writable for PCR16 {}
#[doc = "Pin Control Register 16"]
pub mod pcr16;
#[doc = "Pin Control Register 17\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcr17](pcr17) module"]
pub type PCR17 = crate::Reg<u32, _PCR17>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCR17;
#[doc = "`read()` method returns [pcr17::R](pcr17::R) reader structure"]
impl crate::Readable for PCR17 {}
#[doc = "`write(|w| ..)` method takes [pcr17::W](pcr17::W) writer structure"]
impl crate::Writable for PCR17 {}
#[doc = "Pin Control Register 17"]
pub mod pcr17;
#[doc = "Pin Control Register 18\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcr18](pcr18) module"]
pub type PCR18 = crate::Reg<u32, _PCR18>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCR18;
#[doc = "`read()` method returns [pcr18::R](pcr18::R) reader structure"]
impl crate::Readable for PCR18 {}
#[doc = "`write(|w| ..)` method takes [pcr18::W](pcr18::W) writer structure"]
impl crate::Writable for PCR18 {}
#[doc = "Pin Control Register 18"]
pub mod pcr18;
#[doc = "Pin Control Register 19\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcr19](pcr19) module"]
pub type PCR19 = crate::Reg<u32, _PCR19>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCR19;
#[doc = "`read()` method returns [pcr19::R](pcr19::R) reader structure"]
impl crate::Readable for PCR19 {}
#[doc = "`write(|w| ..)` method takes [pcr19::W](pcr19::W) writer structure"]
impl crate::Writable for PCR19 {}
#[doc = "Pin Control Register 19"]
pub mod pcr19;
#[doc = "Pin Control Register 21\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcr21](pcr21) module"]
pub type PCR21 = crate::Reg<u32, _PCR21>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCR21;
#[doc = "`read()` method returns [pcr21::R](pcr21::R) reader structure"]
impl crate::Readable for PCR21 {}
#[doc = "`write(|w| ..)` method takes [pcr21::W](pcr21::W) writer structure"]
impl crate::Writable for PCR21 {}
#[doc = "Pin Control Register 21"]
pub mod pcr21;
#[doc = "Pin Control Register 22\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcr22](pcr22) module"]
pub type PCR22 = crate::Reg<u32, _PCR22>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCR22;
#[doc = "`read()` method returns [pcr22::R](pcr22::R) reader structure"]
impl crate::Readable for PCR22 {}
#[doc = "`write(|w| ..)` method takes [pcr22::W](pcr22::W) writer structure"]
impl crate::Writable for PCR22 {}
#[doc = "Pin Control Register 22"]
pub mod pcr22;
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
