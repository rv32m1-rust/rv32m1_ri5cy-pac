#[doc = "Writer for register RSTGT_W"]
pub type W = crate::W<u16, super::RSTGT_W>;
#[doc = "Register RSTGT_W `reset()`'s with value 0"]
impl crate::ResetValue for super::RSTGT_W {
  type Type = u16;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "Write proxy for field `RSTGTN`"]
pub struct RSTGTN_W<'a> {
  w: &'a mut W,
}
impl<'a> RSTGTN_W<'a> {
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !0xff) | ((value as u16) & 0xff);
    self.w
  }
}
#[doc = "Write proxy for field `RSTGDP`"]
pub struct RSTGDP_W<'a> {
  w: &'a mut W,
}
impl<'a> RSTGDP_W<'a> {
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u16) & 0xff) << 8);
    self.w
  }
}
impl W {
  #[doc = "Bits 0:7 - RSTGTN"]
  #[inline(always)]
  pub fn rstgtn(&mut self) -> RSTGTN_W {
    RSTGTN_W { w: self }
  }
  #[doc = "Bits 8:15 - RSTGDP"]
  #[inline(always)]
  pub fn rstgdp(&mut self) -> RSTGDP_W {
    RSTGDP_W { w: self }
  }
}
