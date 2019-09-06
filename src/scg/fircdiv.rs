#[doc = "Reader of register FIRCDIV"]
pub type R = crate::R<u32, super::FIRCDIV>;
#[doc = "Writer for register FIRCDIV"]
pub type W = crate::W<u32, super::FIRCDIV>;
#[doc = "Register FIRCDIV `reset()`'s with value 0"]
impl crate::ResetValue for super::FIRCDIV {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "Fast IRC Clock Divide 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIRCDIV1_A {
  #[doc = "0: Output disabled"]
  FIRCDIV1_0,
  #[doc = "1: Divide by 1"]
  FIRCDIV1_1,
  #[doc = "2: Divide by 2"]
  FIRCDIV1_2,
  #[doc = "3: Divide by 4"]
  FIRCDIV1_3,
  #[doc = "4: Divide by 8"]
  FIRCDIV1_4,
  #[doc = "5: Divide by 16"]
  FIRCDIV1_5,
  #[doc = "6: Divide by 32"]
  FIRCDIV1_6,
  #[doc = "7: Divide by 64"]
  FIRCDIV1_7,
}
impl From<FIRCDIV1_A> for u8 {
  #[inline(always)]
  fn from(variant: FIRCDIV1_A) -> Self {
    match variant {
      FIRCDIV1_A::FIRCDIV1_0 => 0,
      FIRCDIV1_A::FIRCDIV1_1 => 1,
      FIRCDIV1_A::FIRCDIV1_2 => 2,
      FIRCDIV1_A::FIRCDIV1_3 => 3,
      FIRCDIV1_A::FIRCDIV1_4 => 4,
      FIRCDIV1_A::FIRCDIV1_5 => 5,
      FIRCDIV1_A::FIRCDIV1_6 => 6,
      FIRCDIV1_A::FIRCDIV1_7 => 7,
    }
  }
}
#[doc = "Reader of field `FIRCDIV1`"]
pub type FIRCDIV1_R = crate::R<u8, FIRCDIV1_A>;
impl FIRCDIV1_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> FIRCDIV1_A {
    match self.bits {
      0 => FIRCDIV1_A::FIRCDIV1_0,
      1 => FIRCDIV1_A::FIRCDIV1_1,
      2 => FIRCDIV1_A::FIRCDIV1_2,
      3 => FIRCDIV1_A::FIRCDIV1_3,
      4 => FIRCDIV1_A::FIRCDIV1_4,
      5 => FIRCDIV1_A::FIRCDIV1_5,
      6 => FIRCDIV1_A::FIRCDIV1_6,
      7 => FIRCDIV1_A::FIRCDIV1_7,
      _ => unreachable!(),
    }
  }
  #[doc = "Checks if the value of the field is `FIRCDIV1_0`"]
  #[inline(always)]
  pub fn is_fircdiv1_0(&self) -> bool {
    *self == FIRCDIV1_A::FIRCDIV1_0
  }
  #[doc = "Checks if the value of the field is `FIRCDIV1_1`"]
  #[inline(always)]
  pub fn is_fircdiv1_1(&self) -> bool {
    *self == FIRCDIV1_A::FIRCDIV1_1
  }
  #[doc = "Checks if the value of the field is `FIRCDIV1_2`"]
  #[inline(always)]
  pub fn is_fircdiv1_2(&self) -> bool {
    *self == FIRCDIV1_A::FIRCDIV1_2
  }
  #[doc = "Checks if the value of the field is `FIRCDIV1_3`"]
  #[inline(always)]
  pub fn is_fircdiv1_3(&self) -> bool {
    *self == FIRCDIV1_A::FIRCDIV1_3
  }
  #[doc = "Checks if the value of the field is `FIRCDIV1_4`"]
  #[inline(always)]
  pub fn is_fircdiv1_4(&self) -> bool {
    *self == FIRCDIV1_A::FIRCDIV1_4
  }
  #[doc = "Checks if the value of the field is `FIRCDIV1_5`"]
  #[inline(always)]
  pub fn is_fircdiv1_5(&self) -> bool {
    *self == FIRCDIV1_A::FIRCDIV1_5
  }
  #[doc = "Checks if the value of the field is `FIRCDIV1_6`"]
  #[inline(always)]
  pub fn is_fircdiv1_6(&self) -> bool {
    *self == FIRCDIV1_A::FIRCDIV1_6
  }
  #[doc = "Checks if the value of the field is `FIRCDIV1_7`"]
  #[inline(always)]
  pub fn is_fircdiv1_7(&self) -> bool {
    *self == FIRCDIV1_A::FIRCDIV1_7
  }
}
#[doc = "Write proxy for field `FIRCDIV1`"]
pub struct FIRCDIV1_W<'a> {
  w: &'a mut W,
}
impl<'a> FIRCDIV1_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: FIRCDIV1_A) -> &'a mut W {
    {
      self.bits(variant.into())
    }
  }
  #[doc = "Output disabled"]
  #[inline(always)]
  pub fn fircdiv1_0(self) -> &'a mut W {
    self.variant(FIRCDIV1_A::FIRCDIV1_0)
  }
  #[doc = "Divide by 1"]
  #[inline(always)]
  pub fn fircdiv1_1(self) -> &'a mut W {
    self.variant(FIRCDIV1_A::FIRCDIV1_1)
  }
  #[doc = "Divide by 2"]
  #[inline(always)]
  pub fn fircdiv1_2(self) -> &'a mut W {
    self.variant(FIRCDIV1_A::FIRCDIV1_2)
  }
  #[doc = "Divide by 4"]
  #[inline(always)]
  pub fn fircdiv1_3(self) -> &'a mut W {
    self.variant(FIRCDIV1_A::FIRCDIV1_3)
  }
  #[doc = "Divide by 8"]
  #[inline(always)]
  pub fn fircdiv1_4(self) -> &'a mut W {
    self.variant(FIRCDIV1_A::FIRCDIV1_4)
  }
  #[doc = "Divide by 16"]
  #[inline(always)]
  pub fn fircdiv1_5(self) -> &'a mut W {
    self.variant(FIRCDIV1_A::FIRCDIV1_5)
  }
  #[doc = "Divide by 32"]
  #[inline(always)]
  pub fn fircdiv1_6(self) -> &'a mut W {
    self.variant(FIRCDIV1_A::FIRCDIV1_6)
  }
  #[doc = "Divide by 64"]
  #[inline(always)]
  pub fn fircdiv1_7(self) -> &'a mut W {
    self.variant(FIRCDIV1_A::FIRCDIV1_7)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
    self.w
  }
}
#[doc = "Fast IRC Clock Divide 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIRCDIV2_A {
  #[doc = "0: Output disabled"]
  FIRCDIV2_0,
  #[doc = "1: Divide by 1"]
  FIRCDIV2_1,
  #[doc = "2: Divide by 2"]
  FIRCDIV2_2,
  #[doc = "3: Divide by 4"]
  FIRCDIV2_3,
  #[doc = "4: Divide by 8"]
  FIRCDIV2_4,
  #[doc = "5: Divide by 16"]
  FIRCDIV2_5,
  #[doc = "6: Divide by 32"]
  FIRCDIV2_6,
  #[doc = "7: Divide by 64"]
  FIRCDIV2_7,
}
impl From<FIRCDIV2_A> for u8 {
  #[inline(always)]
  fn from(variant: FIRCDIV2_A) -> Self {
    match variant {
      FIRCDIV2_A::FIRCDIV2_0 => 0,
      FIRCDIV2_A::FIRCDIV2_1 => 1,
      FIRCDIV2_A::FIRCDIV2_2 => 2,
      FIRCDIV2_A::FIRCDIV2_3 => 3,
      FIRCDIV2_A::FIRCDIV2_4 => 4,
      FIRCDIV2_A::FIRCDIV2_5 => 5,
      FIRCDIV2_A::FIRCDIV2_6 => 6,
      FIRCDIV2_A::FIRCDIV2_7 => 7,
    }
  }
}
#[doc = "Reader of field `FIRCDIV2`"]
pub type FIRCDIV2_R = crate::R<u8, FIRCDIV2_A>;
impl FIRCDIV2_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> FIRCDIV2_A {
    match self.bits {
      0 => FIRCDIV2_A::FIRCDIV2_0,
      1 => FIRCDIV2_A::FIRCDIV2_1,
      2 => FIRCDIV2_A::FIRCDIV2_2,
      3 => FIRCDIV2_A::FIRCDIV2_3,
      4 => FIRCDIV2_A::FIRCDIV2_4,
      5 => FIRCDIV2_A::FIRCDIV2_5,
      6 => FIRCDIV2_A::FIRCDIV2_6,
      7 => FIRCDIV2_A::FIRCDIV2_7,
      _ => unreachable!(),
    }
  }
  #[doc = "Checks if the value of the field is `FIRCDIV2_0`"]
  #[inline(always)]
  pub fn is_fircdiv2_0(&self) -> bool {
    *self == FIRCDIV2_A::FIRCDIV2_0
  }
  #[doc = "Checks if the value of the field is `FIRCDIV2_1`"]
  #[inline(always)]
  pub fn is_fircdiv2_1(&self) -> bool {
    *self == FIRCDIV2_A::FIRCDIV2_1
  }
  #[doc = "Checks if the value of the field is `FIRCDIV2_2`"]
  #[inline(always)]
  pub fn is_fircdiv2_2(&self) -> bool {
    *self == FIRCDIV2_A::FIRCDIV2_2
  }
  #[doc = "Checks if the value of the field is `FIRCDIV2_3`"]
  #[inline(always)]
  pub fn is_fircdiv2_3(&self) -> bool {
    *self == FIRCDIV2_A::FIRCDIV2_3
  }
  #[doc = "Checks if the value of the field is `FIRCDIV2_4`"]
  #[inline(always)]
  pub fn is_fircdiv2_4(&self) -> bool {
    *self == FIRCDIV2_A::FIRCDIV2_4
  }
  #[doc = "Checks if the value of the field is `FIRCDIV2_5`"]
  #[inline(always)]
  pub fn is_fircdiv2_5(&self) -> bool {
    *self == FIRCDIV2_A::FIRCDIV2_5
  }
  #[doc = "Checks if the value of the field is `FIRCDIV2_6`"]
  #[inline(always)]
  pub fn is_fircdiv2_6(&self) -> bool {
    *self == FIRCDIV2_A::FIRCDIV2_6
  }
  #[doc = "Checks if the value of the field is `FIRCDIV2_7`"]
  #[inline(always)]
  pub fn is_fircdiv2_7(&self) -> bool {
    *self == FIRCDIV2_A::FIRCDIV2_7
  }
}
#[doc = "Write proxy for field `FIRCDIV2`"]
pub struct FIRCDIV2_W<'a> {
  w: &'a mut W,
}
impl<'a> FIRCDIV2_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: FIRCDIV2_A) -> &'a mut W {
    {
      self.bits(variant.into())
    }
  }
  #[doc = "Output disabled"]
  #[inline(always)]
  pub fn fircdiv2_0(self) -> &'a mut W {
    self.variant(FIRCDIV2_A::FIRCDIV2_0)
  }
  #[doc = "Divide by 1"]
  #[inline(always)]
  pub fn fircdiv2_1(self) -> &'a mut W {
    self.variant(FIRCDIV2_A::FIRCDIV2_1)
  }
  #[doc = "Divide by 2"]
  #[inline(always)]
  pub fn fircdiv2_2(self) -> &'a mut W {
    self.variant(FIRCDIV2_A::FIRCDIV2_2)
  }
  #[doc = "Divide by 4"]
  #[inline(always)]
  pub fn fircdiv2_3(self) -> &'a mut W {
    self.variant(FIRCDIV2_A::FIRCDIV2_3)
  }
  #[doc = "Divide by 8"]
  #[inline(always)]
  pub fn fircdiv2_4(self) -> &'a mut W {
    self.variant(FIRCDIV2_A::FIRCDIV2_4)
  }
  #[doc = "Divide by 16"]
  #[inline(always)]
  pub fn fircdiv2_5(self) -> &'a mut W {
    self.variant(FIRCDIV2_A::FIRCDIV2_5)
  }
  #[doc = "Divide by 32"]
  #[inline(always)]
  pub fn fircdiv2_6(self) -> &'a mut W {
    self.variant(FIRCDIV2_A::FIRCDIV2_6)
  }
  #[doc = "Divide by 64"]
  #[inline(always)]
  pub fn fircdiv2_7(self) -> &'a mut W {
    self.variant(FIRCDIV2_A::FIRCDIV2_7)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
    self.w
  }
}
#[doc = "Fast IRC Clock Divider 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIRCDIV3_A {
  #[doc = "0: Clock disabled"]
  FIRCDIV3_0,
  #[doc = "1: Divide by 1"]
  FIRCDIV3_1,
  #[doc = "2: Divide by 2"]
  FIRCDIV3_2,
  #[doc = "3: Divide by 4"]
  FIRCDIV3_3,
  #[doc = "4: Divide by 8"]
  FIRCDIV3_4,
  #[doc = "5: Divide by 16"]
  FIRCDIV3_5,
  #[doc = "6: Divide by 32"]
  FIRCDIV3_6,
  #[doc = "7: Divide by 64"]
  FIRCDIV3_7,
}
impl From<FIRCDIV3_A> for u8 {
  #[inline(always)]
  fn from(variant: FIRCDIV3_A) -> Self {
    match variant {
      FIRCDIV3_A::FIRCDIV3_0 => 0,
      FIRCDIV3_A::FIRCDIV3_1 => 1,
      FIRCDIV3_A::FIRCDIV3_2 => 2,
      FIRCDIV3_A::FIRCDIV3_3 => 3,
      FIRCDIV3_A::FIRCDIV3_4 => 4,
      FIRCDIV3_A::FIRCDIV3_5 => 5,
      FIRCDIV3_A::FIRCDIV3_6 => 6,
      FIRCDIV3_A::FIRCDIV3_7 => 7,
    }
  }
}
#[doc = "Reader of field `FIRCDIV3`"]
pub type FIRCDIV3_R = crate::R<u8, FIRCDIV3_A>;
impl FIRCDIV3_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> FIRCDIV3_A {
    match self.bits {
      0 => FIRCDIV3_A::FIRCDIV3_0,
      1 => FIRCDIV3_A::FIRCDIV3_1,
      2 => FIRCDIV3_A::FIRCDIV3_2,
      3 => FIRCDIV3_A::FIRCDIV3_3,
      4 => FIRCDIV3_A::FIRCDIV3_4,
      5 => FIRCDIV3_A::FIRCDIV3_5,
      6 => FIRCDIV3_A::FIRCDIV3_6,
      7 => FIRCDIV3_A::FIRCDIV3_7,
      _ => unreachable!(),
    }
  }
  #[doc = "Checks if the value of the field is `FIRCDIV3_0`"]
  #[inline(always)]
  pub fn is_fircdiv3_0(&self) -> bool {
    *self == FIRCDIV3_A::FIRCDIV3_0
  }
  #[doc = "Checks if the value of the field is `FIRCDIV3_1`"]
  #[inline(always)]
  pub fn is_fircdiv3_1(&self) -> bool {
    *self == FIRCDIV3_A::FIRCDIV3_1
  }
  #[doc = "Checks if the value of the field is `FIRCDIV3_2`"]
  #[inline(always)]
  pub fn is_fircdiv3_2(&self) -> bool {
    *self == FIRCDIV3_A::FIRCDIV3_2
  }
  #[doc = "Checks if the value of the field is `FIRCDIV3_3`"]
  #[inline(always)]
  pub fn is_fircdiv3_3(&self) -> bool {
    *self == FIRCDIV3_A::FIRCDIV3_3
  }
  #[doc = "Checks if the value of the field is `FIRCDIV3_4`"]
  #[inline(always)]
  pub fn is_fircdiv3_4(&self) -> bool {
    *self == FIRCDIV3_A::FIRCDIV3_4
  }
  #[doc = "Checks if the value of the field is `FIRCDIV3_5`"]
  #[inline(always)]
  pub fn is_fircdiv3_5(&self) -> bool {
    *self == FIRCDIV3_A::FIRCDIV3_5
  }
  #[doc = "Checks if the value of the field is `FIRCDIV3_6`"]
  #[inline(always)]
  pub fn is_fircdiv3_6(&self) -> bool {
    *self == FIRCDIV3_A::FIRCDIV3_6
  }
  #[doc = "Checks if the value of the field is `FIRCDIV3_7`"]
  #[inline(always)]
  pub fn is_fircdiv3_7(&self) -> bool {
    *self == FIRCDIV3_A::FIRCDIV3_7
  }
}
#[doc = "Write proxy for field `FIRCDIV3`"]
pub struct FIRCDIV3_W<'a> {
  w: &'a mut W,
}
impl<'a> FIRCDIV3_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: FIRCDIV3_A) -> &'a mut W {
    {
      self.bits(variant.into())
    }
  }
  #[doc = "Clock disabled"]
  #[inline(always)]
  pub fn fircdiv3_0(self) -> &'a mut W {
    self.variant(FIRCDIV3_A::FIRCDIV3_0)
  }
  #[doc = "Divide by 1"]
  #[inline(always)]
  pub fn fircdiv3_1(self) -> &'a mut W {
    self.variant(FIRCDIV3_A::FIRCDIV3_1)
  }
  #[doc = "Divide by 2"]
  #[inline(always)]
  pub fn fircdiv3_2(self) -> &'a mut W {
    self.variant(FIRCDIV3_A::FIRCDIV3_2)
  }
  #[doc = "Divide by 4"]
  #[inline(always)]
  pub fn fircdiv3_3(self) -> &'a mut W {
    self.variant(FIRCDIV3_A::FIRCDIV3_3)
  }
  #[doc = "Divide by 8"]
  #[inline(always)]
  pub fn fircdiv3_4(self) -> &'a mut W {
    self.variant(FIRCDIV3_A::FIRCDIV3_4)
  }
  #[doc = "Divide by 16"]
  #[inline(always)]
  pub fn fircdiv3_5(self) -> &'a mut W {
    self.variant(FIRCDIV3_A::FIRCDIV3_5)
  }
  #[doc = "Divide by 32"]
  #[inline(always)]
  pub fn fircdiv3_6(self) -> &'a mut W {
    self.variant(FIRCDIV3_A::FIRCDIV3_6)
  }
  #[doc = "Divide by 64"]
  #[inline(always)]
  pub fn fircdiv3_7(self) -> &'a mut W {
    self.variant(FIRCDIV3_A::FIRCDIV3_7)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
    self.w
  }
}
impl R {
  #[doc = "Bits 0:2 - Fast IRC Clock Divide 1"]
  #[inline(always)]
  pub fn fircdiv1(&self) -> FIRCDIV1_R {
    FIRCDIV1_R::new((self.bits & 0x07) as u8)
  }
  #[doc = "Bits 8:10 - Fast IRC Clock Divide 2"]
  #[inline(always)]
  pub fn fircdiv2(&self) -> FIRCDIV2_R {
    FIRCDIV2_R::new(((self.bits >> 8) & 0x07) as u8)
  }
  #[doc = "Bits 16:18 - Fast IRC Clock Divider 3"]
  #[inline(always)]
  pub fn fircdiv3(&self) -> FIRCDIV3_R {
    FIRCDIV3_R::new(((self.bits >> 16) & 0x07) as u8)
  }
}
impl W {
  #[doc = "Bits 0:2 - Fast IRC Clock Divide 1"]
  #[inline(always)]
  pub fn fircdiv1(&mut self) -> FIRCDIV1_W {
    FIRCDIV1_W { w: self }
  }
  #[doc = "Bits 8:10 - Fast IRC Clock Divide 2"]
  #[inline(always)]
  pub fn fircdiv2(&mut self) -> FIRCDIV2_W {
    FIRCDIV2_W { w: self }
  }
  #[doc = "Bits 16:18 - Fast IRC Clock Divider 3"]
  #[inline(always)]
  pub fn fircdiv3(&mut self) -> FIRCDIV3_W {
    FIRCDIV3_W { w: self }
  }
}
