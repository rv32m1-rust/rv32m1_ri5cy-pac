#[doc = "Reader of register DBGMADR"]
pub type R = crate::R<u32, super::DBGMADR>;
#[doc = "Writer for register DBGMADR"]
pub type W = crate::W<u32, super::DBGMADR>;
#[doc = "Register DBGMADR `reset()`'s with value 0"]
impl crate::ResetValue for super::DBGMADR {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "Reader of field `DMADDR`"]
pub type DMADDR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DMADDR`"]
pub struct DMADDR_W<'a> {
  w: &'a mut W,
}
impl<'a> DMADDR_W<'a> {
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u32) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x3fff_ffff << 2)) | (((value as u32) & 0x3fff_ffff) << 2);
    self.w
  }
}
impl R {
  #[doc = "Bits 2:31 - Debug Memory Address"]
  #[inline(always)]
  pub fn dmaddr(&self) -> DMADDR_R {
    DMADDR_R::new(((self.bits >> 2) & 0x3fff_ffff) as u32)
  }
}
impl W {
  #[doc = "Bits 2:31 - Debug Memory Address"]
  #[inline(always)]
  pub fn dmaddr(&mut self) -> DMADDR_W {
    DMADDR_W { w: self }
  }
}
