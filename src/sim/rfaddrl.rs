#[doc = "Reader of register RFADDRL"]
pub type R = crate::R<u32, super::RFADDRL>;
#[doc = "Reader of field `MACADDR0`"]
pub type MACADDR0_R = crate::R<u8, u8>;
#[doc = "Reader of field `MACADDR1`"]
pub type MACADDR1_R = crate::R<u8, u8>;
#[doc = "Reader of field `MACADDR2`"]
pub type MACADDR2_R = crate::R<u8, u8>;
#[doc = "Reader of field `MACADDR3`"]
pub type MACADDR3_R = crate::R<u8, u8>;
impl R {
  #[doc = "Bits 0:7 - MACADDR0"]
  #[inline(always)]
  pub fn macaddr0(&self) -> MACADDR0_R {
    MACADDR0_R::new((self.bits & 0xff) as u8)
  }
  #[doc = "Bits 8:15 - MACADDR1"]
  #[inline(always)]
  pub fn macaddr1(&self) -> MACADDR1_R {
    MACADDR1_R::new(((self.bits >> 8) & 0xff) as u8)
  }
  #[doc = "Bits 16:23 - MACADDR2"]
  #[inline(always)]
  pub fn macaddr2(&self) -> MACADDR2_R {
    MACADDR2_R::new(((self.bits >> 16) & 0xff) as u8)
  }
  #[doc = "Bits 24:31 - MACADDR3"]
  #[inline(always)]
  pub fn macaddr3(&self) -> MACADDR3_R {
    MACADDR3_R::new(((self.bits >> 24) & 0xff) as u8)
  }
}
