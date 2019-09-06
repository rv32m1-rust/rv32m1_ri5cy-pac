#[doc = "Reader of register TCD5_SLAST"]
pub type R = crate::R<u32, super::TCD5_SLAST>;
#[doc = "Writer for register TCD5_SLAST"]
pub type W = crate::W<u32, super::TCD5_SLAST>;
#[doc = "Register TCD5_SLAST `reset()`'s with value 0"]
impl crate::ResetValue for super::TCD5_SLAST {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "Reader of field `SLAST`"]
pub type SLAST_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SLAST`"]
pub struct SLAST_W<'a> {
  w: &'a mut W,
}
impl<'a> SLAST_W<'a> {
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u32) -> &'a mut W {
    self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
    self.w
  }
}
impl R {
  #[doc = "Bits 0:31 - Last Source Address Adjustment"]
  #[inline(always)]
  pub fn slast(&self) -> SLAST_R {
    SLAST_R::new((self.bits & 0xffff_ffff) as u32)
  }
}
impl W {
  #[doc = "Bits 0:31 - Last Source Address Adjustment"]
  #[inline(always)]
  pub fn slast(&mut self) -> SLAST_W {
    SLAST_W { w: self }
  }
}
