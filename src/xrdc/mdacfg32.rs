#[doc = "Reader of register MDACFG32"]
pub type R = crate::R<u8, super::MDACFG32>;
#[doc = "Reader of field `NMDAR`"]
pub type NMDAR_R = crate::R<u8, u8>;
#[doc = "Non-CPU Master\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NCM_A {
  #[doc = "0: Bus master is a processor."]
  NCM_0,
  #[doc = "1: Bus master is a non-processor."]
  NCM_1,
}
impl From<NCM_A> for bool {
  #[inline(always)]
  fn from(variant: NCM_A) -> Self {
    match variant {
      NCM_A::NCM_0 => false,
      NCM_A::NCM_1 => true,
    }
  }
}
#[doc = "Reader of field `NCM`"]
pub type NCM_R = crate::R<bool, NCM_A>;
impl NCM_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> NCM_A {
    match self.bits {
      false => NCM_A::NCM_0,
      true => NCM_A::NCM_1,
    }
  }
  #[doc = "Checks if the value of the field is `NCM_0`"]
  #[inline(always)]
  pub fn is_ncm_0(&self) -> bool {
    *self == NCM_A::NCM_0
  }
  #[doc = "Checks if the value of the field is `NCM_1`"]
  #[inline(always)]
  pub fn is_ncm_1(&self) -> bool {
    *self == NCM_A::NCM_1
  }
}
impl R {
  #[doc = "Bits 0:3 - Number of master domain assignment registers for bus master m"]
  #[inline(always)]
  pub fn nmdar(&self) -> NMDAR_R {
    NMDAR_R::new((self.bits & 0x0f) as u8)
  }
  #[doc = "Bit 7 - Non-CPU Master"]
  #[inline(always)]
  pub fn ncm(&self) -> NCM_R {
    NCM_R::new(((self.bits >> 7) & 0x01) != 0)
  }
}
