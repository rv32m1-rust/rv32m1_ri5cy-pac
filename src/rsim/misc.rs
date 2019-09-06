#[doc = "Reader of register MISC"]
pub type R = crate::R<u32, super::MISC>;
#[doc = "Writer for register MISC"]
pub type W = crate::W<u32, super::MISC>;
#[doc = "Register MISC `reset()`'s with value 0x0500_0000"]
impl crate::ResetValue for super::MISC {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0x0500_0000
  }
}
#[doc = "Reader of field `RADIO_VERSION`"]
pub type RADIO_VERSION_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RADIO_VERSION`"]
pub struct RADIO_VERSION_W<'a> {
  w: &'a mut W,
}
impl<'a> RADIO_VERSION_W<'a> {
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
    self.w
  }
}
impl R {
  #[doc = "Bits 24:31 - Radio Version ID number"]
  #[inline(always)]
  pub fn radio_version(&self) -> RADIO_VERSION_R {
    RADIO_VERSION_R::new(((self.bits >> 24) & 0xff) as u8)
  }
}
impl W {
  #[doc = "Bits 24:31 - Radio Version ID number"]
  #[inline(always)]
  pub fn radio_version(&mut self) -> RADIO_VERSION_W {
    RADIO_VERSION_W { w: self }
  }
}
