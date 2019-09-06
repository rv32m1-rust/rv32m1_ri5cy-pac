#[doc = "Reader of register CPO"]
pub type R = crate::R<u32, super::CPO>;
#[doc = "Writer for register CPO"]
pub type W = crate::W<u32, super::CPO>;
#[doc = "Register CPO `reset()`'s with value 0"]
impl crate::ResetValue for super::CPO {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "Compute Operation request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPOREQ_A {
  #[doc = "0: Request is cleared."]
  CPOREQ_0,
  #[doc = "1: Request Compute Operation."]
  CPOREQ_1,
}
impl From<CPOREQ_A> for bool {
  #[inline(always)]
  fn from(variant: CPOREQ_A) -> Self {
    match variant {
      CPOREQ_A::CPOREQ_0 => false,
      CPOREQ_A::CPOREQ_1 => true,
    }
  }
}
#[doc = "Reader of field `CPOREQ`"]
pub type CPOREQ_R = crate::R<bool, CPOREQ_A>;
impl CPOREQ_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> CPOREQ_A {
    match self.bits {
      false => CPOREQ_A::CPOREQ_0,
      true => CPOREQ_A::CPOREQ_1,
    }
  }
  #[doc = "Checks if the value of the field is `CPOREQ_0`"]
  #[inline(always)]
  pub fn is_cporeq_0(&self) -> bool {
    *self == CPOREQ_A::CPOREQ_0
  }
  #[doc = "Checks if the value of the field is `CPOREQ_1`"]
  #[inline(always)]
  pub fn is_cporeq_1(&self) -> bool {
    *self == CPOREQ_A::CPOREQ_1
  }
}
#[doc = "Write proxy for field `CPOREQ`"]
pub struct CPOREQ_W<'a> {
  w: &'a mut W,
}
impl<'a> CPOREQ_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: CPOREQ_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Request is cleared."]
  #[inline(always)]
  pub fn cporeq_0(self) -> &'a mut W {
    self.variant(CPOREQ_A::CPOREQ_0)
  }
  #[doc = "Request Compute Operation."]
  #[inline(always)]
  pub fn cporeq_1(self) -> &'a mut W {
    self.variant(CPOREQ_A::CPOREQ_1)
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
    self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
    self.w
  }
}
#[doc = "Compute Operation acknowledge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPOACK_A {
  #[doc = "0: Compute operation entry has not completed or compute operation exit has completed."]
  CPOACK_0,
  #[doc = "1: Compute operation entry has completed or compute operation exit has not completed."]
  CPOACK_1,
}
impl From<CPOACK_A> for bool {
  #[inline(always)]
  fn from(variant: CPOACK_A) -> Self {
    match variant {
      CPOACK_A::CPOACK_0 => false,
      CPOACK_A::CPOACK_1 => true,
    }
  }
}
#[doc = "Reader of field `CPOACK`"]
pub type CPOACK_R = crate::R<bool, CPOACK_A>;
impl CPOACK_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> CPOACK_A {
    match self.bits {
      false => CPOACK_A::CPOACK_0,
      true => CPOACK_A::CPOACK_1,
    }
  }
  #[doc = "Checks if the value of the field is `CPOACK_0`"]
  #[inline(always)]
  pub fn is_cpoack_0(&self) -> bool {
    *self == CPOACK_A::CPOACK_0
  }
  #[doc = "Checks if the value of the field is `CPOACK_1`"]
  #[inline(always)]
  pub fn is_cpoack_1(&self) -> bool {
    *self == CPOACK_A::CPOACK_1
  }
}
#[doc = "Compute Operation wakeup on interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPOWOI_A {
  #[doc = "0: No effect."]
  CPOWOI_0,
  #[doc = "1: When set, the CPOREQ is cleared on any interrupt or exception vector fetch."]
  CPOWOI_1,
}
impl From<CPOWOI_A> for bool {
  #[inline(always)]
  fn from(variant: CPOWOI_A) -> Self {
    match variant {
      CPOWOI_A::CPOWOI_0 => false,
      CPOWOI_A::CPOWOI_1 => true,
    }
  }
}
#[doc = "Reader of field `CPOWOI`"]
pub type CPOWOI_R = crate::R<bool, CPOWOI_A>;
impl CPOWOI_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> CPOWOI_A {
    match self.bits {
      false => CPOWOI_A::CPOWOI_0,
      true => CPOWOI_A::CPOWOI_1,
    }
  }
  #[doc = "Checks if the value of the field is `CPOWOI_0`"]
  #[inline(always)]
  pub fn is_cpowoi_0(&self) -> bool {
    *self == CPOWOI_A::CPOWOI_0
  }
  #[doc = "Checks if the value of the field is `CPOWOI_1`"]
  #[inline(always)]
  pub fn is_cpowoi_1(&self) -> bool {
    *self == CPOWOI_A::CPOWOI_1
  }
}
#[doc = "Write proxy for field `CPOWOI`"]
pub struct CPOWOI_W<'a> {
  w: &'a mut W,
}
impl<'a> CPOWOI_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: CPOWOI_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "No effect."]
  #[inline(always)]
  pub fn cpowoi_0(self) -> &'a mut W {
    self.variant(CPOWOI_A::CPOWOI_0)
  }
  #[doc = "When set, the CPOREQ is cleared on any interrupt or exception vector fetch."]
  #[inline(always)]
  pub fn cpowoi_1(self) -> &'a mut W {
    self.variant(CPOWOI_A::CPOWOI_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
    self.w
  }
}
impl R {
  #[doc = "Bit 0 - Compute Operation request"]
  #[inline(always)]
  pub fn cporeq(&self) -> CPOREQ_R {
    CPOREQ_R::new((self.bits & 0x01) != 0)
  }
  #[doc = "Bit 1 - Compute Operation acknowledge"]
  #[inline(always)]
  pub fn cpoack(&self) -> CPOACK_R {
    CPOACK_R::new(((self.bits >> 1) & 0x01) != 0)
  }
  #[doc = "Bit 2 - Compute Operation wakeup on interrupt"]
  #[inline(always)]
  pub fn cpowoi(&self) -> CPOWOI_R {
    CPOWOI_R::new(((self.bits >> 2) & 0x01) != 0)
  }
}
impl W {
  #[doc = "Bit 0 - Compute Operation request"]
  #[inline(always)]
  pub fn cporeq(&mut self) -> CPOREQ_W {
    CPOREQ_W { w: self }
  }
  #[doc = "Bit 2 - Compute Operation wakeup on interrupt"]
  #[inline(always)]
  pub fn cpowoi(&mut self) -> CPOWOI_W {
    CPOWOI_W { w: self }
  }
}
