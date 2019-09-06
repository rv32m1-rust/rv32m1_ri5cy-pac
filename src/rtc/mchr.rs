#[doc = "Reader of register MCHR"]
pub type R = crate::R<u32, super::MCHR>;
#[doc = "Writer for register MCHR"]
pub type W = crate::W<u32, super::MCHR>;
#[doc = "Register MCHR `reset()`'s with value 0"]
impl crate::ResetValue for super::MCHR {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "Reader of field `MCH`"]
pub type MCH_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `MCH`"]
pub struct MCH_W<'a> {
  w: &'a mut W,
}
impl<'a> MCH_W<'a> {
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u32) -> &'a mut W {
    self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
    self.w
  }
}
impl R {
  #[doc = "Bits 0:31 - Monotonic Counter High"]
  #[inline(always)]
  pub fn mch(&self) -> MCH_R {
    MCH_R::new((self.bits & 0xffff_ffff) as u32)
  }
}
impl W {
  #[doc = "Bits 0:31 - Monotonic Counter High"]
  #[inline(always)]
  pub fn mch(&mut self) -> MCH_W {
    MCH_W { w: self }
  }
}
