#[doc = "Writer for register GPCHR"]
pub type W = crate::W<u32, super::GPCHR>;
#[doc = "Register GPCHR `reset()`'s with value 0"]
impl crate::ResetValue for super::GPCHR {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "Write proxy for field `GPWD`"]
pub struct GPWD_W<'a> {
  w: &'a mut W,
}
impl<'a> GPWD_W<'a> {
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u16) -> &'a mut W {
    self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
    self.w
  }
}
#[doc = "Write proxy for field `GPWE`"]
pub struct GPWE_W<'a> {
  w: &'a mut W,
}
impl<'a> GPWE_W<'a> {
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u16) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
    self.w
  }
}
impl W {
  #[doc = "Bits 0:15 - Global Pin Write Data"]
  #[inline(always)]
  pub fn gpwd(&mut self) -> GPWD_W {
    GPWD_W { w: self }
  }
  #[doc = "Bits 16:31 - Global Pin Write Enable"]
  #[inline(always)]
  pub fn gpwe(&mut self) -> GPWE_W {
    GPWE_W { w: self }
  }
}
