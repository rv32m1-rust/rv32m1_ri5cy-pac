#[doc = "Reader of register DERR_W3_0"]
pub type R = crate::R<u32, super::DERR_W3_0>;
#[doc = "Writer for register DERR_W3_0"]
pub type W = crate::W<u32, super::DERR_W3_0>;
#[doc = "Register DERR_W3_0 `reset()`'s with value 0"]
impl crate::ResetValue for super::DERR_W3_0 {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "Write proxy for field `RECR`"]
pub struct RECR_W<'a> {
  w: &'a mut W,
}
impl<'a> RECR_W<'a> {
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
    self.w
  }
}
impl R {}
impl W {
  #[doc = "Bits 30:31 - Rearm Error Capture Registers"]
  #[inline(always)]
  pub fn recr(&mut self) -> RECR_W {
    RECR_W { w: self }
  }
}
