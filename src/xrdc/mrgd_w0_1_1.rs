#[doc = "Reader of register MRGD_W0_1_1"]
pub type R = crate::R<u32, super::MRGD_W0_1_1>;
#[doc = "Writer for register MRGD_W0_1_1"]
pub type W = crate::W<u32, super::MRGD_W0_1_1>;
#[doc = "Register MRGD_W0_1_1 `reset()`'s with value 0"]
impl crate::ResetValue for super::MRGD_W0_1_1 {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "Reader of field `SRTADDR`"]
pub type SRTADDR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SRTADDR`"]
pub struct SRTADDR_W<'a> {
  w: &'a mut W,
}
impl<'a> SRTADDR_W<'a> {
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u32) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x07ff_ffff << 5)) | (((value as u32) & 0x07ff_ffff) << 5);
    self.w
  }
}
impl R {
  #[doc = "Bits 5:31 - Start Address"]
  #[inline(always)]
  pub fn srtaddr(&self) -> SRTADDR_R {
    SRTADDR_R::new(((self.bits >> 5) & 0x07ff_ffff) as u32)
  }
}
impl W {
  #[doc = "Bits 5:31 - Start Address"]
  #[inline(always)]
  pub fn srtaddr(&mut self) -> SRTADDR_W {
    SRTADDR_W { w: self }
  }
}
