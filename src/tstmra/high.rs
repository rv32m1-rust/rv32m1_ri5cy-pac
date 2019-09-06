#[doc = "Reader of register HIGH"]
pub type R = crate::R<u32, super::HIGH>;
#[doc = "Reader of field `VALUE`"]
pub type VALUE_R = crate::R<u32, u32>;
impl R {
  #[doc = "Bits 0:23 - Time Stamp Timer High"]
  #[inline(always)]
  pub fn value(&self) -> VALUE_R {
    VALUE_R::new((self.bits & 0x00ff_ffff) as u32)
  }
}
