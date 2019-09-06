#[doc = "Reader of register TX_BUF"]
pub type R = crate::R<u32, super::TX_BUF>;
#[doc = "Writer for register TX_BUF"]
pub type W = crate::W<u32, super::TX_BUF>;
#[doc = "Register TX_BUF `reset()`'s with value 0"]
impl crate::ResetValue for super::TX_BUF {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "Write proxy for field `TX_BYTE`"]
pub struct TX_BYTE_W<'a> {
  w: &'a mut W,
}
impl<'a> TX_BYTE_W<'a> {
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
    self.w
  }
}
impl R {}
impl W {
  #[doc = "Bits 0:7 - Transmit Data Byte"]
  #[inline(always)]
  pub fn tx_byte(&mut self) -> TX_BYTE_W {
    TX_BYTE_W { w: self }
  }
}
