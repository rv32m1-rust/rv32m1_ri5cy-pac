#[doc = "Reader of register PDAC_W0_0_51"]
pub type R = crate::R<u32, super::PDAC_W0_0_51>;
#[doc = "Writer for register PDAC_W0_0_51"]
pub type W = crate::W<u32, super::PDAC_W0_0_51>;
#[doc = "Register PDAC_W0_0_51 `reset()`'s with value 0"]
impl crate::ResetValue for super::PDAC_W0_0_51 {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "Reader of field `D0ACP`"]
pub type D0ACP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `D0ACP`"]
pub struct D0ACP_W<'a> {
  w: &'a mut W,
}
impl<'a> D0ACP_W<'a> {
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
    self.w
  }
}
#[doc = "Reader of field `D1ACP`"]
pub type D1ACP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `D1ACP`"]
pub struct D1ACP_W<'a> {
  w: &'a mut W,
}
impl<'a> D1ACP_W<'a> {
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x07 << 3)) | (((value as u32) & 0x07) << 3);
    self.w
  }
}
#[doc = "Reader of field `D2ACP`"]
pub type D2ACP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `D2ACP`"]
pub struct D2ACP_W<'a> {
  w: &'a mut W,
}
impl<'a> D2ACP_W<'a> {
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x07 << 6)) | (((value as u32) & 0x07) << 6);
    self.w
  }
}
#[doc = "Reader of field `EALO`"]
pub type EALO_R = crate::R<u8, u8>;
impl R {
  #[doc = "Bits 0:2 - Domain 0 access control policy"]
  #[inline(always)]
  pub fn d0acp(&self) -> D0ACP_R {
    D0ACP_R::new((self.bits & 0x07) as u8)
  }
  #[doc = "Bits 3:5 - Domain 1 access control policy"]
  #[inline(always)]
  pub fn d1acp(&self) -> D1ACP_R {
    D1ACP_R::new(((self.bits >> 3) & 0x07) as u8)
  }
  #[doc = "Bits 6:8 - Domain 2 access control policy"]
  #[inline(always)]
  pub fn d2acp(&self) -> D2ACP_R {
    D2ACP_R::new(((self.bits >> 6) & 0x07) as u8)
  }
  #[doc = "Bits 24:27 - Excessive Access Lock Owner"]
  #[inline(always)]
  pub fn ealo(&self) -> EALO_R {
    EALO_R::new(((self.bits >> 24) & 0x0f) as u8)
  }
}
impl W {
  #[doc = "Bits 0:2 - Domain 0 access control policy"]
  #[inline(always)]
  pub fn d0acp(&mut self) -> D0ACP_W {
    D0ACP_W { w: self }
  }
  #[doc = "Bits 3:5 - Domain 1 access control policy"]
  #[inline(always)]
  pub fn d1acp(&mut self) -> D1ACP_W {
    D1ACP_W { w: self }
  }
  #[doc = "Bits 6:8 - Domain 2 access control policy"]
  #[inline(always)]
  pub fn d2acp(&mut self) -> D2ACP_W {
    D2ACP_W { w: self }
  }
}
