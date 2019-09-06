#[doc = "Reader of register CTRL"]
pub type R = crate::R<u32, super::CTRL>;
#[doc = "Writer for register CTRL"]
pub type W = crate::W<u32, super::CTRL>;
#[doc = "Register CTRL `reset()`'s with value 0x8000_0000"]
impl crate::ResetValue for super::CTRL {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0x8000_0000
  }
}
#[doc = "USB Voltage Regulator in Standby Mode during VLPR and VLPW modes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VSTBY_A {
  #[doc = "0: USB voltage regulator is not in standby during VLPR and VLPW modes."]
  VSTBY_0,
  #[doc = "1: USB voltage regulator in standby during VLPR and VLPW modes."]
  VSTBY_1,
}
impl From<VSTBY_A> for bool {
  #[inline(always)]
  fn from(variant: VSTBY_A) -> Self {
    match variant {
      VSTBY_A::VSTBY_0 => false,
      VSTBY_A::VSTBY_1 => true,
    }
  }
}
#[doc = "Reader of field `VSTBY`"]
pub type VSTBY_R = crate::R<bool, VSTBY_A>;
impl VSTBY_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> VSTBY_A {
    match self.bits {
      false => VSTBY_A::VSTBY_0,
      true => VSTBY_A::VSTBY_1,
    }
  }
  #[doc = "Checks if the value of the field is `VSTBY_0`"]
  #[inline(always)]
  pub fn is_vstby_0(&self) -> bool {
    *self == VSTBY_A::VSTBY_0
  }
  #[doc = "Checks if the value of the field is `VSTBY_1`"]
  #[inline(always)]
  pub fn is_vstby_1(&self) -> bool {
    *self == VSTBY_A::VSTBY_1
  }
}
#[doc = "Write proxy for field `VSTBY`"]
pub struct VSTBY_W<'a> {
  w: &'a mut W,
}
impl<'a> VSTBY_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: VSTBY_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "USB voltage regulator is not in standby during VLPR and VLPW modes."]
  #[inline(always)]
  pub fn vstby_0(self) -> &'a mut W {
    self.variant(VSTBY_A::VSTBY_0)
  }
  #[doc = "USB voltage regulator in standby during VLPR and VLPW modes."]
  #[inline(always)]
  pub fn vstby_1(self) -> &'a mut W {
    self.variant(VSTBY_A::VSTBY_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
    self.w
  }
}
#[doc = "USB Voltage Regulator in Standby Mode during Stop, VLPS, LLS and VLLS Modes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSTBY_A {
  #[doc = "0: USB voltage regulator is not in standby during Stop,VLPS,LLS and VLLS modes."]
  SSTBY_0,
  #[doc = "1: USB voltage regulator is in standby during Stop,VLPS,LLS and VLLS modes."]
  SSTBY_1,
}
impl From<SSTBY_A> for bool {
  #[inline(always)]
  fn from(variant: SSTBY_A) -> Self {
    match variant {
      SSTBY_A::SSTBY_0 => false,
      SSTBY_A::SSTBY_1 => true,
    }
  }
}
#[doc = "Reader of field `SSTBY`"]
pub type SSTBY_R = crate::R<bool, SSTBY_A>;
impl SSTBY_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> SSTBY_A {
    match self.bits {
      false => SSTBY_A::SSTBY_0,
      true => SSTBY_A::SSTBY_1,
    }
  }
  #[doc = "Checks if the value of the field is `SSTBY_0`"]
  #[inline(always)]
  pub fn is_sstby_0(&self) -> bool {
    *self == SSTBY_A::SSTBY_0
  }
  #[doc = "Checks if the value of the field is `SSTBY_1`"]
  #[inline(always)]
  pub fn is_sstby_1(&self) -> bool {
    *self == SSTBY_A::SSTBY_1
  }
}
#[doc = "Write proxy for field `SSTBY`"]
pub struct SSTBY_W<'a> {
  w: &'a mut W,
}
impl<'a> SSTBY_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: SSTBY_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "USB voltage regulator is not in standby during Stop,VLPS,LLS and VLLS modes."]
  #[inline(always)]
  pub fn sstby_0(self) -> &'a mut W {
    self.variant(SSTBY_A::SSTBY_0)
  }
  #[doc = "USB voltage regulator is in standby during Stop,VLPS,LLS and VLLS modes."]
  #[inline(always)]
  pub fn sstby_1(self) -> &'a mut W {
    self.variant(SSTBY_A::SSTBY_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
    self.w
  }
}
#[doc = "USB Voltage Regulator Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN_A {
  #[doc = "0: USB voltage regulator is disabled."]
  EN_0,
  #[doc = "1: USB voltage regulator is enabled."]
  EN_1,
}
impl From<EN_A> for bool {
  #[inline(always)]
  fn from(variant: EN_A) -> Self {
    match variant {
      EN_A::EN_0 => false,
      EN_A::EN_1 => true,
    }
  }
}
#[doc = "Reader of field `EN`"]
pub type EN_R = crate::R<bool, EN_A>;
impl EN_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> EN_A {
    match self.bits {
      false => EN_A::EN_0,
      true => EN_A::EN_1,
    }
  }
  #[doc = "Checks if the value of the field is `EN_0`"]
  #[inline(always)]
  pub fn is_en_0(&self) -> bool {
    *self == EN_A::EN_0
  }
  #[doc = "Checks if the value of the field is `EN_1`"]
  #[inline(always)]
  pub fn is_en_1(&self) -> bool {
    *self == EN_A::EN_1
  }
}
#[doc = "Write proxy for field `EN`"]
pub struct EN_W<'a> {
  w: &'a mut W,
}
impl<'a> EN_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: EN_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "USB voltage regulator is disabled."]
  #[inline(always)]
  pub fn en_0(self) -> &'a mut W {
    self.variant(EN_A::EN_0)
  }
  #[doc = "USB voltage regulator is enabled."]
  #[inline(always)]
  pub fn en_1(self) -> &'a mut W {
    self.variant(EN_A::EN_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
    self.w
  }
}
impl R {
  #[doc = "Bit 29 - USB Voltage Regulator in Standby Mode during VLPR and VLPW modes"]
  #[inline(always)]
  pub fn vstby(&self) -> VSTBY_R {
    VSTBY_R::new(((self.bits >> 29) & 0x01) != 0)
  }
  #[doc = "Bit 30 - USB Voltage Regulator in Standby Mode during Stop, VLPS, LLS and VLLS Modes"]
  #[inline(always)]
  pub fn sstby(&self) -> SSTBY_R {
    SSTBY_R::new(((self.bits >> 30) & 0x01) != 0)
  }
  #[doc = "Bit 31 - USB Voltage Regulator Enable"]
  #[inline(always)]
  pub fn en(&self) -> EN_R {
    EN_R::new(((self.bits >> 31) & 0x01) != 0)
  }
}
impl W {
  #[doc = "Bit 29 - USB Voltage Regulator in Standby Mode during VLPR and VLPW modes"]
  #[inline(always)]
  pub fn vstby(&mut self) -> VSTBY_W {
    VSTBY_W { w: self }
  }
  #[doc = "Bit 30 - USB Voltage Regulator in Standby Mode during Stop, VLPS, LLS and VLLS Modes"]
  #[inline(always)]
  pub fn sstby(&mut self) -> SSTBY_W {
    SSTBY_W { w: self }
  }
  #[doc = "Bit 31 - USB Voltage Regulator Enable"]
  #[inline(always)]
  pub fn en(&mut self) -> EN_W {
    EN_W { w: self }
  }
}
