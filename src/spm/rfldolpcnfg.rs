#[doc = "Reader of register RFLDOLPCNFG"]
pub type R = crate::R<u32, super::RFLDOLPCNFG>;
#[doc = "Writer for register RFLDOLPCNFG"]
pub type W = crate::W<u32, super::RFLDOLPCNFG>;
#[doc = "Register RFLDOLPCNFG `reset()`'s with value 0"]
impl crate::ResetValue for super::RFLDOLPCNFG {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "LPSEL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPSEL_A {
  #[doc = "0: RF LDO regulator enters low power state in VLP/Stop modes."]
  LPSEL_0,
  #[doc = "1: RF LDO regulator remains in high power state in VLP/Stop modes."]
  LPSEL_1,
}
impl From<LPSEL_A> for bool {
  #[inline(always)]
  fn from(variant: LPSEL_A) -> Self {
    match variant {
      LPSEL_A::LPSEL_0 => false,
      LPSEL_A::LPSEL_1 => true,
    }
  }
}
#[doc = "Reader of field `LPSEL`"]
pub type LPSEL_R = crate::R<bool, LPSEL_A>;
impl LPSEL_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> LPSEL_A {
    match self.bits {
      false => LPSEL_A::LPSEL_0,
      true => LPSEL_A::LPSEL_1,
    }
  }
  #[doc = "Checks if the value of the field is `LPSEL_0`"]
  #[inline(always)]
  pub fn is_lpsel_0(&self) -> bool {
    *self == LPSEL_A::LPSEL_0
  }
  #[doc = "Checks if the value of the field is `LPSEL_1`"]
  #[inline(always)]
  pub fn is_lpsel_1(&self) -> bool {
    *self == LPSEL_A::LPSEL_1
  }
}
#[doc = "Write proxy for field `LPSEL`"]
pub struct LPSEL_W<'a> {
  w: &'a mut W,
}
impl<'a> LPSEL_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: LPSEL_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "RF LDO regulator enters low power state in VLP/Stop modes."]
  #[inline(always)]
  pub fn lpsel_0(self) -> &'a mut W {
    self.variant(LPSEL_A::LPSEL_0)
  }
  #[doc = "RF LDO regulator remains in high power state in VLP/Stop modes."]
  #[inline(always)]
  pub fn lpsel_1(self) -> &'a mut W {
    self.variant(LPSEL_A::LPSEL_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
    self.w
  }
}
impl R {
  #[doc = "Bit 1 - LPSEL"]
  #[inline(always)]
  pub fn lpsel(&self) -> LPSEL_R {
    LPSEL_R::new(((self.bits >> 1) & 0x01) != 0)
  }
}
impl W {
  #[doc = "Bit 1 - LPSEL"]
  #[inline(always)]
  pub fn lpsel(&mut self) -> LPSEL_W {
    LPSEL_W { w: self }
  }
}
