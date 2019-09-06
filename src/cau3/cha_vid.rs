#[doc = "Reader of register CHA_VID"]
pub type R = crate::R<u32, super::CHA_VID>;
#[doc = "Reader of field `PKHAREV`"]
pub type PKHAREV_R = crate::R<u8, u8>;
#[doc = "Reader of field `PKHAVID`"]
pub type PKHAVID_R = crate::R<u8, u8>;
impl R {
  #[doc = "Bits 16:19 - PK Revision NUmber"]
  #[inline(always)]
  pub fn pkharev(&self) -> PKHAREV_R {
    PKHAREV_R::new(((self.bits >> 16) & 0x0f) as u8)
  }
  #[doc = "Bits 20:23 - PK Version ID"]
  #[inline(always)]
  pub fn pkhavid(&self) -> PKHAVID_R {
    PKHAVID_R::new(((self.bits >> 20) & 0x0f) as u8)
  }
}
