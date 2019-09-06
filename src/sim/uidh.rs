#[doc = "Reader of register UIDH"]
pub type R = crate::R<u32, super::UIDH>;
#[doc = "Reader of field `UID`"]
pub type UID_R = crate::R<u16, u16>;
impl R {
  #[doc = "Bits 0:15 - Unique Identification"]
  #[inline(always)]
  pub fn uid(&self) -> UID_R {
    UID_R::new((self.bits & 0xffff) as u16)
  }
}
