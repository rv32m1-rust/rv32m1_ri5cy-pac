#[doc = "Reader of register FCR"]
pub type R = crate::R<u32, super::FCR>;
#[doc = "Writer for register FCR"]
pub type W = crate::W<u32, super::FCR>;
#[doc = "Register FCR `reset()`'s with value 0"]
impl crate::ResetValue for super::FCR {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "Reader of field `WML`"]
pub type WML_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WML`"]
pub struct WML_W<'a> {
  w: &'a mut W,
}
impl<'a> WML_W<'a> {
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
    self.w
  }
}
impl R {
  #[doc = "Bits 0:3 - Watermark Level"]
  #[inline(always)]
  pub fn wml(&self) -> WML_R {
    WML_R::new((self.bits & 0x0f) as u8)
  }
}
impl W {
  #[doc = "Bits 0:3 - Watermark Level"]
  #[inline(always)]
  pub fn wml(&mut self) -> WML_W {
    WML_W { w: self }
  }
}
