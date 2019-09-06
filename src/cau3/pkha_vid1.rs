#[doc = "Reader of register PKHA_VID1"]
pub type R = crate::R<u32, super::PKHA_VID1>;
#[doc = "Reader of field `MIN_REV`"]
pub type MIN_REV_R = crate::R<u8, u8>;
#[doc = "Reader of field `MAJ_REV`"]
pub type MAJ_REV_R = crate::R<u8, u8>;
#[doc = "Reader of field `IP_ID`"]
pub type IP_ID_R = crate::R<u16, u16>;
impl R {
  #[doc = "Bits 0:7 - Minor Revision Number"]
  #[inline(always)]
  pub fn min_rev(&self) -> MIN_REV_R {
    MIN_REV_R::new((self.bits & 0xff) as u8)
  }
  #[doc = "Bits 8:15 - Major Revision Number"]
  #[inline(always)]
  pub fn maj_rev(&self) -> MAJ_REV_R {
    MAJ_REV_R::new(((self.bits >> 8) & 0xff) as u8)
  }
  #[doc = "Bits 16:31 - Hardware Revision ID"]
  #[inline(always)]
  pub fn ip_id(&self) -> IP_ID_R {
    IP_ID_R::new(((self.bits >> 16) & 0xffff) as u16)
  }
}
