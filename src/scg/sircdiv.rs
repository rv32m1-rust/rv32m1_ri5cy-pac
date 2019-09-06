#[doc = "Reader of register SIRCDIV"]
pub type R = crate::R<u32, super::SIRCDIV>;
#[doc = "Writer for register SIRCDIV"]
pub type W = crate::W<u32, super::SIRCDIV>;
#[doc = "Register SIRCDIV `reset()`'s with value 0"]
impl crate::ResetValue for super::SIRCDIV {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "Slow IRC Clock Divide 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SIRCDIV1_A {
  #[doc = "0: Output disabled"]
  SIRCDIV1_0,
  #[doc = "1: Divide by 1"]
  SIRCDIV1_1,
  #[doc = "2: Divide by 2"]
  SIRCDIV1_2,
  #[doc = "3: Divide by 4"]
  SIRCDIV1_3,
  #[doc = "4: Divide by 8"]
  SIRCDIV1_4,
  #[doc = "5: Divide by 16"]
  SIRCDIV1_5,
  #[doc = "6: Divide by 32"]
  SIRCDIV1_6,
  #[doc = "7: Divide by 64"]
  SIRCDIV1_7,
}
impl From<SIRCDIV1_A> for u8 {
  #[inline(always)]
  fn from(variant: SIRCDIV1_A) -> Self {
    match variant {
      SIRCDIV1_A::SIRCDIV1_0 => 0,
      SIRCDIV1_A::SIRCDIV1_1 => 1,
      SIRCDIV1_A::SIRCDIV1_2 => 2,
      SIRCDIV1_A::SIRCDIV1_3 => 3,
      SIRCDIV1_A::SIRCDIV1_4 => 4,
      SIRCDIV1_A::SIRCDIV1_5 => 5,
      SIRCDIV1_A::SIRCDIV1_6 => 6,
      SIRCDIV1_A::SIRCDIV1_7 => 7,
    }
  }
}
#[doc = "Reader of field `SIRCDIV1`"]
pub type SIRCDIV1_R = crate::R<u8, SIRCDIV1_A>;
impl SIRCDIV1_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> SIRCDIV1_A {
    match self.bits {
      0 => SIRCDIV1_A::SIRCDIV1_0,
      1 => SIRCDIV1_A::SIRCDIV1_1,
      2 => SIRCDIV1_A::SIRCDIV1_2,
      3 => SIRCDIV1_A::SIRCDIV1_3,
      4 => SIRCDIV1_A::SIRCDIV1_4,
      5 => SIRCDIV1_A::SIRCDIV1_5,
      6 => SIRCDIV1_A::SIRCDIV1_6,
      7 => SIRCDIV1_A::SIRCDIV1_7,
      _ => unreachable!(),
    }
  }
  #[doc = "Checks if the value of the field is `SIRCDIV1_0`"]
  #[inline(always)]
  pub fn is_sircdiv1_0(&self) -> bool {
    *self == SIRCDIV1_A::SIRCDIV1_0
  }
  #[doc = "Checks if the value of the field is `SIRCDIV1_1`"]
  #[inline(always)]
  pub fn is_sircdiv1_1(&self) -> bool {
    *self == SIRCDIV1_A::SIRCDIV1_1
  }
  #[doc = "Checks if the value of the field is `SIRCDIV1_2`"]
  #[inline(always)]
  pub fn is_sircdiv1_2(&self) -> bool {
    *self == SIRCDIV1_A::SIRCDIV1_2
  }
  #[doc = "Checks if the value of the field is `SIRCDIV1_3`"]
  #[inline(always)]
  pub fn is_sircdiv1_3(&self) -> bool {
    *self == SIRCDIV1_A::SIRCDIV1_3
  }
  #[doc = "Checks if the value of the field is `SIRCDIV1_4`"]
  #[inline(always)]
  pub fn is_sircdiv1_4(&self) -> bool {
    *self == SIRCDIV1_A::SIRCDIV1_4
  }
  #[doc = "Checks if the value of the field is `SIRCDIV1_5`"]
  #[inline(always)]
  pub fn is_sircdiv1_5(&self) -> bool {
    *self == SIRCDIV1_A::SIRCDIV1_5
  }
  #[doc = "Checks if the value of the field is `SIRCDIV1_6`"]
  #[inline(always)]
  pub fn is_sircdiv1_6(&self) -> bool {
    *self == SIRCDIV1_A::SIRCDIV1_6
  }
  #[doc = "Checks if the value of the field is `SIRCDIV1_7`"]
  #[inline(always)]
  pub fn is_sircdiv1_7(&self) -> bool {
    *self == SIRCDIV1_A::SIRCDIV1_7
  }
}
#[doc = "Write proxy for field `SIRCDIV1`"]
pub struct SIRCDIV1_W<'a> {
  w: &'a mut W,
}
impl<'a> SIRCDIV1_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: SIRCDIV1_A) -> &'a mut W {
    {
      self.bits(variant.into())
    }
  }
  #[doc = "Output disabled"]
  #[inline(always)]
  pub fn sircdiv1_0(self) -> &'a mut W {
    self.variant(SIRCDIV1_A::SIRCDIV1_0)
  }
  #[doc = "Divide by 1"]
  #[inline(always)]
  pub fn sircdiv1_1(self) -> &'a mut W {
    self.variant(SIRCDIV1_A::SIRCDIV1_1)
  }
  #[doc = "Divide by 2"]
  #[inline(always)]
  pub fn sircdiv1_2(self) -> &'a mut W {
    self.variant(SIRCDIV1_A::SIRCDIV1_2)
  }
  #[doc = "Divide by 4"]
  #[inline(always)]
  pub fn sircdiv1_3(self) -> &'a mut W {
    self.variant(SIRCDIV1_A::SIRCDIV1_3)
  }
  #[doc = "Divide by 8"]
  #[inline(always)]
  pub fn sircdiv1_4(self) -> &'a mut W {
    self.variant(SIRCDIV1_A::SIRCDIV1_4)
  }
  #[doc = "Divide by 16"]
  #[inline(always)]
  pub fn sircdiv1_5(self) -> &'a mut W {
    self.variant(SIRCDIV1_A::SIRCDIV1_5)
  }
  #[doc = "Divide by 32"]
  #[inline(always)]
  pub fn sircdiv1_6(self) -> &'a mut W {
    self.variant(SIRCDIV1_A::SIRCDIV1_6)
  }
  #[doc = "Divide by 64"]
  #[inline(always)]
  pub fn sircdiv1_7(self) -> &'a mut W {
    self.variant(SIRCDIV1_A::SIRCDIV1_7)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
    self.w
  }
}
#[doc = "Slow IRC Clock Divide 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SIRCDIV2_A {
  #[doc = "0: Output disabled"]
  SIRCDIV2_0,
  #[doc = "1: Divide by 1"]
  SIRCDIV2_1,
  #[doc = "2: Divide by 2"]
  SIRCDIV2_2,
  #[doc = "3: Divide by 4"]
  SIRCDIV2_3,
  #[doc = "4: Divide by 8"]
  SIRCDIV2_4,
  #[doc = "5: Divide by 16"]
  SIRCDIV2_5,
  #[doc = "6: Divide by 32"]
  SIRCDIV2_6,
  #[doc = "7: Divide by 64"]
  SIRCDIV2_7,
}
impl From<SIRCDIV2_A> for u8 {
  #[inline(always)]
  fn from(variant: SIRCDIV2_A) -> Self {
    match variant {
      SIRCDIV2_A::SIRCDIV2_0 => 0,
      SIRCDIV2_A::SIRCDIV2_1 => 1,
      SIRCDIV2_A::SIRCDIV2_2 => 2,
      SIRCDIV2_A::SIRCDIV2_3 => 3,
      SIRCDIV2_A::SIRCDIV2_4 => 4,
      SIRCDIV2_A::SIRCDIV2_5 => 5,
      SIRCDIV2_A::SIRCDIV2_6 => 6,
      SIRCDIV2_A::SIRCDIV2_7 => 7,
    }
  }
}
#[doc = "Reader of field `SIRCDIV2`"]
pub type SIRCDIV2_R = crate::R<u8, SIRCDIV2_A>;
impl SIRCDIV2_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> SIRCDIV2_A {
    match self.bits {
      0 => SIRCDIV2_A::SIRCDIV2_0,
      1 => SIRCDIV2_A::SIRCDIV2_1,
      2 => SIRCDIV2_A::SIRCDIV2_2,
      3 => SIRCDIV2_A::SIRCDIV2_3,
      4 => SIRCDIV2_A::SIRCDIV2_4,
      5 => SIRCDIV2_A::SIRCDIV2_5,
      6 => SIRCDIV2_A::SIRCDIV2_6,
      7 => SIRCDIV2_A::SIRCDIV2_7,
      _ => unreachable!(),
    }
  }
  #[doc = "Checks if the value of the field is `SIRCDIV2_0`"]
  #[inline(always)]
  pub fn is_sircdiv2_0(&self) -> bool {
    *self == SIRCDIV2_A::SIRCDIV2_0
  }
  #[doc = "Checks if the value of the field is `SIRCDIV2_1`"]
  #[inline(always)]
  pub fn is_sircdiv2_1(&self) -> bool {
    *self == SIRCDIV2_A::SIRCDIV2_1
  }
  #[doc = "Checks if the value of the field is `SIRCDIV2_2`"]
  #[inline(always)]
  pub fn is_sircdiv2_2(&self) -> bool {
    *self == SIRCDIV2_A::SIRCDIV2_2
  }
  #[doc = "Checks if the value of the field is `SIRCDIV2_3`"]
  #[inline(always)]
  pub fn is_sircdiv2_3(&self) -> bool {
    *self == SIRCDIV2_A::SIRCDIV2_3
  }
  #[doc = "Checks if the value of the field is `SIRCDIV2_4`"]
  #[inline(always)]
  pub fn is_sircdiv2_4(&self) -> bool {
    *self == SIRCDIV2_A::SIRCDIV2_4
  }
  #[doc = "Checks if the value of the field is `SIRCDIV2_5`"]
  #[inline(always)]
  pub fn is_sircdiv2_5(&self) -> bool {
    *self == SIRCDIV2_A::SIRCDIV2_5
  }
  #[doc = "Checks if the value of the field is `SIRCDIV2_6`"]
  #[inline(always)]
  pub fn is_sircdiv2_6(&self) -> bool {
    *self == SIRCDIV2_A::SIRCDIV2_6
  }
  #[doc = "Checks if the value of the field is `SIRCDIV2_7`"]
  #[inline(always)]
  pub fn is_sircdiv2_7(&self) -> bool {
    *self == SIRCDIV2_A::SIRCDIV2_7
  }
}
#[doc = "Write proxy for field `SIRCDIV2`"]
pub struct SIRCDIV2_W<'a> {
  w: &'a mut W,
}
impl<'a> SIRCDIV2_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: SIRCDIV2_A) -> &'a mut W {
    {
      self.bits(variant.into())
    }
  }
  #[doc = "Output disabled"]
  #[inline(always)]
  pub fn sircdiv2_0(self) -> &'a mut W {
    self.variant(SIRCDIV2_A::SIRCDIV2_0)
  }
  #[doc = "Divide by 1"]
  #[inline(always)]
  pub fn sircdiv2_1(self) -> &'a mut W {
    self.variant(SIRCDIV2_A::SIRCDIV2_1)
  }
  #[doc = "Divide by 2"]
  #[inline(always)]
  pub fn sircdiv2_2(self) -> &'a mut W {
    self.variant(SIRCDIV2_A::SIRCDIV2_2)
  }
  #[doc = "Divide by 4"]
  #[inline(always)]
  pub fn sircdiv2_3(self) -> &'a mut W {
    self.variant(SIRCDIV2_A::SIRCDIV2_3)
  }
  #[doc = "Divide by 8"]
  #[inline(always)]
  pub fn sircdiv2_4(self) -> &'a mut W {
    self.variant(SIRCDIV2_A::SIRCDIV2_4)
  }
  #[doc = "Divide by 16"]
  #[inline(always)]
  pub fn sircdiv2_5(self) -> &'a mut W {
    self.variant(SIRCDIV2_A::SIRCDIV2_5)
  }
  #[doc = "Divide by 32"]
  #[inline(always)]
  pub fn sircdiv2_6(self) -> &'a mut W {
    self.variant(SIRCDIV2_A::SIRCDIV2_6)
  }
  #[doc = "Divide by 64"]
  #[inline(always)]
  pub fn sircdiv2_7(self) -> &'a mut W {
    self.variant(SIRCDIV2_A::SIRCDIV2_7)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
    self.w
  }
}
#[doc = "Slow IRC Clock Divider 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SIRCDIV3_A {
  #[doc = "0: Output disabled"]
  SIRCDIV3_0,
  #[doc = "1: Divide by 1"]
  SIRCDIV3_1,
  #[doc = "2: Divide by 2"]
  SIRCDIV3_2,
  #[doc = "3: Divide by 4"]
  SIRCDIV3_3,
  #[doc = "4: Divide by 8"]
  SIRCDIV3_4,
  #[doc = "5: Divide by 16"]
  SIRCDIV3_5,
  #[doc = "6: Divide by 32"]
  SIRCDIV3_6,
  #[doc = "7: Divide by 64"]
  SIRCDIV3_7,
}
impl From<SIRCDIV3_A> for u8 {
  #[inline(always)]
  fn from(variant: SIRCDIV3_A) -> Self {
    match variant {
      SIRCDIV3_A::SIRCDIV3_0 => 0,
      SIRCDIV3_A::SIRCDIV3_1 => 1,
      SIRCDIV3_A::SIRCDIV3_2 => 2,
      SIRCDIV3_A::SIRCDIV3_3 => 3,
      SIRCDIV3_A::SIRCDIV3_4 => 4,
      SIRCDIV3_A::SIRCDIV3_5 => 5,
      SIRCDIV3_A::SIRCDIV3_6 => 6,
      SIRCDIV3_A::SIRCDIV3_7 => 7,
    }
  }
}
#[doc = "Reader of field `SIRCDIV3`"]
pub type SIRCDIV3_R = crate::R<u8, SIRCDIV3_A>;
impl SIRCDIV3_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> SIRCDIV3_A {
    match self.bits {
      0 => SIRCDIV3_A::SIRCDIV3_0,
      1 => SIRCDIV3_A::SIRCDIV3_1,
      2 => SIRCDIV3_A::SIRCDIV3_2,
      3 => SIRCDIV3_A::SIRCDIV3_3,
      4 => SIRCDIV3_A::SIRCDIV3_4,
      5 => SIRCDIV3_A::SIRCDIV3_5,
      6 => SIRCDIV3_A::SIRCDIV3_6,
      7 => SIRCDIV3_A::SIRCDIV3_7,
      _ => unreachable!(),
    }
  }
  #[doc = "Checks if the value of the field is `SIRCDIV3_0`"]
  #[inline(always)]
  pub fn is_sircdiv3_0(&self) -> bool {
    *self == SIRCDIV3_A::SIRCDIV3_0
  }
  #[doc = "Checks if the value of the field is `SIRCDIV3_1`"]
  #[inline(always)]
  pub fn is_sircdiv3_1(&self) -> bool {
    *self == SIRCDIV3_A::SIRCDIV3_1
  }
  #[doc = "Checks if the value of the field is `SIRCDIV3_2`"]
  #[inline(always)]
  pub fn is_sircdiv3_2(&self) -> bool {
    *self == SIRCDIV3_A::SIRCDIV3_2
  }
  #[doc = "Checks if the value of the field is `SIRCDIV3_3`"]
  #[inline(always)]
  pub fn is_sircdiv3_3(&self) -> bool {
    *self == SIRCDIV3_A::SIRCDIV3_3
  }
  #[doc = "Checks if the value of the field is `SIRCDIV3_4`"]
  #[inline(always)]
  pub fn is_sircdiv3_4(&self) -> bool {
    *self == SIRCDIV3_A::SIRCDIV3_4
  }
  #[doc = "Checks if the value of the field is `SIRCDIV3_5`"]
  #[inline(always)]
  pub fn is_sircdiv3_5(&self) -> bool {
    *self == SIRCDIV3_A::SIRCDIV3_5
  }
  #[doc = "Checks if the value of the field is `SIRCDIV3_6`"]
  #[inline(always)]
  pub fn is_sircdiv3_6(&self) -> bool {
    *self == SIRCDIV3_A::SIRCDIV3_6
  }
  #[doc = "Checks if the value of the field is `SIRCDIV3_7`"]
  #[inline(always)]
  pub fn is_sircdiv3_7(&self) -> bool {
    *self == SIRCDIV3_A::SIRCDIV3_7
  }
}
#[doc = "Write proxy for field `SIRCDIV3`"]
pub struct SIRCDIV3_W<'a> {
  w: &'a mut W,
}
impl<'a> SIRCDIV3_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: SIRCDIV3_A) -> &'a mut W {
    {
      self.bits(variant.into())
    }
  }
  #[doc = "Output disabled"]
  #[inline(always)]
  pub fn sircdiv3_0(self) -> &'a mut W {
    self.variant(SIRCDIV3_A::SIRCDIV3_0)
  }
  #[doc = "Divide by 1"]
  #[inline(always)]
  pub fn sircdiv3_1(self) -> &'a mut W {
    self.variant(SIRCDIV3_A::SIRCDIV3_1)
  }
  #[doc = "Divide by 2"]
  #[inline(always)]
  pub fn sircdiv3_2(self) -> &'a mut W {
    self.variant(SIRCDIV3_A::SIRCDIV3_2)
  }
  #[doc = "Divide by 4"]
  #[inline(always)]
  pub fn sircdiv3_3(self) -> &'a mut W {
    self.variant(SIRCDIV3_A::SIRCDIV3_3)
  }
  #[doc = "Divide by 8"]
  #[inline(always)]
  pub fn sircdiv3_4(self) -> &'a mut W {
    self.variant(SIRCDIV3_A::SIRCDIV3_4)
  }
  #[doc = "Divide by 16"]
  #[inline(always)]
  pub fn sircdiv3_5(self) -> &'a mut W {
    self.variant(SIRCDIV3_A::SIRCDIV3_5)
  }
  #[doc = "Divide by 32"]
  #[inline(always)]
  pub fn sircdiv3_6(self) -> &'a mut W {
    self.variant(SIRCDIV3_A::SIRCDIV3_6)
  }
  #[doc = "Divide by 64"]
  #[inline(always)]
  pub fn sircdiv3_7(self) -> &'a mut W {
    self.variant(SIRCDIV3_A::SIRCDIV3_7)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
    self.w
  }
}
impl R {
  #[doc = "Bits 0:2 - Slow IRC Clock Divide 1"]
  #[inline(always)]
  pub fn sircdiv1(&self) -> SIRCDIV1_R {
    SIRCDIV1_R::new((self.bits & 0x07) as u8)
  }
  #[doc = "Bits 8:10 - Slow IRC Clock Divide 2"]
  #[inline(always)]
  pub fn sircdiv2(&self) -> SIRCDIV2_R {
    SIRCDIV2_R::new(((self.bits >> 8) & 0x07) as u8)
  }
  #[doc = "Bits 16:18 - Slow IRC Clock Divider 3"]
  #[inline(always)]
  pub fn sircdiv3(&self) -> SIRCDIV3_R {
    SIRCDIV3_R::new(((self.bits >> 16) & 0x07) as u8)
  }
}
impl W {
  #[doc = "Bits 0:2 - Slow IRC Clock Divide 1"]
  #[inline(always)]
  pub fn sircdiv1(&mut self) -> SIRCDIV1_W {
    SIRCDIV1_W { w: self }
  }
  #[doc = "Bits 8:10 - Slow IRC Clock Divide 2"]
  #[inline(always)]
  pub fn sircdiv2(&mut self) -> SIRCDIV2_W {
    SIRCDIV2_W { w: self }
  }
  #[doc = "Bits 16:18 - Slow IRC Clock Divider 3"]
  #[inline(always)]
  pub fn sircdiv3(&mut self) -> SIRCDIV3_W {
    SIRCDIV3_W { w: self }
  }
}
