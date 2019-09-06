#[doc = "Reader of register CC_PC"]
pub type R = crate::R<u32, super::CC_PC>;
#[doc = "Writer for register CC_PC"]
pub type W = crate::W<u32, super::CC_PC>;
#[doc = "Register CC_PC `reset()`'s with value 0x0008_0000"]
impl crate::ResetValue for super::CC_PC {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0x0008_0000
  }
}
#[doc = "Reader of field `PC`"]
pub type PC_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `PC`"]
pub struct PC_W<'a> {
  w: &'a mut W,
}
impl<'a> PC_W<'a> {
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u32) -> &'a mut W {
    self.w.bits = (self.w.bits & !0x000f_ffff) | ((value as u32) & 0x000f_ffff);
    self.w
  }
}
impl R {
  #[doc = "Bits 0:19 - Program Counter"]
  #[inline(always)]
  pub fn pc(&self) -> PC_R {
    PC_R::new((self.bits & 0x000f_ffff) as u32)
  }
}
impl W {
  #[doc = "Bits 0:19 - Program Counter"]
  #[inline(always)]
  pub fn pc(&mut self) -> PC_W {
    PC_W { w: self }
  }
}
