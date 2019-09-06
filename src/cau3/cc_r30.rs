#[doc = "Reader of register CC_R30"]
pub type R = crate::R<u32, super::CC_R30>;
#[doc = "Writer for register CC_R30"]
pub type W = crate::W<u32, super::CC_R30>;
#[doc = "Register CC_R30 `reset()`'s with value 0"]
impl crate::ResetValue for super::CC_R30 {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "Reader of field `SP`"]
pub type SP_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SP`"]
pub struct SP_W<'a> {
  w: &'a mut W,
}
impl<'a> SP_W<'a> {
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u32) -> &'a mut W {
    self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
    self.w
  }
}
impl R {
  #[doc = "Bits 0:31 - Stack Pointer"]
  #[inline(always)]
  pub fn sp(&self) -> SP_R {
    SP_R::new((self.bits & 0xffff_ffff) as u32)
  }
}
impl W {
  #[doc = "Bits 0:31 - Stack Pointer"]
  #[inline(always)]
  pub fn sp(&mut self) -> SP_W {
    SP_W { w: self }
  }
}
