#[doc = "Reader of register RFADDRH"]
pub type R = crate::R<u32, super::RFADDRH>;
#[doc = "Reader of field `MACADDR4`"]
pub type MACADDR4_R = crate::R<u8, u8>;
impl R {
  #[doc = "Bits 0:7 - MACADDR4"]
  #[inline(always)]
  pub fn macaddr4(&self) -> MACADDR4_R {
    MACADDR4_R::new((self.bits & 0xff) as u8)
  }
}
