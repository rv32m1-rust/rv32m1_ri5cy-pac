#[doc = "Reader of register PMSTAT"]
pub type R = crate::R<u32, super::PMSTAT>;
#[doc = "Writer for register PMSTAT"]
pub type W = crate::W<u32, super::PMSTAT>;
#[doc = "Register PMSTAT `reset()`'s with value 0"]
impl crate::ResetValue for super::PMSTAT {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "Power Mode Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PMSTAT_A {
  #[doc = "1: Current power mode is RUN."]
  PMSTAT_1,
  #[doc = "2: Current power mode is any STOP mode."]
  PMSTAT_2,
  #[doc = "4: Current power mode is VLPR."]
  PMSTAT_4,
  #[doc = "128: Current power mode is HSRUN"]
  PMSTAT_128,
}
impl From<PMSTAT_A> for u8 {
  #[inline(always)]
  fn from(variant: PMSTAT_A) -> Self {
    match variant {
      PMSTAT_A::PMSTAT_1 => 1,
      PMSTAT_A::PMSTAT_2 => 2,
      PMSTAT_A::PMSTAT_4 => 4,
      PMSTAT_A::PMSTAT_128 => 128,
    }
  }
}
#[doc = "Reader of field `PMSTAT`"]
pub type PMSTAT_R = crate::R<u8, PMSTAT_A>;
impl PMSTAT_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> crate::Variant<u8, PMSTAT_A> {
    use crate::Variant::*;
    match self.bits {
      1 => Val(PMSTAT_A::PMSTAT_1),
      2 => Val(PMSTAT_A::PMSTAT_2),
      4 => Val(PMSTAT_A::PMSTAT_4),
      128 => Val(PMSTAT_A::PMSTAT_128),
      i => Res(i),
    }
  }
  #[doc = "Checks if the value of the field is `PMSTAT_1`"]
  #[inline(always)]
  pub fn is_pmstat_1(&self) -> bool {
    *self == PMSTAT_A::PMSTAT_1
  }
  #[doc = "Checks if the value of the field is `PMSTAT_2`"]
  #[inline(always)]
  pub fn is_pmstat_2(&self) -> bool {
    *self == PMSTAT_A::PMSTAT_2
  }
  #[doc = "Checks if the value of the field is `PMSTAT_4`"]
  #[inline(always)]
  pub fn is_pmstat_4(&self) -> bool {
    *self == PMSTAT_A::PMSTAT_4
  }
  #[doc = "Checks if the value of the field is `PMSTAT_128`"]
  #[inline(always)]
  pub fn is_pmstat_128(&self) -> bool {
    *self == PMSTAT_A::PMSTAT_128
  }
}
#[doc = "Reader of field `STOPSTAT`"]
pub type STOPSTAT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `STOPSTAT`"]
pub struct STOPSTAT_W<'a> {
  w: &'a mut W,
}
impl<'a> STOPSTAT_W<'a> {
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
    self.w
  }
}
impl R {
  #[doc = "Bits 0:7 - Power Mode Status"]
  #[inline(always)]
  pub fn pmstat(&self) -> PMSTAT_R {
    PMSTAT_R::new((self.bits & 0xff) as u8)
  }
  #[doc = "Bits 24:31 - Stop Entry Status"]
  #[inline(always)]
  pub fn stopstat(&self) -> STOPSTAT_R {
    STOPSTAT_R::new(((self.bits >> 24) & 0xff) as u8)
  }
}
impl W {
  #[doc = "Bits 24:31 - Stop Entry Status"]
  #[inline(always)]
  pub fn stopstat(&mut self) -> STOPSTAT_W {
    STOPSTAT_W { w: self }
  }
}
