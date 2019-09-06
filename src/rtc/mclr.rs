#[doc = "Reader of register MCLR"]
pub type R = crate::R<u32, super::MCLR>;
#[doc = "Writer for register MCLR"]
pub type W = crate::W<u32, super::MCLR>;
#[doc = "Register MCLR `reset()`'s with value 0"]
impl crate::ResetValue for super::MCLR {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "Reader of field `MCL`"]
pub type MCL_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `MCL`"]
pub struct MCL_W<'a> {
  w: &'a mut W,
}
impl<'a> MCL_W<'a> {
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u32) -> &'a mut W {
    self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
    self.w
  }
}
impl R {
  #[doc = "Bits 0:31 - Monotonic Counter Low"]
  #[inline(always)]
  pub fn mcl(&self) -> MCL_R {
    MCL_R::new((self.bits & 0xffff_ffff) as u32)
  }
}
impl W {
  #[doc = "Bits 0:31 - Monotonic Counter Low"]
  #[inline(always)]
  pub fn mcl(&mut self) -> MCL_W {
    MCL_W { w: self }
  }
}
