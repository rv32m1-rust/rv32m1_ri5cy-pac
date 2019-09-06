#[doc = "Reader of register SRAMDSR"]
pub type R = crate::R<u32, super::SRAMDSR>;
#[doc = "Writer for register SRAMDSR"]
pub type W = crate::W<u32, super::SRAMDSR>;
#[doc = "Register SRAMDSR `reset()`'s with value 0"]
impl crate::ResetValue for super::SRAMDSR {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "Reader of field `DSE`"]
pub type DSE_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DSE`"]
pub struct DSE_W<'a> {
  w: &'a mut W,
}
impl<'a> DSE_W<'a> {
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u32) -> &'a mut W {
    self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
    self.w
  }
}
impl R {
  #[doc = "Bits 0:31 - Deep Sleep Enable"]
  #[inline(always)]
  pub fn dse(&self) -> DSE_R {
    DSE_R::new((self.bits & 0xffff_ffff) as u32)
  }
}
impl W {
  #[doc = "Bits 0:31 - Deep Sleep Enable"]
  #[inline(always)]
  pub fn dse(&mut self) -> DSE_W {
    DSE_W { w: self }
  }
}
