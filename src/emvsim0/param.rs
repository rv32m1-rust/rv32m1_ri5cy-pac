#[doc = "Reader of register PARAM"]
pub type R = crate::R<u32, super::PARAM>;
#[doc = "Reader of field `RX_FIFO_DEPTH`"]
pub type RX_FIFO_DEPTH_R = crate::R<u8, u8>;
#[doc = "Reader of field `TX_FIFO_DEPTH`"]
pub type TX_FIFO_DEPTH_R = crate::R<u8, u8>;
impl R {
  #[doc = "Bits 0:7 - Receive FIFO Depth"]
  #[inline(always)]
  pub fn rx_fifo_depth(&self) -> RX_FIFO_DEPTH_R {
    RX_FIFO_DEPTH_R::new((self.bits & 0xff) as u8)
  }
  #[doc = "Bits 8:15 - Transmit FIFO Depth"]
  #[inline(always)]
  pub fn tx_fifo_depth(&self) -> TX_FIFO_DEPTH_R {
    TX_FIFO_DEPTH_R::new(((self.bits >> 8) & 0xff) as u8)
  }
}
