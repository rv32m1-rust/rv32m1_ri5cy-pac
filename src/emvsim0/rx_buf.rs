#[doc = "Reader of register RX_BUF"]
pub type R = crate::R<u32, super::RX_BUF>;
#[doc = "Reader of field `RX_BYTE`"]
pub type RX_BYTE_R = crate::R<u8, u8>;
impl R {
  #[doc = "Bits 0:7 - Receive Data Byte Read"]
  #[inline(always)]
  pub fn rx_byte(&self) -> RX_BYTE_R {
    RX_BYTE_R::new((self.bits & 0xff) as u8)
  }
}
