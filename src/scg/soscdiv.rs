#[doc = "Reader of register SOSCDIV"]
pub type R = crate::R<u32, super::SOSCDIV>;
#[doc = "Writer for register SOSCDIV"]
pub type W = crate::W<u32, super::SOSCDIV>;
#[doc = "Register SOSCDIV `reset()`'s with value 0"]
impl crate::ResetValue for super::SOSCDIV {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "System OSC Clock Divide 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOSCDIV1_A {
  #[doc = "0: Output disabled"]
  SOSCDIV1_0,
  #[doc = "1: Divide by 1"]
  SOSCDIV1_1,
  #[doc = "2: Divide by 2"]
  SOSCDIV1_2,
  #[doc = "3: Divide by 4"]
  SOSCDIV1_3,
  #[doc = "4: Divide by 8"]
  SOSCDIV1_4,
  #[doc = "5: Divide by 16"]
  SOSCDIV1_5,
  #[doc = "6: Divide by 32"]
  SOSCDIV1_6,
  #[doc = "7: Divide by 64"]
  SOSCDIV1_7,
}
impl From<SOSCDIV1_A> for u8 {
  #[inline(always)]
  fn from(variant: SOSCDIV1_A) -> Self {
    match variant {
      SOSCDIV1_A::SOSCDIV1_0 => 0,
      SOSCDIV1_A::SOSCDIV1_1 => 1,
      SOSCDIV1_A::SOSCDIV1_2 => 2,
      SOSCDIV1_A::SOSCDIV1_3 => 3,
      SOSCDIV1_A::SOSCDIV1_4 => 4,
      SOSCDIV1_A::SOSCDIV1_5 => 5,
      SOSCDIV1_A::SOSCDIV1_6 => 6,
      SOSCDIV1_A::SOSCDIV1_7 => 7,
    }
  }
}
#[doc = "Reader of field `SOSCDIV1`"]
pub type SOSCDIV1_R = crate::R<u8, SOSCDIV1_A>;
impl SOSCDIV1_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> SOSCDIV1_A {
    match self.bits {
      0 => SOSCDIV1_A::SOSCDIV1_0,
      1 => SOSCDIV1_A::SOSCDIV1_1,
      2 => SOSCDIV1_A::SOSCDIV1_2,
      3 => SOSCDIV1_A::SOSCDIV1_3,
      4 => SOSCDIV1_A::SOSCDIV1_4,
      5 => SOSCDIV1_A::SOSCDIV1_5,
      6 => SOSCDIV1_A::SOSCDIV1_6,
      7 => SOSCDIV1_A::SOSCDIV1_7,
      _ => unreachable!(),
    }
  }
  #[doc = "Checks if the value of the field is `SOSCDIV1_0`"]
  #[inline(always)]
  pub fn is_soscdiv1_0(&self) -> bool {
    *self == SOSCDIV1_A::SOSCDIV1_0
  }
  #[doc = "Checks if the value of the field is `SOSCDIV1_1`"]
  #[inline(always)]
  pub fn is_soscdiv1_1(&self) -> bool {
    *self == SOSCDIV1_A::SOSCDIV1_1
  }
  #[doc = "Checks if the value of the field is `SOSCDIV1_2`"]
  #[inline(always)]
  pub fn is_soscdiv1_2(&self) -> bool {
    *self == SOSCDIV1_A::SOSCDIV1_2
  }
  #[doc = "Checks if the value of the field is `SOSCDIV1_3`"]
  #[inline(always)]
  pub fn is_soscdiv1_3(&self) -> bool {
    *self == SOSCDIV1_A::SOSCDIV1_3
  }
  #[doc = "Checks if the value of the field is `SOSCDIV1_4`"]
  #[inline(always)]
  pub fn is_soscdiv1_4(&self) -> bool {
    *self == SOSCDIV1_A::SOSCDIV1_4
  }
  #[doc = "Checks if the value of the field is `SOSCDIV1_5`"]
  #[inline(always)]
  pub fn is_soscdiv1_5(&self) -> bool {
    *self == SOSCDIV1_A::SOSCDIV1_5
  }
  #[doc = "Checks if the value of the field is `SOSCDIV1_6`"]
  #[inline(always)]
  pub fn is_soscdiv1_6(&self) -> bool {
    *self == SOSCDIV1_A::SOSCDIV1_6
  }
  #[doc = "Checks if the value of the field is `SOSCDIV1_7`"]
  #[inline(always)]
  pub fn is_soscdiv1_7(&self) -> bool {
    *self == SOSCDIV1_A::SOSCDIV1_7
  }
}
#[doc = "Write proxy for field `SOSCDIV1`"]
pub struct SOSCDIV1_W<'a> {
  w: &'a mut W,
}
impl<'a> SOSCDIV1_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: SOSCDIV1_A) -> &'a mut W {
    {
      self.bits(variant.into())
    }
  }
  #[doc = "Output disabled"]
  #[inline(always)]
  pub fn soscdiv1_0(self) -> &'a mut W {
    self.variant(SOSCDIV1_A::SOSCDIV1_0)
  }
  #[doc = "Divide by 1"]
  #[inline(always)]
  pub fn soscdiv1_1(self) -> &'a mut W {
    self.variant(SOSCDIV1_A::SOSCDIV1_1)
  }
  #[doc = "Divide by 2"]
  #[inline(always)]
  pub fn soscdiv1_2(self) -> &'a mut W {
    self.variant(SOSCDIV1_A::SOSCDIV1_2)
  }
  #[doc = "Divide by 4"]
  #[inline(always)]
  pub fn soscdiv1_3(self) -> &'a mut W {
    self.variant(SOSCDIV1_A::SOSCDIV1_3)
  }
  #[doc = "Divide by 8"]
  #[inline(always)]
  pub fn soscdiv1_4(self) -> &'a mut W {
    self.variant(SOSCDIV1_A::SOSCDIV1_4)
  }
  #[doc = "Divide by 16"]
  #[inline(always)]
  pub fn soscdiv1_5(self) -> &'a mut W {
    self.variant(SOSCDIV1_A::SOSCDIV1_5)
  }
  #[doc = "Divide by 32"]
  #[inline(always)]
  pub fn soscdiv1_6(self) -> &'a mut W {
    self.variant(SOSCDIV1_A::SOSCDIV1_6)
  }
  #[doc = "Divide by 64"]
  #[inline(always)]
  pub fn soscdiv1_7(self) -> &'a mut W {
    self.variant(SOSCDIV1_A::SOSCDIV1_7)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
    self.w
  }
}
#[doc = "System OSC Clock Divide 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOSCDIV2_A {
  #[doc = "0: Output disabled"]
  SOSCDIV2_0,
  #[doc = "1: Divide by 1"]
  SOSCDIV2_1,
  #[doc = "2: Divide by 2"]
  SOSCDIV2_2,
  #[doc = "3: Divide by 4"]
  SOSCDIV2_3,
  #[doc = "4: Divide by 8"]
  SOSCDIV2_4,
  #[doc = "5: Divide by 16"]
  SOSCDIV2_5,
  #[doc = "6: Divide by 32"]
  SOSCDIV2_6,
  #[doc = "7: Divide by 64"]
  SOSCDIV2_7,
}
impl From<SOSCDIV2_A> for u8 {
  #[inline(always)]
  fn from(variant: SOSCDIV2_A) -> Self {
    match variant {
      SOSCDIV2_A::SOSCDIV2_0 => 0,
      SOSCDIV2_A::SOSCDIV2_1 => 1,
      SOSCDIV2_A::SOSCDIV2_2 => 2,
      SOSCDIV2_A::SOSCDIV2_3 => 3,
      SOSCDIV2_A::SOSCDIV2_4 => 4,
      SOSCDIV2_A::SOSCDIV2_5 => 5,
      SOSCDIV2_A::SOSCDIV2_6 => 6,
      SOSCDIV2_A::SOSCDIV2_7 => 7,
    }
  }
}
#[doc = "Reader of field `SOSCDIV2`"]
pub type SOSCDIV2_R = crate::R<u8, SOSCDIV2_A>;
impl SOSCDIV2_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> SOSCDIV2_A {
    match self.bits {
      0 => SOSCDIV2_A::SOSCDIV2_0,
      1 => SOSCDIV2_A::SOSCDIV2_1,
      2 => SOSCDIV2_A::SOSCDIV2_2,
      3 => SOSCDIV2_A::SOSCDIV2_3,
      4 => SOSCDIV2_A::SOSCDIV2_4,
      5 => SOSCDIV2_A::SOSCDIV2_5,
      6 => SOSCDIV2_A::SOSCDIV2_6,
      7 => SOSCDIV2_A::SOSCDIV2_7,
      _ => unreachable!(),
    }
  }
  #[doc = "Checks if the value of the field is `SOSCDIV2_0`"]
  #[inline(always)]
  pub fn is_soscdiv2_0(&self) -> bool {
    *self == SOSCDIV2_A::SOSCDIV2_0
  }
  #[doc = "Checks if the value of the field is `SOSCDIV2_1`"]
  #[inline(always)]
  pub fn is_soscdiv2_1(&self) -> bool {
    *self == SOSCDIV2_A::SOSCDIV2_1
  }
  #[doc = "Checks if the value of the field is `SOSCDIV2_2`"]
  #[inline(always)]
  pub fn is_soscdiv2_2(&self) -> bool {
    *self == SOSCDIV2_A::SOSCDIV2_2
  }
  #[doc = "Checks if the value of the field is `SOSCDIV2_3`"]
  #[inline(always)]
  pub fn is_soscdiv2_3(&self) -> bool {
    *self == SOSCDIV2_A::SOSCDIV2_3
  }
  #[doc = "Checks if the value of the field is `SOSCDIV2_4`"]
  #[inline(always)]
  pub fn is_soscdiv2_4(&self) -> bool {
    *self == SOSCDIV2_A::SOSCDIV2_4
  }
  #[doc = "Checks if the value of the field is `SOSCDIV2_5`"]
  #[inline(always)]
  pub fn is_soscdiv2_5(&self) -> bool {
    *self == SOSCDIV2_A::SOSCDIV2_5
  }
  #[doc = "Checks if the value of the field is `SOSCDIV2_6`"]
  #[inline(always)]
  pub fn is_soscdiv2_6(&self) -> bool {
    *self == SOSCDIV2_A::SOSCDIV2_6
  }
  #[doc = "Checks if the value of the field is `SOSCDIV2_7`"]
  #[inline(always)]
  pub fn is_soscdiv2_7(&self) -> bool {
    *self == SOSCDIV2_A::SOSCDIV2_7
  }
}
#[doc = "Write proxy for field `SOSCDIV2`"]
pub struct SOSCDIV2_W<'a> {
  w: &'a mut W,
}
impl<'a> SOSCDIV2_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: SOSCDIV2_A) -> &'a mut W {
    {
      self.bits(variant.into())
    }
  }
  #[doc = "Output disabled"]
  #[inline(always)]
  pub fn soscdiv2_0(self) -> &'a mut W {
    self.variant(SOSCDIV2_A::SOSCDIV2_0)
  }
  #[doc = "Divide by 1"]
  #[inline(always)]
  pub fn soscdiv2_1(self) -> &'a mut W {
    self.variant(SOSCDIV2_A::SOSCDIV2_1)
  }
  #[doc = "Divide by 2"]
  #[inline(always)]
  pub fn soscdiv2_2(self) -> &'a mut W {
    self.variant(SOSCDIV2_A::SOSCDIV2_2)
  }
  #[doc = "Divide by 4"]
  #[inline(always)]
  pub fn soscdiv2_3(self) -> &'a mut W {
    self.variant(SOSCDIV2_A::SOSCDIV2_3)
  }
  #[doc = "Divide by 8"]
  #[inline(always)]
  pub fn soscdiv2_4(self) -> &'a mut W {
    self.variant(SOSCDIV2_A::SOSCDIV2_4)
  }
  #[doc = "Divide by 16"]
  #[inline(always)]
  pub fn soscdiv2_5(self) -> &'a mut W {
    self.variant(SOSCDIV2_A::SOSCDIV2_5)
  }
  #[doc = "Divide by 32"]
  #[inline(always)]
  pub fn soscdiv2_6(self) -> &'a mut W {
    self.variant(SOSCDIV2_A::SOSCDIV2_6)
  }
  #[doc = "Divide by 64"]
  #[inline(always)]
  pub fn soscdiv2_7(self) -> &'a mut W {
    self.variant(SOSCDIV2_A::SOSCDIV2_7)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
    self.w
  }
}
#[doc = "System OSC Clock Divide 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOSCDIV3_A {
  #[doc = "0: Output disabled"]
  SOSCDIV3_0,
  #[doc = "1: Divide by 1"]
  SOSCDIV3_1,
  #[doc = "2: Divide by 2"]
  SOSCDIV3_2,
  #[doc = "3: Divide by 4"]
  SOSCDIV3_3,
  #[doc = "4: Divide by 8"]
  SOSCDIV3_4,
  #[doc = "5: Divide by 16"]
  SOSCDIV3_5,
  #[doc = "6: Divide by 32"]
  SOSCDIV3_6,
  #[doc = "7: Divide by 64"]
  SOSCDIV3_7,
}
impl From<SOSCDIV3_A> for u8 {
  #[inline(always)]
  fn from(variant: SOSCDIV3_A) -> Self {
    match variant {
      SOSCDIV3_A::SOSCDIV3_0 => 0,
      SOSCDIV3_A::SOSCDIV3_1 => 1,
      SOSCDIV3_A::SOSCDIV3_2 => 2,
      SOSCDIV3_A::SOSCDIV3_3 => 3,
      SOSCDIV3_A::SOSCDIV3_4 => 4,
      SOSCDIV3_A::SOSCDIV3_5 => 5,
      SOSCDIV3_A::SOSCDIV3_6 => 6,
      SOSCDIV3_A::SOSCDIV3_7 => 7,
    }
  }
}
#[doc = "Reader of field `SOSCDIV3`"]
pub type SOSCDIV3_R = crate::R<u8, SOSCDIV3_A>;
impl SOSCDIV3_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> SOSCDIV3_A {
    match self.bits {
      0 => SOSCDIV3_A::SOSCDIV3_0,
      1 => SOSCDIV3_A::SOSCDIV3_1,
      2 => SOSCDIV3_A::SOSCDIV3_2,
      3 => SOSCDIV3_A::SOSCDIV3_3,
      4 => SOSCDIV3_A::SOSCDIV3_4,
      5 => SOSCDIV3_A::SOSCDIV3_5,
      6 => SOSCDIV3_A::SOSCDIV3_6,
      7 => SOSCDIV3_A::SOSCDIV3_7,
      _ => unreachable!(),
    }
  }
  #[doc = "Checks if the value of the field is `SOSCDIV3_0`"]
  #[inline(always)]
  pub fn is_soscdiv3_0(&self) -> bool {
    *self == SOSCDIV3_A::SOSCDIV3_0
  }
  #[doc = "Checks if the value of the field is `SOSCDIV3_1`"]
  #[inline(always)]
  pub fn is_soscdiv3_1(&self) -> bool {
    *self == SOSCDIV3_A::SOSCDIV3_1
  }
  #[doc = "Checks if the value of the field is `SOSCDIV3_2`"]
  #[inline(always)]
  pub fn is_soscdiv3_2(&self) -> bool {
    *self == SOSCDIV3_A::SOSCDIV3_2
  }
  #[doc = "Checks if the value of the field is `SOSCDIV3_3`"]
  #[inline(always)]
  pub fn is_soscdiv3_3(&self) -> bool {
    *self == SOSCDIV3_A::SOSCDIV3_3
  }
  #[doc = "Checks if the value of the field is `SOSCDIV3_4`"]
  #[inline(always)]
  pub fn is_soscdiv3_4(&self) -> bool {
    *self == SOSCDIV3_A::SOSCDIV3_4
  }
  #[doc = "Checks if the value of the field is `SOSCDIV3_5`"]
  #[inline(always)]
  pub fn is_soscdiv3_5(&self) -> bool {
    *self == SOSCDIV3_A::SOSCDIV3_5
  }
  #[doc = "Checks if the value of the field is `SOSCDIV3_6`"]
  #[inline(always)]
  pub fn is_soscdiv3_6(&self) -> bool {
    *self == SOSCDIV3_A::SOSCDIV3_6
  }
  #[doc = "Checks if the value of the field is `SOSCDIV3_7`"]
  #[inline(always)]
  pub fn is_soscdiv3_7(&self) -> bool {
    *self == SOSCDIV3_A::SOSCDIV3_7
  }
}
#[doc = "Write proxy for field `SOSCDIV3`"]
pub struct SOSCDIV3_W<'a> {
  w: &'a mut W,
}
impl<'a> SOSCDIV3_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: SOSCDIV3_A) -> &'a mut W {
    {
      self.bits(variant.into())
    }
  }
  #[doc = "Output disabled"]
  #[inline(always)]
  pub fn soscdiv3_0(self) -> &'a mut W {
    self.variant(SOSCDIV3_A::SOSCDIV3_0)
  }
  #[doc = "Divide by 1"]
  #[inline(always)]
  pub fn soscdiv3_1(self) -> &'a mut W {
    self.variant(SOSCDIV3_A::SOSCDIV3_1)
  }
  #[doc = "Divide by 2"]
  #[inline(always)]
  pub fn soscdiv3_2(self) -> &'a mut W {
    self.variant(SOSCDIV3_A::SOSCDIV3_2)
  }
  #[doc = "Divide by 4"]
  #[inline(always)]
  pub fn soscdiv3_3(self) -> &'a mut W {
    self.variant(SOSCDIV3_A::SOSCDIV3_3)
  }
  #[doc = "Divide by 8"]
  #[inline(always)]
  pub fn soscdiv3_4(self) -> &'a mut W {
    self.variant(SOSCDIV3_A::SOSCDIV3_4)
  }
  #[doc = "Divide by 16"]
  #[inline(always)]
  pub fn soscdiv3_5(self) -> &'a mut W {
    self.variant(SOSCDIV3_A::SOSCDIV3_5)
  }
  #[doc = "Divide by 32"]
  #[inline(always)]
  pub fn soscdiv3_6(self) -> &'a mut W {
    self.variant(SOSCDIV3_A::SOSCDIV3_6)
  }
  #[doc = "Divide by 64"]
  #[inline(always)]
  pub fn soscdiv3_7(self) -> &'a mut W {
    self.variant(SOSCDIV3_A::SOSCDIV3_7)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
    self.w
  }
}
impl R {
  #[doc = "Bits 0:2 - System OSC Clock Divide 1"]
  #[inline(always)]
  pub fn soscdiv1(&self) -> SOSCDIV1_R {
    SOSCDIV1_R::new((self.bits & 0x07) as u8)
  }
  #[doc = "Bits 8:10 - System OSC Clock Divide 2"]
  #[inline(always)]
  pub fn soscdiv2(&self) -> SOSCDIV2_R {
    SOSCDIV2_R::new(((self.bits >> 8) & 0x07) as u8)
  }
  #[doc = "Bits 16:18 - System OSC Clock Divide 3"]
  #[inline(always)]
  pub fn soscdiv3(&self) -> SOSCDIV3_R {
    SOSCDIV3_R::new(((self.bits >> 16) & 0x07) as u8)
  }
}
impl W {
  #[doc = "Bits 0:2 - System OSC Clock Divide 1"]
  #[inline(always)]
  pub fn soscdiv1(&mut self) -> SOSCDIV1_W {
    SOSCDIV1_W { w: self }
  }
  #[doc = "Bits 8:10 - System OSC Clock Divide 2"]
  #[inline(always)]
  pub fn soscdiv2(&mut self) -> SOSCDIV2_W {
    SOSCDIV2_W { w: self }
  }
  #[doc = "Bits 16:18 - System OSC Clock Divide 3"]
  #[inline(always)]
  pub fn soscdiv3(&mut self) -> SOSCDIV3_W {
    SOSCDIV3_W { w: self }
  }
}
