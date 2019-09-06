#[doc = "Reader of register DIVISOR"]
pub type R = crate::R<u32, super::DIVISOR>;
#[doc = "Writer for register DIVISOR"]
pub type W = crate::W<u32, super::DIVISOR>;
#[doc = "Register DIVISOR `reset()`'s with value 0x0174"]
impl crate::ResetValue for super::DIVISOR {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0x0174
  }
}
#[doc = "Divisor (F/D) Value\n\nValue on reset: 372"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIVISOR_VALUE_A {
  #[doc = "0: Invalid. As per ISO 7816 specification, minimum value of F/D is 5"]
  DIVISOR_VALUE_0,
  #[doc = "1: Invalid. As per ISO 7816 specification, minimum value of F/D is 5"]
  DIVISOR_VALUE_1,
  #[doc = "2: Invalid. As per ISO 7816 specification, minimum value of F/D is 5"]
  DIVISOR_VALUE_2,
  #[doc = "3: Invalid. As per ISO 7816 specification, minimum value of F/D is 5"]
  DIVISOR_VALUE_3,
  #[doc = "4: Invalid. As per ISO 7816 specification, minimum value of F/D is 5"]
  DIVISOR_VALUE_4,
  #[doc = "372: Divisor value for F = 372 and D = 1 (default)"]
  DIVISOR_VALUE_372,
}
impl From<DIVISOR_VALUE_A> for u16 {
  #[inline(always)]
  fn from(variant: DIVISOR_VALUE_A) -> Self {
    match variant {
      DIVISOR_VALUE_A::DIVISOR_VALUE_0 => 0,
      DIVISOR_VALUE_A::DIVISOR_VALUE_1 => 1,
      DIVISOR_VALUE_A::DIVISOR_VALUE_2 => 2,
      DIVISOR_VALUE_A::DIVISOR_VALUE_3 => 3,
      DIVISOR_VALUE_A::DIVISOR_VALUE_4 => 4,
      DIVISOR_VALUE_A::DIVISOR_VALUE_372 => 372,
    }
  }
}
#[doc = "Reader of field `DIVISOR_VALUE`"]
pub type DIVISOR_VALUE_R = crate::R<u16, DIVISOR_VALUE_A>;
impl DIVISOR_VALUE_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> crate::Variant<u16, DIVISOR_VALUE_A> {
    use crate::Variant::*;
    match self.bits {
      0 => Val(DIVISOR_VALUE_A::DIVISOR_VALUE_0),
      1 => Val(DIVISOR_VALUE_A::DIVISOR_VALUE_1),
      2 => Val(DIVISOR_VALUE_A::DIVISOR_VALUE_2),
      3 => Val(DIVISOR_VALUE_A::DIVISOR_VALUE_3),
      4 => Val(DIVISOR_VALUE_A::DIVISOR_VALUE_4),
      372 => Val(DIVISOR_VALUE_A::DIVISOR_VALUE_372),
      i => Res(i),
    }
  }
  #[doc = "Checks if the value of the field is `DIVISOR_VALUE_0`"]
  #[inline(always)]
  pub fn is_divisor_value_0(&self) -> bool {
    *self == DIVISOR_VALUE_A::DIVISOR_VALUE_0
  }
  #[doc = "Checks if the value of the field is `DIVISOR_VALUE_1`"]
  #[inline(always)]
  pub fn is_divisor_value_1(&self) -> bool {
    *self == DIVISOR_VALUE_A::DIVISOR_VALUE_1
  }
  #[doc = "Checks if the value of the field is `DIVISOR_VALUE_2`"]
  #[inline(always)]
  pub fn is_divisor_value_2(&self) -> bool {
    *self == DIVISOR_VALUE_A::DIVISOR_VALUE_2
  }
  #[doc = "Checks if the value of the field is `DIVISOR_VALUE_3`"]
  #[inline(always)]
  pub fn is_divisor_value_3(&self) -> bool {
    *self == DIVISOR_VALUE_A::DIVISOR_VALUE_3
  }
  #[doc = "Checks if the value of the field is `DIVISOR_VALUE_4`"]
  #[inline(always)]
  pub fn is_divisor_value_4(&self) -> bool {
    *self == DIVISOR_VALUE_A::DIVISOR_VALUE_4
  }
  #[doc = "Checks if the value of the field is `DIVISOR_VALUE_372`"]
  #[inline(always)]
  pub fn is_divisor_value_372(&self) -> bool {
    *self == DIVISOR_VALUE_A::DIVISOR_VALUE_372
  }
}
#[doc = "Write proxy for field `DIVISOR_VALUE`"]
pub struct DIVISOR_VALUE_W<'a> {
  w: &'a mut W,
}
impl<'a> DIVISOR_VALUE_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: DIVISOR_VALUE_A) -> &'a mut W {
    unsafe { self.bits(variant.into()) }
  }
  #[doc = "Invalid. As per ISO 7816 specification, minimum value of F/D is 5"]
  #[inline(always)]
  pub fn divisor_value_0(self) -> &'a mut W {
    self.variant(DIVISOR_VALUE_A::DIVISOR_VALUE_0)
  }
  #[doc = "Invalid. As per ISO 7816 specification, minimum value of F/D is 5"]
  #[inline(always)]
  pub fn divisor_value_1(self) -> &'a mut W {
    self.variant(DIVISOR_VALUE_A::DIVISOR_VALUE_1)
  }
  #[doc = "Invalid. As per ISO 7816 specification, minimum value of F/D is 5"]
  #[inline(always)]
  pub fn divisor_value_2(self) -> &'a mut W {
    self.variant(DIVISOR_VALUE_A::DIVISOR_VALUE_2)
  }
  #[doc = "Invalid. As per ISO 7816 specification, minimum value of F/D is 5"]
  #[inline(always)]
  pub fn divisor_value_3(self) -> &'a mut W {
    self.variant(DIVISOR_VALUE_A::DIVISOR_VALUE_3)
  }
  #[doc = "Invalid. As per ISO 7816 specification, minimum value of F/D is 5"]
  #[inline(always)]
  pub fn divisor_value_4(self) -> &'a mut W {
    self.variant(DIVISOR_VALUE_A::DIVISOR_VALUE_4)
  }
  #[doc = "Divisor value for F = 372 and D = 1 (default)"]
  #[inline(always)]
  pub fn divisor_value_372(self) -> &'a mut W {
    self.variant(DIVISOR_VALUE_A::DIVISOR_VALUE_372)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u16) -> &'a mut W {
    self.w.bits = (self.w.bits & !0x01ff) | ((value as u32) & 0x01ff);
    self.w
  }
}
impl R {
  #[doc = "Bits 0:8 - Divisor (F/D) Value"]
  #[inline(always)]
  pub fn divisor_value(&self) -> DIVISOR_VALUE_R {
    DIVISOR_VALUE_R::new((self.bits & 0x01ff) as u16)
  }
}
impl W {
  #[doc = "Bits 0:8 - Divisor (F/D) Value"]
  #[inline(always)]
  pub fn divisor_value(&mut self) -> DIVISOR_VALUE_W {
    DIVISOR_VALUE_W { w: self }
  }
}
