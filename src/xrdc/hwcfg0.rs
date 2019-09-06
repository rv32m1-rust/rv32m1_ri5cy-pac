#[doc = "Reader of register HWCFG0"]
pub type R = crate::R<u32, super::HWCFG0>;
#[doc = "Reader of field `NDID`"]
pub type NDID_R = crate::R<u8, u8>;
#[doc = "Reader of field `NMSTR`"]
pub type NMSTR_R = crate::R<u8, u8>;
#[doc = "Reader of field `NMRC`"]
pub type NMRC_R = crate::R<u8, u8>;
#[doc = "Reader of field `NPAC`"]
pub type NPAC_R = crate::R<u8, u8>;
#[doc = "Reader of field `MID`"]
pub type MID_R = crate::R<u8, u8>;
impl R {
  #[doc = "Bits 0:7 - Number of domains"]
  #[inline(always)]
  pub fn ndid(&self) -> NDID_R {
    NDID_R::new((self.bits & 0xff) as u8)
  }
  #[doc = "Bits 8:15 - Number of bus masters"]
  #[inline(always)]
  pub fn nmstr(&self) -> NMSTR_R {
    NMSTR_R::new(((self.bits >> 8) & 0xff) as u8)
  }
  #[doc = "Bits 16:23 - Number of MRCs"]
  #[inline(always)]
  pub fn nmrc(&self) -> NMRC_R {
    NMRC_R::new(((self.bits >> 16) & 0xff) as u8)
  }
  #[doc = "Bits 24:27 - Number of PACs"]
  #[inline(always)]
  pub fn npac(&self) -> NPAC_R {
    NPAC_R::new(((self.bits >> 24) & 0x0f) as u8)
  }
  #[doc = "Bits 28:31 - Module ID"]
  #[inline(always)]
  pub fn mid(&self) -> MID_R {
    MID_R::new(((self.bits >> 28) & 0x0f) as u8)
  }
}
