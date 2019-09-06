#[doc = "Reader of register IDCOMP"]
pub type R = crate::R<u8, super::IDCOMP>;
#[doc = "Reader of field `NID`"]
pub type NID_R = crate::R<u8, u8>;
impl R {
  #[doc = "Bits 0:5 - NID"]
  #[inline(always)]
  pub fn nid(&self) -> NID_R {
    NID_R::new((self.bits & 0x3f) as u8)
  }
}
