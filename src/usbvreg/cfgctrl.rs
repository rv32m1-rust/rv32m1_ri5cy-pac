#[doc = "Reader of register CFGCTRL"]
pub type R = crate::R<u32, super::CFGCTRL>;
#[doc = "Writer for register CFGCTRL"]
pub type W = crate::W<u32, super::CFGCTRL>;
#[doc = "Register CFGCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::CFGCTRL {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "USB Voltage Regulator Enable Write Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum URWE_A {
  #[doc = "0: CTRL\\[EN\\] can not be written."]
  URWE_0,
  #[doc = "1: CTRL\\[EN\\] can be written."]
  URWE_1,
}
impl From<URWE_A> for bool {
  #[inline(always)]
  fn from(variant: URWE_A) -> Self {
    match variant {
      URWE_A::URWE_0 => false,
      URWE_A::URWE_1 => true,
    }
  }
}
#[doc = "Reader of field `URWE`"]
pub type URWE_R = crate::R<bool, URWE_A>;
impl URWE_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> URWE_A {
    match self.bits {
      false => URWE_A::URWE_0,
      true => URWE_A::URWE_1,
    }
  }
  #[doc = "Checks if the value of the field is `URWE_0`"]
  #[inline(always)]
  pub fn is_urwe_0(&self) -> bool {
    *self == URWE_A::URWE_0
  }
  #[doc = "Checks if the value of the field is `URWE_1`"]
  #[inline(always)]
  pub fn is_urwe_1(&self) -> bool {
    *self == URWE_A::URWE_1
  }
}
#[doc = "Write proxy for field `URWE`"]
pub struct URWE_W<'a> {
  w: &'a mut W,
}
impl<'a> URWE_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: URWE_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "CTRL\\[EN\\] can not be written."]
  #[inline(always)]
  pub fn urwe_0(self) -> &'a mut W {
    self.variant(URWE_A::URWE_0)
  }
  #[doc = "CTRL\\[EN\\] can be written."]
  #[inline(always)]
  pub fn urwe_1(self) -> &'a mut W {
    self.variant(URWE_A::URWE_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
    self.w
  }
}
#[doc = "USB Voltage Regulator VLP Standby Write Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UVSWE_A {
  #[doc = "0: CTRL\\[VSTBY\\] cannot be written."]
  UVSWE_0,
  #[doc = "1: CTRL\\[VSTBY\\] can be written."]
  UVSWE_1,
}
impl From<UVSWE_A> for bool {
  #[inline(always)]
  fn from(variant: UVSWE_A) -> Self {
    match variant {
      UVSWE_A::UVSWE_0 => false,
      UVSWE_A::UVSWE_1 => true,
    }
  }
}
#[doc = "Reader of field `UVSWE`"]
pub type UVSWE_R = crate::R<bool, UVSWE_A>;
impl UVSWE_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> UVSWE_A {
    match self.bits {
      false => UVSWE_A::UVSWE_0,
      true => UVSWE_A::UVSWE_1,
    }
  }
  #[doc = "Checks if the value of the field is `UVSWE_0`"]
  #[inline(always)]
  pub fn is_uvswe_0(&self) -> bool {
    *self == UVSWE_A::UVSWE_0
  }
  #[doc = "Checks if the value of the field is `UVSWE_1`"]
  #[inline(always)]
  pub fn is_uvswe_1(&self) -> bool {
    *self == UVSWE_A::UVSWE_1
  }
}
#[doc = "Write proxy for field `UVSWE`"]
pub struct UVSWE_W<'a> {
  w: &'a mut W,
}
impl<'a> UVSWE_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: UVSWE_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "CTRL\\[VSTBY\\] cannot be written."]
  #[inline(always)]
  pub fn uvswe_0(self) -> &'a mut W {
    self.variant(UVSWE_A::UVSWE_0)
  }
  #[doc = "CTRL\\[VSTBY\\] can be written."]
  #[inline(always)]
  pub fn uvswe_1(self) -> &'a mut W {
    self.variant(UVSWE_A::UVSWE_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
    self.w
  }
}
#[doc = "USB Voltage Rregulator Stop Standby Write Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USSWE_A {
  #[doc = "0: CTRL\\[SSTBY\\] field cannot be written."]
  USSWE_0,
  #[doc = "1: CTRL\\[SSTBY\\] can be written."]
  USSWE_1,
}
impl From<USSWE_A> for bool {
  #[inline(always)]
  fn from(variant: USSWE_A) -> Self {
    match variant {
      USSWE_A::USSWE_0 => false,
      USSWE_A::USSWE_1 => true,
    }
  }
}
#[doc = "Reader of field `USSWE`"]
pub type USSWE_R = crate::R<bool, USSWE_A>;
impl USSWE_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> USSWE_A {
    match self.bits {
      false => USSWE_A::USSWE_0,
      true => USSWE_A::USSWE_1,
    }
  }
  #[doc = "Checks if the value of the field is `USSWE_0`"]
  #[inline(always)]
  pub fn is_usswe_0(&self) -> bool {
    *self == USSWE_A::USSWE_0
  }
  #[doc = "Checks if the value of the field is `USSWE_1`"]
  #[inline(always)]
  pub fn is_usswe_1(&self) -> bool {
    *self == USSWE_A::USSWE_1
  }
}
#[doc = "Write proxy for field `USSWE`"]
pub struct USSWE_W<'a> {
  w: &'a mut W,
}
impl<'a> USSWE_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: USSWE_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "CTRL\\[SSTBY\\] field cannot be written."]
  #[inline(always)]
  pub fn usswe_0(self) -> &'a mut W {
    self.variant(USSWE_A::USSWE_0)
  }
  #[doc = "CTRL\\[SSTBY\\] can be written."]
  #[inline(always)]
  pub fn usswe_1(self) -> &'a mut W {
    self.variant(USSWE_A::USSWE_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
    self.w
  }
}
impl R {
  #[doc = "Bit 24 - USB Voltage Regulator Enable Write Enable"]
  #[inline(always)]
  pub fn urwe(&self) -> URWE_R {
    URWE_R::new(((self.bits >> 24) & 0x01) != 0)
  }
  #[doc = "Bit 25 - USB Voltage Regulator VLP Standby Write Enable"]
  #[inline(always)]
  pub fn uvswe(&self) -> UVSWE_R {
    UVSWE_R::new(((self.bits >> 25) & 0x01) != 0)
  }
  #[doc = "Bit 26 - USB Voltage Rregulator Stop Standby Write Enable"]
  #[inline(always)]
  pub fn usswe(&self) -> USSWE_R {
    USSWE_R::new(((self.bits >> 26) & 0x01) != 0)
  }
}
impl W {
  #[doc = "Bit 24 - USB Voltage Regulator Enable Write Enable"]
  #[inline(always)]
  pub fn urwe(&mut self) -> URWE_W {
    URWE_W { w: self }
  }
  #[doc = "Bit 25 - USB Voltage Regulator VLP Standby Write Enable"]
  #[inline(always)]
  pub fn uvswe(&mut self) -> UVSWE_W {
    UVSWE_W { w: self }
  }
  #[doc = "Bit 26 - USB Voltage Rregulator Stop Standby Write Enable"]
  #[inline(always)]
  pub fn usswe(&mut self) -> USSWE_W {
    USSWE_W { w: self }
  }
}
