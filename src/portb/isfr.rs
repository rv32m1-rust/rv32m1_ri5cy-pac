#[doc = "Reader of register ISFR"]
pub type R = crate::R<u32, super::ISFR>;
#[doc = "Writer for register ISFR"]
pub type W = crate::W<u32, super::ISFR>;
#[doc = "Register ISFR `reset()`'s with value 0"]
impl crate::ResetValue for super::ISFR {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "Reader of field `ISF`"]
pub type ISF_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ISF`"]
pub struct ISF_W<'a> {
  w: &'a mut W,
}
impl<'a> ISF_W<'a> {
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u32) -> &'a mut W {
    self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
    self.w
  }
}
impl R {
  #[doc = "Bits 0:31 - Interrupt Status Flag"]
  #[inline(always)]
  pub fn isf(&self) -> ISF_R {
    ISF_R::new((self.bits & 0xffff_ffff) as u32)
  }
}
impl W {
  #[doc = "Bits 0:31 - Interrupt Status Flag"]
  #[inline(always)]
  pub fn isf(&mut self) -> ISF_W {
    ISF_W { w: self }
  }
}
