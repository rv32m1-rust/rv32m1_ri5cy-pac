#[doc = "Reader of register CC_R12"]
pub type R = crate::R<u32, super::CC_R12>;
#[doc = "Writer for register CC_R12"]
pub type W = crate::W<u32, super::CC_R12>;
#[doc = "Register CC_R12 `reset()`'s with value 0"]
impl crate::ResetValue for super::CC_R12 {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "Reader of field `R`"]
pub type R_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `R`"]
pub struct R_W<'a> {
  w: &'a mut W,
}
impl<'a> R_W<'a> {
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u32) -> &'a mut W {
    self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
    self.w
  }
}
impl R {
  #[doc = "Bits 0:31 - CryptoCore general purpose register R"]
  #[inline(always)]
  pub fn r(&self) -> R_R {
    R_R::new((self.bits & 0xffff_ffff) as u32)
  }
}
impl W {
  #[doc = "Bits 0:31 - CryptoCore general purpose register R"]
  #[inline(always)]
  pub fn r(&mut self) -> R_W {
    R_W { w: self }
  }
}
