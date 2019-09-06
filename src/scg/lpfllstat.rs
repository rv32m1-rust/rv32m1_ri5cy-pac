#[doc = "Reader of register LPFLLSTAT"]
pub type R = crate::R<u32, super::LPFLLSTAT>;
#[doc = "Writer for register LPFLLSTAT"]
pub type W = crate::W<u32, super::LPFLLSTAT>;
#[doc = "Register LPFLLSTAT `reset()`'s with value 0"]
impl crate::ResetValue for super::LPFLLSTAT {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "Reader of field `AUTOTRIM`"]
pub type AUTOTRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AUTOTRIM`"]
pub struct AUTOTRIM_W<'a> {
  w: &'a mut W,
}
impl<'a> AUTOTRIM_W<'a> {
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
    self.w
  }
}
impl R {
  #[doc = "Bits 0:7 - Auto Tune Trim Status"]
  #[inline(always)]
  pub fn autotrim(&self) -> AUTOTRIM_R {
    AUTOTRIM_R::new((self.bits & 0xff) as u8)
  }
}
impl W {
  #[doc = "Bits 0:7 - Auto Tune Trim Status"]
  #[inline(always)]
  pub fn autotrim(&mut self) -> AUTOTRIM_W {
    AUTOTRIM_W { w: self }
  }
}
