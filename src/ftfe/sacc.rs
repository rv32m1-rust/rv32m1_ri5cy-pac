#[doc = "Reader of register SACC%s"]
pub type R = crate::R<u8, super::SACC>;
#[doc = "Reader of field `SA`"]
pub type SA_R = crate::R<u8, u8>;
impl R {
  #[doc = "Bits 0:7 - Primary Supervisor-only access control"]
  #[inline(always)]
  pub fn sa(&self) -> SA_R {
    SA_R::new((self.bits & 0xff) as u8)
  }
}
