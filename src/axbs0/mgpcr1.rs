#[doc = "Reader of register MGPCR1"]
pub type R = crate::R<u32, super::MGPCR1>;
#[doc = "Writer for register MGPCR1"]
pub type W = crate::W<u32, super::MGPCR1>;
#[doc = "Register MGPCR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::MGPCR1 {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "Arbitrates On Undefined Length Bursts\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AULB_A {
  #[doc = "0: No arbitration is allowed during an undefined length burst"]
  AULB_0,
  #[doc = "1: Arbitration is allowed at any time during an undefined length burst"]
  AULB_1,
  #[doc = "2: Arbitration is allowed after four beats of an undefined length burst"]
  AULB_2,
  #[doc = "3: Arbitration is allowed after eight beats of an undefined length burst"]
  AULB_3,
  #[doc = "4: Arbitration is allowed after 16 beats of an undefined length burst"]
  AULB_4,
}
impl From<AULB_A> for u8 {
  #[inline(always)]
  fn from(variant: AULB_A) -> Self {
    match variant {
      AULB_A::AULB_0 => 0,
      AULB_A::AULB_1 => 1,
      AULB_A::AULB_2 => 2,
      AULB_A::AULB_3 => 3,
      AULB_A::AULB_4 => 4,
    }
  }
}
#[doc = "Reader of field `AULB`"]
pub type AULB_R = crate::R<u8, AULB_A>;
impl AULB_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> crate::Variant<u8, AULB_A> {
    use crate::Variant::*;
    match self.bits {
      0 => Val(AULB_A::AULB_0),
      1 => Val(AULB_A::AULB_1),
      2 => Val(AULB_A::AULB_2),
      3 => Val(AULB_A::AULB_3),
      4 => Val(AULB_A::AULB_4),
      i => Res(i),
    }
  }
  #[doc = "Checks if the value of the field is `AULB_0`"]
  #[inline(always)]
  pub fn is_aulb_0(&self) -> bool {
    *self == AULB_A::AULB_0
  }
  #[doc = "Checks if the value of the field is `AULB_1`"]
  #[inline(always)]
  pub fn is_aulb_1(&self) -> bool {
    *self == AULB_A::AULB_1
  }
  #[doc = "Checks if the value of the field is `AULB_2`"]
  #[inline(always)]
  pub fn is_aulb_2(&self) -> bool {
    *self == AULB_A::AULB_2
  }
  #[doc = "Checks if the value of the field is `AULB_3`"]
  #[inline(always)]
  pub fn is_aulb_3(&self) -> bool {
    *self == AULB_A::AULB_3
  }
  #[doc = "Checks if the value of the field is `AULB_4`"]
  #[inline(always)]
  pub fn is_aulb_4(&self) -> bool {
    *self == AULB_A::AULB_4
  }
}
#[doc = "Write proxy for field `AULB`"]
pub struct AULB_W<'a> {
  w: &'a mut W,
}
impl<'a> AULB_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: AULB_A) -> &'a mut W {
    unsafe { self.bits(variant.into()) }
  }
  #[doc = "No arbitration is allowed during an undefined length burst"]
  #[inline(always)]
  pub fn aulb_0(self) -> &'a mut W {
    self.variant(AULB_A::AULB_0)
  }
  #[doc = "Arbitration is allowed at any time during an undefined length burst"]
  #[inline(always)]
  pub fn aulb_1(self) -> &'a mut W {
    self.variant(AULB_A::AULB_1)
  }
  #[doc = "Arbitration is allowed after four beats of an undefined length burst"]
  #[inline(always)]
  pub fn aulb_2(self) -> &'a mut W {
    self.variant(AULB_A::AULB_2)
  }
  #[doc = "Arbitration is allowed after eight beats of an undefined length burst"]
  #[inline(always)]
  pub fn aulb_3(self) -> &'a mut W {
    self.variant(AULB_A::AULB_3)
  }
  #[doc = "Arbitration is allowed after 16 beats of an undefined length burst"]
  #[inline(always)]
  pub fn aulb_4(self) -> &'a mut W {
    self.variant(AULB_A::AULB_4)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
    self.w
  }
}
impl R {
  #[doc = "Bits 0:2 - Arbitrates On Undefined Length Bursts"]
  #[inline(always)]
  pub fn aulb(&self) -> AULB_R {
    AULB_R::new((self.bits & 0x07) as u8)
  }
}
impl W {
  #[doc = "Bits 0:2 - Arbitrates On Undefined Length Bursts"]
  #[inline(always)]
  pub fn aulb(&mut self) -> AULB_W {
    AULB_W { w: self }
  }
}
