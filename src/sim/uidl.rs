#[doc = "Reader of register UIDL"]
pub type R = crate::R<u32, super::UIDL>;
#[doc = "Reader of field `UID`"]
pub type UID_R = crate::R<u32, u32>;
impl R {
  #[doc = "Bits 0:31 - Unique Identification"]
  #[inline(always)]
  pub fn uid(&self) -> UID_R {
    UID_R::new((self.bits & 0xffff_ffff) as u32)
  }
}
