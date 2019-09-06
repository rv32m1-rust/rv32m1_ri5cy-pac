#[doc = "Reader of register PERID"]
pub type R = crate::R<u8, super::PERID>;
#[doc = "Reader of field `ID`"]
pub type ID_R = crate::R<u8, u8>;
impl R {
  #[doc = "Bits 0:5 - Peripheral Identification"]
  #[inline(always)]
  pub fn id(&self) -> ID_R {
    ID_R::new((self.bits & 0x3f) as u8)
  }
}
