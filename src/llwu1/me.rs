#[doc = "Reader of register ME"]
pub type R = crate::R<u32, super::ME>;
#[doc = "Writer for register ME"]
pub type W = crate::W<u32, super::ME>;
#[doc = "Register ME `reset()`'s with value 0"]
impl crate::ResetValue for super::ME {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "Wakeup module enable for module n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUME0_A {
  #[doc = "0: Internal module flag not used as wakeup source"]
  WUME0_0,
  #[doc = "1: Internal module flag used as wakeup source"]
  WUME0_1,
}
impl From<WUME0_A> for bool {
  #[inline(always)]
  fn from(variant: WUME0_A) -> Self {
    match variant {
      WUME0_A::WUME0_0 => false,
      WUME0_A::WUME0_1 => true,
    }
  }
}
#[doc = "Reader of field `WUME0`"]
pub type WUME0_R = crate::R<bool, WUME0_A>;
impl WUME0_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> WUME0_A {
    match self.bits {
      false => WUME0_A::WUME0_0,
      true => WUME0_A::WUME0_1,
    }
  }
  #[doc = "Checks if the value of the field is `WUME0_0`"]
  #[inline(always)]
  pub fn is_wume0_0(&self) -> bool {
    *self == WUME0_A::WUME0_0
  }
  #[doc = "Checks if the value of the field is `WUME0_1`"]
  #[inline(always)]
  pub fn is_wume0_1(&self) -> bool {
    *self == WUME0_A::WUME0_1
  }
}
#[doc = "Write proxy for field `WUME0`"]
pub struct WUME0_W<'a> {
  w: &'a mut W,
}
impl<'a> WUME0_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUME0_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Internal module flag not used as wakeup source"]
  #[inline(always)]
  pub fn wume0_0(self) -> &'a mut W {
    self.variant(WUME0_A::WUME0_0)
  }
  #[doc = "Internal module flag used as wakeup source"]
  #[inline(always)]
  pub fn wume0_1(self) -> &'a mut W {
    self.variant(WUME0_A::WUME0_1)
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
#[doc = "Wakeup module enable for module n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUME1_A {
  #[doc = "0: Internal module flag not used as wakeup source"]
  WUME1_0,
  #[doc = "1: Internal module flag used as wakeup source"]
  WUME1_1,
}
impl From<WUME1_A> for bool {
  #[inline(always)]
  fn from(variant: WUME1_A) -> Self {
    match variant {
      WUME1_A::WUME1_0 => false,
      WUME1_A::WUME1_1 => true,
    }
  }
}
#[doc = "Reader of field `WUME1`"]
pub type WUME1_R = crate::R<bool, WUME1_A>;
impl WUME1_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> WUME1_A {
    match self.bits {
      false => WUME1_A::WUME1_0,
      true => WUME1_A::WUME1_1,
    }
  }
  #[doc = "Checks if the value of the field is `WUME1_0`"]
  #[inline(always)]
  pub fn is_wume1_0(&self) -> bool {
    *self == WUME1_A::WUME1_0
  }
  #[doc = "Checks if the value of the field is `WUME1_1`"]
  #[inline(always)]
  pub fn is_wume1_1(&self) -> bool {
    *self == WUME1_A::WUME1_1
  }
}
#[doc = "Write proxy for field `WUME1`"]
pub struct WUME1_W<'a> {
  w: &'a mut W,
}
impl<'a> WUME1_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUME1_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Internal module flag not used as wakeup source"]
  #[inline(always)]
  pub fn wume1_0(self) -> &'a mut W {
    self.variant(WUME1_A::WUME1_0)
  }
  #[doc = "Internal module flag used as wakeup source"]
  #[inline(always)]
  pub fn wume1_1(self) -> &'a mut W {
    self.variant(WUME1_A::WUME1_1)
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
#[doc = "Wakeup module enable for module n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUME2_A {
  #[doc = "0: Internal module flag not used as wakeup source"]
  WUME2_0,
  #[doc = "1: Internal module flag used as wakeup source"]
  WUME2_1,
}
impl From<WUME2_A> for bool {
  #[inline(always)]
  fn from(variant: WUME2_A) -> Self {
    match variant {
      WUME2_A::WUME2_0 => false,
      WUME2_A::WUME2_1 => true,
    }
  }
}
#[doc = "Reader of field `WUME2`"]
pub type WUME2_R = crate::R<bool, WUME2_A>;
impl WUME2_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> WUME2_A {
    match self.bits {
      false => WUME2_A::WUME2_0,
      true => WUME2_A::WUME2_1,
    }
  }
  #[doc = "Checks if the value of the field is `WUME2_0`"]
  #[inline(always)]
  pub fn is_wume2_0(&self) -> bool {
    *self == WUME2_A::WUME2_0
  }
  #[doc = "Checks if the value of the field is `WUME2_1`"]
  #[inline(always)]
  pub fn is_wume2_1(&self) -> bool {
    *self == WUME2_A::WUME2_1
  }
}
#[doc = "Write proxy for field `WUME2`"]
pub struct WUME2_W<'a> {
  w: &'a mut W,
}
impl<'a> WUME2_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUME2_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Internal module flag not used as wakeup source"]
  #[inline(always)]
  pub fn wume2_0(self) -> &'a mut W {
    self.variant(WUME2_A::WUME2_0)
  }
  #[doc = "Internal module flag used as wakeup source"]
  #[inline(always)]
  pub fn wume2_1(self) -> &'a mut W {
    self.variant(WUME2_A::WUME2_1)
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
#[doc = "Wakeup module enable for module n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUME3_A {
  #[doc = "0: Internal module flag not used as wakeup source"]
  WUME3_0,
  #[doc = "1: Internal module flag used as wakeup source"]
  WUME3_1,
}
impl From<WUME3_A> for bool {
  #[inline(always)]
  fn from(variant: WUME3_A) -> Self {
    match variant {
      WUME3_A::WUME3_0 => false,
      WUME3_A::WUME3_1 => true,
    }
  }
}
#[doc = "Reader of field `WUME3`"]
pub type WUME3_R = crate::R<bool, WUME3_A>;
impl WUME3_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> WUME3_A {
    match self.bits {
      false => WUME3_A::WUME3_0,
      true => WUME3_A::WUME3_1,
    }
  }
  #[doc = "Checks if the value of the field is `WUME3_0`"]
  #[inline(always)]
  pub fn is_wume3_0(&self) -> bool {
    *self == WUME3_A::WUME3_0
  }
  #[doc = "Checks if the value of the field is `WUME3_1`"]
  #[inline(always)]
  pub fn is_wume3_1(&self) -> bool {
    *self == WUME3_A::WUME3_1
  }
}
#[doc = "Write proxy for field `WUME3`"]
pub struct WUME3_W<'a> {
  w: &'a mut W,
}
impl<'a> WUME3_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUME3_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Internal module flag not used as wakeup source"]
  #[inline(always)]
  pub fn wume3_0(self) -> &'a mut W {
    self.variant(WUME3_A::WUME3_0)
  }
  #[doc = "Internal module flag used as wakeup source"]
  #[inline(always)]
  pub fn wume3_1(self) -> &'a mut W {
    self.variant(WUME3_A::WUME3_1)
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
#[doc = "Wakeup module enable for module n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESERVED4_A {
  #[doc = "0: Internal module flag not used as wakeup source"]
  RESERVED4_0,
  #[doc = "1: Internal module flag used as wakeup source"]
  RESERVED4_1,
}
impl From<RESERVED4_A> for bool {
  #[inline(always)]
  fn from(variant: RESERVED4_A) -> Self {
    match variant {
      RESERVED4_A::RESERVED4_0 => false,
      RESERVED4_A::RESERVED4_1 => true,
    }
  }
}
#[doc = "Reader of field `Reserved4`"]
pub type RESERVED4_R = crate::R<bool, RESERVED4_A>;
impl RESERVED4_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> RESERVED4_A {
    match self.bits {
      false => RESERVED4_A::RESERVED4_0,
      true => RESERVED4_A::RESERVED4_1,
    }
  }
  #[doc = "Checks if the value of the field is `RESERVED4_0`"]
  #[inline(always)]
  pub fn is_reserved4_0(&self) -> bool {
    *self == RESERVED4_A::RESERVED4_0
  }
  #[doc = "Checks if the value of the field is `RESERVED4_1`"]
  #[inline(always)]
  pub fn is_reserved4_1(&self) -> bool {
    *self == RESERVED4_A::RESERVED4_1
  }
}
#[doc = "Wakeup module enable for module n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUME5_A {
  #[doc = "0: Internal module flag not used as wakeup source"]
  WUME5_0,
  #[doc = "1: Internal module flag used as wakeup source"]
  WUME5_1,
}
impl From<WUME5_A> for bool {
  #[inline(always)]
  fn from(variant: WUME5_A) -> Self {
    match variant {
      WUME5_A::WUME5_0 => false,
      WUME5_A::WUME5_1 => true,
    }
  }
}
#[doc = "Reader of field `WUME5`"]
pub type WUME5_R = crate::R<bool, WUME5_A>;
impl WUME5_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> WUME5_A {
    match self.bits {
      false => WUME5_A::WUME5_0,
      true => WUME5_A::WUME5_1,
    }
  }
  #[doc = "Checks if the value of the field is `WUME5_0`"]
  #[inline(always)]
  pub fn is_wume5_0(&self) -> bool {
    *self == WUME5_A::WUME5_0
  }
  #[doc = "Checks if the value of the field is `WUME5_1`"]
  #[inline(always)]
  pub fn is_wume5_1(&self) -> bool {
    *self == WUME5_A::WUME5_1
  }
}
#[doc = "Write proxy for field `WUME5`"]
pub struct WUME5_W<'a> {
  w: &'a mut W,
}
impl<'a> WUME5_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUME5_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Internal module flag not used as wakeup source"]
  #[inline(always)]
  pub fn wume5_0(self) -> &'a mut W {
    self.variant(WUME5_A::WUME5_0)
  }
  #[doc = "Internal module flag used as wakeup source"]
  #[inline(always)]
  pub fn wume5_1(self) -> &'a mut W {
    self.variant(WUME5_A::WUME5_1)
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
#[doc = "Wakeup module enable for module n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUME6_A {
  #[doc = "0: Internal module flag not used as wakeup source"]
  WUME6_0,
  #[doc = "1: Internal module flag used as wakeup source"]
  WUME6_1,
}
impl From<WUME6_A> for bool {
  #[inline(always)]
  fn from(variant: WUME6_A) -> Self {
    match variant {
      WUME6_A::WUME6_0 => false,
      WUME6_A::WUME6_1 => true,
    }
  }
}
#[doc = "Reader of field `WUME6`"]
pub type WUME6_R = crate::R<bool, WUME6_A>;
impl WUME6_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> WUME6_A {
    match self.bits {
      false => WUME6_A::WUME6_0,
      true => WUME6_A::WUME6_1,
    }
  }
  #[doc = "Checks if the value of the field is `WUME6_0`"]
  #[inline(always)]
  pub fn is_wume6_0(&self) -> bool {
    *self == WUME6_A::WUME6_0
  }
  #[doc = "Checks if the value of the field is `WUME6_1`"]
  #[inline(always)]
  pub fn is_wume6_1(&self) -> bool {
    *self == WUME6_A::WUME6_1
  }
}
#[doc = "Write proxy for field `WUME6`"]
pub struct WUME6_W<'a> {
  w: &'a mut W,
}
impl<'a> WUME6_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUME6_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Internal module flag not used as wakeup source"]
  #[inline(always)]
  pub fn wume6_0(self) -> &'a mut W {
    self.variant(WUME6_A::WUME6_0)
  }
  #[doc = "Internal module flag used as wakeup source"]
  #[inline(always)]
  pub fn wume6_1(self) -> &'a mut W {
    self.variant(WUME6_A::WUME6_1)
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
#[doc = "Wakeup module enable for module n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUME7_A {
  #[doc = "0: Internal module flag not used as wakeup source"]
  WUME7_0,
  #[doc = "1: Internal module flag used as wakeup source"]
  WUME7_1,
}
impl From<WUME7_A> for bool {
  #[inline(always)]
  fn from(variant: WUME7_A) -> Self {
    match variant {
      WUME7_A::WUME7_0 => false,
      WUME7_A::WUME7_1 => true,
    }
  }
}
#[doc = "Reader of field `WUME7`"]
pub type WUME7_R = crate::R<bool, WUME7_A>;
impl WUME7_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> WUME7_A {
    match self.bits {
      false => WUME7_A::WUME7_0,
      true => WUME7_A::WUME7_1,
    }
  }
  #[doc = "Checks if the value of the field is `WUME7_0`"]
  #[inline(always)]
  pub fn is_wume7_0(&self) -> bool {
    *self == WUME7_A::WUME7_0
  }
  #[doc = "Checks if the value of the field is `WUME7_1`"]
  #[inline(always)]
  pub fn is_wume7_1(&self) -> bool {
    *self == WUME7_A::WUME7_1
  }
}
#[doc = "Write proxy for field `WUME7`"]
pub struct WUME7_W<'a> {
  w: &'a mut W,
}
impl<'a> WUME7_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUME7_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Internal module flag not used as wakeup source"]
  #[inline(always)]
  pub fn wume7_0(self) -> &'a mut W {
    self.variant(WUME7_A::WUME7_0)
  }
  #[doc = "Internal module flag used as wakeup source"]
  #[inline(always)]
  pub fn wume7_1(self) -> &'a mut W {
    self.variant(WUME7_A::WUME7_1)
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
impl R {
  #[doc = "Bit 0 - Wakeup module enable for module n"]
  #[inline(always)]
  pub fn wume0(&self) -> WUME0_R {
    WUME0_R::new((self.bits & 0x01) != 0)
  }
  #[doc = "Bit 1 - Wakeup module enable for module n"]
  #[inline(always)]
  pub fn wume1(&self) -> WUME1_R {
    WUME1_R::new(((self.bits >> 1) & 0x01) != 0)
  }
  #[doc = "Bit 2 - Wakeup module enable for module n"]
  #[inline(always)]
  pub fn wume2(&self) -> WUME2_R {
    WUME2_R::new(((self.bits >> 2) & 0x01) != 0)
  }
  #[doc = "Bit 3 - Wakeup module enable for module n"]
  #[inline(always)]
  pub fn wume3(&self) -> WUME3_R {
    WUME3_R::new(((self.bits >> 3) & 0x01) != 0)
  }
  #[doc = "Bit 4 - Wakeup module enable for module n"]
  #[inline(always)]
  pub fn reserved4(&self) -> RESERVED4_R {
    RESERVED4_R::new(((self.bits >> 4) & 0x01) != 0)
  }
  #[doc = "Bit 5 - Wakeup module enable for module n"]
  #[inline(always)]
  pub fn wume5(&self) -> WUME5_R {
    WUME5_R::new(((self.bits >> 5) & 0x01) != 0)
  }
  #[doc = "Bit 6 - Wakeup module enable for module n"]
  #[inline(always)]
  pub fn wume6(&self) -> WUME6_R {
    WUME6_R::new(((self.bits >> 6) & 0x01) != 0)
  }
  #[doc = "Bit 7 - Wakeup module enable for module n"]
  #[inline(always)]
  pub fn wume7(&self) -> WUME7_R {
    WUME7_R::new(((self.bits >> 7) & 0x01) != 0)
  }
}
impl W {
  #[doc = "Bit 0 - Wakeup module enable for module n"]
  #[inline(always)]
  pub fn wume0(&mut self) -> WUME0_W {
    WUME0_W { w: self }
  }
  #[doc = "Bit 1 - Wakeup module enable for module n"]
  #[inline(always)]
  pub fn wume1(&mut self) -> WUME1_W {
    WUME1_W { w: self }
  }
  #[doc = "Bit 2 - Wakeup module enable for module n"]
  #[inline(always)]
  pub fn wume2(&mut self) -> WUME2_W {
    WUME2_W { w: self }
  }
  #[doc = "Bit 3 - Wakeup module enable for module n"]
  #[inline(always)]
  pub fn wume3(&mut self) -> WUME3_W {
    WUME3_W { w: self }
  }
  #[doc = "Bit 5 - Wakeup module enable for module n"]
  #[inline(always)]
  pub fn wume5(&mut self) -> WUME5_W {
    WUME5_W { w: self }
  }
  #[doc = "Bit 6 - Wakeup module enable for module n"]
  #[inline(always)]
  pub fn wume6(&mut self) -> WUME6_W {
    WUME6_W { w: self }
  }
  #[doc = "Bit 7 - Wakeup module enable for module n"]
  #[inline(always)]
  pub fn wume7(&mut self) -> WUME7_W {
    WUME7_W { w: self }
  }
}
