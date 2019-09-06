#[doc = "Reader of register OTGCTL"]
pub type R = crate::R<u8, super::OTGCTL>;
#[doc = "Writer for register OTGCTL"]
pub type W = crate::W<u8, super::OTGCTL>;
#[doc = "Register OTGCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::OTGCTL {
  type Type = u8;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "D+ Data Line pullup resistor enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DPHIGH_A {
  #[doc = "0: D+ pullup resistor is not enabled"]
  DPHIGH_0,
  #[doc = "1: D+ pullup resistor is enabled"]
  DPHIGH_1,
}
impl From<DPHIGH_A> for bool {
  #[inline(always)]
  fn from(variant: DPHIGH_A) -> Self {
    match variant {
      DPHIGH_A::DPHIGH_0 => false,
      DPHIGH_A::DPHIGH_1 => true,
    }
  }
}
#[doc = "Reader of field `DPHIGH`"]
pub type DPHIGH_R = crate::R<bool, DPHIGH_A>;
impl DPHIGH_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> DPHIGH_A {
    match self.bits {
      false => DPHIGH_A::DPHIGH_0,
      true => DPHIGH_A::DPHIGH_1,
    }
  }
  #[doc = "Checks if the value of the field is `DPHIGH_0`"]
  #[inline(always)]
  pub fn is_dphigh_0(&self) -> bool {
    *self == DPHIGH_A::DPHIGH_0
  }
  #[doc = "Checks if the value of the field is `DPHIGH_1`"]
  #[inline(always)]
  pub fn is_dphigh_1(&self) -> bool {
    *self == DPHIGH_A::DPHIGH_1
  }
}
#[doc = "Write proxy for field `DPHIGH`"]
pub struct DPHIGH_W<'a> {
  w: &'a mut W,
}
impl<'a> DPHIGH_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: DPHIGH_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "D+ pullup resistor is not enabled"]
  #[inline(always)]
  pub fn dphigh_0(self) -> &'a mut W {
    self.variant(DPHIGH_A::DPHIGH_0)
  }
  #[doc = "D+ pullup resistor is enabled"]
  #[inline(always)]
  pub fn dphigh_1(self) -> &'a mut W {
    self.variant(DPHIGH_A::DPHIGH_1)
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
  #[doc = "Bit 7 - D+ Data Line pullup resistor enable"]
  #[inline(always)]
  pub fn dphigh(&self) -> DPHIGH_R {
    DPHIGH_R::new(((self.bits >> 7) & 0x01) != 0)
  }
}
impl W {
  #[doc = "Bit 7 - D+ Data Line pullup resistor enable"]
  #[inline(always)]
  pub fn dphigh(&mut self) -> DPHIGH_W {
    DPHIGH_W { w: self }
  }
}
