#[doc = "Writer for register PKE_[%s]"]
pub type W = crate::W<u32, super::PKE_>;
#[doc = "Register PKE_[%s] `reset()`'s with value 0"]
impl crate::ResetValue for super::PKE_ {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "Write proxy for field `PKHA_E`"]
pub struct PKHA_E_W<'a> {
  w: &'a mut W,
}
impl<'a> PKHA_E_W<'a> {
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u32) -> &'a mut W {
    self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
    self.w
  }
}
impl W {
  #[doc = "Bits 0:31 - E VALUE"]
  #[inline(always)]
  pub fn pkha_e(&mut self) -> PKHA_E_W {
    PKHA_E_W { w: self }
  }
}
