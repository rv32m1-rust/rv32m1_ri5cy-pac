#[doc = "Reader of register CLK_RECOVER_CTRL"]
pub type R = crate::R<u8, super::CLK_RECOVER_CTRL>;
#[doc = "Writer for register CLK_RECOVER_CTRL"]
pub type W = crate::W<u8, super::CLK_RECOVER_CTRL>;
#[doc = "Register CLK_RECOVER_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::CLK_RECOVER_CTRL {
  type Type = u8;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "Restart from IFR trim value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESTART_IFRTRIM_EN_A {
  #[doc = "0: Trim fine adjustment always works based on the previous updated trim fine value (default)."]
  RESTART_IFRTRIM_EN_0,
  #[doc = "1: Trim fine restarts from the IFR trim value, whenever bus_reset/bus_resume is detected or module enable is desasserted."]
  RESTART_IFRTRIM_EN_1,
}
impl From<RESTART_IFRTRIM_EN_A> for bool {
  #[inline(always)]
  fn from(variant: RESTART_IFRTRIM_EN_A) -> Self {
    match variant {
      RESTART_IFRTRIM_EN_A::RESTART_IFRTRIM_EN_0 => false,
      RESTART_IFRTRIM_EN_A::RESTART_IFRTRIM_EN_1 => true,
    }
  }
}
#[doc = "Reader of field `RESTART_IFRTRIM_EN`"]
pub type RESTART_IFRTRIM_EN_R = crate::R<bool, RESTART_IFRTRIM_EN_A>;
impl RESTART_IFRTRIM_EN_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> RESTART_IFRTRIM_EN_A {
    match self.bits {
      false => RESTART_IFRTRIM_EN_A::RESTART_IFRTRIM_EN_0,
      true => RESTART_IFRTRIM_EN_A::RESTART_IFRTRIM_EN_1,
    }
  }
  #[doc = "Checks if the value of the field is `RESTART_IFRTRIM_EN_0`"]
  #[inline(always)]
  pub fn is_restart_ifrtrim_en_0(&self) -> bool {
    *self == RESTART_IFRTRIM_EN_A::RESTART_IFRTRIM_EN_0
  }
  #[doc = "Checks if the value of the field is `RESTART_IFRTRIM_EN_1`"]
  #[inline(always)]
  pub fn is_restart_ifrtrim_en_1(&self) -> bool {
    *self == RESTART_IFRTRIM_EN_A::RESTART_IFRTRIM_EN_1
  }
}
#[doc = "Write proxy for field `RESTART_IFRTRIM_EN`"]
pub struct RESTART_IFRTRIM_EN_W<'a> {
  w: &'a mut W,
}
impl<'a> RESTART_IFRTRIM_EN_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: RESTART_IFRTRIM_EN_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Trim fine adjustment always works based on the previous updated trim fine value (default)."]
  #[inline(always)]
  pub fn restart_ifrtrim_en_0(self) -> &'a mut W {
    self.variant(RESTART_IFRTRIM_EN_A::RESTART_IFRTRIM_EN_0)
  }
  #[doc = "Trim fine restarts from the IFR trim value, whenever bus_reset/bus_resume is detected or module enable is desasserted."]
  #[inline(always)]
  pub fn restart_ifrtrim_en_1(self) -> &'a mut W {
    self.variant(RESTART_IFRTRIM_EN_A::RESTART_IFRTRIM_EN_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u8) & 0x01) << 5);
    self.w
  }
}
#[doc = "Reset/resume to rough phase enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESET_RESUME_ROUGH_EN_A {
  #[doc = "0: Always works in tracking phase after the first time rough phase, to track transition (default)."]
  RESET_RESUME_ROUGH_EN_0,
  #[doc = "1: Go back to rough stage whenever a bus reset or bus resume occurs."]
  RESET_RESUME_ROUGH_EN_1,
}
impl From<RESET_RESUME_ROUGH_EN_A> for bool {
  #[inline(always)]
  fn from(variant: RESET_RESUME_ROUGH_EN_A) -> Self {
    match variant {
      RESET_RESUME_ROUGH_EN_A::RESET_RESUME_ROUGH_EN_0 => false,
      RESET_RESUME_ROUGH_EN_A::RESET_RESUME_ROUGH_EN_1 => true,
    }
  }
}
#[doc = "Reader of field `RESET_RESUME_ROUGH_EN`"]
pub type RESET_RESUME_ROUGH_EN_R = crate::R<bool, RESET_RESUME_ROUGH_EN_A>;
impl RESET_RESUME_ROUGH_EN_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> RESET_RESUME_ROUGH_EN_A {
    match self.bits {
      false => RESET_RESUME_ROUGH_EN_A::RESET_RESUME_ROUGH_EN_0,
      true => RESET_RESUME_ROUGH_EN_A::RESET_RESUME_ROUGH_EN_1,
    }
  }
  #[doc = "Checks if the value of the field is `RESET_RESUME_ROUGH_EN_0`"]
  #[inline(always)]
  pub fn is_reset_resume_rough_en_0(&self) -> bool {
    *self == RESET_RESUME_ROUGH_EN_A::RESET_RESUME_ROUGH_EN_0
  }
  #[doc = "Checks if the value of the field is `RESET_RESUME_ROUGH_EN_1`"]
  #[inline(always)]
  pub fn is_reset_resume_rough_en_1(&self) -> bool {
    *self == RESET_RESUME_ROUGH_EN_A::RESET_RESUME_ROUGH_EN_1
  }
}
#[doc = "Write proxy for field `RESET_RESUME_ROUGH_EN`"]
pub struct RESET_RESUME_ROUGH_EN_W<'a> {
  w: &'a mut W,
}
impl<'a> RESET_RESUME_ROUGH_EN_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: RESET_RESUME_ROUGH_EN_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Always works in tracking phase after the first time rough phase, to track transition (default)."]
  #[inline(always)]
  pub fn reset_resume_rough_en_0(self) -> &'a mut W {
    self.variant(RESET_RESUME_ROUGH_EN_A::RESET_RESUME_ROUGH_EN_0)
  }
  #[doc = "Go back to rough stage whenever a bus reset or bus resume occurs."]
  #[inline(always)]
  pub fn reset_resume_rough_en_1(self) -> &'a mut W {
    self.variant(RESET_RESUME_ROUGH_EN_A::RESET_RESUME_ROUGH_EN_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u8) & 0x01) << 6);
    self.w
  }
}
#[doc = "Crystal-less USB enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLOCK_RECOVER_EN_A {
  #[doc = "0: Disable clock recovery block (default)"]
  CLOCK_RECOVER_EN_0,
  #[doc = "1: Enable clock recovery block"]
  CLOCK_RECOVER_EN_1,
}
impl From<CLOCK_RECOVER_EN_A> for bool {
  #[inline(always)]
  fn from(variant: CLOCK_RECOVER_EN_A) -> Self {
    match variant {
      CLOCK_RECOVER_EN_A::CLOCK_RECOVER_EN_0 => false,
      CLOCK_RECOVER_EN_A::CLOCK_RECOVER_EN_1 => true,
    }
  }
}
#[doc = "Reader of field `CLOCK_RECOVER_EN`"]
pub type CLOCK_RECOVER_EN_R = crate::R<bool, CLOCK_RECOVER_EN_A>;
impl CLOCK_RECOVER_EN_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> CLOCK_RECOVER_EN_A {
    match self.bits {
      false => CLOCK_RECOVER_EN_A::CLOCK_RECOVER_EN_0,
      true => CLOCK_RECOVER_EN_A::CLOCK_RECOVER_EN_1,
    }
  }
  #[doc = "Checks if the value of the field is `CLOCK_RECOVER_EN_0`"]
  #[inline(always)]
  pub fn is_clock_recover_en_0(&self) -> bool {
    *self == CLOCK_RECOVER_EN_A::CLOCK_RECOVER_EN_0
  }
  #[doc = "Checks if the value of the field is `CLOCK_RECOVER_EN_1`"]
  #[inline(always)]
  pub fn is_clock_recover_en_1(&self) -> bool {
    *self == CLOCK_RECOVER_EN_A::CLOCK_RECOVER_EN_1
  }
}
#[doc = "Write proxy for field `CLOCK_RECOVER_EN`"]
pub struct CLOCK_RECOVER_EN_W<'a> {
  w: &'a mut W,
}
impl<'a> CLOCK_RECOVER_EN_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: CLOCK_RECOVER_EN_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Disable clock recovery block (default)"]
  #[inline(always)]
  pub fn clock_recover_en_0(self) -> &'a mut W {
    self.variant(CLOCK_RECOVER_EN_A::CLOCK_RECOVER_EN_0)
  }
  #[doc = "Enable clock recovery block"]
  #[inline(always)]
  pub fn clock_recover_en_1(self) -> &'a mut W {
    self.variant(CLOCK_RECOVER_EN_A::CLOCK_RECOVER_EN_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u8) & 0x01) << 7);
    self.w
  }
}
impl R {
  #[doc = "Bit 5 - Restart from IFR trim value"]
  #[inline(always)]
  pub fn restart_ifrtrim_en(&self) -> RESTART_IFRTRIM_EN_R {
    RESTART_IFRTRIM_EN_R::new(((self.bits >> 5) & 0x01) != 0)
  }
  #[doc = "Bit 6 - Reset/resume to rough phase enable"]
  #[inline(always)]
  pub fn reset_resume_rough_en(&self) -> RESET_RESUME_ROUGH_EN_R {
    RESET_RESUME_ROUGH_EN_R::new(((self.bits >> 6) & 0x01) != 0)
  }
  #[doc = "Bit 7 - Crystal-less USB enable"]
  #[inline(always)]
  pub fn clock_recover_en(&self) -> CLOCK_RECOVER_EN_R {
    CLOCK_RECOVER_EN_R::new(((self.bits >> 7) & 0x01) != 0)
  }
}
impl W {
  #[doc = "Bit 5 - Restart from IFR trim value"]
  #[inline(always)]
  pub fn restart_ifrtrim_en(&mut self) -> RESTART_IFRTRIM_EN_W {
    RESTART_IFRTRIM_EN_W { w: self }
  }
  #[doc = "Bit 6 - Reset/resume to rough phase enable"]
  #[inline(always)]
  pub fn reset_resume_rough_en(&mut self) -> RESET_RESUME_ROUGH_EN_W {
    RESET_RESUME_ROUGH_EN_W { w: self }
  }
  #[doc = "Bit 7 - Crystal-less USB enable"]
  #[inline(always)]
  pub fn clock_recover_en(&mut self) -> CLOCK_RECOVER_EN_W {
    CLOCK_RECOVER_EN_W { w: self }
  }
}
