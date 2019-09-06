#[doc = "Reader of register CMDL10"]
pub type R = crate::R<u32, super::CMDL10>;
#[doc = "Writer for register CMDL10"]
pub type W = crate::W<u32, super::CMDL10>;
#[doc = "Register CMDL10 `reset()`'s with value 0"]
impl crate::ResetValue for super::CMDL10 {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "Input channel select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCH_A {
  #[doc = "0: Select CH0A or CH0B"]
  ADCH_0,
  #[doc = "1: Select CH1A or CH1B"]
  ADCH_1,
  #[doc = "2: Select CH2A or CH2B"]
  ADCH_2,
  #[doc = "3: Select CH3A or CH3B"]
  ADCH_3,
  #[doc = "4: Select corresponding channel CHnA or CHnB"]
  ADCH_4,
  #[doc = "5: Select corresponding channel CHnA or CHnB"]
  ADCH_5,
  #[doc = "6: Select corresponding channel CHnA or CHnB"]
  ADCH_6,
  #[doc = "7: Select corresponding channel CHnA or CHnB"]
  ADCH_7,
  #[doc = "8: Select corresponding channel CHnA or CHnB"]
  ADCH_8,
  #[doc = "9: Select corresponding channel CHnA or CHnB"]
  ADCH_9,
  #[doc = "30: Select CH30A or CH30B"]
  ADCH_30,
  #[doc = "31: Select CH31A or CH31B"]
  ADCH_31,
}
impl From<ADCH_A> for u8 {
  #[inline(always)]
  fn from(variant: ADCH_A) -> Self {
    match variant {
      ADCH_A::ADCH_0 => 0,
      ADCH_A::ADCH_1 => 1,
      ADCH_A::ADCH_2 => 2,
      ADCH_A::ADCH_3 => 3,
      ADCH_A::ADCH_4 => 4,
      ADCH_A::ADCH_5 => 5,
      ADCH_A::ADCH_6 => 6,
      ADCH_A::ADCH_7 => 7,
      ADCH_A::ADCH_8 => 8,
      ADCH_A::ADCH_9 => 9,
      ADCH_A::ADCH_30 => 30,
      ADCH_A::ADCH_31 => 31,
    }
  }
}
#[doc = "Reader of field `ADCH`"]
pub type ADCH_R = crate::R<u8, ADCH_A>;
impl ADCH_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> crate::Variant<u8, ADCH_A> {
    use crate::Variant::*;
    match self.bits {
      0 => Val(ADCH_A::ADCH_0),
      1 => Val(ADCH_A::ADCH_1),
      2 => Val(ADCH_A::ADCH_2),
      3 => Val(ADCH_A::ADCH_3),
      4 => Val(ADCH_A::ADCH_4),
      5 => Val(ADCH_A::ADCH_5),
      6 => Val(ADCH_A::ADCH_6),
      7 => Val(ADCH_A::ADCH_7),
      8 => Val(ADCH_A::ADCH_8),
      9 => Val(ADCH_A::ADCH_9),
      30 => Val(ADCH_A::ADCH_30),
      31 => Val(ADCH_A::ADCH_31),
      i => Res(i),
    }
  }
  #[doc = "Checks if the value of the field is `ADCH_0`"]
  #[inline(always)]
  pub fn is_adch_0(&self) -> bool {
    *self == ADCH_A::ADCH_0
  }
  #[doc = "Checks if the value of the field is `ADCH_1`"]
  #[inline(always)]
  pub fn is_adch_1(&self) -> bool {
    *self == ADCH_A::ADCH_1
  }
  #[doc = "Checks if the value of the field is `ADCH_2`"]
  #[inline(always)]
  pub fn is_adch_2(&self) -> bool {
    *self == ADCH_A::ADCH_2
  }
  #[doc = "Checks if the value of the field is `ADCH_3`"]
  #[inline(always)]
  pub fn is_adch_3(&self) -> bool {
    *self == ADCH_A::ADCH_3
  }
  #[doc = "Checks if the value of the field is `ADCH_4`"]
  #[inline(always)]
  pub fn is_adch_4(&self) -> bool {
    *self == ADCH_A::ADCH_4
  }
  #[doc = "Checks if the value of the field is `ADCH_5`"]
  #[inline(always)]
  pub fn is_adch_5(&self) -> bool {
    *self == ADCH_A::ADCH_5
  }
  #[doc = "Checks if the value of the field is `ADCH_6`"]
  #[inline(always)]
  pub fn is_adch_6(&self) -> bool {
    *self == ADCH_A::ADCH_6
  }
  #[doc = "Checks if the value of the field is `ADCH_7`"]
  #[inline(always)]
  pub fn is_adch_7(&self) -> bool {
    *self == ADCH_A::ADCH_7
  }
  #[doc = "Checks if the value of the field is `ADCH_8`"]
  #[inline(always)]
  pub fn is_adch_8(&self) -> bool {
    *self == ADCH_A::ADCH_8
  }
  #[doc = "Checks if the value of the field is `ADCH_9`"]
  #[inline(always)]
  pub fn is_adch_9(&self) -> bool {
    *self == ADCH_A::ADCH_9
  }
  #[doc = "Checks if the value of the field is `ADCH_30`"]
  #[inline(always)]
  pub fn is_adch_30(&self) -> bool {
    *self == ADCH_A::ADCH_30
  }
  #[doc = "Checks if the value of the field is `ADCH_31`"]
  #[inline(always)]
  pub fn is_adch_31(&self) -> bool {
    *self == ADCH_A::ADCH_31
  }
}
#[doc = "Write proxy for field `ADCH`"]
pub struct ADCH_W<'a> {
  w: &'a mut W,
}
impl<'a> ADCH_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: ADCH_A) -> &'a mut W {
    unsafe { self.bits(variant.into()) }
  }
  #[doc = "Select CH0A or CH0B"]
  #[inline(always)]
  pub fn adch_0(self) -> &'a mut W {
    self.variant(ADCH_A::ADCH_0)
  }
  #[doc = "Select CH1A or CH1B"]
  #[inline(always)]
  pub fn adch_1(self) -> &'a mut W {
    self.variant(ADCH_A::ADCH_1)
  }
  #[doc = "Select CH2A or CH2B"]
  #[inline(always)]
  pub fn adch_2(self) -> &'a mut W {
    self.variant(ADCH_A::ADCH_2)
  }
  #[doc = "Select CH3A or CH3B"]
  #[inline(always)]
  pub fn adch_3(self) -> &'a mut W {
    self.variant(ADCH_A::ADCH_3)
  }
  #[doc = "Select corresponding channel CHnA or CHnB"]
  #[inline(always)]
  pub fn adch_4(self) -> &'a mut W {
    self.variant(ADCH_A::ADCH_4)
  }
  #[doc = "Select corresponding channel CHnA or CHnB"]
  #[inline(always)]
  pub fn adch_5(self) -> &'a mut W {
    self.variant(ADCH_A::ADCH_5)
  }
  #[doc = "Select corresponding channel CHnA or CHnB"]
  #[inline(always)]
  pub fn adch_6(self) -> &'a mut W {
    self.variant(ADCH_A::ADCH_6)
  }
  #[doc = "Select corresponding channel CHnA or CHnB"]
  #[inline(always)]
  pub fn adch_7(self) -> &'a mut W {
    self.variant(ADCH_A::ADCH_7)
  }
  #[doc = "Select corresponding channel CHnA or CHnB"]
  #[inline(always)]
  pub fn adch_8(self) -> &'a mut W {
    self.variant(ADCH_A::ADCH_8)
  }
  #[doc = "Select corresponding channel CHnA or CHnB"]
  #[inline(always)]
  pub fn adch_9(self) -> &'a mut W {
    self.variant(ADCH_A::ADCH_9)
  }
  #[doc = "Select CH30A or CH30B"]
  #[inline(always)]
  pub fn adch_30(self) -> &'a mut W {
    self.variant(ADCH_A::ADCH_30)
  }
  #[doc = "Select CH31A or CH31B"]
  #[inline(always)]
  pub fn adch_31(self) -> &'a mut W {
    self.variant(ADCH_A::ADCH_31)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
    self.w
  }
}
#[doc = "A-side vs. B-side Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ABSEL_A {
  #[doc = "0: The associated A-side channel is converted."]
  ABSEL_0,
  #[doc = "1: The associated B-side channel is converted."]
  ABSEL_1,
}
impl From<ABSEL_A> for bool {
  #[inline(always)]
  fn from(variant: ABSEL_A) -> Self {
    match variant {
      ABSEL_A::ABSEL_0 => false,
      ABSEL_A::ABSEL_1 => true,
    }
  }
}
#[doc = "Reader of field `ABSEL`"]
pub type ABSEL_R = crate::R<bool, ABSEL_A>;
impl ABSEL_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> ABSEL_A {
    match self.bits {
      false => ABSEL_A::ABSEL_0,
      true => ABSEL_A::ABSEL_1,
    }
  }
  #[doc = "Checks if the value of the field is `ABSEL_0`"]
  #[inline(always)]
  pub fn is_absel_0(&self) -> bool {
    *self == ABSEL_A::ABSEL_0
  }
  #[doc = "Checks if the value of the field is `ABSEL_1`"]
  #[inline(always)]
  pub fn is_absel_1(&self) -> bool {
    *self == ABSEL_A::ABSEL_1
  }
}
#[doc = "Write proxy for field `ABSEL`"]
pub struct ABSEL_W<'a> {
  w: &'a mut W,
}
impl<'a> ABSEL_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: ABSEL_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "The associated A-side channel is converted."]
  #[inline(always)]
  pub fn absel_0(self) -> &'a mut W {
    self.variant(ABSEL_A::ABSEL_0)
  }
  #[doc = "The associated B-side channel is converted."]
  #[inline(always)]
  pub fn absel_1(self) -> &'a mut W {
    self.variant(ABSEL_A::ABSEL_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
    self.w
  }
}
impl R {
  #[doc = "Bits 0:4 - Input channel select"]
  #[inline(always)]
  pub fn adch(&self) -> ADCH_R {
    ADCH_R::new((self.bits & 0x1f) as u8)
  }
  #[doc = "Bit 5 - A-side vs. B-side Select"]
  #[inline(always)]
  pub fn absel(&self) -> ABSEL_R {
    ABSEL_R::new(((self.bits >> 5) & 0x01) != 0)
  }
}
impl W {
  #[doc = "Bits 0:4 - Input channel select"]
  #[inline(always)]
  pub fn adch(&mut self) -> ADCH_W {
    ADCH_W { w: self }
  }
  #[doc = "Bit 5 - A-side vs. B-side Select"]
  #[inline(always)]
  pub fn absel(&mut self) -> ABSEL_W {
    ABSEL_W { w: self }
  }
}
