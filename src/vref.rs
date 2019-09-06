#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
  #[doc = "0x00 - VREF Trim Register"]
  pub trm: TRM,
  #[doc = "0x01 - VREF Status and Control Register"]
  pub sc: SC,
  _reserved2: [u8; 3usize],
  #[doc = "0x05 - VREF Trim 2.1V Register"]
  pub trm4: TRM4,
}
#[doc = "VREF Trim Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [trm](trm) module"]
pub type TRM = crate::Reg<u8, _TRM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRM;
#[doc = "`read()` method returns [trm::R](trm::R) reader structure"]
impl crate::Readable for TRM {}
#[doc = "`write(|w| ..)` method takes [trm::W](trm::W) writer structure"]
impl crate::Writable for TRM {}
#[doc = "VREF Trim Register"]
pub mod trm;
#[doc = "VREF Status and Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sc](sc) module"]
pub type SC = crate::Reg<u8, _SC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SC;
#[doc = "`read()` method returns [sc::R](sc::R) reader structure"]
impl crate::Readable for SC {}
#[doc = "`write(|w| ..)` method takes [sc::W](sc::W) writer structure"]
impl crate::Writable for SC {}
#[doc = "VREF Status and Control Register"]
pub mod sc;
#[doc = "VREF Trim 2.1V Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [trm4](trm4) module"]
pub type TRM4 = crate::Reg<u8, _TRM4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRM4;
#[doc = "`read()` method returns [trm4::R](trm4::R) reader structure"]
impl crate::Readable for TRM4 {}
#[doc = "`write(|w| ..)` method takes [trm4::W](trm4::W) writer structure"]
impl crate::Writable for TRM4 {}
#[doc = "VREF Trim 2.1V Register"]
pub mod trm4;
