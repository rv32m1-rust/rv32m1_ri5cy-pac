#[doc = "Reader of register FPR"]
pub type R = crate::R<u32, super::FPR>;
#[doc = "Reader of field `FIFO_RPT`"]
pub type FIFO_RPT_R = crate::R<u8, u8>;
#[doc = "Reader of field `FIFO_WPT`"]
pub type FIFO_WPT_R = crate::R<u8, u8>;
impl R {
  #[doc = "Bits 0:3 - FIFO Read Pointer"]
  #[inline(always)]
  pub fn fifo_rpt(&self) -> FIFO_RPT_R {
    FIFO_RPT_R::new((self.bits & 0x0f) as u8)
  }
  #[doc = "Bits 16:19 - FIFO Write Pointer"]
  #[inline(always)]
  pub fn fifo_wpt(&self) -> FIFO_WPT_R {
    FIFO_WPT_R::new(((self.bits >> 16) & 0x0f) as u8)
  }
}
