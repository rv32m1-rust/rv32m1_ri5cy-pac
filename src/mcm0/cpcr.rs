#[doc = "Reader of register CPCR"]
pub type R = crate::R<u32, super::CPCR>;
#[doc = "Writer for register CPCR"]
pub type W = crate::W<u32, super::CPCR>;
#[doc = "Register CPCR `reset()`'s with value 0"]
impl crate::ResetValue for super::CPCR {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "Crossbar round-robin arbitration enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CBRR_A {
  #[doc = "0: Fixed-priority arbitration"]
  CBRR_0,
  #[doc = "1: Round-robin arbitration"]
  CBRR_1,
}
impl From<CBRR_A> for bool {
  #[inline(always)]
  fn from(variant: CBRR_A) -> Self {
    match variant {
      CBRR_A::CBRR_0 => false,
      CBRR_A::CBRR_1 => true,
    }
  }
}
#[doc = "Reader of field `CBRR`"]
pub type CBRR_R = crate::R<bool, CBRR_A>;
impl CBRR_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> CBRR_A {
    match self.bits {
      false => CBRR_A::CBRR_0,
      true => CBRR_A::CBRR_1,
    }
  }
  #[doc = "Checks if the value of the field is `CBRR_0`"]
  #[inline(always)]
  pub fn is_cbrr_0(&self) -> bool {
    *self == CBRR_A::CBRR_0
  }
  #[doc = "Checks if the value of the field is `CBRR_1`"]
  #[inline(always)]
  pub fn is_cbrr_1(&self) -> bool {
    *self == CBRR_A::CBRR_1
  }
}
#[doc = "Write proxy for field `CBRR`"]
pub struct CBRR_W<'a> {
  w: &'a mut W,
}
impl<'a> CBRR_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: CBRR_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Fixed-priority arbitration"]
  #[inline(always)]
  pub fn cbrr_0(self) -> &'a mut W {
    self.variant(CBRR_A::CBRR_0)
  }
  #[doc = "Round-robin arbitration"]
  #[inline(always)]
  pub fn cbrr_1(self) -> &'a mut W {
    self.variant(CBRR_A::CBRR_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
    self.w
  }
}
impl R {
  #[doc = "Bit 9 - Crossbar round-robin arbitration enable"]
  #[inline(always)]
  pub fn cbrr(&self) -> CBRR_R {
    CBRR_R::new(((self.bits >> 9) & 0x01) != 0)
  }
}
impl W {
  #[doc = "Bit 9 - Crossbar round-robin arbitration enable"]
  #[inline(always)]
  pub fn cbrr(&mut self) -> CBRR_W {
    CBRR_W { w: self }
  }
}
