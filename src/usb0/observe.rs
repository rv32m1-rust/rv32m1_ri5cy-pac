#[doc = "Reader of register OBSERVE"]
pub type R = crate::R<u8, super::OBSERVE>;
#[doc = "DMPD\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMPD_A {
  #[doc = "0: D- pulldown is disabled."]
  DMPD_0,
  #[doc = "1: D- pulldown is enabled."]
  DMPD_1,
}
impl From<DMPD_A> for bool {
  #[inline(always)]
  fn from(variant: DMPD_A) -> Self {
    match variant {
      DMPD_A::DMPD_0 => false,
      DMPD_A::DMPD_1 => true,
    }
  }
}
#[doc = "Reader of field `DMPD`"]
pub type DMPD_R = crate::R<bool, DMPD_A>;
impl DMPD_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> DMPD_A {
    match self.bits {
      false => DMPD_A::DMPD_0,
      true => DMPD_A::DMPD_1,
    }
  }
  #[doc = "Checks if the value of the field is `DMPD_0`"]
  #[inline(always)]
  pub fn is_dmpd_0(&self) -> bool {
    *self == DMPD_A::DMPD_0
  }
  #[doc = "Checks if the value of the field is `DMPD_1`"]
  #[inline(always)]
  pub fn is_dmpd_1(&self) -> bool {
    *self == DMPD_A::DMPD_1
  }
}
#[doc = "DPPD\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DPPD_A {
  #[doc = "0: D+ pulldown is disabled."]
  DPPD_0,
  #[doc = "1: D+ pulldown is enabled."]
  DPPD_1,
}
impl From<DPPD_A> for bool {
  #[inline(always)]
  fn from(variant: DPPD_A) -> Self {
    match variant {
      DPPD_A::DPPD_0 => false,
      DPPD_A::DPPD_1 => true,
    }
  }
}
#[doc = "Reader of field `DPPD`"]
pub type DPPD_R = crate::R<bool, DPPD_A>;
impl DPPD_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> DPPD_A {
    match self.bits {
      false => DPPD_A::DPPD_0,
      true => DPPD_A::DPPD_1,
    }
  }
  #[doc = "Checks if the value of the field is `DPPD_0`"]
  #[inline(always)]
  pub fn is_dppd_0(&self) -> bool {
    *self == DPPD_A::DPPD_0
  }
  #[doc = "Checks if the value of the field is `DPPD_1`"]
  #[inline(always)]
  pub fn is_dppd_1(&self) -> bool {
    *self == DPPD_A::DPPD_1
  }
}
#[doc = "DPPU\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DPPU_A {
  #[doc = "0: D+ pullup disabled."]
  DPPU_0,
  #[doc = "1: D+ pullup enabled."]
  DPPU_1,
}
impl From<DPPU_A> for bool {
  #[inline(always)]
  fn from(variant: DPPU_A) -> Self {
    match variant {
      DPPU_A::DPPU_0 => false,
      DPPU_A::DPPU_1 => true,
    }
  }
}
#[doc = "Reader of field `DPPU`"]
pub type DPPU_R = crate::R<bool, DPPU_A>;
impl DPPU_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> DPPU_A {
    match self.bits {
      false => DPPU_A::DPPU_0,
      true => DPPU_A::DPPU_1,
    }
  }
  #[doc = "Checks if the value of the field is `DPPU_0`"]
  #[inline(always)]
  pub fn is_dppu_0(&self) -> bool {
    *self == DPPU_A::DPPU_0
  }
  #[doc = "Checks if the value of the field is `DPPU_1`"]
  #[inline(always)]
  pub fn is_dppu_1(&self) -> bool {
    *self == DPPU_A::DPPU_1
  }
}
impl R {
  #[doc = "Bit 4 - DMPD"]
  #[inline(always)]
  pub fn dmpd(&self) -> DMPD_R {
    DMPD_R::new(((self.bits >> 4) & 0x01) != 0)
  }
  #[doc = "Bit 6 - DPPD"]
  #[inline(always)]
  pub fn dppd(&self) -> DPPD_R {
    DPPD_R::new(((self.bits >> 6) & 0x01) != 0)
  }
  #[doc = "Bit 7 - DPPU"]
  #[inline(always)]
  pub fn dppu(&self) -> DPPU_R {
    DPPU_R::new(((self.bits >> 7) & 0x01) != 0)
  }
}
