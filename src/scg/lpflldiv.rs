#[doc = "Reader of register LPFLLDIV"]
pub type R = crate::R<u32, super::LPFLLDIV>;
#[doc = "Writer for register LPFLLDIV"]
pub type W = crate::W<u32, super::LPFLLDIV>;
#[doc = "Register LPFLLDIV `reset()`'s with value 0"]
impl crate::ResetValue for super::LPFLLDIV {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "LPFLL Clock Divide 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPFLLDIV1_A {
  #[doc = "0: Output disabled"]
  LPFLLDIV1_0,
  #[doc = "1: Divide by 1"]
  LPFLLDIV1_1,
  #[doc = "2: Divide by 2"]
  LPFLLDIV1_2,
  #[doc = "3: Divide by 4"]
  LPFLLDIV1_3,
  #[doc = "4: Divide by 8"]
  LPFLLDIV1_4,
  #[doc = "5: Divide by 16"]
  LPFLLDIV1_5,
  #[doc = "6: Divide by 32"]
  LPFLLDIV1_6,
  #[doc = "7: Divide by 64"]
  LPFLLDIV1_7,
}
impl From<LPFLLDIV1_A> for u8 {
  #[inline(always)]
  fn from(variant: LPFLLDIV1_A) -> Self {
    match variant {
      LPFLLDIV1_A::LPFLLDIV1_0 => 0,
      LPFLLDIV1_A::LPFLLDIV1_1 => 1,
      LPFLLDIV1_A::LPFLLDIV1_2 => 2,
      LPFLLDIV1_A::LPFLLDIV1_3 => 3,
      LPFLLDIV1_A::LPFLLDIV1_4 => 4,
      LPFLLDIV1_A::LPFLLDIV1_5 => 5,
      LPFLLDIV1_A::LPFLLDIV1_6 => 6,
      LPFLLDIV1_A::LPFLLDIV1_7 => 7,
    }
  }
}
#[doc = "Reader of field `LPFLLDIV1`"]
pub type LPFLLDIV1_R = crate::R<u8, LPFLLDIV1_A>;
impl LPFLLDIV1_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> LPFLLDIV1_A {
    match self.bits {
      0 => LPFLLDIV1_A::LPFLLDIV1_0,
      1 => LPFLLDIV1_A::LPFLLDIV1_1,
      2 => LPFLLDIV1_A::LPFLLDIV1_2,
      3 => LPFLLDIV1_A::LPFLLDIV1_3,
      4 => LPFLLDIV1_A::LPFLLDIV1_4,
      5 => LPFLLDIV1_A::LPFLLDIV1_5,
      6 => LPFLLDIV1_A::LPFLLDIV1_6,
      7 => LPFLLDIV1_A::LPFLLDIV1_7,
      _ => unreachable!(),
    }
  }
  #[doc = "Checks if the value of the field is `LPFLLDIV1_0`"]
  #[inline(always)]
  pub fn is_lpflldiv1_0(&self) -> bool {
    *self == LPFLLDIV1_A::LPFLLDIV1_0
  }
  #[doc = "Checks if the value of the field is `LPFLLDIV1_1`"]
  #[inline(always)]
  pub fn is_lpflldiv1_1(&self) -> bool {
    *self == LPFLLDIV1_A::LPFLLDIV1_1
  }
  #[doc = "Checks if the value of the field is `LPFLLDIV1_2`"]
  #[inline(always)]
  pub fn is_lpflldiv1_2(&self) -> bool {
    *self == LPFLLDIV1_A::LPFLLDIV1_2
  }
  #[doc = "Checks if the value of the field is `LPFLLDIV1_3`"]
  #[inline(always)]
  pub fn is_lpflldiv1_3(&self) -> bool {
    *self == LPFLLDIV1_A::LPFLLDIV1_3
  }
  #[doc = "Checks if the value of the field is `LPFLLDIV1_4`"]
  #[inline(always)]
  pub fn is_lpflldiv1_4(&self) -> bool {
    *self == LPFLLDIV1_A::LPFLLDIV1_4
  }
  #[doc = "Checks if the value of the field is `LPFLLDIV1_5`"]
  #[inline(always)]
  pub fn is_lpflldiv1_5(&self) -> bool {
    *self == LPFLLDIV1_A::LPFLLDIV1_5
  }
  #[doc = "Checks if the value of the field is `LPFLLDIV1_6`"]
  #[inline(always)]
  pub fn is_lpflldiv1_6(&self) -> bool {
    *self == LPFLLDIV1_A::LPFLLDIV1_6
  }
  #[doc = "Checks if the value of the field is `LPFLLDIV1_7`"]
  #[inline(always)]
  pub fn is_lpflldiv1_7(&self) -> bool {
    *self == LPFLLDIV1_A::LPFLLDIV1_7
  }
}
#[doc = "Write proxy for field `LPFLLDIV1`"]
pub struct LPFLLDIV1_W<'a> {
  w: &'a mut W,
}
impl<'a> LPFLLDIV1_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: LPFLLDIV1_A) -> &'a mut W {
    {
      self.bits(variant.into())
    }
  }
  #[doc = "Output disabled"]
  #[inline(always)]
  pub fn lpflldiv1_0(self) -> &'a mut W {
    self.variant(LPFLLDIV1_A::LPFLLDIV1_0)
  }
  #[doc = "Divide by 1"]
  #[inline(always)]
  pub fn lpflldiv1_1(self) -> &'a mut W {
    self.variant(LPFLLDIV1_A::LPFLLDIV1_1)
  }
  #[doc = "Divide by 2"]
  #[inline(always)]
  pub fn lpflldiv1_2(self) -> &'a mut W {
    self.variant(LPFLLDIV1_A::LPFLLDIV1_2)
  }
  #[doc = "Divide by 4"]
  #[inline(always)]
  pub fn lpflldiv1_3(self) -> &'a mut W {
    self.variant(LPFLLDIV1_A::LPFLLDIV1_3)
  }
  #[doc = "Divide by 8"]
  #[inline(always)]
  pub fn lpflldiv1_4(self) -> &'a mut W {
    self.variant(LPFLLDIV1_A::LPFLLDIV1_4)
  }
  #[doc = "Divide by 16"]
  #[inline(always)]
  pub fn lpflldiv1_5(self) -> &'a mut W {
    self.variant(LPFLLDIV1_A::LPFLLDIV1_5)
  }
  #[doc = "Divide by 32"]
  #[inline(always)]
  pub fn lpflldiv1_6(self) -> &'a mut W {
    self.variant(LPFLLDIV1_A::LPFLLDIV1_6)
  }
  #[doc = "Divide by 64"]
  #[inline(always)]
  pub fn lpflldiv1_7(self) -> &'a mut W {
    self.variant(LPFLLDIV1_A::LPFLLDIV1_7)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
    self.w
  }
}
#[doc = "LPFLL Clock Divide 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPFLLDIV2_A {
  #[doc = "0: Output disabled"]
  LPFLLDIV2_0,
  #[doc = "1: Divide by 1"]
  LPFLLDIV2_1,
  #[doc = "2: Divide by 2"]
  LPFLLDIV2_2,
  #[doc = "3: Divide by 4"]
  LPFLLDIV2_3,
  #[doc = "4: Divide by 8"]
  LPFLLDIV2_4,
  #[doc = "5: Divide by 16"]
  LPFLLDIV2_5,
  #[doc = "6: Divide by 32"]
  LPFLLDIV2_6,
  #[doc = "7: Divide by 64"]
  LPFLLDIV2_7,
}
impl From<LPFLLDIV2_A> for u8 {
  #[inline(always)]
  fn from(variant: LPFLLDIV2_A) -> Self {
    match variant {
      LPFLLDIV2_A::LPFLLDIV2_0 => 0,
      LPFLLDIV2_A::LPFLLDIV2_1 => 1,
      LPFLLDIV2_A::LPFLLDIV2_2 => 2,
      LPFLLDIV2_A::LPFLLDIV2_3 => 3,
      LPFLLDIV2_A::LPFLLDIV2_4 => 4,
      LPFLLDIV2_A::LPFLLDIV2_5 => 5,
      LPFLLDIV2_A::LPFLLDIV2_6 => 6,
      LPFLLDIV2_A::LPFLLDIV2_7 => 7,
    }
  }
}
#[doc = "Reader of field `LPFLLDIV2`"]
pub type LPFLLDIV2_R = crate::R<u8, LPFLLDIV2_A>;
impl LPFLLDIV2_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> LPFLLDIV2_A {
    match self.bits {
      0 => LPFLLDIV2_A::LPFLLDIV2_0,
      1 => LPFLLDIV2_A::LPFLLDIV2_1,
      2 => LPFLLDIV2_A::LPFLLDIV2_2,
      3 => LPFLLDIV2_A::LPFLLDIV2_3,
      4 => LPFLLDIV2_A::LPFLLDIV2_4,
      5 => LPFLLDIV2_A::LPFLLDIV2_5,
      6 => LPFLLDIV2_A::LPFLLDIV2_6,
      7 => LPFLLDIV2_A::LPFLLDIV2_7,
      _ => unreachable!(),
    }
  }
  #[doc = "Checks if the value of the field is `LPFLLDIV2_0`"]
  #[inline(always)]
  pub fn is_lpflldiv2_0(&self) -> bool {
    *self == LPFLLDIV2_A::LPFLLDIV2_0
  }
  #[doc = "Checks if the value of the field is `LPFLLDIV2_1`"]
  #[inline(always)]
  pub fn is_lpflldiv2_1(&self) -> bool {
    *self == LPFLLDIV2_A::LPFLLDIV2_1
  }
  #[doc = "Checks if the value of the field is `LPFLLDIV2_2`"]
  #[inline(always)]
  pub fn is_lpflldiv2_2(&self) -> bool {
    *self == LPFLLDIV2_A::LPFLLDIV2_2
  }
  #[doc = "Checks if the value of the field is `LPFLLDIV2_3`"]
  #[inline(always)]
  pub fn is_lpflldiv2_3(&self) -> bool {
    *self == LPFLLDIV2_A::LPFLLDIV2_3
  }
  #[doc = "Checks if the value of the field is `LPFLLDIV2_4`"]
  #[inline(always)]
  pub fn is_lpflldiv2_4(&self) -> bool {
    *self == LPFLLDIV2_A::LPFLLDIV2_4
  }
  #[doc = "Checks if the value of the field is `LPFLLDIV2_5`"]
  #[inline(always)]
  pub fn is_lpflldiv2_5(&self) -> bool {
    *self == LPFLLDIV2_A::LPFLLDIV2_5
  }
  #[doc = "Checks if the value of the field is `LPFLLDIV2_6`"]
  #[inline(always)]
  pub fn is_lpflldiv2_6(&self) -> bool {
    *self == LPFLLDIV2_A::LPFLLDIV2_6
  }
  #[doc = "Checks if the value of the field is `LPFLLDIV2_7`"]
  #[inline(always)]
  pub fn is_lpflldiv2_7(&self) -> bool {
    *self == LPFLLDIV2_A::LPFLLDIV2_7
  }
}
#[doc = "Write proxy for field `LPFLLDIV2`"]
pub struct LPFLLDIV2_W<'a> {
  w: &'a mut W,
}
impl<'a> LPFLLDIV2_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: LPFLLDIV2_A) -> &'a mut W {
    {
      self.bits(variant.into())
    }
  }
  #[doc = "Output disabled"]
  #[inline(always)]
  pub fn lpflldiv2_0(self) -> &'a mut W {
    self.variant(LPFLLDIV2_A::LPFLLDIV2_0)
  }
  #[doc = "Divide by 1"]
  #[inline(always)]
  pub fn lpflldiv2_1(self) -> &'a mut W {
    self.variant(LPFLLDIV2_A::LPFLLDIV2_1)
  }
  #[doc = "Divide by 2"]
  #[inline(always)]
  pub fn lpflldiv2_2(self) -> &'a mut W {
    self.variant(LPFLLDIV2_A::LPFLLDIV2_2)
  }
  #[doc = "Divide by 4"]
  #[inline(always)]
  pub fn lpflldiv2_3(self) -> &'a mut W {
    self.variant(LPFLLDIV2_A::LPFLLDIV2_3)
  }
  #[doc = "Divide by 8"]
  #[inline(always)]
  pub fn lpflldiv2_4(self) -> &'a mut W {
    self.variant(LPFLLDIV2_A::LPFLLDIV2_4)
  }
  #[doc = "Divide by 16"]
  #[inline(always)]
  pub fn lpflldiv2_5(self) -> &'a mut W {
    self.variant(LPFLLDIV2_A::LPFLLDIV2_5)
  }
  #[doc = "Divide by 32"]
  #[inline(always)]
  pub fn lpflldiv2_6(self) -> &'a mut W {
    self.variant(LPFLLDIV2_A::LPFLLDIV2_6)
  }
  #[doc = "Divide by 64"]
  #[inline(always)]
  pub fn lpflldiv2_7(self) -> &'a mut W {
    self.variant(LPFLLDIV2_A::LPFLLDIV2_7)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
    self.w
  }
}
#[doc = "LPFLL Clock Divide 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPFLLDIV3_A {
  #[doc = "0: Clock disabled"]
  LPFLLDIV3_0,
  #[doc = "1: Divide by 1"]
  LPFLLDIV3_1,
  #[doc = "2: Divide by 2"]
  LPFLLDIV3_2,
  #[doc = "3: Divide by 4"]
  LPFLLDIV3_3,
  #[doc = "4: Divide by 8"]
  LPFLLDIV3_4,
  #[doc = "5: Divide by 16"]
  LPFLLDIV3_5,
  #[doc = "6: Divide by 32"]
  LPFLLDIV3_6,
  #[doc = "7: Divide by 64"]
  LPFLLDIV3_7,
}
impl From<LPFLLDIV3_A> for u8 {
  #[inline(always)]
  fn from(variant: LPFLLDIV3_A) -> Self {
    match variant {
      LPFLLDIV3_A::LPFLLDIV3_0 => 0,
      LPFLLDIV3_A::LPFLLDIV3_1 => 1,
      LPFLLDIV3_A::LPFLLDIV3_2 => 2,
      LPFLLDIV3_A::LPFLLDIV3_3 => 3,
      LPFLLDIV3_A::LPFLLDIV3_4 => 4,
      LPFLLDIV3_A::LPFLLDIV3_5 => 5,
      LPFLLDIV3_A::LPFLLDIV3_6 => 6,
      LPFLLDIV3_A::LPFLLDIV3_7 => 7,
    }
  }
}
#[doc = "Reader of field `LPFLLDIV3`"]
pub type LPFLLDIV3_R = crate::R<u8, LPFLLDIV3_A>;
impl LPFLLDIV3_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> LPFLLDIV3_A {
    match self.bits {
      0 => LPFLLDIV3_A::LPFLLDIV3_0,
      1 => LPFLLDIV3_A::LPFLLDIV3_1,
      2 => LPFLLDIV3_A::LPFLLDIV3_2,
      3 => LPFLLDIV3_A::LPFLLDIV3_3,
      4 => LPFLLDIV3_A::LPFLLDIV3_4,
      5 => LPFLLDIV3_A::LPFLLDIV3_5,
      6 => LPFLLDIV3_A::LPFLLDIV3_6,
      7 => LPFLLDIV3_A::LPFLLDIV3_7,
      _ => unreachable!(),
    }
  }
  #[doc = "Checks if the value of the field is `LPFLLDIV3_0`"]
  #[inline(always)]
  pub fn is_lpflldiv3_0(&self) -> bool {
    *self == LPFLLDIV3_A::LPFLLDIV3_0
  }
  #[doc = "Checks if the value of the field is `LPFLLDIV3_1`"]
  #[inline(always)]
  pub fn is_lpflldiv3_1(&self) -> bool {
    *self == LPFLLDIV3_A::LPFLLDIV3_1
  }
  #[doc = "Checks if the value of the field is `LPFLLDIV3_2`"]
  #[inline(always)]
  pub fn is_lpflldiv3_2(&self) -> bool {
    *self == LPFLLDIV3_A::LPFLLDIV3_2
  }
  #[doc = "Checks if the value of the field is `LPFLLDIV3_3`"]
  #[inline(always)]
  pub fn is_lpflldiv3_3(&self) -> bool {
    *self == LPFLLDIV3_A::LPFLLDIV3_3
  }
  #[doc = "Checks if the value of the field is `LPFLLDIV3_4`"]
  #[inline(always)]
  pub fn is_lpflldiv3_4(&self) -> bool {
    *self == LPFLLDIV3_A::LPFLLDIV3_4
  }
  #[doc = "Checks if the value of the field is `LPFLLDIV3_5`"]
  #[inline(always)]
  pub fn is_lpflldiv3_5(&self) -> bool {
    *self == LPFLLDIV3_A::LPFLLDIV3_5
  }
  #[doc = "Checks if the value of the field is `LPFLLDIV3_6`"]
  #[inline(always)]
  pub fn is_lpflldiv3_6(&self) -> bool {
    *self == LPFLLDIV3_A::LPFLLDIV3_6
  }
  #[doc = "Checks if the value of the field is `LPFLLDIV3_7`"]
  #[inline(always)]
  pub fn is_lpflldiv3_7(&self) -> bool {
    *self == LPFLLDIV3_A::LPFLLDIV3_7
  }
}
#[doc = "Write proxy for field `LPFLLDIV3`"]
pub struct LPFLLDIV3_W<'a> {
  w: &'a mut W,
}
impl<'a> LPFLLDIV3_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: LPFLLDIV3_A) -> &'a mut W {
    {
      self.bits(variant.into())
    }
  }
  #[doc = "Clock disabled"]
  #[inline(always)]
  pub fn lpflldiv3_0(self) -> &'a mut W {
    self.variant(LPFLLDIV3_A::LPFLLDIV3_0)
  }
  #[doc = "Divide by 1"]
  #[inline(always)]
  pub fn lpflldiv3_1(self) -> &'a mut W {
    self.variant(LPFLLDIV3_A::LPFLLDIV3_1)
  }
  #[doc = "Divide by 2"]
  #[inline(always)]
  pub fn lpflldiv3_2(self) -> &'a mut W {
    self.variant(LPFLLDIV3_A::LPFLLDIV3_2)
  }
  #[doc = "Divide by 4"]
  #[inline(always)]
  pub fn lpflldiv3_3(self) -> &'a mut W {
    self.variant(LPFLLDIV3_A::LPFLLDIV3_3)
  }
  #[doc = "Divide by 8"]
  #[inline(always)]
  pub fn lpflldiv3_4(self) -> &'a mut W {
    self.variant(LPFLLDIV3_A::LPFLLDIV3_4)
  }
  #[doc = "Divide by 16"]
  #[inline(always)]
  pub fn lpflldiv3_5(self) -> &'a mut W {
    self.variant(LPFLLDIV3_A::LPFLLDIV3_5)
  }
  #[doc = "Divide by 32"]
  #[inline(always)]
  pub fn lpflldiv3_6(self) -> &'a mut W {
    self.variant(LPFLLDIV3_A::LPFLLDIV3_6)
  }
  #[doc = "Divide by 64"]
  #[inline(always)]
  pub fn lpflldiv3_7(self) -> &'a mut W {
    self.variant(LPFLLDIV3_A::LPFLLDIV3_7)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
    self.w
  }
}
impl R {
  #[doc = "Bits 0:2 - LPFLL Clock Divide 1"]
  #[inline(always)]
  pub fn lpflldiv1(&self) -> LPFLLDIV1_R {
    LPFLLDIV1_R::new((self.bits & 0x07) as u8)
  }
  #[doc = "Bits 8:10 - LPFLL Clock Divide 2"]
  #[inline(always)]
  pub fn lpflldiv2(&self) -> LPFLLDIV2_R {
    LPFLLDIV2_R::new(((self.bits >> 8) & 0x07) as u8)
  }
  #[doc = "Bits 16:18 - LPFLL Clock Divide 3"]
  #[inline(always)]
  pub fn lpflldiv3(&self) -> LPFLLDIV3_R {
    LPFLLDIV3_R::new(((self.bits >> 16) & 0x07) as u8)
  }
}
impl W {
  #[doc = "Bits 0:2 - LPFLL Clock Divide 1"]
  #[inline(always)]
  pub fn lpflldiv1(&mut self) -> LPFLLDIV1_W {
    LPFLLDIV1_W { w: self }
  }
  #[doc = "Bits 8:10 - LPFLL Clock Divide 2"]
  #[inline(always)]
  pub fn lpflldiv2(&mut self) -> LPFLLDIV2_W {
    LPFLLDIV2_W { w: self }
  }
  #[doc = "Bits 16:18 - LPFLL Clock Divide 3"]
  #[inline(always)]
  pub fn lpflldiv3(&mut self) -> LPFLLDIV3_W {
    LPFLLDIV3_W { w: self }
  }
}
