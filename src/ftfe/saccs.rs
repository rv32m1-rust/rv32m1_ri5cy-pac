#[doc = "Reader of register SACCS%s"]
pub type R = crate::R<u8, super::SACCS>;
#[doc = "Reader of field `SA_S`"]
pub type SA_S_R = crate::R<u8, u8>;
impl R {
  #[doc = "Bits 0:7 - Secondary Supervisor-only access control"]
  #[inline(always)]
  pub fn sa_s(&self) -> SA_S_R {
    SA_S_R::new((self.bits & 0xff) as u8)
  }
}
