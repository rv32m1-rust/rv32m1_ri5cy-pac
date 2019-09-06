#[doc = "Reader of register PARAM"]
pub type R = crate::R<u32, super::PARAM>;
#[doc = "Reader of field `PWRD_INDPT`"]
pub type PWRD_INDPT_R = crate::R<bool, bool>;
impl R {
  #[doc = "Bit 0 - Power Domains Independent"]
  #[inline(always)]
  pub fn pwrd_indpt(&self) -> PWRD_INDPT_R {
    PWRD_INDPT_R::new((self.bits & 0x01) != 0)
  }
}
