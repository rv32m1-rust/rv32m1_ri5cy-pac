#[doc = "Reader of register FACSSS"]
pub type R = crate::R<u8, super::FACSSS>;
#[doc = "Reader of field `SGSIZE_S`"]
pub type SGSIZE_S_R = crate::R<u8, u8>;
impl R {
  #[doc = "Bits 0:7 - Segment Size"]
  #[inline(always)]
  pub fn sgsize_s(&self) -> SGSIZE_S_R {
    SGSIZE_S_R::new((self.bits & 0xff) as u8)
  }
}
