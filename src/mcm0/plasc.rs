#[doc = "Reader of register PLASC"]
pub type R = crate::R<u16, super::PLASC>;
#[doc = "Reader of field `ASC`"]
pub type ASC_R = crate::R<u8, u8>;
impl R {
  #[doc = "Bits 0:7 - Each bit in the ASC field indicates whether there is a corresponding connection to the crossbar switch's slave input port."]
  #[inline(always)]
  pub fn asc(&self) -> ASC_R {
    ASC_R::new((self.bits & 0xff) as u8)
  }
}
