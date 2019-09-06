#[doc = "Reader of register STATUS"]
pub type R = crate::R<u32, super::STATUS>;
#[doc = "Writer for register STATUS"]
pub type W = crate::W<u32, super::STATUS>;
#[doc = "Register STATUS `reset()`'s with value 0"]
impl crate::ResetValue for super::STATUS {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "Channel 0 Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH0F_A {
  #[doc = "0: No channel event has occurred."]
  CH0F_0,
  #[doc = "1: A channel event has occurred."]
  CH0F_1,
}
impl From<CH0F_A> for bool {
  #[inline(always)]
  fn from(variant: CH0F_A) -> Self {
    match variant {
      CH0F_A::CH0F_0 => false,
      CH0F_A::CH0F_1 => true,
    }
  }
}
#[doc = "Reader of field `CH0F`"]
pub type CH0F_R = crate::R<bool, CH0F_A>;
impl CH0F_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> CH0F_A {
    match self.bits {
      false => CH0F_A::CH0F_0,
      true => CH0F_A::CH0F_1,
    }
  }
  #[doc = "Checks if the value of the field is `CH0F_0`"]
  #[inline(always)]
  pub fn is_ch0f_0(&self) -> bool {
    *self == CH0F_A::CH0F_0
  }
  #[doc = "Checks if the value of the field is `CH0F_1`"]
  #[inline(always)]
  pub fn is_ch0f_1(&self) -> bool {
    *self == CH0F_A::CH0F_1
  }
}
#[doc = "Write proxy for field `CH0F`"]
pub struct CH0F_W<'a> {
  w: &'a mut W,
}
impl<'a> CH0F_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: CH0F_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "No channel event has occurred."]
  #[inline(always)]
  pub fn ch0f_0(self) -> &'a mut W {
    self.variant(CH0F_A::CH0F_0)
  }
  #[doc = "A channel event has occurred."]
  #[inline(always)]
  pub fn ch0f_1(self) -> &'a mut W {
    self.variant(CH0F_A::CH0F_1)
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
#[doc = "Channel 1 Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH1F_A {
  #[doc = "0: No channel event has occurred."]
  CH1F_0,
  #[doc = "1: A channel event has occurred."]
  CH1F_1,
}
impl From<CH1F_A> for bool {
  #[inline(always)]
  fn from(variant: CH1F_A) -> Self {
    match variant {
      CH1F_A::CH1F_0 => false,
      CH1F_A::CH1F_1 => true,
    }
  }
}
#[doc = "Reader of field `CH1F`"]
pub type CH1F_R = crate::R<bool, CH1F_A>;
impl CH1F_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> CH1F_A {
    match self.bits {
      false => CH1F_A::CH1F_0,
      true => CH1F_A::CH1F_1,
    }
  }
  #[doc = "Checks if the value of the field is `CH1F_0`"]
  #[inline(always)]
  pub fn is_ch1f_0(&self) -> bool {
    *self == CH1F_A::CH1F_0
  }
  #[doc = "Checks if the value of the field is `CH1F_1`"]
  #[inline(always)]
  pub fn is_ch1f_1(&self) -> bool {
    *self == CH1F_A::CH1F_1
  }
}
#[doc = "Write proxy for field `CH1F`"]
pub struct CH1F_W<'a> {
  w: &'a mut W,
}
impl<'a> CH1F_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: CH1F_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "No channel event has occurred."]
  #[inline(always)]
  pub fn ch1f_0(self) -> &'a mut W {
    self.variant(CH1F_A::CH1F_0)
  }
  #[doc = "A channel event has occurred."]
  #[inline(always)]
  pub fn ch1f_1(self) -> &'a mut W {
    self.variant(CH1F_A::CH1F_1)
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
#[doc = "Timer Overflow Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TOF_A {
  #[doc = "0: no description available"]
  TOF_0,
  #[doc = "1: no description available"]
  TOF_1,
}
impl From<TOF_A> for bool {
  #[inline(always)]
  fn from(variant: TOF_A) -> Self {
    match variant {
      TOF_A::TOF_0 => false,
      TOF_A::TOF_1 => true,
    }
  }
}
#[doc = "Reader of field `TOF`"]
pub type TOF_R = crate::R<bool, TOF_A>;
impl TOF_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> TOF_A {
    match self.bits {
      false => TOF_A::TOF_0,
      true => TOF_A::TOF_1,
    }
  }
  #[doc = "Checks if the value of the field is `TOF_0`"]
  #[inline(always)]
  pub fn is_tof_0(&self) -> bool {
    *self == TOF_A::TOF_0
  }
  #[doc = "Checks if the value of the field is `TOF_1`"]
  #[inline(always)]
  pub fn is_tof_1(&self) -> bool {
    *self == TOF_A::TOF_1
  }
}
#[doc = "Write proxy for field `TOF`"]
pub struct TOF_W<'a> {
  w: &'a mut W,
}
impl<'a> TOF_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: TOF_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "no description available"]
  #[inline(always)]
  pub fn tof_0(self) -> &'a mut W {
    self.variant(TOF_A::TOF_0)
  }
  #[doc = "no description available"]
  #[inline(always)]
  pub fn tof_1(self) -> &'a mut W {
    self.variant(TOF_A::TOF_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
    self.w
  }
}
impl R {
  #[doc = "Bit 0 - Channel 0 Flag"]
  #[inline(always)]
  pub fn ch0f(&self) -> CH0F_R {
    CH0F_R::new((self.bits & 0x01) != 0)
  }
  #[doc = "Bit 1 - Channel 1 Flag"]
  #[inline(always)]
  pub fn ch1f(&self) -> CH1F_R {
    CH1F_R::new(((self.bits >> 1) & 0x01) != 0)
  }
  #[doc = "Bit 8 - Timer Overflow Flag"]
  #[inline(always)]
  pub fn tof(&self) -> TOF_R {
    TOF_R::new(((self.bits >> 8) & 0x01) != 0)
  }
}
impl W {
  #[doc = "Bit 0 - Channel 0 Flag"]
  #[inline(always)]
  pub fn ch0f(&mut self) -> CH0F_W {
    CH0F_W { w: self }
  }
  #[doc = "Bit 1 - Channel 1 Flag"]
  #[inline(always)]
  pub fn ch1f(&mut self) -> CH1F_W {
    CH1F_W { w: self }
  }
  #[doc = "Bit 8 - Timer Overflow Flag"]
  #[inline(always)]
  pub fn tof(&mut self) -> TOF_W {
    TOF_W { w: self }
  }
}
