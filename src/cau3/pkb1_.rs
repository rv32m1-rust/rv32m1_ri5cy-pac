#[doc = "Reader of register PKB1_[%s]"]
pub type R = crate::R<u32, super::PKB1_>;
#[doc = "Writer for register PKB1_[%s]"]
pub type W = crate::W<u32, super::PKB1_>;
#[doc = "Register PKB1_[%s] `reset()`'s with value 0"]
impl crate::ResetValue for super::PKB1_ {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "Reader of field `PKHA_B1`"]
pub type PKHA_B1_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `PKHA_B1`"]
pub struct PKHA_B1_W<'a> {
  w: &'a mut W,
}
impl<'a> PKHA_B1_W<'a> {
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u32) -> &'a mut W {
    self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
    self.w
  }
}
impl R {
  #[doc = "Bits 0:31 - B1 VALUE"]
  #[inline(always)]
  pub fn pkha_b1(&self) -> PKHA_B1_R {
    PKHA_B1_R::new((self.bits & 0xffff_ffff) as u32)
  }
}
impl W {
  #[doc = "Bits 0:31 - B1 VALUE"]
  #[inline(always)]
  pub fn pkha_b1(&mut self) -> PKHA_B1_W {
    PKHA_B1_W { w: self }
  }
}
