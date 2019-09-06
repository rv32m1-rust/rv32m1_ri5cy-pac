#[doc = "Reader of register DFCR"]
pub type R = crate::R<u32, super::DFCR>;
#[doc = "Writer for register DFCR"]
pub type W = crate::W<u32, super::DFCR>;
#[doc = "Register DFCR `reset()`'s with value 0"]
impl crate::ResetValue for super::DFCR {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "Clock Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CS_A {
  #[doc = "0: Digital filters are clocked by the bus clock."]
  CS_0,
  #[doc = "1: no description available"]
  CS_1,
}
impl From<CS_A> for bool {
  #[inline(always)]
  fn from(variant: CS_A) -> Self {
    match variant {
      CS_A::CS_0 => false,
      CS_A::CS_1 => true,
    }
  }
}
#[doc = "Reader of field `CS`"]
pub type CS_R = crate::R<bool, CS_A>;
impl CS_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> CS_A {
    match self.bits {
      false => CS_A::CS_0,
      true => CS_A::CS_1,
    }
  }
  #[doc = "Checks if the value of the field is `CS_0`"]
  #[inline(always)]
  pub fn is_cs_0(&self) -> bool {
    *self == CS_A::CS_0
  }
  #[doc = "Checks if the value of the field is `CS_1`"]
  #[inline(always)]
  pub fn is_cs_1(&self) -> bool {
    *self == CS_A::CS_1
  }
}
#[doc = "Write proxy for field `CS`"]
pub struct CS_W<'a> {
  w: &'a mut W,
}
impl<'a> CS_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: CS_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Digital filters are clocked by the bus clock."]
  #[inline(always)]
  pub fn cs_0(self) -> &'a mut W {
    self.variant(CS_A::CS_0)
  }
  #[doc = "no description available"]
  #[inline(always)]
  pub fn cs_1(self) -> &'a mut W {
    self.variant(CS_A::CS_1)
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
impl R {
  #[doc = "Bit 0 - Clock Source"]
  #[inline(always)]
  pub fn cs(&self) -> CS_R {
    CS_R::new((self.bits & 0x01) != 0)
  }
}
impl W {
  #[doc = "Bit 0 - Clock Source"]
  #[inline(always)]
  pub fn cs(&mut self) -> CS_W {
    CS_W { w: self }
  }
}
