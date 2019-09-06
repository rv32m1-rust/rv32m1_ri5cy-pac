#[doc = "Reader of register DERRLOC[%s]"]
pub type R = crate::R<u32, super::DERRLOC>;
#[doc = "Reader of field `MRCINST`"]
pub type MRCINST_R = crate::R<u16, u16>;
#[doc = "Reader of field `PACINST`"]
pub type PACINST_R = crate::R<u8, u8>;
impl R {
  #[doc = "Bits 0:15 - MRC instance"]
  #[inline(always)]
  pub fn mrcinst(&self) -> MRCINST_R {
    MRCINST_R::new((self.bits & 0xffff) as u16)
  }
  #[doc = "Bits 16:19 - PAC instance"]
  #[inline(always)]
  pub fn pacinst(&self) -> PACINST_R {
    PACINST_R::new(((self.bits >> 16) & 0x0f) as u8)
  }
}
