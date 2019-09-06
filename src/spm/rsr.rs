#[doc = "Reader of register RSR"]
pub type R = crate::R<u32, super::RSR>;
#[doc = "Reader of field `REGSEL`"]
pub type REGSEL_R = crate::R<u8, u8>;
#[doc = "MCU Power Mode Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MCUPMSTAT_A {
  #[doc = "1: Last Low Power mode is STOP."]
  MCUPMSTAT_1,
  #[doc = "2: Last Low Power mode is VLPS."]
  MCUPMSTAT_2,
  #[doc = "4: Last Low Power mode is LLS."]
  MCUPMSTAT_4,
  #[doc = "8: Last Low Power mode is VLLS23."]
  MCUPMSTAT_8,
  #[doc = "16: Last Low Power mode is VLLS01."]
  MCUPMSTAT_16,
}
impl From<MCUPMSTAT_A> for u8 {
  #[inline(always)]
  fn from(variant: MCUPMSTAT_A) -> Self {
    match variant {
      MCUPMSTAT_A::MCUPMSTAT_1 => 1,
      MCUPMSTAT_A::MCUPMSTAT_2 => 2,
      MCUPMSTAT_A::MCUPMSTAT_4 => 4,
      MCUPMSTAT_A::MCUPMSTAT_8 => 8,
      MCUPMSTAT_A::MCUPMSTAT_16 => 16,
    }
  }
}
#[doc = "Reader of field `MCUPMSTAT`"]
pub type MCUPMSTAT_R = crate::R<u8, MCUPMSTAT_A>;
impl MCUPMSTAT_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> crate::Variant<u8, MCUPMSTAT_A> {
    use crate::Variant::*;
    match self.bits {
      1 => Val(MCUPMSTAT_A::MCUPMSTAT_1),
      2 => Val(MCUPMSTAT_A::MCUPMSTAT_2),
      4 => Val(MCUPMSTAT_A::MCUPMSTAT_4),
      8 => Val(MCUPMSTAT_A::MCUPMSTAT_8),
      16 => Val(MCUPMSTAT_A::MCUPMSTAT_16),
      i => Res(i),
    }
  }
  #[doc = "Checks if the value of the field is `MCUPMSTAT_1`"]
  #[inline(always)]
  pub fn is_mcupmstat_1(&self) -> bool {
    *self == MCUPMSTAT_A::MCUPMSTAT_1
  }
  #[doc = "Checks if the value of the field is `MCUPMSTAT_2`"]
  #[inline(always)]
  pub fn is_mcupmstat_2(&self) -> bool {
    *self == MCUPMSTAT_A::MCUPMSTAT_2
  }
  #[doc = "Checks if the value of the field is `MCUPMSTAT_4`"]
  #[inline(always)]
  pub fn is_mcupmstat_4(&self) -> bool {
    *self == MCUPMSTAT_A::MCUPMSTAT_4
  }
  #[doc = "Checks if the value of the field is `MCUPMSTAT_8`"]
  #[inline(always)]
  pub fn is_mcupmstat_8(&self) -> bool {
    *self == MCUPMSTAT_A::MCUPMSTAT_8
  }
  #[doc = "Checks if the value of the field is `MCUPMSTAT_16`"]
  #[inline(always)]
  pub fn is_mcupmstat_16(&self) -> bool {
    *self == MCUPMSTAT_A::MCUPMSTAT_16
  }
}
#[doc = "RADIO Power Mode Status\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RFPMSTAT_A {
  #[doc = "1: Current Power mode is VLPS."]
  RFPMSTAT_1,
  #[doc = "2: Current Power mode is LLS."]
  RFPMSTAT_2,
  #[doc = "4: Current Power mode is VLLS."]
  RFPMSTAT_4,
}
impl From<RFPMSTAT_A> for u8 {
  #[inline(always)]
  fn from(variant: RFPMSTAT_A) -> Self {
    match variant {
      RFPMSTAT_A::RFPMSTAT_1 => 1,
      RFPMSTAT_A::RFPMSTAT_2 => 2,
      RFPMSTAT_A::RFPMSTAT_4 => 4,
    }
  }
}
#[doc = "Reader of field `RFPMSTAT`"]
pub type RFPMSTAT_R = crate::R<u8, RFPMSTAT_A>;
impl RFPMSTAT_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> crate::Variant<u8, RFPMSTAT_A> {
    use crate::Variant::*;
    match self.bits {
      1 => Val(RFPMSTAT_A::RFPMSTAT_1),
      2 => Val(RFPMSTAT_A::RFPMSTAT_2),
      4 => Val(RFPMSTAT_A::RFPMSTAT_4),
      i => Res(i),
    }
  }
  #[doc = "Checks if the value of the field is `RFPMSTAT_1`"]
  #[inline(always)]
  pub fn is_rfpmstat_1(&self) -> bool {
    *self == RFPMSTAT_A::RFPMSTAT_1
  }
  #[doc = "Checks if the value of the field is `RFPMSTAT_2`"]
  #[inline(always)]
  pub fn is_rfpmstat_2(&self) -> bool {
    *self == RFPMSTAT_A::RFPMSTAT_2
  }
  #[doc = "Checks if the value of the field is `RFPMSTAT_4`"]
  #[inline(always)]
  pub fn is_rfpmstat_4(&self) -> bool {
    *self == RFPMSTAT_A::RFPMSTAT_4
  }
}
#[doc = "RADIO Run Force Power Mode Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RFRUNFORCE_A {
  #[doc = "0: Radio Run Force Regulator Off"]
  RFRUNFORCE_0,
  #[doc = "1: Radio Run Force Regulator On."]
  RFRUNFORCE_1,
}
impl From<RFRUNFORCE_A> for bool {
  #[inline(always)]
  fn from(variant: RFRUNFORCE_A) -> Self {
    match variant {
      RFRUNFORCE_A::RFRUNFORCE_0 => false,
      RFRUNFORCE_A::RFRUNFORCE_1 => true,
    }
  }
}
#[doc = "Reader of field `RFRUNFORCE`"]
pub type RFRUNFORCE_R = crate::R<bool, RFRUNFORCE_A>;
impl RFRUNFORCE_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> RFRUNFORCE_A {
    match self.bits {
      false => RFRUNFORCE_A::RFRUNFORCE_0,
      true => RFRUNFORCE_A::RFRUNFORCE_1,
    }
  }
  #[doc = "Checks if the value of the field is `RFRUNFORCE_0`"]
  #[inline(always)]
  pub fn is_rfrunforce_0(&self) -> bool {
    *self == RFRUNFORCE_A::RFRUNFORCE_0
  }
  #[doc = "Checks if the value of the field is `RFRUNFORCE_1`"]
  #[inline(always)]
  pub fn is_rfrunforce_1(&self) -> bool {
    *self == RFRUNFORCE_A::RFRUNFORCE_1
  }
}
impl R {
  #[doc = "Bits 0:2 - REGSEL"]
  #[inline(always)]
  pub fn regsel(&self) -> REGSEL_R {
    REGSEL_R::new((self.bits & 0x07) as u8)
  }
  #[doc = "Bits 16:20 - MCU Power Mode Status"]
  #[inline(always)]
  pub fn mcupmstat(&self) -> MCUPMSTAT_R {
    MCUPMSTAT_R::new(((self.bits >> 16) & 0x1f) as u8)
  }
  #[doc = "Bits 24:26 - RADIO Power Mode Status"]
  #[inline(always)]
  pub fn rfpmstat(&self) -> RFPMSTAT_R {
    RFPMSTAT_R::new(((self.bits >> 24) & 0x07) as u8)
  }
  #[doc = "Bit 27 - RADIO Run Force Power Mode Status"]
  #[inline(always)]
  pub fn rfrunforce(&self) -> RFRUNFORCE_R {
    RFRUNFORCE_R::new(((self.bits >> 27) & 0x01) != 0)
  }
}
