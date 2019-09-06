#[doc = "Reader of register XACCS%s"]
pub type R = crate::R<u8, super::XACCS>;
#[doc = "Reader of field `XA_S`"]
pub type XA_S_R = crate::R<u8, u8>;
impl R {
  #[doc = "Bits 0:7 - Execute-only access control"]
  #[inline(always)]
  pub fn xa_s(&self) -> XA_S_R {
    XA_S_R::new((self.bits & 0xff) as u8)
  }
}
