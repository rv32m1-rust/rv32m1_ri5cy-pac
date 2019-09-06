#[doc = "Reader of register GATE%s"]
pub type R = crate::R<u8, super::GATE>;
#[doc = "Writer for register GATE%s"]
pub type W = crate::W<u8, super::GATE>;
#[doc = "Register GATE%s `reset()`'s with value 0"]
impl crate::ResetValue for super::GATE {
  type Type = u8;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "GTFSM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GTFSM_A {
  #[doc = "0: The gate is unlocked (free)."]
  GTFSM_0,
  #[doc = "1: The gate has been locked by processor 0."]
  GTFSM_1,
  #[doc = "2: The gate has been locked by processor 1."]
  GTFSM_2,
  #[doc = "3: The gate has been locked by processor 2."]
  GTFSM_3,
  #[doc = "4: The gate has been locked by processor 3."]
  GTFSM_4,
  #[doc = "5: The gate has been locked by processor 4."]
  GTFSM_5,
  #[doc = "6: The gate has been locked by processor 5."]
  GTFSM_6,
  #[doc = "7: The gate has been locked by processor 6."]
  GTFSM_7,
  #[doc = "8: The gate has been locked by processor 7."]
  GTFSM_8,
  #[doc = "9: The gate has been locked by processor 8."]
  GTFSM_9,
  #[doc = "10: The gate has been locked by processor 9."]
  GTFSM_10,
  #[doc = "11: The gate has been locked by processor 10."]
  GTFSM_11,
  #[doc = "12: The gate has been locked by processor 11."]
  GTFSM_12,
  #[doc = "13: The gate has been locked by processor 12."]
  GTFSM_13,
  #[doc = "14: The gate has been locked by processor 13."]
  GTFSM_14,
  #[doc = "15: The gate has been locked by processor 14."]
  GTFSM_15,
}
impl From<GTFSM_A> for u8 {
  #[inline(always)]
  fn from(variant: GTFSM_A) -> Self {
    match variant {
      GTFSM_A::GTFSM_0 => 0,
      GTFSM_A::GTFSM_1 => 1,
      GTFSM_A::GTFSM_2 => 2,
      GTFSM_A::GTFSM_3 => 3,
      GTFSM_A::GTFSM_4 => 4,
      GTFSM_A::GTFSM_5 => 5,
      GTFSM_A::GTFSM_6 => 6,
      GTFSM_A::GTFSM_7 => 7,
      GTFSM_A::GTFSM_8 => 8,
      GTFSM_A::GTFSM_9 => 9,
      GTFSM_A::GTFSM_10 => 10,
      GTFSM_A::GTFSM_11 => 11,
      GTFSM_A::GTFSM_12 => 12,
      GTFSM_A::GTFSM_13 => 13,
      GTFSM_A::GTFSM_14 => 14,
      GTFSM_A::GTFSM_15 => 15,
    }
  }
}
#[doc = "Reader of field `GTFSM`"]
pub type GTFSM_R = crate::R<u8, GTFSM_A>;
impl GTFSM_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> GTFSM_A {
    match self.bits {
      0 => GTFSM_A::GTFSM_0,
      1 => GTFSM_A::GTFSM_1,
      2 => GTFSM_A::GTFSM_2,
      3 => GTFSM_A::GTFSM_3,
      4 => GTFSM_A::GTFSM_4,
      5 => GTFSM_A::GTFSM_5,
      6 => GTFSM_A::GTFSM_6,
      7 => GTFSM_A::GTFSM_7,
      8 => GTFSM_A::GTFSM_8,
      9 => GTFSM_A::GTFSM_9,
      10 => GTFSM_A::GTFSM_10,
      11 => GTFSM_A::GTFSM_11,
      12 => GTFSM_A::GTFSM_12,
      13 => GTFSM_A::GTFSM_13,
      14 => GTFSM_A::GTFSM_14,
      15 => GTFSM_A::GTFSM_15,
      _ => unreachable!(),
    }
  }
  #[doc = "Checks if the value of the field is `GTFSM_0`"]
  #[inline(always)]
  pub fn is_gtfsm_0(&self) -> bool {
    *self == GTFSM_A::GTFSM_0
  }
  #[doc = "Checks if the value of the field is `GTFSM_1`"]
  #[inline(always)]
  pub fn is_gtfsm_1(&self) -> bool {
    *self == GTFSM_A::GTFSM_1
  }
  #[doc = "Checks if the value of the field is `GTFSM_2`"]
  #[inline(always)]
  pub fn is_gtfsm_2(&self) -> bool {
    *self == GTFSM_A::GTFSM_2
  }
  #[doc = "Checks if the value of the field is `GTFSM_3`"]
  #[inline(always)]
  pub fn is_gtfsm_3(&self) -> bool {
    *self == GTFSM_A::GTFSM_3
  }
  #[doc = "Checks if the value of the field is `GTFSM_4`"]
  #[inline(always)]
  pub fn is_gtfsm_4(&self) -> bool {
    *self == GTFSM_A::GTFSM_4
  }
  #[doc = "Checks if the value of the field is `GTFSM_5`"]
  #[inline(always)]
  pub fn is_gtfsm_5(&self) -> bool {
    *self == GTFSM_A::GTFSM_5
  }
  #[doc = "Checks if the value of the field is `GTFSM_6`"]
  #[inline(always)]
  pub fn is_gtfsm_6(&self) -> bool {
    *self == GTFSM_A::GTFSM_6
  }
  #[doc = "Checks if the value of the field is `GTFSM_7`"]
  #[inline(always)]
  pub fn is_gtfsm_7(&self) -> bool {
    *self == GTFSM_A::GTFSM_7
  }
  #[doc = "Checks if the value of the field is `GTFSM_8`"]
  #[inline(always)]
  pub fn is_gtfsm_8(&self) -> bool {
    *self == GTFSM_A::GTFSM_8
  }
  #[doc = "Checks if the value of the field is `GTFSM_9`"]
  #[inline(always)]
  pub fn is_gtfsm_9(&self) -> bool {
    *self == GTFSM_A::GTFSM_9
  }
  #[doc = "Checks if the value of the field is `GTFSM_10`"]
  #[inline(always)]
  pub fn is_gtfsm_10(&self) -> bool {
    *self == GTFSM_A::GTFSM_10
  }
  #[doc = "Checks if the value of the field is `GTFSM_11`"]
  #[inline(always)]
  pub fn is_gtfsm_11(&self) -> bool {
    *self == GTFSM_A::GTFSM_11
  }
  #[doc = "Checks if the value of the field is `GTFSM_12`"]
  #[inline(always)]
  pub fn is_gtfsm_12(&self) -> bool {
    *self == GTFSM_A::GTFSM_12
  }
  #[doc = "Checks if the value of the field is `GTFSM_13`"]
  #[inline(always)]
  pub fn is_gtfsm_13(&self) -> bool {
    *self == GTFSM_A::GTFSM_13
  }
  #[doc = "Checks if the value of the field is `GTFSM_14`"]
  #[inline(always)]
  pub fn is_gtfsm_14(&self) -> bool {
    *self == GTFSM_A::GTFSM_14
  }
  #[doc = "Checks if the value of the field is `GTFSM_15`"]
  #[inline(always)]
  pub fn is_gtfsm_15(&self) -> bool {
    *self == GTFSM_A::GTFSM_15
  }
}
#[doc = "Write proxy for field `GTFSM`"]
pub struct GTFSM_W<'a> {
  w: &'a mut W,
}
impl<'a> GTFSM_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: GTFSM_A) -> &'a mut W {
    {
      self.bits(variant.into())
    }
  }
  #[doc = "The gate is unlocked (free)."]
  #[inline(always)]
  pub fn gtfsm_0(self) -> &'a mut W {
    self.variant(GTFSM_A::GTFSM_0)
  }
  #[doc = "The gate has been locked by processor 0."]
  #[inline(always)]
  pub fn gtfsm_1(self) -> &'a mut W {
    self.variant(GTFSM_A::GTFSM_1)
  }
  #[doc = "The gate has been locked by processor 1."]
  #[inline(always)]
  pub fn gtfsm_2(self) -> &'a mut W {
    self.variant(GTFSM_A::GTFSM_2)
  }
  #[doc = "The gate has been locked by processor 2."]
  #[inline(always)]
  pub fn gtfsm_3(self) -> &'a mut W {
    self.variant(GTFSM_A::GTFSM_3)
  }
  #[doc = "The gate has been locked by processor 3."]
  #[inline(always)]
  pub fn gtfsm_4(self) -> &'a mut W {
    self.variant(GTFSM_A::GTFSM_4)
  }
  #[doc = "The gate has been locked by processor 4."]
  #[inline(always)]
  pub fn gtfsm_5(self) -> &'a mut W {
    self.variant(GTFSM_A::GTFSM_5)
  }
  #[doc = "The gate has been locked by processor 5."]
  #[inline(always)]
  pub fn gtfsm_6(self) -> &'a mut W {
    self.variant(GTFSM_A::GTFSM_6)
  }
  #[doc = "The gate has been locked by processor 6."]
  #[inline(always)]
  pub fn gtfsm_7(self) -> &'a mut W {
    self.variant(GTFSM_A::GTFSM_7)
  }
  #[doc = "The gate has been locked by processor 7."]
  #[inline(always)]
  pub fn gtfsm_8(self) -> &'a mut W {
    self.variant(GTFSM_A::GTFSM_8)
  }
  #[doc = "The gate has been locked by processor 8."]
  #[inline(always)]
  pub fn gtfsm_9(self) -> &'a mut W {
    self.variant(GTFSM_A::GTFSM_9)
  }
  #[doc = "The gate has been locked by processor 9."]
  #[inline(always)]
  pub fn gtfsm_10(self) -> &'a mut W {
    self.variant(GTFSM_A::GTFSM_10)
  }
  #[doc = "The gate has been locked by processor 10."]
  #[inline(always)]
  pub fn gtfsm_11(self) -> &'a mut W {
    self.variant(GTFSM_A::GTFSM_11)
  }
  #[doc = "The gate has been locked by processor 11."]
  #[inline(always)]
  pub fn gtfsm_12(self) -> &'a mut W {
    self.variant(GTFSM_A::GTFSM_12)
  }
  #[doc = "The gate has been locked by processor 12."]
  #[inline(always)]
  pub fn gtfsm_13(self) -> &'a mut W {
    self.variant(GTFSM_A::GTFSM_13)
  }
  #[doc = "The gate has been locked by processor 13."]
  #[inline(always)]
  pub fn gtfsm_14(self) -> &'a mut W {
    self.variant(GTFSM_A::GTFSM_14)
  }
  #[doc = "The gate has been locked by processor 14."]
  #[inline(always)]
  pub fn gtfsm_15(self) -> &'a mut W {
    self.variant(GTFSM_A::GTFSM_15)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !0x0f) | ((value as u8) & 0x0f);
    self.w
  }
}
impl R {
  #[doc = "Bits 0:3 - GTFSM"]
  #[inline(always)]
  pub fn gtfsm(&self) -> GTFSM_R {
    GTFSM_R::new((self.bits & 0x0f) as u8)
  }
}
impl W {
  #[doc = "Bits 0:3 - GTFSM"]
  #[inline(always)]
  pub fn gtfsm(&mut self) -> GTFSM_W {
    GTFSM_W { w: self }
  }
}
