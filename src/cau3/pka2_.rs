#[doc = "Reader of register PKA2_[%s]"]
pub type R = crate::R<u32, super::PKA2_>;
#[doc = "Writer for register PKA2_[%s]"]
pub type W = crate::W<u32, super::PKA2_>;
#[doc = "Register PKA2_[%s] `reset()`'s with value 0"]
impl crate::ResetValue for super::PKA2_ {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "Reader of field `PKHA_A2`"]
pub type PKHA_A2_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `PKHA_A2`"]
pub struct PKHA_A2_W<'a> {
  w: &'a mut W,
}
impl<'a> PKHA_A2_W<'a> {
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u32) -> &'a mut W {
    self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
    self.w
  }
}
impl R {
  #[doc = "Bits 0:31 - A2 VALUE"]
  #[inline(always)]
  pub fn pkha_a2(&self) -> PKHA_A2_R {
    PKHA_A2_R::new((self.bits & 0xffff_ffff) as u32)
  }
}
impl W {
  #[doc = "Bits 0:31 - A2 VALUE"]
  #[inline(always)]
  pub fn pkha_a2(&mut self) -> PKHA_A2_W {
    PKHA_A2_W { w: self }
  }
}
