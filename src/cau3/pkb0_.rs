#[doc = "Reader of register PKB0_[%s]"]
pub type R = crate::R<u32, super::PKB0_>;
#[doc = "Writer for register PKB0_[%s]"]
pub type W = crate::W<u32, super::PKB0_>;
#[doc = "Register PKB0_[%s] `reset()`'s with value 0"]
impl crate::ResetValue for super::PKB0_ {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "Reader of field `PKHA_B0`"]
pub type PKHA_B0_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `PKHA_B0`"]
pub struct PKHA_B0_W<'a> {
  w: &'a mut W,
}
impl<'a> PKHA_B0_W<'a> {
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u32) -> &'a mut W {
    self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
    self.w
  }
}
impl R {
  #[doc = "Bits 0:31 - B0 VALUE"]
  #[inline(always)]
  pub fn pkha_b0(&self) -> PKHA_B0_R {
    PKHA_B0_R::new((self.bits & 0xffff_ffff) as u32)
  }
}
impl W {
  #[doc = "Bits 0:31 - B0 VALUE"]
  #[inline(always)]
  pub fn pkha_b0(&mut self) -> PKHA_B0_W {
    PKHA_B0_W { w: self }
  }
}
