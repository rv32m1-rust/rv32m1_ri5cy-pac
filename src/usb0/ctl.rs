#[doc = "Reader of register CTL"]
pub type R = crate::R<u8, super::CTL>;
#[doc = "Writer for register CTL"]
pub type W = crate::W<u8, super::CTL>;
#[doc = "Register CTL `reset()`'s with value 0"]
impl crate::ResetValue for super::CTL {
  type Type = u8;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "USB Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBENSOFEN_A {
  #[doc = "0: Disables the USB Module."]
  USBENSOFEN_0,
  #[doc = "1: Enables the USB Module."]
  USBENSOFEN_1,
}
impl From<USBENSOFEN_A> for bool {
  #[inline(always)]
  fn from(variant: USBENSOFEN_A) -> Self {
    match variant {
      USBENSOFEN_A::USBENSOFEN_0 => false,
      USBENSOFEN_A::USBENSOFEN_1 => true,
    }
  }
}
#[doc = "Reader of field `USBENSOFEN`"]
pub type USBENSOFEN_R = crate::R<bool, USBENSOFEN_A>;
impl USBENSOFEN_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> USBENSOFEN_A {
    match self.bits {
      false => USBENSOFEN_A::USBENSOFEN_0,
      true => USBENSOFEN_A::USBENSOFEN_1,
    }
  }
  #[doc = "Checks if the value of the field is `USBENSOFEN_0`"]
  #[inline(always)]
  pub fn is_usbensofen_0(&self) -> bool {
    *self == USBENSOFEN_A::USBENSOFEN_0
  }
  #[doc = "Checks if the value of the field is `USBENSOFEN_1`"]
  #[inline(always)]
  pub fn is_usbensofen_1(&self) -> bool {
    *self == USBENSOFEN_A::USBENSOFEN_1
  }
}
#[doc = "Write proxy for field `USBENSOFEN`"]
pub struct USBENSOFEN_W<'a> {
  w: &'a mut W,
}
impl<'a> USBENSOFEN_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: USBENSOFEN_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Disables the USB Module."]
  #[inline(always)]
  pub fn usbensofen_0(self) -> &'a mut W {
    self.variant(USBENSOFEN_A::USBENSOFEN_0)
  }
  #[doc = "Enables the USB Module."]
  #[inline(always)]
  pub fn usbensofen_1(self) -> &'a mut W {
    self.variant(USBENSOFEN_A::USBENSOFEN_1)
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
    self.w.bits = (self.w.bits & !0x01) | ((value as u8) & 0x01);
    self.w
  }
}
#[doc = "Reader of field `ODDRST`"]
pub type ODDRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ODDRST`"]
pub struct ODDRST_W<'a> {
  w: &'a mut W,
}
impl<'a> ODDRST_W<'a> {
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
    self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u8) & 0x01) << 1);
    self.w
  }
}
#[doc = "Reader of field `RESUME`"]
pub type RESUME_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESUME`"]
pub struct RESUME_W<'a> {
  w: &'a mut W,
}
impl<'a> RESUME_W<'a> {
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
    self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u8) & 0x01) << 2);
    self.w
  }
}
#[doc = "Host mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HOSTMODEEN_A {
  #[doc = "0: USB Module operates in Device mode."]
  HOSTMODEEN_0,
  #[doc = "1: USB Module operates in Host mode. In Host mode, the USB module performs USB transactions under the programmed control of the host processor."]
  HOSTMODEEN_1,
}
impl From<HOSTMODEEN_A> for bool {
  #[inline(always)]
  fn from(variant: HOSTMODEEN_A) -> Self {
    match variant {
      HOSTMODEEN_A::HOSTMODEEN_0 => false,
      HOSTMODEEN_A::HOSTMODEEN_1 => true,
    }
  }
}
#[doc = "Reader of field `HOSTMODEEN`"]
pub type HOSTMODEEN_R = crate::R<bool, HOSTMODEEN_A>;
impl HOSTMODEEN_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> HOSTMODEEN_A {
    match self.bits {
      false => HOSTMODEEN_A::HOSTMODEEN_0,
      true => HOSTMODEEN_A::HOSTMODEEN_1,
    }
  }
  #[doc = "Checks if the value of the field is `HOSTMODEEN_0`"]
  #[inline(always)]
  pub fn is_hostmodeen_0(&self) -> bool {
    *self == HOSTMODEEN_A::HOSTMODEEN_0
  }
  #[doc = "Checks if the value of the field is `HOSTMODEEN_1`"]
  #[inline(always)]
  pub fn is_hostmodeen_1(&self) -> bool {
    *self == HOSTMODEEN_A::HOSTMODEEN_1
  }
}
#[doc = "Write proxy for field `HOSTMODEEN`"]
pub struct HOSTMODEEN_W<'a> {
  w: &'a mut W,
}
impl<'a> HOSTMODEEN_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: HOSTMODEEN_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "USB Module operates in Device mode."]
  #[inline(always)]
  pub fn hostmodeen_0(self) -> &'a mut W {
    self.variant(HOSTMODEEN_A::HOSTMODEEN_0)
  }
  #[doc = "USB Module operates in Host mode. In Host mode, the USB module performs USB transactions under the programmed control of the host processor."]
  #[inline(always)]
  pub fn hostmodeen_1(self) -> &'a mut W {
    self.variant(HOSTMODEEN_A::HOSTMODEEN_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u8) & 0x01) << 3);
    self.w
  }
}
#[doc = "Reader of field `TXSUSPENDTOKENBUSY`"]
pub type TXSUSPENDTOKENBUSY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXSUSPENDTOKENBUSY`"]
pub struct TXSUSPENDTOKENBUSY_W<'a> {
  w: &'a mut W,
}
impl<'a> TXSUSPENDTOKENBUSY_W<'a> {
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
    self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u8) & 0x01) << 5);
    self.w
  }
}
#[doc = "Reader of field `SE0`"]
pub type SE0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SE0`"]
pub struct SE0_W<'a> {
  w: &'a mut W,
}
impl<'a> SE0_W<'a> {
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
    self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u8) & 0x01) << 6);
    self.w
  }
}
#[doc = "Reader of field `JSTATE`"]
pub type JSTATE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `JSTATE`"]
pub struct JSTATE_W<'a> {
  w: &'a mut W,
}
impl<'a> JSTATE_W<'a> {
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
    self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u8) & 0x01) << 7);
    self.w
  }
}
impl R {
  #[doc = "Bit 0 - USB Enable"]
  #[inline(always)]
  pub fn usbensofen(&self) -> USBENSOFEN_R {
    USBENSOFEN_R::new((self.bits & 0x01) != 0)
  }
  #[doc = "Bit 1 - ODDRST"]
  #[inline(always)]
  pub fn oddrst(&self) -> ODDRST_R {
    ODDRST_R::new(((self.bits >> 1) & 0x01) != 0)
  }
  #[doc = "Bit 2 - Resume"]
  #[inline(always)]
  pub fn resume(&self) -> RESUME_R {
    RESUME_R::new(((self.bits >> 2) & 0x01) != 0)
  }
  #[doc = "Bit 3 - Host mode enable"]
  #[inline(always)]
  pub fn hostmodeen(&self) -> HOSTMODEEN_R {
    HOSTMODEEN_R::new(((self.bits >> 3) & 0x01) != 0)
  }
  #[doc = "Bit 5 - TXSUSPEND or TOKENBUSY"]
  #[inline(always)]
  pub fn txsuspendtokenbusy(&self) -> TXSUSPENDTOKENBUSY_R {
    TXSUSPENDTOKENBUSY_R::new(((self.bits >> 5) & 0x01) != 0)
  }
  #[doc = "Bit 6 - Live USB Single-Ended Zero signal"]
  #[inline(always)]
  pub fn se0(&self) -> SE0_R {
    SE0_R::new(((self.bits >> 6) & 0x01) != 0)
  }
  #[doc = "Bit 7 - Live USB differential receiver JSTATE signal"]
  #[inline(always)]
  pub fn jstate(&self) -> JSTATE_R {
    JSTATE_R::new(((self.bits >> 7) & 0x01) != 0)
  }
}
impl W {
  #[doc = "Bit 0 - USB Enable"]
  #[inline(always)]
  pub fn usbensofen(&mut self) -> USBENSOFEN_W {
    USBENSOFEN_W { w: self }
  }
  #[doc = "Bit 1 - ODDRST"]
  #[inline(always)]
  pub fn oddrst(&mut self) -> ODDRST_W {
    ODDRST_W { w: self }
  }
  #[doc = "Bit 2 - Resume"]
  #[inline(always)]
  pub fn resume(&mut self) -> RESUME_W {
    RESUME_W { w: self }
  }
  #[doc = "Bit 3 - Host mode enable"]
  #[inline(always)]
  pub fn hostmodeen(&mut self) -> HOSTMODEEN_W {
    HOSTMODEEN_W { w: self }
  }
  #[doc = "Bit 5 - TXSUSPEND or TOKENBUSY"]
  #[inline(always)]
  pub fn txsuspendtokenbusy(&mut self) -> TXSUSPENDTOKENBUSY_W {
    TXSUSPENDTOKENBUSY_W { w: self }
  }
  #[doc = "Bit 6 - Live USB Single-Ended Zero signal"]
  #[inline(always)]
  pub fn se0(&mut self) -> SE0_W {
    SE0_W { w: self }
  }
  #[doc = "Bit 7 - Live USB differential receiver JSTATE signal"]
  #[inline(always)]
  pub fn jstate(&mut self) -> JSTATE_W {
    JSTATE_W { w: self }
  }
}
