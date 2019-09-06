#[doc = "Reader of register CKLFSR"]
pub type R = crate::R<u32, super::CKLFSR>;
#[doc = "Writer for register CKLFSR"]
pub type W = crate::W<u32, super::CKLFSR>;
#[doc = "Register CKLFSR `reset()`'s with value 0"]
impl crate::ResetValue for super::CKLFSR {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "Reader of field `LFSR`"]
pub type LFSR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `LFSR`"]
pub struct LFSR_W<'a> {
  w: &'a mut W,
}
impl<'a> LFSR_W<'a> {
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u32) -> &'a mut W {
    self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
    self.w
  }
}
impl R {
  #[doc = "Bits 0:31 - Linear Feedback Shift Register"]
  #[inline(always)]
  pub fn lfsr(&self) -> LFSR_R {
    LFSR_R::new((self.bits & 0xffff_ffff) as u32)
  }
}
impl W {
  #[doc = "Bits 0:31 - Linear Feedback Shift Register"]
  #[inline(always)]
  pub fn lfsr(&mut self) -> LFSR_W {
    LFSR_W { w: self }
  }
}
