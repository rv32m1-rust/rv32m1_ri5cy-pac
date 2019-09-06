#[doc = "Reader of register BDTPAGE1"]
pub type R = crate::R<u8, super::BDTPAGE1>;
#[doc = "Writer for register BDTPAGE1"]
pub type W = crate::W<u8, super::BDTPAGE1>;
#[doc = "Register BDTPAGE1 `reset()`'s with value 0"]
impl crate::ResetValue for super::BDTPAGE1 {
  type Type = u8;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "Reader of field `BDTBA`"]
pub type BDTBA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BDTBA`"]
pub struct BDTBA_W<'a> {
  w: &'a mut W,
}
impl<'a> BDTBA_W<'a> {
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x7f << 1)) | (((value as u8) & 0x7f) << 1);
    self.w
  }
}
impl R {
  #[doc = "Bits 1:7 - BDTBA"]
  #[inline(always)]
  pub fn bdtba(&self) -> BDTBA_R {
    BDTBA_R::new(((self.bits >> 1) & 0x7f) as u8)
  }
}
impl W {
  #[doc = "Bits 1:7 - BDTBA"]
  #[inline(always)]
  pub fn bdtba(&mut self) -> BDTBA_W {
    BDTBA_W { w: self }
  }
}
