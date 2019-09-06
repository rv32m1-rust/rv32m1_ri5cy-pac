#[doc = "Writer for register GICHR"]
pub type W = crate::W<u32, super::GICHR>;
#[doc = "Register GICHR `reset()`'s with value 0"]
impl crate::ResetValue for super::GICHR {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "Write proxy for field `GIWE`"]
pub struct GIWE_W<'a> {
  w: &'a mut W,
}
impl<'a> GIWE_W<'a> {
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u16) -> &'a mut W {
    self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
    self.w
  }
}
#[doc = "Write proxy for field `GIWD`"]
pub struct GIWD_W<'a> {
  w: &'a mut W,
}
impl<'a> GIWD_W<'a> {
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u16) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
    self.w
  }
}
impl W {
  #[doc = "Bits 0:15 - Global Interrupt Write Enable"]
  #[inline(always)]
  pub fn giwe(&mut self) -> GIWE_W {
    GIWE_W { w: self }
  }
  #[doc = "Bits 16:31 - Global Interrupt Write Data"]
  #[inline(always)]
  pub fn giwd(&mut self) -> GIWD_W {
    GIWD_W { w: self }
  }
}
