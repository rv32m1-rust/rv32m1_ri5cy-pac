#[doc = "Reader of register RCR3"]
pub type R = crate::R<u32, super::RCR3>;
#[doc = "Writer for register RCR3"]
pub type W = crate::W<u32, super::RCR3>;
#[doc = "Register RCR3 `reset()`'s with value 0"]
impl crate::ResetValue for super::RCR3 {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "Reader of field `WDFL`"]
pub type WDFL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WDFL`"]
pub struct WDFL_W<'a> {
  w: &'a mut W,
}
impl<'a> WDFL_W<'a> {
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
    self.w
  }
}
#[doc = "Reader of field `RCE`"]
pub type RCE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RCE`"]
pub struct RCE_W<'a> {
  w: &'a mut W,
}
impl<'a> RCE_W<'a> {
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
    self.w
  }
}
#[doc = "Write proxy for field `CFR`"]
pub struct CFR_W<'a> {
  w: &'a mut W,
}
impl<'a> CFR_W<'a> {
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
    self.w
  }
}
impl R {
  #[doc = "Bits 0:4 - Word Flag Configuration"]
  #[inline(always)]
  pub fn wdfl(&self) -> WDFL_R {
    WDFL_R::new((self.bits & 0x1f) as u8)
  }
  #[doc = "Bits 16:17 - Receive Channel Enable"]
  #[inline(always)]
  pub fn rce(&self) -> RCE_R {
    RCE_R::new(((self.bits >> 16) & 0x03) as u8)
  }
}
impl W {
  #[doc = "Bits 0:4 - Word Flag Configuration"]
  #[inline(always)]
  pub fn wdfl(&mut self) -> WDFL_W {
    WDFL_W { w: self }
  }
  #[doc = "Bits 16:17 - Receive Channel Enable"]
  #[inline(always)]
  pub fn rce(&mut self) -> RCE_W {
    RCE_W { w: self }
  }
  #[doc = "Bits 24:25 - Channel FIFO Reset"]
  #[inline(always)]
  pub fn cfr(&mut self) -> CFR_W {
    CFR_W { w: self }
  }
}
