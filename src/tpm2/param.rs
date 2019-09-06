#[doc = "Reader of register PARAM"]
pub type R = crate::R<u32, super::PARAM>;
#[doc = "Reader of field `CHAN`"]
pub type CHAN_R = crate::R<u8, u8>;
#[doc = "Reader of field `TRIG`"]
pub type TRIG_R = crate::R<u8, u8>;
#[doc = "Reader of field `WIDTH`"]
pub type WIDTH_R = crate::R<u8, u8>;
impl R {
  #[doc = "Bits 0:7 - Channel Count"]
  #[inline(always)]
  pub fn chan(&self) -> CHAN_R {
    CHAN_R::new((self.bits & 0xff) as u8)
  }
  #[doc = "Bits 8:15 - Trigger Count"]
  #[inline(always)]
  pub fn trig(&self) -> TRIG_R {
    TRIG_R::new(((self.bits >> 8) & 0xff) as u8)
  }
  #[doc = "Bits 16:23 - Counter Width"]
  #[inline(always)]
  pub fn width(&self) -> WIDTH_R {
    WIDTH_R::new(((self.bits >> 16) & 0xff) as u8)
  }
}
