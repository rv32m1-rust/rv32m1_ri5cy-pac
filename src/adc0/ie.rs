#[doc = "Reader of register IE"]
pub type R = crate::R<u32, super::IE>;
#[doc = "Writer for register IE"]
pub type W = crate::W<u32, super::IE>;
#[doc = "Register IE `reset()`'s with value 0"]
impl crate::ResetValue for super::IE {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "FIFO Watermark Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FWMIE_A {
  #[doc = "0: FIFO watermark interrupts are not enabled."]
  FWMIE_0,
  #[doc = "1: FIFO watermark interrupts are enabled."]
  FWMIE_1,
}
impl From<FWMIE_A> for bool {
  #[inline(always)]
  fn from(variant: FWMIE_A) -> Self {
    match variant {
      FWMIE_A::FWMIE_0 => false,
      FWMIE_A::FWMIE_1 => true,
    }
  }
}
#[doc = "Reader of field `FWMIE`"]
pub type FWMIE_R = crate::R<bool, FWMIE_A>;
impl FWMIE_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> FWMIE_A {
    match self.bits {
      false => FWMIE_A::FWMIE_0,
      true => FWMIE_A::FWMIE_1,
    }
  }
  #[doc = "Checks if the value of the field is `FWMIE_0`"]
  #[inline(always)]
  pub fn is_fwmie_0(&self) -> bool {
    *self == FWMIE_A::FWMIE_0
  }
  #[doc = "Checks if the value of the field is `FWMIE_1`"]
  #[inline(always)]
  pub fn is_fwmie_1(&self) -> bool {
    *self == FWMIE_A::FWMIE_1
  }
}
#[doc = "Write proxy for field `FWMIE`"]
pub struct FWMIE_W<'a> {
  w: &'a mut W,
}
impl<'a> FWMIE_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: FWMIE_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "FIFO watermark interrupts are not enabled."]
  #[inline(always)]
  pub fn fwmie_0(self) -> &'a mut W {
    self.variant(FWMIE_A::FWMIE_0)
  }
  #[doc = "FIFO watermark interrupts are enabled."]
  #[inline(always)]
  pub fn fwmie_1(self) -> &'a mut W {
    self.variant(FWMIE_A::FWMIE_1)
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
#[doc = "Result FIFO Overflow Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FOFIE_A {
  #[doc = "0: FIFO overflow interrupts are not enabled."]
  FOFIE_0,
  #[doc = "1: FIFO overflow interrupts are enabled."]
  FOFIE_1,
}
impl From<FOFIE_A> for bool {
  #[inline(always)]
  fn from(variant: FOFIE_A) -> Self {
    match variant {
      FOFIE_A::FOFIE_0 => false,
      FOFIE_A::FOFIE_1 => true,
    }
  }
}
#[doc = "Reader of field `FOFIE`"]
pub type FOFIE_R = crate::R<bool, FOFIE_A>;
impl FOFIE_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> FOFIE_A {
    match self.bits {
      false => FOFIE_A::FOFIE_0,
      true => FOFIE_A::FOFIE_1,
    }
  }
  #[doc = "Checks if the value of the field is `FOFIE_0`"]
  #[inline(always)]
  pub fn is_fofie_0(&self) -> bool {
    *self == FOFIE_A::FOFIE_0
  }
  #[doc = "Checks if the value of the field is `FOFIE_1`"]
  #[inline(always)]
  pub fn is_fofie_1(&self) -> bool {
    *self == FOFIE_A::FOFIE_1
  }
}
#[doc = "Write proxy for field `FOFIE`"]
pub struct FOFIE_W<'a> {
  w: &'a mut W,
}
impl<'a> FOFIE_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: FOFIE_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "FIFO overflow interrupts are not enabled."]
  #[inline(always)]
  pub fn fofie_0(self) -> &'a mut W {
    self.variant(FOFIE_A::FOFIE_0)
  }
  #[doc = "FIFO overflow interrupts are enabled."]
  #[inline(always)]
  pub fn fofie_1(self) -> &'a mut W {
    self.variant(FOFIE_A::FOFIE_1)
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
impl R {
  #[doc = "Bit 0 - FIFO Watermark Interrupt Enable"]
  #[inline(always)]
  pub fn fwmie(&self) -> FWMIE_R {
    FWMIE_R::new((self.bits & 0x01) != 0)
  }
  #[doc = "Bit 1 - Result FIFO Overflow Interrupt Enable"]
  #[inline(always)]
  pub fn fofie(&self) -> FOFIE_R {
    FOFIE_R::new(((self.bits >> 1) & 0x01) != 0)
  }
}
impl W {
  #[doc = "Bit 0 - FIFO Watermark Interrupt Enable"]
  #[inline(always)]
  pub fn fwmie(&mut self) -> FWMIE_W {
    FWMIE_W { w: self }
  }
  #[doc = "Bit 1 - Result FIFO Overflow Interrupt Enable"]
  #[inline(always)]
  pub fn fofie(&mut self) -> FOFIE_W {
    FOFIE_W { w: self }
  }
}
