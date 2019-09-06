#[doc = "Reader of register FSTDBYCTL"]
pub type R = crate::R<u8, super::FSTDBYCTL>;
#[doc = "Standy Mode Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STDBYDIS_A {
  #[doc = "0: Standby mode enabled for flash blocks selected by STDBYx"]
  STDBYDIS_0,
  #[doc = "1: Standby mode disabled (STDBYx ignored)"]
  STDBYDIS_1,
}
impl From<STDBYDIS_A> for bool {
  #[inline(always)]
  fn from(variant: STDBYDIS_A) -> Self {
    match variant {
      STDBYDIS_A::STDBYDIS_0 => false,
      STDBYDIS_A::STDBYDIS_1 => true,
    }
  }
}
#[doc = "Reader of field `STDBYDIS`"]
pub type STDBYDIS_R = crate::R<bool, STDBYDIS_A>;
impl STDBYDIS_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> STDBYDIS_A {
    match self.bits {
      false => STDBYDIS_A::STDBYDIS_0,
      true => STDBYDIS_A::STDBYDIS_1,
    }
  }
  #[doc = "Checks if the value of the field is `STDBYDIS_0`"]
  #[inline(always)]
  pub fn is_stdbydis_0(&self) -> bool {
    *self == STDBYDIS_A::STDBYDIS_0
  }
  #[doc = "Checks if the value of the field is `STDBYDIS_1`"]
  #[inline(always)]
  pub fn is_stdbydis_1(&self) -> bool {
    *self == STDBYDIS_A::STDBYDIS_1
  }
}
impl R {
  #[doc = "Bit 0 - Standy Mode Disable"]
  #[inline(always)]
  pub fn stdbydis(&self) -> STDBYDIS_R {
    STDBYDIS_R::new((self.bits & 0x01) != 0)
  }
}
