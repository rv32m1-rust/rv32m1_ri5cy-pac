#[doc = "Reader of register FIRCSTAT"]
pub type R = crate::R<u32, super::FIRCSTAT>;
#[doc = "Writer for register FIRCSTAT"]
pub type W = crate::W<u32, super::FIRCSTAT>;
#[doc = "Register FIRCSTAT `reset()`'s with value 0"]
impl crate::ResetValue for super::FIRCSTAT {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "Reader of field `TRIMFINE`"]
pub type TRIMFINE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TRIMFINE`"]
pub struct TRIMFINE_W<'a> {
  w: &'a mut W,
}
impl<'a> TRIMFINE_W<'a> {
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
    self.w
  }
}
#[doc = "Reader of field `TRIMCOAR`"]
pub type TRIMCOAR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TRIMCOAR`"]
pub struct TRIMCOAR_W<'a> {
  w: &'a mut W,
}
impl<'a> TRIMCOAR_W<'a> {
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x3f << 8)) | (((value as u32) & 0x3f) << 8);
    self.w
  }
}
impl R {
  #[doc = "Bits 0:6 - Trim Fine Status"]
  #[inline(always)]
  pub fn trimfine(&self) -> TRIMFINE_R {
    TRIMFINE_R::new((self.bits & 0x7f) as u8)
  }
  #[doc = "Bits 8:13 - Trim Coarse"]
  #[inline(always)]
  pub fn trimcoar(&self) -> TRIMCOAR_R {
    TRIMCOAR_R::new(((self.bits >> 8) & 0x3f) as u8)
  }
}
impl W {
  #[doc = "Bits 0:6 - Trim Fine Status"]
  #[inline(always)]
  pub fn trimfine(&mut self) -> TRIMFINE_W {
    TRIMFINE_W { w: self }
  }
  #[doc = "Bits 8:13 - Trim Coarse"]
  #[inline(always)]
  pub fn trimcoar(&mut self) -> TRIMCOAR_W {
    TRIMCOAR_W { w: self }
  }
}
