#[doc = "Reader of register DBGPBR"]
pub type R = crate::R<u32, super::DBGPBR>;
#[doc = "Writer for register DBGPBR"]
pub type W = crate::W<u32, super::DBGPBR>;
#[doc = "Register DBGPBR `reset()`'s with value 0"]
impl crate::ResetValue for super::DBGPBR {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "Reader of field `PCBKPT`"]
pub type PCBKPT_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `PCBKPT`"]
pub struct PCBKPT_W<'a> {
  w: &'a mut W,
}
impl<'a> PCBKPT_W<'a> {
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u32) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x0003_ffff << 2)) | (((value as u32) & 0x0003_ffff) << 2);
    self.w
  }
}
impl R {
  #[doc = "Bits 2:19 - PC Breakpoint"]
  #[inline(always)]
  pub fn pcbkpt(&self) -> PCBKPT_R {
    PCBKPT_R::new(((self.bits >> 2) & 0x0003_ffff) as u32)
  }
}
impl W {
  #[doc = "Bits 2:19 - PC Breakpoint"]
  #[inline(always)]
  pub fn pcbkpt(&mut self) -> PCBKPT_W {
    PCBKPT_W { w: self }
  }
}
