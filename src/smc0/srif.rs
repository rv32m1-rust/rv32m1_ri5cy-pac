#[doc = "Reader of register SRIF"]
pub type R = crate::R<u32, super::SRIF>;
#[doc = "Writer for register SRIF"]
pub type W = crate::W<u32, super::SRIF>;
#[doc = "Register SRIF `reset()`'s with value 0"]
impl crate::ResetValue for super::SRIF {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "Pin Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN_A {
  #[doc = "0: Reset source not pending."]
  PIN_0,
  #[doc = "1: Reset source pending."]
  PIN_1,
}
impl From<PIN_A> for bool {
  #[inline(always)]
  fn from(variant: PIN_A) -> Self {
    match variant {
      PIN_A::PIN_0 => false,
      PIN_A::PIN_1 => true,
    }
  }
}
#[doc = "Reader of field `PIN`"]
pub type PIN_R = crate::R<bool, PIN_A>;
impl PIN_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> PIN_A {
    match self.bits {
      false => PIN_A::PIN_0,
      true => PIN_A::PIN_1,
    }
  }
  #[doc = "Checks if the value of the field is `PIN_0`"]
  #[inline(always)]
  pub fn is_pin_0(&self) -> bool {
    *self == PIN_A::PIN_0
  }
  #[doc = "Checks if the value of the field is `PIN_1`"]
  #[inline(always)]
  pub fn is_pin_1(&self) -> bool {
    *self == PIN_A::PIN_1
  }
}
#[doc = "Write proxy for field `PIN`"]
pub struct PIN_W<'a> {
  w: &'a mut W,
}
impl<'a> PIN_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: PIN_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Reset source not pending."]
  #[inline(always)]
  pub fn pin_0(self) -> &'a mut W {
    self.variant(PIN_A::PIN_0)
  }
  #[doc = "Reset source pending."]
  #[inline(always)]
  pub fn pin_1(self) -> &'a mut W {
    self.variant(PIN_A::PIN_1)
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
#[doc = "MDM Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MDM_A {
  #[doc = "0: Reset source not pending."]
  MDM_0,
  #[doc = "1: Reset source pending."]
  MDM_1,
}
impl From<MDM_A> for bool {
  #[inline(always)]
  fn from(variant: MDM_A) -> Self {
    match variant {
      MDM_A::MDM_0 => false,
      MDM_A::MDM_1 => true,
    }
  }
}
#[doc = "Reader of field `MDM`"]
pub type MDM_R = crate::R<bool, MDM_A>;
impl MDM_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> MDM_A {
    match self.bits {
      false => MDM_A::MDM_0,
      true => MDM_A::MDM_1,
    }
  }
  #[doc = "Checks if the value of the field is `MDM_0`"]
  #[inline(always)]
  pub fn is_mdm_0(&self) -> bool {
    *self == MDM_A::MDM_0
  }
  #[doc = "Checks if the value of the field is `MDM_1`"]
  #[inline(always)]
  pub fn is_mdm_1(&self) -> bool {
    *self == MDM_A::MDM_1
  }
}
#[doc = "Write proxy for field `MDM`"]
pub struct MDM_W<'a> {
  w: &'a mut W,
}
impl<'a> MDM_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: MDM_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Reset source not pending."]
  #[inline(always)]
  pub fn mdm_0(self) -> &'a mut W {
    self.variant(MDM_A::MDM_0)
  }
  #[doc = "Reset source pending."]
  #[inline(always)]
  pub fn mdm_1(self) -> &'a mut W {
    self.variant(MDM_A::MDM_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
    self.w
  }
}
#[doc = "Stop Timeout Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STOPACK_A {
  #[doc = "0: Reset source not pending."]
  STOPACK_0,
  #[doc = "1: Reset source pending."]
  STOPACK_1,
}
impl From<STOPACK_A> for bool {
  #[inline(always)]
  fn from(variant: STOPACK_A) -> Self {
    match variant {
      STOPACK_A::STOPACK_0 => false,
      STOPACK_A::STOPACK_1 => true,
    }
  }
}
#[doc = "Reader of field `STOPACK`"]
pub type STOPACK_R = crate::R<bool, STOPACK_A>;
impl STOPACK_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> STOPACK_A {
    match self.bits {
      false => STOPACK_A::STOPACK_0,
      true => STOPACK_A::STOPACK_1,
    }
  }
  #[doc = "Checks if the value of the field is `STOPACK_0`"]
  #[inline(always)]
  pub fn is_stopack_0(&self) -> bool {
    *self == STOPACK_A::STOPACK_0
  }
  #[doc = "Checks if the value of the field is `STOPACK_1`"]
  #[inline(always)]
  pub fn is_stopack_1(&self) -> bool {
    *self == STOPACK_A::STOPACK_1
  }
}
#[doc = "Write proxy for field `STOPACK`"]
pub struct STOPACK_W<'a> {
  w: &'a mut W,
}
impl<'a> STOPACK_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: STOPACK_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Reset source not pending."]
  #[inline(always)]
  pub fn stopack_0(self) -> &'a mut W {
    self.variant(STOPACK_A::STOPACK_0)
  }
  #[doc = "Reset source pending."]
  #[inline(always)]
  pub fn stopack_1(self) -> &'a mut W {
    self.variant(STOPACK_A::STOPACK_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
    self.w
  }
}
#[doc = "Watchdog Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDOG_A {
  #[doc = "0: Reset source not pending."]
  WDOG_0,
  #[doc = "1: Reset source pending."]
  WDOG_1,
}
impl From<WDOG_A> for bool {
  #[inline(always)]
  fn from(variant: WDOG_A) -> Self {
    match variant {
      WDOG_A::WDOG_0 => false,
      WDOG_A::WDOG_1 => true,
    }
  }
}
#[doc = "Reader of field `WDOG`"]
pub type WDOG_R = crate::R<bool, WDOG_A>;
impl WDOG_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> WDOG_A {
    match self.bits {
      false => WDOG_A::WDOG_0,
      true => WDOG_A::WDOG_1,
    }
  }
  #[doc = "Checks if the value of the field is `WDOG_0`"]
  #[inline(always)]
  pub fn is_wdog_0(&self) -> bool {
    *self == WDOG_A::WDOG_0
  }
  #[doc = "Checks if the value of the field is `WDOG_1`"]
  #[inline(always)]
  pub fn is_wdog_1(&self) -> bool {
    *self == WDOG_A::WDOG_1
  }
}
#[doc = "Write proxy for field `WDOG`"]
pub struct WDOG_W<'a> {
  w: &'a mut W,
}
impl<'a> WDOG_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WDOG_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Reset source not pending."]
  #[inline(always)]
  pub fn wdog_0(self) -> &'a mut W {
    self.variant(WDOG_A::WDOG_0)
  }
  #[doc = "Reset source pending."]
  #[inline(always)]
  pub fn wdog_1(self) -> &'a mut W {
    self.variant(WDOG_A::WDOG_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
    self.w
  }
}
#[doc = "Software Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SW_A {
  #[doc = "0: Reset source not pending."]
  SW_0,
  #[doc = "1: Reset source pending."]
  SW_1,
}
impl From<SW_A> for bool {
  #[inline(always)]
  fn from(variant: SW_A) -> Self {
    match variant {
      SW_A::SW_0 => false,
      SW_A::SW_1 => true,
    }
  }
}
#[doc = "Reader of field `SW`"]
pub type SW_R = crate::R<bool, SW_A>;
impl SW_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> SW_A {
    match self.bits {
      false => SW_A::SW_0,
      true => SW_A::SW_1,
    }
  }
  #[doc = "Checks if the value of the field is `SW_0`"]
  #[inline(always)]
  pub fn is_sw_0(&self) -> bool {
    *self == SW_A::SW_0
  }
  #[doc = "Checks if the value of the field is `SW_1`"]
  #[inline(always)]
  pub fn is_sw_1(&self) -> bool {
    *self == SW_A::SW_1
  }
}
#[doc = "Write proxy for field `SW`"]
pub struct SW_W<'a> {
  w: &'a mut W,
}
impl<'a> SW_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: SW_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Reset source not pending."]
  #[inline(always)]
  pub fn sw_0(self) -> &'a mut W {
    self.variant(SW_A::SW_0)
  }
  #[doc = "Reset source pending."]
  #[inline(always)]
  pub fn sw_1(self) -> &'a mut W {
    self.variant(SW_A::SW_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
    self.w
  }
}
#[doc = "Lockup Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCKUP_A {
  #[doc = "0: Reset source not pending."]
  LOCKUP_0,
  #[doc = "1: Reset source pending."]
  LOCKUP_1,
}
impl From<LOCKUP_A> for bool {
  #[inline(always)]
  fn from(variant: LOCKUP_A) -> Self {
    match variant {
      LOCKUP_A::LOCKUP_0 => false,
      LOCKUP_A::LOCKUP_1 => true,
    }
  }
}
#[doc = "Reader of field `LOCKUP`"]
pub type LOCKUP_R = crate::R<bool, LOCKUP_A>;
impl LOCKUP_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> LOCKUP_A {
    match self.bits {
      false => LOCKUP_A::LOCKUP_0,
      true => LOCKUP_A::LOCKUP_1,
    }
  }
  #[doc = "Checks if the value of the field is `LOCKUP_0`"]
  #[inline(always)]
  pub fn is_lockup_0(&self) -> bool {
    *self == LOCKUP_A::LOCKUP_0
  }
  #[doc = "Checks if the value of the field is `LOCKUP_1`"]
  #[inline(always)]
  pub fn is_lockup_1(&self) -> bool {
    *self == LOCKUP_A::LOCKUP_1
  }
}
#[doc = "Write proxy for field `LOCKUP`"]
pub struct LOCKUP_W<'a> {
  w: &'a mut W,
}
impl<'a> LOCKUP_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: LOCKUP_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Reset source not pending."]
  #[inline(always)]
  pub fn lockup_0(self) -> &'a mut W {
    self.variant(LOCKUP_A::LOCKUP_0)
  }
  #[doc = "Reset source pending."]
  #[inline(always)]
  pub fn lockup_1(self) -> &'a mut W {
    self.variant(LOCKUP_A::LOCKUP_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
    self.w
  }
}
#[doc = "Core1 Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CORE1_A {
  #[doc = "0: Reset source not pending."]
  CORE1_0,
  #[doc = "1: Reset source pending."]
  CORE1_1,
}
impl From<CORE1_A> for bool {
  #[inline(always)]
  fn from(variant: CORE1_A) -> Self {
    match variant {
      CORE1_A::CORE1_0 => false,
      CORE1_A::CORE1_1 => true,
    }
  }
}
#[doc = "Reader of field `CORE1`"]
pub type CORE1_R = crate::R<bool, CORE1_A>;
impl CORE1_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> CORE1_A {
    match self.bits {
      false => CORE1_A::CORE1_0,
      true => CORE1_A::CORE1_1,
    }
  }
  #[doc = "Checks if the value of the field is `CORE1_0`"]
  #[inline(always)]
  pub fn is_core1_0(&self) -> bool {
    *self == CORE1_A::CORE1_0
  }
  #[doc = "Checks if the value of the field is `CORE1_1`"]
  #[inline(always)]
  pub fn is_core1_1(&self) -> bool {
    *self == CORE1_A::CORE1_1
  }
}
#[doc = "Write proxy for field `CORE1`"]
pub struct CORE1_W<'a> {
  w: &'a mut W,
}
impl<'a> CORE1_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: CORE1_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Reset source not pending."]
  #[inline(always)]
  pub fn core1_0(self) -> &'a mut W {
    self.variant(CORE1_A::CORE1_0)
  }
  #[doc = "Reset source pending."]
  #[inline(always)]
  pub fn core1_1(self) -> &'a mut W {
    self.variant(CORE1_A::CORE1_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
    self.w
  }
}
impl R {
  #[doc = "Bit 8 - Pin Reset"]
  #[inline(always)]
  pub fn pin(&self) -> PIN_R {
    PIN_R::new(((self.bits >> 8) & 0x01) != 0)
  }
  #[doc = "Bit 9 - MDM Reset"]
  #[inline(always)]
  pub fn mdm(&self) -> MDM_R {
    MDM_R::new(((self.bits >> 9) & 0x01) != 0)
  }
  #[doc = "Bit 11 - Stop Timeout Reset"]
  #[inline(always)]
  pub fn stopack(&self) -> STOPACK_R {
    STOPACK_R::new(((self.bits >> 11) & 0x01) != 0)
  }
  #[doc = "Bit 13 - Watchdog Reset"]
  #[inline(always)]
  pub fn wdog(&self) -> WDOG_R {
    WDOG_R::new(((self.bits >> 13) & 0x01) != 0)
  }
  #[doc = "Bit 14 - Software Reset"]
  #[inline(always)]
  pub fn sw(&self) -> SW_R {
    SW_R::new(((self.bits >> 14) & 0x01) != 0)
  }
  #[doc = "Bit 15 - Lockup Reset"]
  #[inline(always)]
  pub fn lockup(&self) -> LOCKUP_R {
    LOCKUP_R::new(((self.bits >> 15) & 0x01) != 0)
  }
  #[doc = "Bit 17 - Core1 Reset"]
  #[inline(always)]
  pub fn core1(&self) -> CORE1_R {
    CORE1_R::new(((self.bits >> 17) & 0x01) != 0)
  }
}
impl W {
  #[doc = "Bit 8 - Pin Reset"]
  #[inline(always)]
  pub fn pin(&mut self) -> PIN_W {
    PIN_W { w: self }
  }
  #[doc = "Bit 9 - MDM Reset"]
  #[inline(always)]
  pub fn mdm(&mut self) -> MDM_W {
    MDM_W { w: self }
  }
  #[doc = "Bit 11 - Stop Timeout Reset"]
  #[inline(always)]
  pub fn stopack(&mut self) -> STOPACK_W {
    STOPACK_W { w: self }
  }
  #[doc = "Bit 13 - Watchdog Reset"]
  #[inline(always)]
  pub fn wdog(&mut self) -> WDOG_W {
    WDOG_W { w: self }
  }
  #[doc = "Bit 14 - Software Reset"]
  #[inline(always)]
  pub fn sw(&mut self) -> SW_W {
    SW_W { w: self }
  }
  #[doc = "Bit 15 - Lockup Reset"]
  #[inline(always)]
  pub fn lockup(&mut self) -> LOCKUP_W {
    LOCKUP_W { w: self }
  }
  #[doc = "Bit 17 - Core1 Reset"]
  #[inline(always)]
  pub fn core1(&mut self) -> CORE1_W {
    CORE1_W { w: self }
  }
}
