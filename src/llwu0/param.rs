#[doc = "Reader of register PARAM"]
pub type R = crate::R<u32, super::PARAM>;
#[doc = "Reader of field `FILTERS`"]
pub type FILTERS_R = crate::R<u8, u8>;
#[doc = "Reader of field `DMAS`"]
pub type DMAS_R = crate::R<u8, u8>;
#[doc = "Reader of field `MODULES`"]
pub type MODULES_R = crate::R<u8, u8>;
#[doc = "Reader of field `PINS`"]
pub type PINS_R = crate::R<u8, u8>;
impl R {
  #[doc = "Bits 0:7 - Filter Number"]
  #[inline(always)]
  pub fn filters(&self) -> FILTERS_R {
    FILTERS_R::new((self.bits & 0xff) as u8)
  }
  #[doc = "Bits 8:15 - DMA Number"]
  #[inline(always)]
  pub fn dmas(&self) -> DMAS_R {
    DMAS_R::new(((self.bits >> 8) & 0xff) as u8)
  }
  #[doc = "Bits 16:23 - Module Number"]
  #[inline(always)]
  pub fn modules(&self) -> MODULES_R {
    MODULES_R::new(((self.bits >> 16) & 0xff) as u8)
  }
  #[doc = "Bits 24:31 - Pin Number"]
  #[inline(always)]
  pub fn pins(&self) -> PINS_R {
    PINS_R::new(((self.bits >> 24) & 0xff) as u8)
  }
}
