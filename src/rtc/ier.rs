#[doc = "Reader of register IER"]
pub type R = crate::R<u32, super::IER>;
#[doc = "Writer for register IER"]
pub type W = crate::W<u32, super::IER>;
#[doc = "Register IER `reset()`'s with value 0x07"]
impl crate::ResetValue for super::IER {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0x07
  }
}
#[doc = "Time Invalid Interrupt Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIIE_A {
  #[doc = "0: Time invalid flag does not generate an interrupt."]
  TIIE_0,
  #[doc = "1: Time invalid flag does generate an interrupt."]
  TIIE_1,
}
impl From<TIIE_A> for bool {
  #[inline(always)]
  fn from(variant: TIIE_A) -> Self {
    match variant {
      TIIE_A::TIIE_0 => false,
      TIIE_A::TIIE_1 => true,
    }
  }
}
#[doc = "Reader of field `TIIE`"]
pub type TIIE_R = crate::R<bool, TIIE_A>;
impl TIIE_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> TIIE_A {
    match self.bits {
      false => TIIE_A::TIIE_0,
      true => TIIE_A::TIIE_1,
    }
  }
  #[doc = "Checks if the value of the field is `TIIE_0`"]
  #[inline(always)]
  pub fn is_tiie_0(&self) -> bool {
    *self == TIIE_A::TIIE_0
  }
  #[doc = "Checks if the value of the field is `TIIE_1`"]
  #[inline(always)]
  pub fn is_tiie_1(&self) -> bool {
    *self == TIIE_A::TIIE_1
  }
}
#[doc = "Write proxy for field `TIIE`"]
pub struct TIIE_W<'a> {
  w: &'a mut W,
}
impl<'a> TIIE_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: TIIE_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Time invalid flag does not generate an interrupt."]
  #[inline(always)]
  pub fn tiie_0(self) -> &'a mut W {
    self.variant(TIIE_A::TIIE_0)
  }
  #[doc = "Time invalid flag does generate an interrupt."]
  #[inline(always)]
  pub fn tiie_1(self) -> &'a mut W {
    self.variant(TIIE_A::TIIE_1)
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
#[doc = "Time Overflow Interrupt Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TOIE_A {
  #[doc = "0: Time overflow flag does not generate an interrupt."]
  TOIE_0,
  #[doc = "1: Time overflow flag does generate an interrupt."]
  TOIE_1,
}
impl From<TOIE_A> for bool {
  #[inline(always)]
  fn from(variant: TOIE_A) -> Self {
    match variant {
      TOIE_A::TOIE_0 => false,
      TOIE_A::TOIE_1 => true,
    }
  }
}
#[doc = "Reader of field `TOIE`"]
pub type TOIE_R = crate::R<bool, TOIE_A>;
impl TOIE_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> TOIE_A {
    match self.bits {
      false => TOIE_A::TOIE_0,
      true => TOIE_A::TOIE_1,
    }
  }
  #[doc = "Checks if the value of the field is `TOIE_0`"]
  #[inline(always)]
  pub fn is_toie_0(&self) -> bool {
    *self == TOIE_A::TOIE_0
  }
  #[doc = "Checks if the value of the field is `TOIE_1`"]
  #[inline(always)]
  pub fn is_toie_1(&self) -> bool {
    *self == TOIE_A::TOIE_1
  }
}
#[doc = "Write proxy for field `TOIE`"]
pub struct TOIE_W<'a> {
  w: &'a mut W,
}
impl<'a> TOIE_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: TOIE_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Time overflow flag does not generate an interrupt."]
  #[inline(always)]
  pub fn toie_0(self) -> &'a mut W {
    self.variant(TOIE_A::TOIE_0)
  }
  #[doc = "Time overflow flag does generate an interrupt."]
  #[inline(always)]
  pub fn toie_1(self) -> &'a mut W {
    self.variant(TOIE_A::TOIE_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
    self.w
  }
}
#[doc = "Time Alarm Interrupt Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TAIE_A {
  #[doc = "0: Time alarm flag does not generate an interrupt."]
  TAIE_0,
  #[doc = "1: Time alarm flag does generate an interrupt."]
  TAIE_1,
}
impl From<TAIE_A> for bool {
  #[inline(always)]
  fn from(variant: TAIE_A) -> Self {
    match variant {
      TAIE_A::TAIE_0 => false,
      TAIE_A::TAIE_1 => true,
    }
  }
}
#[doc = "Reader of field `TAIE`"]
pub type TAIE_R = crate::R<bool, TAIE_A>;
impl TAIE_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> TAIE_A {
    match self.bits {
      false => TAIE_A::TAIE_0,
      true => TAIE_A::TAIE_1,
    }
  }
  #[doc = "Checks if the value of the field is `TAIE_0`"]
  #[inline(always)]
  pub fn is_taie_0(&self) -> bool {
    *self == TAIE_A::TAIE_0
  }
  #[doc = "Checks if the value of the field is `TAIE_1`"]
  #[inline(always)]
  pub fn is_taie_1(&self) -> bool {
    *self == TAIE_A::TAIE_1
  }
}
#[doc = "Write proxy for field `TAIE`"]
pub struct TAIE_W<'a> {
  w: &'a mut W,
}
impl<'a> TAIE_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: TAIE_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Time alarm flag does not generate an interrupt."]
  #[inline(always)]
  pub fn taie_0(self) -> &'a mut W {
    self.variant(TAIE_A::TAIE_0)
  }
  #[doc = "Time alarm flag does generate an interrupt."]
  #[inline(always)]
  pub fn taie_1(self) -> &'a mut W {
    self.variant(TAIE_A::TAIE_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
    self.w
  }
}
#[doc = "Monotonic Overflow Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MOIE_A {
  #[doc = "0: Monotonic overflow flag does not generate an interrupt."]
  MOIE_0,
  #[doc = "1: Monotonic overflow flag does generate an interrupt."]
  MOIE_1,
}
impl From<MOIE_A> for bool {
  #[inline(always)]
  fn from(variant: MOIE_A) -> Self {
    match variant {
      MOIE_A::MOIE_0 => false,
      MOIE_A::MOIE_1 => true,
    }
  }
}
#[doc = "Reader of field `MOIE`"]
pub type MOIE_R = crate::R<bool, MOIE_A>;
impl MOIE_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> MOIE_A {
    match self.bits {
      false => MOIE_A::MOIE_0,
      true => MOIE_A::MOIE_1,
    }
  }
  #[doc = "Checks if the value of the field is `MOIE_0`"]
  #[inline(always)]
  pub fn is_moie_0(&self) -> bool {
    *self == MOIE_A::MOIE_0
  }
  #[doc = "Checks if the value of the field is `MOIE_1`"]
  #[inline(always)]
  pub fn is_moie_1(&self) -> bool {
    *self == MOIE_A::MOIE_1
  }
}
#[doc = "Write proxy for field `MOIE`"]
pub struct MOIE_W<'a> {
  w: &'a mut W,
}
impl<'a> MOIE_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: MOIE_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Monotonic overflow flag does not generate an interrupt."]
  #[inline(always)]
  pub fn moie_0(self) -> &'a mut W {
    self.variant(MOIE_A::MOIE_0)
  }
  #[doc = "Monotonic overflow flag does generate an interrupt."]
  #[inline(always)]
  pub fn moie_1(self) -> &'a mut W {
    self.variant(MOIE_A::MOIE_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
    self.w
  }
}
#[doc = "Time Seconds Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSIE_A {
  #[doc = "0: Seconds interrupt is disabled."]
  TSIE_0,
  #[doc = "1: Seconds interrupt is enabled."]
  TSIE_1,
}
impl From<TSIE_A> for bool {
  #[inline(always)]
  fn from(variant: TSIE_A) -> Self {
    match variant {
      TSIE_A::TSIE_0 => false,
      TSIE_A::TSIE_1 => true,
    }
  }
}
#[doc = "Reader of field `TSIE`"]
pub type TSIE_R = crate::R<bool, TSIE_A>;
impl TSIE_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> TSIE_A {
    match self.bits {
      false => TSIE_A::TSIE_0,
      true => TSIE_A::TSIE_1,
    }
  }
  #[doc = "Checks if the value of the field is `TSIE_0`"]
  #[inline(always)]
  pub fn is_tsie_0(&self) -> bool {
    *self == TSIE_A::TSIE_0
  }
  #[doc = "Checks if the value of the field is `TSIE_1`"]
  #[inline(always)]
  pub fn is_tsie_1(&self) -> bool {
    *self == TSIE_A::TSIE_1
  }
}
#[doc = "Write proxy for field `TSIE`"]
pub struct TSIE_W<'a> {
  w: &'a mut W,
}
impl<'a> TSIE_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: TSIE_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Seconds interrupt is disabled."]
  #[inline(always)]
  pub fn tsie_0(self) -> &'a mut W {
    self.variant(TSIE_A::TSIE_0)
  }
  #[doc = "Seconds interrupt is enabled."]
  #[inline(always)]
  pub fn tsie_1(self) -> &'a mut W {
    self.variant(TSIE_A::TSIE_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
    self.w
  }
}
#[doc = "Wakeup Pin On\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WPON_A {
  #[doc = "0: No effect."]
  WPON_0,
  #[doc = "1: If the RTC_WAKEUP pin is enabled, then the pin will assert."]
  WPON_1,
}
impl From<WPON_A> for bool {
  #[inline(always)]
  fn from(variant: WPON_A) -> Self {
    match variant {
      WPON_A::WPON_0 => false,
      WPON_A::WPON_1 => true,
    }
  }
}
#[doc = "Reader of field `WPON`"]
pub type WPON_R = crate::R<bool, WPON_A>;
impl WPON_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> WPON_A {
    match self.bits {
      false => WPON_A::WPON_0,
      true => WPON_A::WPON_1,
    }
  }
  #[doc = "Checks if the value of the field is `WPON_0`"]
  #[inline(always)]
  pub fn is_wpon_0(&self) -> bool {
    *self == WPON_A::WPON_0
  }
  #[doc = "Checks if the value of the field is `WPON_1`"]
  #[inline(always)]
  pub fn is_wpon_1(&self) -> bool {
    *self == WPON_A::WPON_1
  }
}
#[doc = "Write proxy for field `WPON`"]
pub struct WPON_W<'a> {
  w: &'a mut W,
}
impl<'a> WPON_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WPON_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "No effect."]
  #[inline(always)]
  pub fn wpon_0(self) -> &'a mut W {
    self.variant(WPON_A::WPON_0)
  }
  #[doc = "If the RTC_WAKEUP pin is enabled, then the pin will assert."]
  #[inline(always)]
  pub fn wpon_1(self) -> &'a mut W {
    self.variant(WPON_A::WPON_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
    self.w
  }
}
#[doc = "Timer Seconds Interrupt Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSIC_A {
  #[doc = "0: 1 Hz."]
  TSIC_0,
  #[doc = "1: 2 Hz."]
  TSIC_1,
  #[doc = "2: 4 Hz."]
  TSIC_2,
  #[doc = "3: 8 Hz."]
  TSIC_3,
  #[doc = "4: 16 Hz."]
  TSIC_4,
  #[doc = "5: 32 Hz."]
  TSIC_5,
  #[doc = "6: 64 Hz."]
  TSIC_6,
  #[doc = "7: 128 Hz."]
  TSIC_7,
}
impl From<TSIC_A> for u8 {
  #[inline(always)]
  fn from(variant: TSIC_A) -> Self {
    match variant {
      TSIC_A::TSIC_0 => 0,
      TSIC_A::TSIC_1 => 1,
      TSIC_A::TSIC_2 => 2,
      TSIC_A::TSIC_3 => 3,
      TSIC_A::TSIC_4 => 4,
      TSIC_A::TSIC_5 => 5,
      TSIC_A::TSIC_6 => 6,
      TSIC_A::TSIC_7 => 7,
    }
  }
}
#[doc = "Reader of field `TSIC`"]
pub type TSIC_R = crate::R<u8, TSIC_A>;
impl TSIC_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> TSIC_A {
    match self.bits {
      0 => TSIC_A::TSIC_0,
      1 => TSIC_A::TSIC_1,
      2 => TSIC_A::TSIC_2,
      3 => TSIC_A::TSIC_3,
      4 => TSIC_A::TSIC_4,
      5 => TSIC_A::TSIC_5,
      6 => TSIC_A::TSIC_6,
      7 => TSIC_A::TSIC_7,
      _ => unreachable!(),
    }
  }
  #[doc = "Checks if the value of the field is `TSIC_0`"]
  #[inline(always)]
  pub fn is_tsic_0(&self) -> bool {
    *self == TSIC_A::TSIC_0
  }
  #[doc = "Checks if the value of the field is `TSIC_1`"]
  #[inline(always)]
  pub fn is_tsic_1(&self) -> bool {
    *self == TSIC_A::TSIC_1
  }
  #[doc = "Checks if the value of the field is `TSIC_2`"]
  #[inline(always)]
  pub fn is_tsic_2(&self) -> bool {
    *self == TSIC_A::TSIC_2
  }
  #[doc = "Checks if the value of the field is `TSIC_3`"]
  #[inline(always)]
  pub fn is_tsic_3(&self) -> bool {
    *self == TSIC_A::TSIC_3
  }
  #[doc = "Checks if the value of the field is `TSIC_4`"]
  #[inline(always)]
  pub fn is_tsic_4(&self) -> bool {
    *self == TSIC_A::TSIC_4
  }
  #[doc = "Checks if the value of the field is `TSIC_5`"]
  #[inline(always)]
  pub fn is_tsic_5(&self) -> bool {
    *self == TSIC_A::TSIC_5
  }
  #[doc = "Checks if the value of the field is `TSIC_6`"]
  #[inline(always)]
  pub fn is_tsic_6(&self) -> bool {
    *self == TSIC_A::TSIC_6
  }
  #[doc = "Checks if the value of the field is `TSIC_7`"]
  #[inline(always)]
  pub fn is_tsic_7(&self) -> bool {
    *self == TSIC_A::TSIC_7
  }
}
#[doc = "Write proxy for field `TSIC`"]
pub struct TSIC_W<'a> {
  w: &'a mut W,
}
impl<'a> TSIC_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: TSIC_A) -> &'a mut W {
    {
      self.bits(variant.into())
    }
  }
  #[doc = "1 Hz."]
  #[inline(always)]
  pub fn tsic_0(self) -> &'a mut W {
    self.variant(TSIC_A::TSIC_0)
  }
  #[doc = "2 Hz."]
  #[inline(always)]
  pub fn tsic_1(self) -> &'a mut W {
    self.variant(TSIC_A::TSIC_1)
  }
  #[doc = "4 Hz."]
  #[inline(always)]
  pub fn tsic_2(self) -> &'a mut W {
    self.variant(TSIC_A::TSIC_2)
  }
  #[doc = "8 Hz."]
  #[inline(always)]
  pub fn tsic_3(self) -> &'a mut W {
    self.variant(TSIC_A::TSIC_3)
  }
  #[doc = "16 Hz."]
  #[inline(always)]
  pub fn tsic_4(self) -> &'a mut W {
    self.variant(TSIC_A::TSIC_4)
  }
  #[doc = "32 Hz."]
  #[inline(always)]
  pub fn tsic_5(self) -> &'a mut W {
    self.variant(TSIC_A::TSIC_5)
  }
  #[doc = "64 Hz."]
  #[inline(always)]
  pub fn tsic_6(self) -> &'a mut W {
    self.variant(TSIC_A::TSIC_6)
  }
  #[doc = "128 Hz."]
  #[inline(always)]
  pub fn tsic_7(self) -> &'a mut W {
    self.variant(TSIC_A::TSIC_7)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
    self.w
  }
}
impl R {
  #[doc = "Bit 0 - Time Invalid Interrupt Enable"]
  #[inline(always)]
  pub fn tiie(&self) -> TIIE_R {
    TIIE_R::new((self.bits & 0x01) != 0)
  }
  #[doc = "Bit 1 - Time Overflow Interrupt Enable"]
  #[inline(always)]
  pub fn toie(&self) -> TOIE_R {
    TOIE_R::new(((self.bits >> 1) & 0x01) != 0)
  }
  #[doc = "Bit 2 - Time Alarm Interrupt Enable"]
  #[inline(always)]
  pub fn taie(&self) -> TAIE_R {
    TAIE_R::new(((self.bits >> 2) & 0x01) != 0)
  }
  #[doc = "Bit 3 - Monotonic Overflow Interrupt Enable"]
  #[inline(always)]
  pub fn moie(&self) -> MOIE_R {
    MOIE_R::new(((self.bits >> 3) & 0x01) != 0)
  }
  #[doc = "Bit 4 - Time Seconds Interrupt Enable"]
  #[inline(always)]
  pub fn tsie(&self) -> TSIE_R {
    TSIE_R::new(((self.bits >> 4) & 0x01) != 0)
  }
  #[doc = "Bit 7 - Wakeup Pin On"]
  #[inline(always)]
  pub fn wpon(&self) -> WPON_R {
    WPON_R::new(((self.bits >> 7) & 0x01) != 0)
  }
  #[doc = "Bits 16:18 - Timer Seconds Interrupt Configuration"]
  #[inline(always)]
  pub fn tsic(&self) -> TSIC_R {
    TSIC_R::new(((self.bits >> 16) & 0x07) as u8)
  }
}
impl W {
  #[doc = "Bit 0 - Time Invalid Interrupt Enable"]
  #[inline(always)]
  pub fn tiie(&mut self) -> TIIE_W {
    TIIE_W { w: self }
  }
  #[doc = "Bit 1 - Time Overflow Interrupt Enable"]
  #[inline(always)]
  pub fn toie(&mut self) -> TOIE_W {
    TOIE_W { w: self }
  }
  #[doc = "Bit 2 - Time Alarm Interrupt Enable"]
  #[inline(always)]
  pub fn taie(&mut self) -> TAIE_W {
    TAIE_W { w: self }
  }
  #[doc = "Bit 3 - Monotonic Overflow Interrupt Enable"]
  #[inline(always)]
  pub fn moie(&mut self) -> MOIE_W {
    MOIE_W { w: self }
  }
  #[doc = "Bit 4 - Time Seconds Interrupt Enable"]
  #[inline(always)]
  pub fn tsie(&mut self) -> TSIE_W {
    TSIE_W { w: self }
  }
  #[doc = "Bit 7 - Wakeup Pin On"]
  #[inline(always)]
  pub fn wpon(&mut self) -> WPON_W {
    WPON_W { w: self }
  }
  #[doc = "Bits 16:18 - Timer Seconds Interrupt Configuration"]
  #[inline(always)]
  pub fn tsic(&mut self) -> TSIC_W {
    TSIC_W { w: self }
  }
}
