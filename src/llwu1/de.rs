#[doc = "Reader of register DE"]
pub type R = crate::R<u32, super::DE>;
#[doc = "Writer for register DE"]
pub type W = crate::W<u32, super::DE>;
#[doc = "Register DE `reset()`'s with value 0"]
impl crate::ResetValue for super::DE {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "DMA/Trigger wakeup enable for module n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUDE0_A {
  #[doc = "0: Internal module request not enabled as a DMA/Trigger wakeup source"]
  WUDE0_0,
  #[doc = "1: Internal module request enabled as a DMA/Trigger wakeup source"]
  WUDE0_1,
}
impl From<WUDE0_A> for bool {
  #[inline(always)]
  fn from(variant: WUDE0_A) -> Self {
    match variant {
      WUDE0_A::WUDE0_0 => false,
      WUDE0_A::WUDE0_1 => true,
    }
  }
}
#[doc = "Reader of field `WUDE0`"]
pub type WUDE0_R = crate::R<bool, WUDE0_A>;
impl WUDE0_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> WUDE0_A {
    match self.bits {
      false => WUDE0_A::WUDE0_0,
      true => WUDE0_A::WUDE0_1,
    }
  }
  #[doc = "Checks if the value of the field is `WUDE0_0`"]
  #[inline(always)]
  pub fn is_wude0_0(&self) -> bool {
    *self == WUDE0_A::WUDE0_0
  }
  #[doc = "Checks if the value of the field is `WUDE0_1`"]
  #[inline(always)]
  pub fn is_wude0_1(&self) -> bool {
    *self == WUDE0_A::WUDE0_1
  }
}
#[doc = "Write proxy for field `WUDE0`"]
pub struct WUDE0_W<'a> {
  w: &'a mut W,
}
impl<'a> WUDE0_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUDE0_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Internal module request not enabled as a DMA/Trigger wakeup source"]
  #[inline(always)]
  pub fn wude0_0(self) -> &'a mut W {
    self.variant(WUDE0_A::WUDE0_0)
  }
  #[doc = "Internal module request enabled as a DMA/Trigger wakeup source"]
  #[inline(always)]
  pub fn wude0_1(self) -> &'a mut W {
    self.variant(WUDE0_A::WUDE0_1)
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
#[doc = "DMA/Trigger wakeup enable for module n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUDE1_A {
  #[doc = "0: Internal module request not enabled as a DMA/Trigger wakeup source"]
  WUDE1_0,
  #[doc = "1: Internal module request enabled as a DMA/Trigger wakeup source"]
  WUDE1_1,
}
impl From<WUDE1_A> for bool {
  #[inline(always)]
  fn from(variant: WUDE1_A) -> Self {
    match variant {
      WUDE1_A::WUDE1_0 => false,
      WUDE1_A::WUDE1_1 => true,
    }
  }
}
#[doc = "Reader of field `WUDE1`"]
pub type WUDE1_R = crate::R<bool, WUDE1_A>;
impl WUDE1_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> WUDE1_A {
    match self.bits {
      false => WUDE1_A::WUDE1_0,
      true => WUDE1_A::WUDE1_1,
    }
  }
  #[doc = "Checks if the value of the field is `WUDE1_0`"]
  #[inline(always)]
  pub fn is_wude1_0(&self) -> bool {
    *self == WUDE1_A::WUDE1_0
  }
  #[doc = "Checks if the value of the field is `WUDE1_1`"]
  #[inline(always)]
  pub fn is_wude1_1(&self) -> bool {
    *self == WUDE1_A::WUDE1_1
  }
}
#[doc = "Write proxy for field `WUDE1`"]
pub struct WUDE1_W<'a> {
  w: &'a mut W,
}
impl<'a> WUDE1_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUDE1_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Internal module request not enabled as a DMA/Trigger wakeup source"]
  #[inline(always)]
  pub fn wude1_0(self) -> &'a mut W {
    self.variant(WUDE1_A::WUDE1_0)
  }
  #[doc = "Internal module request enabled as a DMA/Trigger wakeup source"]
  #[inline(always)]
  pub fn wude1_1(self) -> &'a mut W {
    self.variant(WUDE1_A::WUDE1_1)
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
#[doc = "DMA/Trigger wakeup enable for module n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUDE2_A {
  #[doc = "0: Internal module request not enabled as a DMA/Trigger wakeup source"]
  WUDE2_0,
  #[doc = "1: Internal module request enabled as a DMA/Trigger wakeup source"]
  WUDE2_1,
}
impl From<WUDE2_A> for bool {
  #[inline(always)]
  fn from(variant: WUDE2_A) -> Self {
    match variant {
      WUDE2_A::WUDE2_0 => false,
      WUDE2_A::WUDE2_1 => true,
    }
  }
}
#[doc = "Reader of field `WUDE2`"]
pub type WUDE2_R = crate::R<bool, WUDE2_A>;
impl WUDE2_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> WUDE2_A {
    match self.bits {
      false => WUDE2_A::WUDE2_0,
      true => WUDE2_A::WUDE2_1,
    }
  }
  #[doc = "Checks if the value of the field is `WUDE2_0`"]
  #[inline(always)]
  pub fn is_wude2_0(&self) -> bool {
    *self == WUDE2_A::WUDE2_0
  }
  #[doc = "Checks if the value of the field is `WUDE2_1`"]
  #[inline(always)]
  pub fn is_wude2_1(&self) -> bool {
    *self == WUDE2_A::WUDE2_1
  }
}
#[doc = "Write proxy for field `WUDE2`"]
pub struct WUDE2_W<'a> {
  w: &'a mut W,
}
impl<'a> WUDE2_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUDE2_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Internal module request not enabled as a DMA/Trigger wakeup source"]
  #[inline(always)]
  pub fn wude2_0(self) -> &'a mut W {
    self.variant(WUDE2_A::WUDE2_0)
  }
  #[doc = "Internal module request enabled as a DMA/Trigger wakeup source"]
  #[inline(always)]
  pub fn wude2_1(self) -> &'a mut W {
    self.variant(WUDE2_A::WUDE2_1)
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
#[doc = "DMA/Trigger wakeup enable for module n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESERVED3_A {
  #[doc = "0: Internal module request not enabled as a DMA/Trigger wakeup source"]
  RESERVED3_0,
  #[doc = "1: Internal module request enabled as a DMA/Trigger wakeup source"]
  RESERVED3_1,
}
impl From<RESERVED3_A> for bool {
  #[inline(always)]
  fn from(variant: RESERVED3_A) -> Self {
    match variant {
      RESERVED3_A::RESERVED3_0 => false,
      RESERVED3_A::RESERVED3_1 => true,
    }
  }
}
#[doc = "Reader of field `Reserved3`"]
pub type RESERVED3_R = crate::R<bool, RESERVED3_A>;
impl RESERVED3_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> RESERVED3_A {
    match self.bits {
      false => RESERVED3_A::RESERVED3_0,
      true => RESERVED3_A::RESERVED3_1,
    }
  }
  #[doc = "Checks if the value of the field is `RESERVED3_0`"]
  #[inline(always)]
  pub fn is_reserved3_0(&self) -> bool {
    *self == RESERVED3_A::RESERVED3_0
  }
  #[doc = "Checks if the value of the field is `RESERVED3_1`"]
  #[inline(always)]
  pub fn is_reserved3_1(&self) -> bool {
    *self == RESERVED3_A::RESERVED3_1
  }
}
#[doc = "DMA/Trigger wakeup enable for module n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUDE4_A {
  #[doc = "0: Internal module request not enabled as a DMA/Trigger wakeup source"]
  WUDE4_0,
  #[doc = "1: Internal module request enabled as a DMA/Trigger wakeup source"]
  WUDE4_1,
}
impl From<WUDE4_A> for bool {
  #[inline(always)]
  fn from(variant: WUDE4_A) -> Self {
    match variant {
      WUDE4_A::WUDE4_0 => false,
      WUDE4_A::WUDE4_1 => true,
    }
  }
}
#[doc = "Reader of field `WUDE4`"]
pub type WUDE4_R = crate::R<bool, WUDE4_A>;
impl WUDE4_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> WUDE4_A {
    match self.bits {
      false => WUDE4_A::WUDE4_0,
      true => WUDE4_A::WUDE4_1,
    }
  }
  #[doc = "Checks if the value of the field is `WUDE4_0`"]
  #[inline(always)]
  pub fn is_wude4_0(&self) -> bool {
    *self == WUDE4_A::WUDE4_0
  }
  #[doc = "Checks if the value of the field is `WUDE4_1`"]
  #[inline(always)]
  pub fn is_wude4_1(&self) -> bool {
    *self == WUDE4_A::WUDE4_1
  }
}
#[doc = "Write proxy for field `WUDE4`"]
pub struct WUDE4_W<'a> {
  w: &'a mut W,
}
impl<'a> WUDE4_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUDE4_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Internal module request not enabled as a DMA/Trigger wakeup source"]
  #[inline(always)]
  pub fn wude4_0(self) -> &'a mut W {
    self.variant(WUDE4_A::WUDE4_0)
  }
  #[doc = "Internal module request enabled as a DMA/Trigger wakeup source"]
  #[inline(always)]
  pub fn wude4_1(self) -> &'a mut W {
    self.variant(WUDE4_A::WUDE4_1)
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
#[doc = "DMA/Trigger wakeup enable for module n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUDE5_A {
  #[doc = "0: Internal module request not enabled as a DMA/Trigger wakeup source"]
  WUDE5_0,
  #[doc = "1: Internal module request enabled as a DMA/Trigger wakeup source"]
  WUDE5_1,
}
impl From<WUDE5_A> for bool {
  #[inline(always)]
  fn from(variant: WUDE5_A) -> Self {
    match variant {
      WUDE5_A::WUDE5_0 => false,
      WUDE5_A::WUDE5_1 => true,
    }
  }
}
#[doc = "Reader of field `WUDE5`"]
pub type WUDE5_R = crate::R<bool, WUDE5_A>;
impl WUDE5_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> WUDE5_A {
    match self.bits {
      false => WUDE5_A::WUDE5_0,
      true => WUDE5_A::WUDE5_1,
    }
  }
  #[doc = "Checks if the value of the field is `WUDE5_0`"]
  #[inline(always)]
  pub fn is_wude5_0(&self) -> bool {
    *self == WUDE5_A::WUDE5_0
  }
  #[doc = "Checks if the value of the field is `WUDE5_1`"]
  #[inline(always)]
  pub fn is_wude5_1(&self) -> bool {
    *self == WUDE5_A::WUDE5_1
  }
}
#[doc = "Write proxy for field `WUDE5`"]
pub struct WUDE5_W<'a> {
  w: &'a mut W,
}
impl<'a> WUDE5_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUDE5_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Internal module request not enabled as a DMA/Trigger wakeup source"]
  #[inline(always)]
  pub fn wude5_0(self) -> &'a mut W {
    self.variant(WUDE5_A::WUDE5_0)
  }
  #[doc = "Internal module request enabled as a DMA/Trigger wakeup source"]
  #[inline(always)]
  pub fn wude5_1(self) -> &'a mut W {
    self.variant(WUDE5_A::WUDE5_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
    self.w
  }
}
#[doc = "DMA/Trigger wakeup enable for module n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUDE6_A {
  #[doc = "0: Internal module request not enabled as a DMA/Trigger wakeup source"]
  WUDE6_0,
  #[doc = "1: Internal module request enabled as a DMA/Trigger wakeup source"]
  WUDE6_1,
}
impl From<WUDE6_A> for bool {
  #[inline(always)]
  fn from(variant: WUDE6_A) -> Self {
    match variant {
      WUDE6_A::WUDE6_0 => false,
      WUDE6_A::WUDE6_1 => true,
    }
  }
}
#[doc = "Reader of field `WUDE6`"]
pub type WUDE6_R = crate::R<bool, WUDE6_A>;
impl WUDE6_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> WUDE6_A {
    match self.bits {
      false => WUDE6_A::WUDE6_0,
      true => WUDE6_A::WUDE6_1,
    }
  }
  #[doc = "Checks if the value of the field is `WUDE6_0`"]
  #[inline(always)]
  pub fn is_wude6_0(&self) -> bool {
    *self == WUDE6_A::WUDE6_0
  }
  #[doc = "Checks if the value of the field is `WUDE6_1`"]
  #[inline(always)]
  pub fn is_wude6_1(&self) -> bool {
    *self == WUDE6_A::WUDE6_1
  }
}
#[doc = "Write proxy for field `WUDE6`"]
pub struct WUDE6_W<'a> {
  w: &'a mut W,
}
impl<'a> WUDE6_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUDE6_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Internal module request not enabled as a DMA/Trigger wakeup source"]
  #[inline(always)]
  pub fn wude6_0(self) -> &'a mut W {
    self.variant(WUDE6_A::WUDE6_0)
  }
  #[doc = "Internal module request enabled as a DMA/Trigger wakeup source"]
  #[inline(always)]
  pub fn wude6_1(self) -> &'a mut W {
    self.variant(WUDE6_A::WUDE6_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
    self.w
  }
}
#[doc = "DMA/Trigger wakeup enable for module n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESERVED7_A {
  #[doc = "0: Internal module request not enabled as a DMA/Trigger wakeup source"]
  RESERVED7_0,
  #[doc = "1: Internal module request enabled as a DMA/Trigger wakeup source"]
  RESERVED7_1,
}
impl From<RESERVED7_A> for bool {
  #[inline(always)]
  fn from(variant: RESERVED7_A) -> Self {
    match variant {
      RESERVED7_A::RESERVED7_0 => false,
      RESERVED7_A::RESERVED7_1 => true,
    }
  }
}
#[doc = "Reader of field `Reserved7`"]
pub type RESERVED7_R = crate::R<bool, RESERVED7_A>;
impl RESERVED7_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> RESERVED7_A {
    match self.bits {
      false => RESERVED7_A::RESERVED7_0,
      true => RESERVED7_A::RESERVED7_1,
    }
  }
  #[doc = "Checks if the value of the field is `RESERVED7_0`"]
  #[inline(always)]
  pub fn is_reserved7_0(&self) -> bool {
    *self == RESERVED7_A::RESERVED7_0
  }
  #[doc = "Checks if the value of the field is `RESERVED7_1`"]
  #[inline(always)]
  pub fn is_reserved7_1(&self) -> bool {
    *self == RESERVED7_A::RESERVED7_1
  }
}
impl R {
  #[doc = "Bit 0 - DMA/Trigger wakeup enable for module n"]
  #[inline(always)]
  pub fn wude0(&self) -> WUDE0_R {
    WUDE0_R::new((self.bits & 0x01) != 0)
  }
  #[doc = "Bit 1 - DMA/Trigger wakeup enable for module n"]
  #[inline(always)]
  pub fn wude1(&self) -> WUDE1_R {
    WUDE1_R::new(((self.bits >> 1) & 0x01) != 0)
  }
  #[doc = "Bit 2 - DMA/Trigger wakeup enable for module n"]
  #[inline(always)]
  pub fn wude2(&self) -> WUDE2_R {
    WUDE2_R::new(((self.bits >> 2) & 0x01) != 0)
  }
  #[doc = "Bit 3 - DMA/Trigger wakeup enable for module n"]
  #[inline(always)]
  pub fn reserved3(&self) -> RESERVED3_R {
    RESERVED3_R::new(((self.bits >> 3) & 0x01) != 0)
  }
  #[doc = "Bit 4 - DMA/Trigger wakeup enable for module n"]
  #[inline(always)]
  pub fn wude4(&self) -> WUDE4_R {
    WUDE4_R::new(((self.bits >> 4) & 0x01) != 0)
  }
  #[doc = "Bit 5 - DMA/Trigger wakeup enable for module n"]
  #[inline(always)]
  pub fn wude5(&self) -> WUDE5_R {
    WUDE5_R::new(((self.bits >> 5) & 0x01) != 0)
  }
  #[doc = "Bit 6 - DMA/Trigger wakeup enable for module n"]
  #[inline(always)]
  pub fn wude6(&self) -> WUDE6_R {
    WUDE6_R::new(((self.bits >> 6) & 0x01) != 0)
  }
  #[doc = "Bit 7 - DMA/Trigger wakeup enable for module n"]
  #[inline(always)]
  pub fn reserved7(&self) -> RESERVED7_R {
    RESERVED7_R::new(((self.bits >> 7) & 0x01) != 0)
  }
}
impl W {
  #[doc = "Bit 0 - DMA/Trigger wakeup enable for module n"]
  #[inline(always)]
  pub fn wude0(&mut self) -> WUDE0_W {
    WUDE0_W { w: self }
  }
  #[doc = "Bit 1 - DMA/Trigger wakeup enable for module n"]
  #[inline(always)]
  pub fn wude1(&mut self) -> WUDE1_W {
    WUDE1_W { w: self }
  }
  #[doc = "Bit 2 - DMA/Trigger wakeup enable for module n"]
  #[inline(always)]
  pub fn wude2(&mut self) -> WUDE2_W {
    WUDE2_W { w: self }
  }
  #[doc = "Bit 4 - DMA/Trigger wakeup enable for module n"]
  #[inline(always)]
  pub fn wude4(&mut self) -> WUDE4_W {
    WUDE4_W { w: self }
  }
  #[doc = "Bit 5 - DMA/Trigger wakeup enable for module n"]
  #[inline(always)]
  pub fn wude5(&mut self) -> WUDE5_W {
    WUDE5_W { w: self }
  }
  #[doc = "Bit 6 - DMA/Trigger wakeup enable for module n"]
  #[inline(always)]
  pub fn wude6(&mut self) -> WUDE6_W {
    WUDE6_W { w: self }
  }
}
