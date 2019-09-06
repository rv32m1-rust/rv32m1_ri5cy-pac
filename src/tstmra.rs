#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
  #[doc = "0x00 - Time Stamp Timer Register Low"]
  pub low: LOW,
  #[doc = "0x04 - Time Stamp Timer Register High"]
  pub high: HIGH,
}
#[doc = "Time Stamp Timer Register Low\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [low](low) module"]
pub type LOW = crate::Reg<u32, _LOW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LOW;
#[doc = "`read()` method returns [low::R](low::R) reader structure"]
impl crate::Readable for LOW {}
#[doc = "Time Stamp Timer Register Low"]
pub mod low;
#[doc = "Time Stamp Timer Register High\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [high](high) module"]
pub type HIGH = crate::Reg<u32, _HIGH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HIGH;
#[doc = "`read()` method returns [high::R](high::R) reader structure"]
impl crate::Readable for HIGH {}
#[doc = "Time Stamp Timer Register High"]
pub mod high;
