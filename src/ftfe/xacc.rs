#[doc = "Reader of register XACC%s"]
pub type R = crate::R<u8, super::XACC>;
#[doc = "Reader of field `XA`"]
pub type XA_R = crate::R<u8, u8>;
impl R {
  #[doc = "Bits 0:7 - Execute-only access control"]
  #[inline(always)]
  pub fn xa(&self) -> XA_R {
    XA_R::new((self.bits & 0xff) as u8)
  }
}
