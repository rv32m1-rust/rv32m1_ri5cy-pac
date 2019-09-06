#[doc = "Reader of register GSR"]
pub type R = crate::R<u32, super::GSR>;
#[doc = "CAU3 Done Interrupt occurred\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CDI_A {
  #[doc = "0: CAU3 Done Interrupt did not occur"]
  CDI_0,
  #[doc = "1: CAU3 Done Interrupt occurred"]
  CDI_1,
}
impl From<CDI_A> for bool {
  #[inline(always)]
  fn from(variant: CDI_A) -> Self {
    match variant {
      CDI_A::CDI_0 => false,
      CDI_A::CDI_1 => true,
    }
  }
}
#[doc = "Reader of field `CDI`"]
pub type CDI_R = crate::R<bool, CDI_A>;
impl CDI_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> CDI_A {
    match self.bits {
      false => CDI_A::CDI_0,
      true => CDI_A::CDI_1,
    }
  }
  #[doc = "Checks if the value of the field is `CDI_0`"]
  #[inline(always)]
  pub fn is_cdi_0(&self) -> bool {
    *self == CDI_A::CDI_0
  }
  #[doc = "Checks if the value of the field is `CDI_1`"]
  #[inline(always)]
  pub fn is_cdi_1(&self) -> bool {
    *self == CDI_A::CDI_1
  }
}
#[doc = "CAU3 Error Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CEI_A {
  #[doc = "0: CAU3 Error Interrupt did not occur"]
  CEI_0,
  #[doc = "1: CAU3 Error Interrupt occurred"]
  CEI_1,
}
impl From<CEI_A> for bool {
  #[inline(always)]
  fn from(variant: CEI_A) -> Self {
    match variant {
      CEI_A::CEI_0 => false,
      CEI_A::CEI_1 => true,
    }
  }
}
#[doc = "Reader of field `CEI`"]
pub type CEI_R = crate::R<bool, CEI_A>;
impl CEI_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> CEI_A {
    match self.bits {
      false => CEI_A::CEI_0,
      true => CEI_A::CEI_1,
    }
  }
  #[doc = "Checks if the value of the field is `CEI_0`"]
  #[inline(always)]
  pub fn is_cei_0(&self) -> bool {
    *self == CEI_A::CEI_0
  }
  #[doc = "Checks if the value of the field is `CEI_1`"]
  #[inline(always)]
  pub fn is_cei_1(&self) -> bool {
    *self == CEI_A::CEI_1
  }
}
#[doc = "PKHA Done or Error Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEI_A {
  #[doc = "0: PKHA interrupt did not occur"]
  PEI_0,
  #[doc = "1: PKHA interrupt had occurred"]
  PEI_1,
}
impl From<PEI_A> for bool {
  #[inline(always)]
  fn from(variant: PEI_A) -> Self {
    match variant {
      PEI_A::PEI_0 => false,
      PEI_A::PEI_1 => true,
    }
  }
}
#[doc = "Reader of field `PEI`"]
pub type PEI_R = crate::R<bool, PEI_A>;
impl PEI_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> PEI_A {
    match self.bits {
      false => PEI_A::PEI_0,
      true => PEI_A::PEI_1,
    }
  }
  #[doc = "Checks if the value of the field is `PEI_0`"]
  #[inline(always)]
  pub fn is_pei_0(&self) -> bool {
    *self == PEI_A::PEI_0
  }
  #[doc = "Checks if the value of the field is `PEI_1`"]
  #[inline(always)]
  pub fn is_pei_1(&self) -> bool {
    *self == PEI_A::PEI_1
  }
}
#[doc = "PKHA Busy\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PBSY_A {
  #[doc = "0: PKHA not busy"]
  PBSY_0,
  #[doc = "1: PKHA busy"]
  PBSY_1,
}
impl From<PBSY_A> for bool {
  #[inline(always)]
  fn from(variant: PBSY_A) -> Self {
    match variant {
      PBSY_A::PBSY_0 => false,
      PBSY_A::PBSY_1 => true,
    }
  }
}
#[doc = "Reader of field `PBSY`"]
pub type PBSY_R = crate::R<bool, PBSY_A>;
impl PBSY_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> PBSY_A {
    match self.bits {
      false => PBSY_A::PBSY_0,
      true => PBSY_A::PBSY_1,
    }
  }
  #[doc = "Checks if the value of the field is `PBSY_0`"]
  #[inline(always)]
  pub fn is_pbsy_0(&self) -> bool {
    *self == PBSY_A::PBSY_0
  }
  #[doc = "Checks if the value of the field is `PBSY_1`"]
  #[inline(always)]
  pub fn is_pbsy_1(&self) -> bool {
    *self == PBSY_A::PBSY_1
  }
}
impl R {
  #[doc = "Bit 10 - CAU3 Done Interrupt occurred"]
  #[inline(always)]
  pub fn cdi(&self) -> CDI_R {
    CDI_R::new(((self.bits >> 10) & 0x01) != 0)
  }
  #[doc = "Bit 14 - CAU3 Error Interrupt"]
  #[inline(always)]
  pub fn cei(&self) -> CEI_R {
    CEI_R::new(((self.bits >> 14) & 0x01) != 0)
  }
  #[doc = "Bit 15 - PKHA Done or Error Interrupt"]
  #[inline(always)]
  pub fn pei(&self) -> PEI_R {
    PEI_R::new(((self.bits >> 15) & 0x01) != 0)
  }
  #[doc = "Bit 31 - PKHA Busy"]
  #[inline(always)]
  pub fn pbsy(&self) -> PBSY_R {
    PBSY_R::new(((self.bits >> 31) & 0x01) != 0)
  }
}
