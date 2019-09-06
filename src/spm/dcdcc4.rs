#[doc = "Reader of register DCDCC4"]
pub type R = crate::R<u32, super::DCDCC4>;
#[doc = "Writer for register DCDCC4"]
pub type W = crate::W<u32, super::DCDCC4>;
#[doc = "Register DCDCC4 `reset()`'s with value 0"]
impl crate::ResetValue for super::DCDCC4 {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "Reader of field `INTEGRATOR_VALUE`"]
pub type INTEGRATOR_VALUE_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `INTEGRATOR_VALUE`"]
pub struct INTEGRATOR_VALUE_W<'a> {
  w: &'a mut W,
}
impl<'a> INTEGRATOR_VALUE_W<'a> {
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u32) -> &'a mut W {
    self.w.bits = (self.w.bits & !0x0007_ffff) | ((value as u32) & 0x0007_ffff);
    self.w
  }
}
#[doc = "INTEGRATOR VALUE SELECT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTEGRATOR_VALUE_SELECT_A {
  #[doc = "0: Select the saved value in hardware"]
  INTEGRATOR_VALUE_SELECT_0,
  #[doc = "1: Select the integrator value in this register"]
  INTEGRATOR_VALUE_SELECT_1,
}
impl From<INTEGRATOR_VALUE_SELECT_A> for bool {
  #[inline(always)]
  fn from(variant: INTEGRATOR_VALUE_SELECT_A) -> Self {
    match variant {
      INTEGRATOR_VALUE_SELECT_A::INTEGRATOR_VALUE_SELECT_0 => false,
      INTEGRATOR_VALUE_SELECT_A::INTEGRATOR_VALUE_SELECT_1 => true,
    }
  }
}
#[doc = "Reader of field `INTEGRATOR_VALUE_SELECT`"]
pub type INTEGRATOR_VALUE_SELECT_R = crate::R<bool, INTEGRATOR_VALUE_SELECT_A>;
impl INTEGRATOR_VALUE_SELECT_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> INTEGRATOR_VALUE_SELECT_A {
    match self.bits {
      false => INTEGRATOR_VALUE_SELECT_A::INTEGRATOR_VALUE_SELECT_0,
      true => INTEGRATOR_VALUE_SELECT_A::INTEGRATOR_VALUE_SELECT_1,
    }
  }
  #[doc = "Checks if the value of the field is `INTEGRATOR_VALUE_SELECT_0`"]
  #[inline(always)]
  pub fn is_integrator_value_select_0(&self) -> bool {
    *self == INTEGRATOR_VALUE_SELECT_A::INTEGRATOR_VALUE_SELECT_0
  }
  #[doc = "Checks if the value of the field is `INTEGRATOR_VALUE_SELECT_1`"]
  #[inline(always)]
  pub fn is_integrator_value_select_1(&self) -> bool {
    *self == INTEGRATOR_VALUE_SELECT_A::INTEGRATOR_VALUE_SELECT_1
  }
}
#[doc = "Write proxy for field `INTEGRATOR_VALUE_SELECT`"]
pub struct INTEGRATOR_VALUE_SELECT_W<'a> {
  w: &'a mut W,
}
impl<'a> INTEGRATOR_VALUE_SELECT_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: INTEGRATOR_VALUE_SELECT_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Select the saved value in hardware"]
  #[inline(always)]
  pub fn integrator_value_select_0(self) -> &'a mut W {
    self.variant(INTEGRATOR_VALUE_SELECT_A::INTEGRATOR_VALUE_SELECT_0)
  }
  #[doc = "Select the integrator value in this register"]
  #[inline(always)]
  pub fn integrator_value_select_1(self) -> &'a mut W {
    self.variant(INTEGRATOR_VALUE_SELECT_A::INTEGRATOR_VALUE_SELECT_1)
  }
  #[doc = r"Sets the field bit"]
  #[inline(always)]
  pub fn set_bit(self) -> &'a mut W {
    self.bit(true)
  }
  #[doc = r"Clears the field bit"]
  #[inline(always)]
  pub fn clear_bit(self) -> &'a mut W {
    self.bit(false)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub fn bit(self, value: bool) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
    self.w
  }
}
#[doc = "Reader of field `PULSE_RUN_SPEEDUP`"]
pub type PULSE_RUN_SPEEDUP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PULSE_RUN_SPEEDUP`"]
pub struct PULSE_RUN_SPEEDUP_W<'a> {
  w: &'a mut W,
}
impl<'a> PULSE_RUN_SPEEDUP_W<'a> {
  #[doc = r"Sets the field bit"]
  #[inline(always)]
  pub fn set_bit(self) -> &'a mut W {
    self.bit(true)
  }
  #[doc = r"Clears the field bit"]
  #[inline(always)]
  pub fn clear_bit(self) -> &'a mut W {
    self.bit(false)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub fn bit(self, value: bool) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
    self.w
  }
}
impl R {
  #[doc = "Bits 0:18 - INTEGRATOR VALUE"]
  #[inline(always)]
  pub fn integrator_value(&self) -> INTEGRATOR_VALUE_R {
    INTEGRATOR_VALUE_R::new((self.bits & 0x0007_ffff) as u32)
  }
  #[doc = "Bit 19 - INTEGRATOR VALUE SELECT"]
  #[inline(always)]
  pub fn integrator_value_select(&self) -> INTEGRATOR_VALUE_SELECT_R {
    INTEGRATOR_VALUE_SELECT_R::new(((self.bits >> 19) & 0x01) != 0)
  }
  #[doc = "Bit 20 - PULSE RUN SPEEDUP"]
  #[inline(always)]
  pub fn pulse_run_speedup(&self) -> PULSE_RUN_SPEEDUP_R {
    PULSE_RUN_SPEEDUP_R::new(((self.bits >> 20) & 0x01) != 0)
  }
}
impl W {
  #[doc = "Bits 0:18 - INTEGRATOR VALUE"]
  #[inline(always)]
  pub fn integrator_value(&mut self) -> INTEGRATOR_VALUE_W {
    INTEGRATOR_VALUE_W { w: self }
  }
  #[doc = "Bit 19 - INTEGRATOR VALUE SELECT"]
  #[inline(always)]
  pub fn integrator_value_select(&mut self) -> INTEGRATOR_VALUE_SELECT_W {
    INTEGRATOR_VALUE_SELECT_W { w: self }
  }
  #[doc = "Bit 20 - PULSE RUN SPEEDUP"]
  #[inline(always)]
  pub fn pulse_run_speedup(&mut self) -> PULSE_RUN_SPEEDUP_W {
    PULSE_RUN_SPEEDUP_W { w: self }
  }
}
