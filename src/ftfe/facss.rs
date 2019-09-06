#[doc = "Reader of register FACSS"]
pub type R = crate::R<u8, super::FACSS>;
#[doc = "Reader of field `SGSIZE`"]
pub type SGSIZE_R = crate::R<u8, u8>;
impl R {
  #[doc = "Bits 0:7 - Segment Size"]
  #[inline(always)]
  pub fn sgsize(&self) -> SGSIZE_R {
    SGSIZE_R::new((self.bits & 0xff) as u8)
  }
}
