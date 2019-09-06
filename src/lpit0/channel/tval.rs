#[doc = "Reader of register TVAL"]
pub type R = crate::R<u32, super::TVAL>;
#[doc = "Writer for register TVAL"]
pub type W = crate::W<u32, super::TVAL>;
#[doc = "Register TVAL `reset()`'s with value 0"]
impl crate::ResetValue for super::TVAL {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "Timer Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMR_VAL_A {
  #[doc = "0: Invalid load value in compare mode"]
  TMR_VAL_0,
  #[doc = "1: Invalid load value in compare mode"]
  TMR_VAL_1,
  #[doc = "2: In compare mode: the value to be loaded; in capture mode, the value of the timer"]
  TMR_VAL_2,
  #[doc = "3: In compare mode: the value to be loaded; in capture mode, the value of the timer"]
  TMR_VAL_3,
  #[doc = "4: In compare mode: the value to be loaded; in capture mode, the value of the timer"]
  TMR_VAL_4,
  #[doc = "5: In compare mode: the value to be loaded; in capture mode, the value of the timer"]
  TMR_VAL_5,
  #[doc = "6: In compare mode: the value to be loaded; in capture mode, the value of the timer"]
  TMR_VAL_6,
  #[doc = "7: In compare mode: the value to be loaded; in capture mode, the value of the timer"]
  TMR_VAL_7,
  #[doc = "8: In compare mode: the value to be loaded; in capture mode, the value of the timer"]
  TMR_VAL_8,
  #[doc = "9: In compare mode: the value to be loaded; in capture mode, the value of the timer"]
  TMR_VAL_9,
}
impl From<TMR_VAL_A> for u32 {
  #[inline(always)]
  fn from(variant: TMR_VAL_A) -> Self {
    match variant {
      TMR_VAL_A::TMR_VAL_0 => 0,
      TMR_VAL_A::TMR_VAL_1 => 1,
      TMR_VAL_A::TMR_VAL_2 => 2,
      TMR_VAL_A::TMR_VAL_3 => 3,
      TMR_VAL_A::TMR_VAL_4 => 4,
      TMR_VAL_A::TMR_VAL_5 => 5,
      TMR_VAL_A::TMR_VAL_6 => 6,
      TMR_VAL_A::TMR_VAL_7 => 7,
      TMR_VAL_A::TMR_VAL_8 => 8,
      TMR_VAL_A::TMR_VAL_9 => 9,
    }
  }
}
#[doc = "Reader of field `TMR_VAL`"]
pub type TMR_VAL_R = crate::R<u32, TMR_VAL_A>;
impl TMR_VAL_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> crate::Variant<u32, TMR_VAL_A> {
    use crate::Variant::*;
    match self.bits {
      0 => Val(TMR_VAL_A::TMR_VAL_0),
      1 => Val(TMR_VAL_A::TMR_VAL_1),
      2 => Val(TMR_VAL_A::TMR_VAL_2),
      3 => Val(TMR_VAL_A::TMR_VAL_3),
      4 => Val(TMR_VAL_A::TMR_VAL_4),
      5 => Val(TMR_VAL_A::TMR_VAL_5),
      6 => Val(TMR_VAL_A::TMR_VAL_6),
      7 => Val(TMR_VAL_A::TMR_VAL_7),
      8 => Val(TMR_VAL_A::TMR_VAL_8),
      9 => Val(TMR_VAL_A::TMR_VAL_9),
      i => Res(i),
    }
  }
  #[doc = "Checks if the value of the field is `TMR_VAL_0`"]
  #[inline(always)]
  pub fn is_tmr_val_0(&self) -> bool {
    *self == TMR_VAL_A::TMR_VAL_0
  }
  #[doc = "Checks if the value of the field is `TMR_VAL_1`"]
  #[inline(always)]
  pub fn is_tmr_val_1(&self) -> bool {
    *self == TMR_VAL_A::TMR_VAL_1
  }
  #[doc = "Checks if the value of the field is `TMR_VAL_2`"]
  #[inline(always)]
  pub fn is_tmr_val_2(&self) -> bool {
    *self == TMR_VAL_A::TMR_VAL_2
  }
  #[doc = "Checks if the value of the field is `TMR_VAL_3`"]
  #[inline(always)]
  pub fn is_tmr_val_3(&self) -> bool {
    *self == TMR_VAL_A::TMR_VAL_3
  }
  #[doc = "Checks if the value of the field is `TMR_VAL_4`"]
  #[inline(always)]
  pub fn is_tmr_val_4(&self) -> bool {
    *self == TMR_VAL_A::TMR_VAL_4
  }
  #[doc = "Checks if the value of the field is `TMR_VAL_5`"]
  #[inline(always)]
  pub fn is_tmr_val_5(&self) -> bool {
    *self == TMR_VAL_A::TMR_VAL_5
  }
  #[doc = "Checks if the value of the field is `TMR_VAL_6`"]
  #[inline(always)]
  pub fn is_tmr_val_6(&self) -> bool {
    *self == TMR_VAL_A::TMR_VAL_6
  }
  #[doc = "Checks if the value of the field is `TMR_VAL_7`"]
  #[inline(always)]
  pub fn is_tmr_val_7(&self) -> bool {
    *self == TMR_VAL_A::TMR_VAL_7
  }
  #[doc = "Checks if the value of the field is `TMR_VAL_8`"]
  #[inline(always)]
  pub fn is_tmr_val_8(&self) -> bool {
    *self == TMR_VAL_A::TMR_VAL_8
  }
  #[doc = "Checks if the value of the field is `TMR_VAL_9`"]
  #[inline(always)]
  pub fn is_tmr_val_9(&self) -> bool {
    *self == TMR_VAL_A::TMR_VAL_9
  }
}
#[doc = "Write proxy for field `TMR_VAL`"]
pub struct TMR_VAL_W<'a> {
  w: &'a mut W,
}
impl<'a> TMR_VAL_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: TMR_VAL_A) -> &'a mut W {
    unsafe { self.bits(variant.into()) }
  }
  #[doc = "Invalid load value in compare mode"]
  #[inline(always)]
  pub fn tmr_val_0(self) -> &'a mut W {
    self.variant(TMR_VAL_A::TMR_VAL_0)
  }
  #[doc = "Invalid load value in compare mode"]
  #[inline(always)]
  pub fn tmr_val_1(self) -> &'a mut W {
    self.variant(TMR_VAL_A::TMR_VAL_1)
  }
  #[doc = "In compare mode: the value to be loaded; in capture mode, the value of the timer"]
  #[inline(always)]
  pub fn tmr_val_2(self) -> &'a mut W {
    self.variant(TMR_VAL_A::TMR_VAL_2)
  }
  #[doc = "In compare mode: the value to be loaded; in capture mode, the value of the timer"]
  #[inline(always)]
  pub fn tmr_val_3(self) -> &'a mut W {
    self.variant(TMR_VAL_A::TMR_VAL_3)
  }
  #[doc = "In compare mode: the value to be loaded; in capture mode, the value of the timer"]
  #[inline(always)]
  pub fn tmr_val_4(self) -> &'a mut W {
    self.variant(TMR_VAL_A::TMR_VAL_4)
  }
  #[doc = "In compare mode: the value to be loaded; in capture mode, the value of the timer"]
  #[inline(always)]
  pub fn tmr_val_5(self) -> &'a mut W {
    self.variant(TMR_VAL_A::TMR_VAL_5)
  }
  #[doc = "In compare mode: the value to be loaded; in capture mode, the value of the timer"]
  #[inline(always)]
  pub fn tmr_val_6(self) -> &'a mut W {
    self.variant(TMR_VAL_A::TMR_VAL_6)
  }
  #[doc = "In compare mode: the value to be loaded; in capture mode, the value of the timer"]
  #[inline(always)]
  pub fn tmr_val_7(self) -> &'a mut W {
    self.variant(TMR_VAL_A::TMR_VAL_7)
  }
  #[doc = "In compare mode: the value to be loaded; in capture mode, the value of the timer"]
  #[inline(always)]
  pub fn tmr_val_8(self) -> &'a mut W {
    self.variant(TMR_VAL_A::TMR_VAL_8)
  }
  #[doc = "In compare mode: the value to be loaded; in capture mode, the value of the timer"]
  #[inline(always)]
  pub fn tmr_val_9(self) -> &'a mut W {
    self.variant(TMR_VAL_A::TMR_VAL_9)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u32) -> &'a mut W {
    self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
    self.w
  }
}
impl R {
  #[doc = "Bits 0:31 - Timer Value"]
  #[inline(always)]
  pub fn tmr_val(&self) -> TMR_VAL_R {
    TMR_VAL_R::new((self.bits & 0xffff_ffff) as u32)
  }
}
impl W {
  #[doc = "Bits 0:31 - Timer Value"]
  #[inline(always)]
  pub fn tmr_val(&mut self) -> TMR_VAL_W {
    TMR_VAL_W { w: self }
  }
}
