#[doc = "Reader of register PF"]
pub type R = crate::R<u32, super::PF>;
#[doc = "Writer for register PF"]
pub type W = crate::W<u32, super::PF>;
#[doc = "Register PF `reset()`'s with value 0"]
impl crate::ResetValue for super::PF {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "Wakeup flag for LLWU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUF0_A {
  #[doc = "0: LLWU_Pn input was not a wakeup source"]
  WUF0_0,
  #[doc = "1: LLWU_Pn input was a wakeup source"]
  WUF0_1,
}
impl From<WUF0_A> for bool {
  #[inline(always)]
  fn from(variant: WUF0_A) -> Self {
    match variant {
      WUF0_A::WUF0_0 => false,
      WUF0_A::WUF0_1 => true,
    }
  }
}
#[doc = "Reader of field `WUF0`"]
pub type WUF0_R = crate::R<bool, WUF0_A>;
impl WUF0_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> WUF0_A {
    match self.bits {
      false => WUF0_A::WUF0_0,
      true => WUF0_A::WUF0_1,
    }
  }
  #[doc = "Checks if the value of the field is `WUF0_0`"]
  #[inline(always)]
  pub fn is_wuf0_0(&self) -> bool {
    *self == WUF0_A::WUF0_0
  }
  #[doc = "Checks if the value of the field is `WUF0_1`"]
  #[inline(always)]
  pub fn is_wuf0_1(&self) -> bool {
    *self == WUF0_A::WUF0_1
  }
}
#[doc = "Write proxy for field `WUF0`"]
pub struct WUF0_W<'a> {
  w: &'a mut W,
}
impl<'a> WUF0_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUF0_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "LLWU_Pn input was not a wakeup source"]
  #[inline(always)]
  pub fn wuf0_0(self) -> &'a mut W {
    self.variant(WUF0_A::WUF0_0)
  }
  #[doc = "LLWU_Pn input was a wakeup source"]
  #[inline(always)]
  pub fn wuf0_1(self) -> &'a mut W {
    self.variant(WUF0_A::WUF0_1)
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
#[doc = "Wakeup flag for LLWU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUF1_A {
  #[doc = "0: LLWU_Pn input was not a wakeup source"]
  WUF1_0,
  #[doc = "1: LLWU_Pn input was a wakeup source"]
  WUF1_1,
}
impl From<WUF1_A> for bool {
  #[inline(always)]
  fn from(variant: WUF1_A) -> Self {
    match variant {
      WUF1_A::WUF1_0 => false,
      WUF1_A::WUF1_1 => true,
    }
  }
}
#[doc = "Reader of field `WUF1`"]
pub type WUF1_R = crate::R<bool, WUF1_A>;
impl WUF1_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> WUF1_A {
    match self.bits {
      false => WUF1_A::WUF1_0,
      true => WUF1_A::WUF1_1,
    }
  }
  #[doc = "Checks if the value of the field is `WUF1_0`"]
  #[inline(always)]
  pub fn is_wuf1_0(&self) -> bool {
    *self == WUF1_A::WUF1_0
  }
  #[doc = "Checks if the value of the field is `WUF1_1`"]
  #[inline(always)]
  pub fn is_wuf1_1(&self) -> bool {
    *self == WUF1_A::WUF1_1
  }
}
#[doc = "Write proxy for field `WUF1`"]
pub struct WUF1_W<'a> {
  w: &'a mut W,
}
impl<'a> WUF1_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUF1_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "LLWU_Pn input was not a wakeup source"]
  #[inline(always)]
  pub fn wuf1_0(self) -> &'a mut W {
    self.variant(WUF1_A::WUF1_0)
  }
  #[doc = "LLWU_Pn input was a wakeup source"]
  #[inline(always)]
  pub fn wuf1_1(self) -> &'a mut W {
    self.variant(WUF1_A::WUF1_1)
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
#[doc = "Wakeup flag for LLWU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUF2_A {
  #[doc = "0: LLWU_Pn input was not a wakeup source"]
  WUF2_0,
  #[doc = "1: LLWU_Pn input was a wakeup source"]
  WUF2_1,
}
impl From<WUF2_A> for bool {
  #[inline(always)]
  fn from(variant: WUF2_A) -> Self {
    match variant {
      WUF2_A::WUF2_0 => false,
      WUF2_A::WUF2_1 => true,
    }
  }
}
#[doc = "Reader of field `WUF2`"]
pub type WUF2_R = crate::R<bool, WUF2_A>;
impl WUF2_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> WUF2_A {
    match self.bits {
      false => WUF2_A::WUF2_0,
      true => WUF2_A::WUF2_1,
    }
  }
  #[doc = "Checks if the value of the field is `WUF2_0`"]
  #[inline(always)]
  pub fn is_wuf2_0(&self) -> bool {
    *self == WUF2_A::WUF2_0
  }
  #[doc = "Checks if the value of the field is `WUF2_1`"]
  #[inline(always)]
  pub fn is_wuf2_1(&self) -> bool {
    *self == WUF2_A::WUF2_1
  }
}
#[doc = "Write proxy for field `WUF2`"]
pub struct WUF2_W<'a> {
  w: &'a mut W,
}
impl<'a> WUF2_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUF2_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "LLWU_Pn input was not a wakeup source"]
  #[inline(always)]
  pub fn wuf2_0(self) -> &'a mut W {
    self.variant(WUF2_A::WUF2_0)
  }
  #[doc = "LLWU_Pn input was a wakeup source"]
  #[inline(always)]
  pub fn wuf2_1(self) -> &'a mut W {
    self.variant(WUF2_A::WUF2_1)
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
#[doc = "Wakeup flag for LLWU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUF3_A {
  #[doc = "0: LLWU_Pn input was not a wakeup source"]
  WUF3_0,
  #[doc = "1: LLWU_Pn input was a wakeup source"]
  WUF3_1,
}
impl From<WUF3_A> for bool {
  #[inline(always)]
  fn from(variant: WUF3_A) -> Self {
    match variant {
      WUF3_A::WUF3_0 => false,
      WUF3_A::WUF3_1 => true,
    }
  }
}
#[doc = "Reader of field `WUF3`"]
pub type WUF3_R = crate::R<bool, WUF3_A>;
impl WUF3_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> WUF3_A {
    match self.bits {
      false => WUF3_A::WUF3_0,
      true => WUF3_A::WUF3_1,
    }
  }
  #[doc = "Checks if the value of the field is `WUF3_0`"]
  #[inline(always)]
  pub fn is_wuf3_0(&self) -> bool {
    *self == WUF3_A::WUF3_0
  }
  #[doc = "Checks if the value of the field is `WUF3_1`"]
  #[inline(always)]
  pub fn is_wuf3_1(&self) -> bool {
    *self == WUF3_A::WUF3_1
  }
}
#[doc = "Write proxy for field `WUF3`"]
pub struct WUF3_W<'a> {
  w: &'a mut W,
}
impl<'a> WUF3_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUF3_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "LLWU_Pn input was not a wakeup source"]
  #[inline(always)]
  pub fn wuf3_0(self) -> &'a mut W {
    self.variant(WUF3_A::WUF3_0)
  }
  #[doc = "LLWU_Pn input was a wakeup source"]
  #[inline(always)]
  pub fn wuf3_1(self) -> &'a mut W {
    self.variant(WUF3_A::WUF3_1)
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
#[doc = "Wakeup flag for LLWU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUF4_A {
  #[doc = "0: LLWU_Pn input was not a wakeup source"]
  WUF4_0,
  #[doc = "1: LLWU_Pn input was a wakeup source"]
  WUF4_1,
}
impl From<WUF4_A> for bool {
  #[inline(always)]
  fn from(variant: WUF4_A) -> Self {
    match variant {
      WUF4_A::WUF4_0 => false,
      WUF4_A::WUF4_1 => true,
    }
  }
}
#[doc = "Reader of field `WUF4`"]
pub type WUF4_R = crate::R<bool, WUF4_A>;
impl WUF4_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> WUF4_A {
    match self.bits {
      false => WUF4_A::WUF4_0,
      true => WUF4_A::WUF4_1,
    }
  }
  #[doc = "Checks if the value of the field is `WUF4_0`"]
  #[inline(always)]
  pub fn is_wuf4_0(&self) -> bool {
    *self == WUF4_A::WUF4_0
  }
  #[doc = "Checks if the value of the field is `WUF4_1`"]
  #[inline(always)]
  pub fn is_wuf4_1(&self) -> bool {
    *self == WUF4_A::WUF4_1
  }
}
#[doc = "Write proxy for field `WUF4`"]
pub struct WUF4_W<'a> {
  w: &'a mut W,
}
impl<'a> WUF4_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUF4_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "LLWU_Pn input was not a wakeup source"]
  #[inline(always)]
  pub fn wuf4_0(self) -> &'a mut W {
    self.variant(WUF4_A::WUF4_0)
  }
  #[doc = "LLWU_Pn input was a wakeup source"]
  #[inline(always)]
  pub fn wuf4_1(self) -> &'a mut W {
    self.variant(WUF4_A::WUF4_1)
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
#[doc = "Wakeup flag for LLWU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUF5_A {
  #[doc = "0: LLWU_Pn input was not a wakeup source"]
  WUF5_0,
  #[doc = "1: LLWU_Pn input was a wakeup source"]
  WUF5_1,
}
impl From<WUF5_A> for bool {
  #[inline(always)]
  fn from(variant: WUF5_A) -> Self {
    match variant {
      WUF5_A::WUF5_0 => false,
      WUF5_A::WUF5_1 => true,
    }
  }
}
#[doc = "Reader of field `WUF5`"]
pub type WUF5_R = crate::R<bool, WUF5_A>;
impl WUF5_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> WUF5_A {
    match self.bits {
      false => WUF5_A::WUF5_0,
      true => WUF5_A::WUF5_1,
    }
  }
  #[doc = "Checks if the value of the field is `WUF5_0`"]
  #[inline(always)]
  pub fn is_wuf5_0(&self) -> bool {
    *self == WUF5_A::WUF5_0
  }
  #[doc = "Checks if the value of the field is `WUF5_1`"]
  #[inline(always)]
  pub fn is_wuf5_1(&self) -> bool {
    *self == WUF5_A::WUF5_1
  }
}
#[doc = "Write proxy for field `WUF5`"]
pub struct WUF5_W<'a> {
  w: &'a mut W,
}
impl<'a> WUF5_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUF5_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "LLWU_Pn input was not a wakeup source"]
  #[inline(always)]
  pub fn wuf5_0(self) -> &'a mut W {
    self.variant(WUF5_A::WUF5_0)
  }
  #[doc = "LLWU_Pn input was a wakeup source"]
  #[inline(always)]
  pub fn wuf5_1(self) -> &'a mut W {
    self.variant(WUF5_A::WUF5_1)
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
#[doc = "Wakeup flag for LLWU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUF6_A {
  #[doc = "0: LLWU_Pn input was not a wakeup source"]
  WUF6_0,
  #[doc = "1: LLWU_Pn input was a wakeup source"]
  WUF6_1,
}
impl From<WUF6_A> for bool {
  #[inline(always)]
  fn from(variant: WUF6_A) -> Self {
    match variant {
      WUF6_A::WUF6_0 => false,
      WUF6_A::WUF6_1 => true,
    }
  }
}
#[doc = "Reader of field `WUF6`"]
pub type WUF6_R = crate::R<bool, WUF6_A>;
impl WUF6_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> WUF6_A {
    match self.bits {
      false => WUF6_A::WUF6_0,
      true => WUF6_A::WUF6_1,
    }
  }
  #[doc = "Checks if the value of the field is `WUF6_0`"]
  #[inline(always)]
  pub fn is_wuf6_0(&self) -> bool {
    *self == WUF6_A::WUF6_0
  }
  #[doc = "Checks if the value of the field is `WUF6_1`"]
  #[inline(always)]
  pub fn is_wuf6_1(&self) -> bool {
    *self == WUF6_A::WUF6_1
  }
}
#[doc = "Write proxy for field `WUF6`"]
pub struct WUF6_W<'a> {
  w: &'a mut W,
}
impl<'a> WUF6_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUF6_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "LLWU_Pn input was not a wakeup source"]
  #[inline(always)]
  pub fn wuf6_0(self) -> &'a mut W {
    self.variant(WUF6_A::WUF6_0)
  }
  #[doc = "LLWU_Pn input was a wakeup source"]
  #[inline(always)]
  pub fn wuf6_1(self) -> &'a mut W {
    self.variant(WUF6_A::WUF6_1)
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
#[doc = "Wakeup flag for LLWU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUF7_A {
  #[doc = "0: LLWU_Pn input was not a wakeup source"]
  WUF7_0,
  #[doc = "1: LLWU_Pn input was a wakeup source"]
  WUF7_1,
}
impl From<WUF7_A> for bool {
  #[inline(always)]
  fn from(variant: WUF7_A) -> Self {
    match variant {
      WUF7_A::WUF7_0 => false,
      WUF7_A::WUF7_1 => true,
    }
  }
}
#[doc = "Reader of field `WUF7`"]
pub type WUF7_R = crate::R<bool, WUF7_A>;
impl WUF7_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> WUF7_A {
    match self.bits {
      false => WUF7_A::WUF7_0,
      true => WUF7_A::WUF7_1,
    }
  }
  #[doc = "Checks if the value of the field is `WUF7_0`"]
  #[inline(always)]
  pub fn is_wuf7_0(&self) -> bool {
    *self == WUF7_A::WUF7_0
  }
  #[doc = "Checks if the value of the field is `WUF7_1`"]
  #[inline(always)]
  pub fn is_wuf7_1(&self) -> bool {
    *self == WUF7_A::WUF7_1
  }
}
#[doc = "Write proxy for field `WUF7`"]
pub struct WUF7_W<'a> {
  w: &'a mut W,
}
impl<'a> WUF7_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUF7_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "LLWU_Pn input was not a wakeup source"]
  #[inline(always)]
  pub fn wuf7_0(self) -> &'a mut W {
    self.variant(WUF7_A::WUF7_0)
  }
  #[doc = "LLWU_Pn input was a wakeup source"]
  #[inline(always)]
  pub fn wuf7_1(self) -> &'a mut W {
    self.variant(WUF7_A::WUF7_1)
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
#[doc = "Wakeup flag for LLWU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUF8_A {
  #[doc = "0: LLWU_Pn input was not a wakeup source"]
  WUF8_0,
  #[doc = "1: LLWU_Pn input was a wakeup source"]
  WUF8_1,
}
impl From<WUF8_A> for bool {
  #[inline(always)]
  fn from(variant: WUF8_A) -> Self {
    match variant {
      WUF8_A::WUF8_0 => false,
      WUF8_A::WUF8_1 => true,
    }
  }
}
#[doc = "Reader of field `WUF8`"]
pub type WUF8_R = crate::R<bool, WUF8_A>;
impl WUF8_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> WUF8_A {
    match self.bits {
      false => WUF8_A::WUF8_0,
      true => WUF8_A::WUF8_1,
    }
  }
  #[doc = "Checks if the value of the field is `WUF8_0`"]
  #[inline(always)]
  pub fn is_wuf8_0(&self) -> bool {
    *self == WUF8_A::WUF8_0
  }
  #[doc = "Checks if the value of the field is `WUF8_1`"]
  #[inline(always)]
  pub fn is_wuf8_1(&self) -> bool {
    *self == WUF8_A::WUF8_1
  }
}
#[doc = "Write proxy for field `WUF8`"]
pub struct WUF8_W<'a> {
  w: &'a mut W,
}
impl<'a> WUF8_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUF8_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "LLWU_Pn input was not a wakeup source"]
  #[inline(always)]
  pub fn wuf8_0(self) -> &'a mut W {
    self.variant(WUF8_A::WUF8_0)
  }
  #[doc = "LLWU_Pn input was a wakeup source"]
  #[inline(always)]
  pub fn wuf8_1(self) -> &'a mut W {
    self.variant(WUF8_A::WUF8_1)
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
#[doc = "Wakeup flag for LLWU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUF9_A {
  #[doc = "0: LLWU_Pn input was not a wakeup source"]
  WUF9_0,
  #[doc = "1: LLWU_Pn input was a wakeup source"]
  WUF9_1,
}
impl From<WUF9_A> for bool {
  #[inline(always)]
  fn from(variant: WUF9_A) -> Self {
    match variant {
      WUF9_A::WUF9_0 => false,
      WUF9_A::WUF9_1 => true,
    }
  }
}
#[doc = "Reader of field `WUF9`"]
pub type WUF9_R = crate::R<bool, WUF9_A>;
impl WUF9_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> WUF9_A {
    match self.bits {
      false => WUF9_A::WUF9_0,
      true => WUF9_A::WUF9_1,
    }
  }
  #[doc = "Checks if the value of the field is `WUF9_0`"]
  #[inline(always)]
  pub fn is_wuf9_0(&self) -> bool {
    *self == WUF9_A::WUF9_0
  }
  #[doc = "Checks if the value of the field is `WUF9_1`"]
  #[inline(always)]
  pub fn is_wuf9_1(&self) -> bool {
    *self == WUF9_A::WUF9_1
  }
}
#[doc = "Write proxy for field `WUF9`"]
pub struct WUF9_W<'a> {
  w: &'a mut W,
}
impl<'a> WUF9_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUF9_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "LLWU_Pn input was not a wakeup source"]
  #[inline(always)]
  pub fn wuf9_0(self) -> &'a mut W {
    self.variant(WUF9_A::WUF9_0)
  }
  #[doc = "LLWU_Pn input was a wakeup source"]
  #[inline(always)]
  pub fn wuf9_1(self) -> &'a mut W {
    self.variant(WUF9_A::WUF9_1)
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
#[doc = "Wakeup flag for LLWU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUF10_A {
  #[doc = "0: LLWU_Pn input was not a wakeup source"]
  WUF10_0,
  #[doc = "1: LLWU_Pn input was a wakeup source"]
  WUF10_1,
}
impl From<WUF10_A> for bool {
  #[inline(always)]
  fn from(variant: WUF10_A) -> Self {
    match variant {
      WUF10_A::WUF10_0 => false,
      WUF10_A::WUF10_1 => true,
    }
  }
}
#[doc = "Reader of field `WUF10`"]
pub type WUF10_R = crate::R<bool, WUF10_A>;
impl WUF10_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> WUF10_A {
    match self.bits {
      false => WUF10_A::WUF10_0,
      true => WUF10_A::WUF10_1,
    }
  }
  #[doc = "Checks if the value of the field is `WUF10_0`"]
  #[inline(always)]
  pub fn is_wuf10_0(&self) -> bool {
    *self == WUF10_A::WUF10_0
  }
  #[doc = "Checks if the value of the field is `WUF10_1`"]
  #[inline(always)]
  pub fn is_wuf10_1(&self) -> bool {
    *self == WUF10_A::WUF10_1
  }
}
#[doc = "Write proxy for field `WUF10`"]
pub struct WUF10_W<'a> {
  w: &'a mut W,
}
impl<'a> WUF10_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUF10_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "LLWU_Pn input was not a wakeup source"]
  #[inline(always)]
  pub fn wuf10_0(self) -> &'a mut W {
    self.variant(WUF10_A::WUF10_0)
  }
  #[doc = "LLWU_Pn input was a wakeup source"]
  #[inline(always)]
  pub fn wuf10_1(self) -> &'a mut W {
    self.variant(WUF10_A::WUF10_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
    self.w
  }
}
#[doc = "Wakeup flag for LLWU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUF11_A {
  #[doc = "0: LLWU_Pn input was not a wakeup source"]
  WUF11_0,
  #[doc = "1: LLWU_Pn input was a wakeup source"]
  WUF11_1,
}
impl From<WUF11_A> for bool {
  #[inline(always)]
  fn from(variant: WUF11_A) -> Self {
    match variant {
      WUF11_A::WUF11_0 => false,
      WUF11_A::WUF11_1 => true,
    }
  }
}
#[doc = "Reader of field `WUF11`"]
pub type WUF11_R = crate::R<bool, WUF11_A>;
impl WUF11_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> WUF11_A {
    match self.bits {
      false => WUF11_A::WUF11_0,
      true => WUF11_A::WUF11_1,
    }
  }
  #[doc = "Checks if the value of the field is `WUF11_0`"]
  #[inline(always)]
  pub fn is_wuf11_0(&self) -> bool {
    *self == WUF11_A::WUF11_0
  }
  #[doc = "Checks if the value of the field is `WUF11_1`"]
  #[inline(always)]
  pub fn is_wuf11_1(&self) -> bool {
    *self == WUF11_A::WUF11_1
  }
}
#[doc = "Write proxy for field `WUF11`"]
pub struct WUF11_W<'a> {
  w: &'a mut W,
}
impl<'a> WUF11_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUF11_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "LLWU_Pn input was not a wakeup source"]
  #[inline(always)]
  pub fn wuf11_0(self) -> &'a mut W {
    self.variant(WUF11_A::WUF11_0)
  }
  #[doc = "LLWU_Pn input was a wakeup source"]
  #[inline(always)]
  pub fn wuf11_1(self) -> &'a mut W {
    self.variant(WUF11_A::WUF11_1)
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
#[doc = "Wakeup flag for LLWU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUF12_A {
  #[doc = "0: LLWU_Pn input was not a wakeup source"]
  WUF12_0,
  #[doc = "1: LLWU_Pn input was a wakeup source"]
  WUF12_1,
}
impl From<WUF12_A> for bool {
  #[inline(always)]
  fn from(variant: WUF12_A) -> Self {
    match variant {
      WUF12_A::WUF12_0 => false,
      WUF12_A::WUF12_1 => true,
    }
  }
}
#[doc = "Reader of field `WUF12`"]
pub type WUF12_R = crate::R<bool, WUF12_A>;
impl WUF12_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> WUF12_A {
    match self.bits {
      false => WUF12_A::WUF12_0,
      true => WUF12_A::WUF12_1,
    }
  }
  #[doc = "Checks if the value of the field is `WUF12_0`"]
  #[inline(always)]
  pub fn is_wuf12_0(&self) -> bool {
    *self == WUF12_A::WUF12_0
  }
  #[doc = "Checks if the value of the field is `WUF12_1`"]
  #[inline(always)]
  pub fn is_wuf12_1(&self) -> bool {
    *self == WUF12_A::WUF12_1
  }
}
#[doc = "Write proxy for field `WUF12`"]
pub struct WUF12_W<'a> {
  w: &'a mut W,
}
impl<'a> WUF12_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUF12_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "LLWU_Pn input was not a wakeup source"]
  #[inline(always)]
  pub fn wuf12_0(self) -> &'a mut W {
    self.variant(WUF12_A::WUF12_0)
  }
  #[doc = "LLWU_Pn input was a wakeup source"]
  #[inline(always)]
  pub fn wuf12_1(self) -> &'a mut W {
    self.variant(WUF12_A::WUF12_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
    self.w
  }
}
#[doc = "Wakeup flag for LLWU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUF13_A {
  #[doc = "0: LLWU_Pn input was not a wakeup source"]
  WUF13_0,
  #[doc = "1: LLWU_Pn input was a wakeup source"]
  WUF13_1,
}
impl From<WUF13_A> for bool {
  #[inline(always)]
  fn from(variant: WUF13_A) -> Self {
    match variant {
      WUF13_A::WUF13_0 => false,
      WUF13_A::WUF13_1 => true,
    }
  }
}
#[doc = "Reader of field `WUF13`"]
pub type WUF13_R = crate::R<bool, WUF13_A>;
impl WUF13_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> WUF13_A {
    match self.bits {
      false => WUF13_A::WUF13_0,
      true => WUF13_A::WUF13_1,
    }
  }
  #[doc = "Checks if the value of the field is `WUF13_0`"]
  #[inline(always)]
  pub fn is_wuf13_0(&self) -> bool {
    *self == WUF13_A::WUF13_0
  }
  #[doc = "Checks if the value of the field is `WUF13_1`"]
  #[inline(always)]
  pub fn is_wuf13_1(&self) -> bool {
    *self == WUF13_A::WUF13_1
  }
}
#[doc = "Write proxy for field `WUF13`"]
pub struct WUF13_W<'a> {
  w: &'a mut W,
}
impl<'a> WUF13_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUF13_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "LLWU_Pn input was not a wakeup source"]
  #[inline(always)]
  pub fn wuf13_0(self) -> &'a mut W {
    self.variant(WUF13_A::WUF13_0)
  }
  #[doc = "LLWU_Pn input was a wakeup source"]
  #[inline(always)]
  pub fn wuf13_1(self) -> &'a mut W {
    self.variant(WUF13_A::WUF13_1)
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
#[doc = "Wakeup flag for LLWU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUF14_A {
  #[doc = "0: LLWU_Pn input was not a wakeup source"]
  WUF14_0,
  #[doc = "1: LLWU_Pn input was a wakeup source"]
  WUF14_1,
}
impl From<WUF14_A> for bool {
  #[inline(always)]
  fn from(variant: WUF14_A) -> Self {
    match variant {
      WUF14_A::WUF14_0 => false,
      WUF14_A::WUF14_1 => true,
    }
  }
}
#[doc = "Reader of field `WUF14`"]
pub type WUF14_R = crate::R<bool, WUF14_A>;
impl WUF14_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> WUF14_A {
    match self.bits {
      false => WUF14_A::WUF14_0,
      true => WUF14_A::WUF14_1,
    }
  }
  #[doc = "Checks if the value of the field is `WUF14_0`"]
  #[inline(always)]
  pub fn is_wuf14_0(&self) -> bool {
    *self == WUF14_A::WUF14_0
  }
  #[doc = "Checks if the value of the field is `WUF14_1`"]
  #[inline(always)]
  pub fn is_wuf14_1(&self) -> bool {
    *self == WUF14_A::WUF14_1
  }
}
#[doc = "Write proxy for field `WUF14`"]
pub struct WUF14_W<'a> {
  w: &'a mut W,
}
impl<'a> WUF14_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUF14_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "LLWU_Pn input was not a wakeup source"]
  #[inline(always)]
  pub fn wuf14_0(self) -> &'a mut W {
    self.variant(WUF14_A::WUF14_0)
  }
  #[doc = "LLWU_Pn input was a wakeup source"]
  #[inline(always)]
  pub fn wuf14_1(self) -> &'a mut W {
    self.variant(WUF14_A::WUF14_1)
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
#[doc = "Wakeup flag for LLWU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUF15_A {
  #[doc = "0: LLWU_Pn input was not a wakeup source"]
  WUF15_0,
  #[doc = "1: LLWU_Pn input was a wakeup source"]
  WUF15_1,
}
impl From<WUF15_A> for bool {
  #[inline(always)]
  fn from(variant: WUF15_A) -> Self {
    match variant {
      WUF15_A::WUF15_0 => false,
      WUF15_A::WUF15_1 => true,
    }
  }
}
#[doc = "Reader of field `WUF15`"]
pub type WUF15_R = crate::R<bool, WUF15_A>;
impl WUF15_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> WUF15_A {
    match self.bits {
      false => WUF15_A::WUF15_0,
      true => WUF15_A::WUF15_1,
    }
  }
  #[doc = "Checks if the value of the field is `WUF15_0`"]
  #[inline(always)]
  pub fn is_wuf15_0(&self) -> bool {
    *self == WUF15_A::WUF15_0
  }
  #[doc = "Checks if the value of the field is `WUF15_1`"]
  #[inline(always)]
  pub fn is_wuf15_1(&self) -> bool {
    *self == WUF15_A::WUF15_1
  }
}
#[doc = "Write proxy for field `WUF15`"]
pub struct WUF15_W<'a> {
  w: &'a mut W,
}
impl<'a> WUF15_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUF15_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "LLWU_Pn input was not a wakeup source"]
  #[inline(always)]
  pub fn wuf15_0(self) -> &'a mut W {
    self.variant(WUF15_A::WUF15_0)
  }
  #[doc = "LLWU_Pn input was a wakeup source"]
  #[inline(always)]
  pub fn wuf15_1(self) -> &'a mut W {
    self.variant(WUF15_A::WUF15_1)
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
#[doc = "Wakeup flag for LLWU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUF16_A {
  #[doc = "0: LLWU_Pn input was not a wakeup source"]
  WUF16_0,
  #[doc = "1: LLWU_Pn input was a wakeup source"]
  WUF16_1,
}
impl From<WUF16_A> for bool {
  #[inline(always)]
  fn from(variant: WUF16_A) -> Self {
    match variant {
      WUF16_A::WUF16_0 => false,
      WUF16_A::WUF16_1 => true,
    }
  }
}
#[doc = "Reader of field `WUF16`"]
pub type WUF16_R = crate::R<bool, WUF16_A>;
impl WUF16_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> WUF16_A {
    match self.bits {
      false => WUF16_A::WUF16_0,
      true => WUF16_A::WUF16_1,
    }
  }
  #[doc = "Checks if the value of the field is `WUF16_0`"]
  #[inline(always)]
  pub fn is_wuf16_0(&self) -> bool {
    *self == WUF16_A::WUF16_0
  }
  #[doc = "Checks if the value of the field is `WUF16_1`"]
  #[inline(always)]
  pub fn is_wuf16_1(&self) -> bool {
    *self == WUF16_A::WUF16_1
  }
}
#[doc = "Write proxy for field `WUF16`"]
pub struct WUF16_W<'a> {
  w: &'a mut W,
}
impl<'a> WUF16_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUF16_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "LLWU_Pn input was not a wakeup source"]
  #[inline(always)]
  pub fn wuf16_0(self) -> &'a mut W {
    self.variant(WUF16_A::WUF16_0)
  }
  #[doc = "LLWU_Pn input was a wakeup source"]
  #[inline(always)]
  pub fn wuf16_1(self) -> &'a mut W {
    self.variant(WUF16_A::WUF16_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
    self.w
  }
}
#[doc = "Wakeup flag for LLWU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUF17_A {
  #[doc = "0: LLWU_Pn input was not a wakeup source"]
  WUF17_0,
  #[doc = "1: LLWU_Pn input was a wakeup source"]
  WUF17_1,
}
impl From<WUF17_A> for bool {
  #[inline(always)]
  fn from(variant: WUF17_A) -> Self {
    match variant {
      WUF17_A::WUF17_0 => false,
      WUF17_A::WUF17_1 => true,
    }
  }
}
#[doc = "Reader of field `WUF17`"]
pub type WUF17_R = crate::R<bool, WUF17_A>;
impl WUF17_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> WUF17_A {
    match self.bits {
      false => WUF17_A::WUF17_0,
      true => WUF17_A::WUF17_1,
    }
  }
  #[doc = "Checks if the value of the field is `WUF17_0`"]
  #[inline(always)]
  pub fn is_wuf17_0(&self) -> bool {
    *self == WUF17_A::WUF17_0
  }
  #[doc = "Checks if the value of the field is `WUF17_1`"]
  #[inline(always)]
  pub fn is_wuf17_1(&self) -> bool {
    *self == WUF17_A::WUF17_1
  }
}
#[doc = "Write proxy for field `WUF17`"]
pub struct WUF17_W<'a> {
  w: &'a mut W,
}
impl<'a> WUF17_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUF17_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "LLWU_Pn input was not a wakeup source"]
  #[inline(always)]
  pub fn wuf17_0(self) -> &'a mut W {
    self.variant(WUF17_A::WUF17_0)
  }
  #[doc = "LLWU_Pn input was a wakeup source"]
  #[inline(always)]
  pub fn wuf17_1(self) -> &'a mut W {
    self.variant(WUF17_A::WUF17_1)
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
#[doc = "Wakeup flag for LLWU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUF18_A {
  #[doc = "0: LLWU_Pn input was not a wakeup source"]
  WUF18_0,
  #[doc = "1: LLWU_Pn input was a wakeup source"]
  WUF18_1,
}
impl From<WUF18_A> for bool {
  #[inline(always)]
  fn from(variant: WUF18_A) -> Self {
    match variant {
      WUF18_A::WUF18_0 => false,
      WUF18_A::WUF18_1 => true,
    }
  }
}
#[doc = "Reader of field `WUF18`"]
pub type WUF18_R = crate::R<bool, WUF18_A>;
impl WUF18_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> WUF18_A {
    match self.bits {
      false => WUF18_A::WUF18_0,
      true => WUF18_A::WUF18_1,
    }
  }
  #[doc = "Checks if the value of the field is `WUF18_0`"]
  #[inline(always)]
  pub fn is_wuf18_0(&self) -> bool {
    *self == WUF18_A::WUF18_0
  }
  #[doc = "Checks if the value of the field is `WUF18_1`"]
  #[inline(always)]
  pub fn is_wuf18_1(&self) -> bool {
    *self == WUF18_A::WUF18_1
  }
}
#[doc = "Write proxy for field `WUF18`"]
pub struct WUF18_W<'a> {
  w: &'a mut W,
}
impl<'a> WUF18_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUF18_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "LLWU_Pn input was not a wakeup source"]
  #[inline(always)]
  pub fn wuf18_0(self) -> &'a mut W {
    self.variant(WUF18_A::WUF18_0)
  }
  #[doc = "LLWU_Pn input was a wakeup source"]
  #[inline(always)]
  pub fn wuf18_1(self) -> &'a mut W {
    self.variant(WUF18_A::WUF18_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
    self.w
  }
}
#[doc = "Wakeup flag for LLWU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUF19_A {
  #[doc = "0: LLWU_Pn input was not a wakeup source"]
  WUF19_0,
  #[doc = "1: LLWU_Pn input was a wakeup source"]
  WUF19_1,
}
impl From<WUF19_A> for bool {
  #[inline(always)]
  fn from(variant: WUF19_A) -> Self {
    match variant {
      WUF19_A::WUF19_0 => false,
      WUF19_A::WUF19_1 => true,
    }
  }
}
#[doc = "Reader of field `WUF19`"]
pub type WUF19_R = crate::R<bool, WUF19_A>;
impl WUF19_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> WUF19_A {
    match self.bits {
      false => WUF19_A::WUF19_0,
      true => WUF19_A::WUF19_1,
    }
  }
  #[doc = "Checks if the value of the field is `WUF19_0`"]
  #[inline(always)]
  pub fn is_wuf19_0(&self) -> bool {
    *self == WUF19_A::WUF19_0
  }
  #[doc = "Checks if the value of the field is `WUF19_1`"]
  #[inline(always)]
  pub fn is_wuf19_1(&self) -> bool {
    *self == WUF19_A::WUF19_1
  }
}
#[doc = "Write proxy for field `WUF19`"]
pub struct WUF19_W<'a> {
  w: &'a mut W,
}
impl<'a> WUF19_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUF19_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "LLWU_Pn input was not a wakeup source"]
  #[inline(always)]
  pub fn wuf19_0(self) -> &'a mut W {
    self.variant(WUF19_A::WUF19_0)
  }
  #[doc = "LLWU_Pn input was a wakeup source"]
  #[inline(always)]
  pub fn wuf19_1(self) -> &'a mut W {
    self.variant(WUF19_A::WUF19_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
    self.w
  }
}
#[doc = "Wakeup flag for LLWU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUF20_A {
  #[doc = "0: LLWU_Pn input was not a wakeup source"]
  WUF20_0,
  #[doc = "1: LLWU_Pn input was a wakeup source"]
  WUF20_1,
}
impl From<WUF20_A> for bool {
  #[inline(always)]
  fn from(variant: WUF20_A) -> Self {
    match variant {
      WUF20_A::WUF20_0 => false,
      WUF20_A::WUF20_1 => true,
    }
  }
}
#[doc = "Reader of field `WUF20`"]
pub type WUF20_R = crate::R<bool, WUF20_A>;
impl WUF20_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> WUF20_A {
    match self.bits {
      false => WUF20_A::WUF20_0,
      true => WUF20_A::WUF20_1,
    }
  }
  #[doc = "Checks if the value of the field is `WUF20_0`"]
  #[inline(always)]
  pub fn is_wuf20_0(&self) -> bool {
    *self == WUF20_A::WUF20_0
  }
  #[doc = "Checks if the value of the field is `WUF20_1`"]
  #[inline(always)]
  pub fn is_wuf20_1(&self) -> bool {
    *self == WUF20_A::WUF20_1
  }
}
#[doc = "Write proxy for field `WUF20`"]
pub struct WUF20_W<'a> {
  w: &'a mut W,
}
impl<'a> WUF20_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUF20_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "LLWU_Pn input was not a wakeup source"]
  #[inline(always)]
  pub fn wuf20_0(self) -> &'a mut W {
    self.variant(WUF20_A::WUF20_0)
  }
  #[doc = "LLWU_Pn input was a wakeup source"]
  #[inline(always)]
  pub fn wuf20_1(self) -> &'a mut W {
    self.variant(WUF20_A::WUF20_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
    self.w
  }
}
#[doc = "Wakeup flag for LLWU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUF21_A {
  #[doc = "0: LLWU_Pn input was not a wakeup source"]
  WUF21_0,
  #[doc = "1: LLWU_Pn input was a wakeup source"]
  WUF21_1,
}
impl From<WUF21_A> for bool {
  #[inline(always)]
  fn from(variant: WUF21_A) -> Self {
    match variant {
      WUF21_A::WUF21_0 => false,
      WUF21_A::WUF21_1 => true,
    }
  }
}
#[doc = "Reader of field `WUF21`"]
pub type WUF21_R = crate::R<bool, WUF21_A>;
impl WUF21_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> WUF21_A {
    match self.bits {
      false => WUF21_A::WUF21_0,
      true => WUF21_A::WUF21_1,
    }
  }
  #[doc = "Checks if the value of the field is `WUF21_0`"]
  #[inline(always)]
  pub fn is_wuf21_0(&self) -> bool {
    *self == WUF21_A::WUF21_0
  }
  #[doc = "Checks if the value of the field is `WUF21_1`"]
  #[inline(always)]
  pub fn is_wuf21_1(&self) -> bool {
    *self == WUF21_A::WUF21_1
  }
}
#[doc = "Write proxy for field `WUF21`"]
pub struct WUF21_W<'a> {
  w: &'a mut W,
}
impl<'a> WUF21_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUF21_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "LLWU_Pn input was not a wakeup source"]
  #[inline(always)]
  pub fn wuf21_0(self) -> &'a mut W {
    self.variant(WUF21_A::WUF21_0)
  }
  #[doc = "LLWU_Pn input was a wakeup source"]
  #[inline(always)]
  pub fn wuf21_1(self) -> &'a mut W {
    self.variant(WUF21_A::WUF21_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
    self.w
  }
}
#[doc = "Wakeup flag for LLWU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUF22_A {
  #[doc = "0: LLWU_Pn input was not a wakeup source"]
  WUF22_0,
  #[doc = "1: LLWU_Pn input was a wakeup source"]
  WUF22_1,
}
impl From<WUF22_A> for bool {
  #[inline(always)]
  fn from(variant: WUF22_A) -> Self {
    match variant {
      WUF22_A::WUF22_0 => false,
      WUF22_A::WUF22_1 => true,
    }
  }
}
#[doc = "Reader of field `WUF22`"]
pub type WUF22_R = crate::R<bool, WUF22_A>;
impl WUF22_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> WUF22_A {
    match self.bits {
      false => WUF22_A::WUF22_0,
      true => WUF22_A::WUF22_1,
    }
  }
  #[doc = "Checks if the value of the field is `WUF22_0`"]
  #[inline(always)]
  pub fn is_wuf22_0(&self) -> bool {
    *self == WUF22_A::WUF22_0
  }
  #[doc = "Checks if the value of the field is `WUF22_1`"]
  #[inline(always)]
  pub fn is_wuf22_1(&self) -> bool {
    *self == WUF22_A::WUF22_1
  }
}
#[doc = "Write proxy for field `WUF22`"]
pub struct WUF22_W<'a> {
  w: &'a mut W,
}
impl<'a> WUF22_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUF22_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "LLWU_Pn input was not a wakeup source"]
  #[inline(always)]
  pub fn wuf22_0(self) -> &'a mut W {
    self.variant(WUF22_A::WUF22_0)
  }
  #[doc = "LLWU_Pn input was a wakeup source"]
  #[inline(always)]
  pub fn wuf22_1(self) -> &'a mut W {
    self.variant(WUF22_A::WUF22_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
    self.w
  }
}
#[doc = "Wakeup flag for LLWU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUF23_A {
  #[doc = "0: LLWU_Pn input was not a wakeup source"]
  WUF23_0,
  #[doc = "1: LLWU_Pn input was a wakeup source"]
  WUF23_1,
}
impl From<WUF23_A> for bool {
  #[inline(always)]
  fn from(variant: WUF23_A) -> Self {
    match variant {
      WUF23_A::WUF23_0 => false,
      WUF23_A::WUF23_1 => true,
    }
  }
}
#[doc = "Reader of field `WUF23`"]
pub type WUF23_R = crate::R<bool, WUF23_A>;
impl WUF23_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> WUF23_A {
    match self.bits {
      false => WUF23_A::WUF23_0,
      true => WUF23_A::WUF23_1,
    }
  }
  #[doc = "Checks if the value of the field is `WUF23_0`"]
  #[inline(always)]
  pub fn is_wuf23_0(&self) -> bool {
    *self == WUF23_A::WUF23_0
  }
  #[doc = "Checks if the value of the field is `WUF23_1`"]
  #[inline(always)]
  pub fn is_wuf23_1(&self) -> bool {
    *self == WUF23_A::WUF23_1
  }
}
#[doc = "Write proxy for field `WUF23`"]
pub struct WUF23_W<'a> {
  w: &'a mut W,
}
impl<'a> WUF23_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUF23_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "LLWU_Pn input was not a wakeup source"]
  #[inline(always)]
  pub fn wuf23_0(self) -> &'a mut W {
    self.variant(WUF23_A::WUF23_0)
  }
  #[doc = "LLWU_Pn input was a wakeup source"]
  #[inline(always)]
  pub fn wuf23_1(self) -> &'a mut W {
    self.variant(WUF23_A::WUF23_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
    self.w
  }
}
#[doc = "Wakeup flag for LLWU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUF24_A {
  #[doc = "0: LLWU_Pn input was not a wakeup source"]
  WUF24_0,
  #[doc = "1: LLWU_Pn input was a wakeup source"]
  WUF24_1,
}
impl From<WUF24_A> for bool {
  #[inline(always)]
  fn from(variant: WUF24_A) -> Self {
    match variant {
      WUF24_A::WUF24_0 => false,
      WUF24_A::WUF24_1 => true,
    }
  }
}
#[doc = "Reader of field `WUF24`"]
pub type WUF24_R = crate::R<bool, WUF24_A>;
impl WUF24_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> WUF24_A {
    match self.bits {
      false => WUF24_A::WUF24_0,
      true => WUF24_A::WUF24_1,
    }
  }
  #[doc = "Checks if the value of the field is `WUF24_0`"]
  #[inline(always)]
  pub fn is_wuf24_0(&self) -> bool {
    *self == WUF24_A::WUF24_0
  }
  #[doc = "Checks if the value of the field is `WUF24_1`"]
  #[inline(always)]
  pub fn is_wuf24_1(&self) -> bool {
    *self == WUF24_A::WUF24_1
  }
}
#[doc = "Write proxy for field `WUF24`"]
pub struct WUF24_W<'a> {
  w: &'a mut W,
}
impl<'a> WUF24_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUF24_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "LLWU_Pn input was not a wakeup source"]
  #[inline(always)]
  pub fn wuf24_0(self) -> &'a mut W {
    self.variant(WUF24_A::WUF24_0)
  }
  #[doc = "LLWU_Pn input was a wakeup source"]
  #[inline(always)]
  pub fn wuf24_1(self) -> &'a mut W {
    self.variant(WUF24_A::WUF24_1)
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
#[doc = "Wakeup flag for LLWU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUF25_A {
  #[doc = "0: LLWU_Pn input was not a wakeup source"]
  WUF25_0,
  #[doc = "1: LLWU_Pn input was a wakeup source"]
  WUF25_1,
}
impl From<WUF25_A> for bool {
  #[inline(always)]
  fn from(variant: WUF25_A) -> Self {
    match variant {
      WUF25_A::WUF25_0 => false,
      WUF25_A::WUF25_1 => true,
    }
  }
}
#[doc = "Reader of field `WUF25`"]
pub type WUF25_R = crate::R<bool, WUF25_A>;
impl WUF25_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> WUF25_A {
    match self.bits {
      false => WUF25_A::WUF25_0,
      true => WUF25_A::WUF25_1,
    }
  }
  #[doc = "Checks if the value of the field is `WUF25_0`"]
  #[inline(always)]
  pub fn is_wuf25_0(&self) -> bool {
    *self == WUF25_A::WUF25_0
  }
  #[doc = "Checks if the value of the field is `WUF25_1`"]
  #[inline(always)]
  pub fn is_wuf25_1(&self) -> bool {
    *self == WUF25_A::WUF25_1
  }
}
#[doc = "Write proxy for field `WUF25`"]
pub struct WUF25_W<'a> {
  w: &'a mut W,
}
impl<'a> WUF25_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUF25_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "LLWU_Pn input was not a wakeup source"]
  #[inline(always)]
  pub fn wuf25_0(self) -> &'a mut W {
    self.variant(WUF25_A::WUF25_0)
  }
  #[doc = "LLWU_Pn input was a wakeup source"]
  #[inline(always)]
  pub fn wuf25_1(self) -> &'a mut W {
    self.variant(WUF25_A::WUF25_1)
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
#[doc = "Wakeup flag for LLWU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUF26_A {
  #[doc = "0: LLWU_Pn input was not a wakeup source"]
  WUF26_0,
  #[doc = "1: LLWU_Pn input was a wakeup source"]
  WUF26_1,
}
impl From<WUF26_A> for bool {
  #[inline(always)]
  fn from(variant: WUF26_A) -> Self {
    match variant {
      WUF26_A::WUF26_0 => false,
      WUF26_A::WUF26_1 => true,
    }
  }
}
#[doc = "Reader of field `WUF26`"]
pub type WUF26_R = crate::R<bool, WUF26_A>;
impl WUF26_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> WUF26_A {
    match self.bits {
      false => WUF26_A::WUF26_0,
      true => WUF26_A::WUF26_1,
    }
  }
  #[doc = "Checks if the value of the field is `WUF26_0`"]
  #[inline(always)]
  pub fn is_wuf26_0(&self) -> bool {
    *self == WUF26_A::WUF26_0
  }
  #[doc = "Checks if the value of the field is `WUF26_1`"]
  #[inline(always)]
  pub fn is_wuf26_1(&self) -> bool {
    *self == WUF26_A::WUF26_1
  }
}
#[doc = "Write proxy for field `WUF26`"]
pub struct WUF26_W<'a> {
  w: &'a mut W,
}
impl<'a> WUF26_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUF26_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "LLWU_Pn input was not a wakeup source"]
  #[inline(always)]
  pub fn wuf26_0(self) -> &'a mut W {
    self.variant(WUF26_A::WUF26_0)
  }
  #[doc = "LLWU_Pn input was a wakeup source"]
  #[inline(always)]
  pub fn wuf26_1(self) -> &'a mut W {
    self.variant(WUF26_A::WUF26_1)
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
#[doc = "Wakeup flag for LLWU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESERVED27_A {
  #[doc = "0: LLWU_Pn input was not a wakeup source"]
  RESERVED27_0,
  #[doc = "1: LLWU_Pn input was a wakeup source"]
  RESERVED27_1,
}
impl From<RESERVED27_A> for bool {
  #[inline(always)]
  fn from(variant: RESERVED27_A) -> Self {
    match variant {
      RESERVED27_A::RESERVED27_0 => false,
      RESERVED27_A::RESERVED27_1 => true,
    }
  }
}
#[doc = "Reader of field `Reserved27`"]
pub type RESERVED27_R = crate::R<bool, RESERVED27_A>;
impl RESERVED27_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> RESERVED27_A {
    match self.bits {
      false => RESERVED27_A::RESERVED27_0,
      true => RESERVED27_A::RESERVED27_1,
    }
  }
  #[doc = "Checks if the value of the field is `RESERVED27_0`"]
  #[inline(always)]
  pub fn is_reserved27_0(&self) -> bool {
    *self == RESERVED27_A::RESERVED27_0
  }
  #[doc = "Checks if the value of the field is `RESERVED27_1`"]
  #[inline(always)]
  pub fn is_reserved27_1(&self) -> bool {
    *self == RESERVED27_A::RESERVED27_1
  }
}
#[doc = "Wakeup flag for LLWU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESERVED28_A {
  #[doc = "0: LLWU_Pn input was not a wakeup source"]
  RESERVED28_0,
  #[doc = "1: LLWU_Pn input was a wakeup source"]
  RESERVED28_1,
}
impl From<RESERVED28_A> for bool {
  #[inline(always)]
  fn from(variant: RESERVED28_A) -> Self {
    match variant {
      RESERVED28_A::RESERVED28_0 => false,
      RESERVED28_A::RESERVED28_1 => true,
    }
  }
}
#[doc = "Reader of field `Reserved28`"]
pub type RESERVED28_R = crate::R<bool, RESERVED28_A>;
impl RESERVED28_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> RESERVED28_A {
    match self.bits {
      false => RESERVED28_A::RESERVED28_0,
      true => RESERVED28_A::RESERVED28_1,
    }
  }
  #[doc = "Checks if the value of the field is `RESERVED28_0`"]
  #[inline(always)]
  pub fn is_reserved28_0(&self) -> bool {
    *self == RESERVED28_A::RESERVED28_0
  }
  #[doc = "Checks if the value of the field is `RESERVED28_1`"]
  #[inline(always)]
  pub fn is_reserved28_1(&self) -> bool {
    *self == RESERVED28_A::RESERVED28_1
  }
}
#[doc = "Wakeup flag for LLWU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUF29_A {
  #[doc = "0: LLWU_Pn input was not a wakeup source"]
  WUF29_0,
  #[doc = "1: LLWU_Pn input was a wakeup source"]
  WUF29_1,
}
impl From<WUF29_A> for bool {
  #[inline(always)]
  fn from(variant: WUF29_A) -> Self {
    match variant {
      WUF29_A::WUF29_0 => false,
      WUF29_A::WUF29_1 => true,
    }
  }
}
#[doc = "Reader of field `WUF29`"]
pub type WUF29_R = crate::R<bool, WUF29_A>;
impl WUF29_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> WUF29_A {
    match self.bits {
      false => WUF29_A::WUF29_0,
      true => WUF29_A::WUF29_1,
    }
  }
  #[doc = "Checks if the value of the field is `WUF29_0`"]
  #[inline(always)]
  pub fn is_wuf29_0(&self) -> bool {
    *self == WUF29_A::WUF29_0
  }
  #[doc = "Checks if the value of the field is `WUF29_1`"]
  #[inline(always)]
  pub fn is_wuf29_1(&self) -> bool {
    *self == WUF29_A::WUF29_1
  }
}
#[doc = "Write proxy for field `WUF29`"]
pub struct WUF29_W<'a> {
  w: &'a mut W,
}
impl<'a> WUF29_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUF29_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "LLWU_Pn input was not a wakeup source"]
  #[inline(always)]
  pub fn wuf29_0(self) -> &'a mut W {
    self.variant(WUF29_A::WUF29_0)
  }
  #[doc = "LLWU_Pn input was a wakeup source"]
  #[inline(always)]
  pub fn wuf29_1(self) -> &'a mut W {
    self.variant(WUF29_A::WUF29_1)
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
#[doc = "Wakeup flag for LLWU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUF30_A {
  #[doc = "0: LLWU_Pn input was not a wakeup source"]
  WUF30_0,
  #[doc = "1: LLWU_Pn input was a wakeup source"]
  WUF30_1,
}
impl From<WUF30_A> for bool {
  #[inline(always)]
  fn from(variant: WUF30_A) -> Self {
    match variant {
      WUF30_A::WUF30_0 => false,
      WUF30_A::WUF30_1 => true,
    }
  }
}
#[doc = "Reader of field `WUF30`"]
pub type WUF30_R = crate::R<bool, WUF30_A>;
impl WUF30_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> WUF30_A {
    match self.bits {
      false => WUF30_A::WUF30_0,
      true => WUF30_A::WUF30_1,
    }
  }
  #[doc = "Checks if the value of the field is `WUF30_0`"]
  #[inline(always)]
  pub fn is_wuf30_0(&self) -> bool {
    *self == WUF30_A::WUF30_0
  }
  #[doc = "Checks if the value of the field is `WUF30_1`"]
  #[inline(always)]
  pub fn is_wuf30_1(&self) -> bool {
    *self == WUF30_A::WUF30_1
  }
}
#[doc = "Write proxy for field `WUF30`"]
pub struct WUF30_W<'a> {
  w: &'a mut W,
}
impl<'a> WUF30_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUF30_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "LLWU_Pn input was not a wakeup source"]
  #[inline(always)]
  pub fn wuf30_0(self) -> &'a mut W {
    self.variant(WUF30_A::WUF30_0)
  }
  #[doc = "LLWU_Pn input was a wakeup source"]
  #[inline(always)]
  pub fn wuf30_1(self) -> &'a mut W {
    self.variant(WUF30_A::WUF30_1)
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
#[doc = "Wakeup flag for LLWU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUF31_A {
  #[doc = "0: LLWU_Pn input was not a wakeup source"]
  WUF31_0,
  #[doc = "1: LLWU_Pn input was a wakeup source"]
  WUF31_1,
}
impl From<WUF31_A> for bool {
  #[inline(always)]
  fn from(variant: WUF31_A) -> Self {
    match variant {
      WUF31_A::WUF31_0 => false,
      WUF31_A::WUF31_1 => true,
    }
  }
}
#[doc = "Reader of field `WUF31`"]
pub type WUF31_R = crate::R<bool, WUF31_A>;
impl WUF31_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> WUF31_A {
    match self.bits {
      false => WUF31_A::WUF31_0,
      true => WUF31_A::WUF31_1,
    }
  }
  #[doc = "Checks if the value of the field is `WUF31_0`"]
  #[inline(always)]
  pub fn is_wuf31_0(&self) -> bool {
    *self == WUF31_A::WUF31_0
  }
  #[doc = "Checks if the value of the field is `WUF31_1`"]
  #[inline(always)]
  pub fn is_wuf31_1(&self) -> bool {
    *self == WUF31_A::WUF31_1
  }
}
#[doc = "Write proxy for field `WUF31`"]
pub struct WUF31_W<'a> {
  w: &'a mut W,
}
impl<'a> WUF31_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUF31_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "LLWU_Pn input was not a wakeup source"]
  #[inline(always)]
  pub fn wuf31_0(self) -> &'a mut W {
    self.variant(WUF31_A::WUF31_0)
  }
  #[doc = "LLWU_Pn input was a wakeup source"]
  #[inline(always)]
  pub fn wuf31_1(self) -> &'a mut W {
    self.variant(WUF31_A::WUF31_1)
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
  #[doc = "Bit 0 - Wakeup flag for LLWU_Pn"]
  #[inline(always)]
  pub fn wuf0(&self) -> WUF0_R {
    WUF0_R::new((self.bits & 0x01) != 0)
  }
  #[doc = "Bit 1 - Wakeup flag for LLWU_Pn"]
  #[inline(always)]
  pub fn wuf1(&self) -> WUF1_R {
    WUF1_R::new(((self.bits >> 1) & 0x01) != 0)
  }
  #[doc = "Bit 2 - Wakeup flag for LLWU_Pn"]
  #[inline(always)]
  pub fn wuf2(&self) -> WUF2_R {
    WUF2_R::new(((self.bits >> 2) & 0x01) != 0)
  }
  #[doc = "Bit 3 - Wakeup flag for LLWU_Pn"]
  #[inline(always)]
  pub fn wuf3(&self) -> WUF3_R {
    WUF3_R::new(((self.bits >> 3) & 0x01) != 0)
  }
  #[doc = "Bit 4 - Wakeup flag for LLWU_Pn"]
  #[inline(always)]
  pub fn wuf4(&self) -> WUF4_R {
    WUF4_R::new(((self.bits >> 4) & 0x01) != 0)
  }
  #[doc = "Bit 5 - Wakeup flag for LLWU_Pn"]
  #[inline(always)]
  pub fn wuf5(&self) -> WUF5_R {
    WUF5_R::new(((self.bits >> 5) & 0x01) != 0)
  }
  #[doc = "Bit 6 - Wakeup flag for LLWU_Pn"]
  #[inline(always)]
  pub fn wuf6(&self) -> WUF6_R {
    WUF6_R::new(((self.bits >> 6) & 0x01) != 0)
  }
  #[doc = "Bit 7 - Wakeup flag for LLWU_Pn"]
  #[inline(always)]
  pub fn wuf7(&self) -> WUF7_R {
    WUF7_R::new(((self.bits >> 7) & 0x01) != 0)
  }
  #[doc = "Bit 8 - Wakeup flag for LLWU_Pn"]
  #[inline(always)]
  pub fn wuf8(&self) -> WUF8_R {
    WUF8_R::new(((self.bits >> 8) & 0x01) != 0)
  }
  #[doc = "Bit 9 - Wakeup flag for LLWU_Pn"]
  #[inline(always)]
  pub fn wuf9(&self) -> WUF9_R {
    WUF9_R::new(((self.bits >> 9) & 0x01) != 0)
  }
  #[doc = "Bit 10 - Wakeup flag for LLWU_Pn"]
  #[inline(always)]
  pub fn wuf10(&self) -> WUF10_R {
    WUF10_R::new(((self.bits >> 10) & 0x01) != 0)
  }
  #[doc = "Bit 11 - Wakeup flag for LLWU_Pn"]
  #[inline(always)]
  pub fn wuf11(&self) -> WUF11_R {
    WUF11_R::new(((self.bits >> 11) & 0x01) != 0)
  }
  #[doc = "Bit 12 - Wakeup flag for LLWU_Pn"]
  #[inline(always)]
  pub fn wuf12(&self) -> WUF12_R {
    WUF12_R::new(((self.bits >> 12) & 0x01) != 0)
  }
  #[doc = "Bit 13 - Wakeup flag for LLWU_Pn"]
  #[inline(always)]
  pub fn wuf13(&self) -> WUF13_R {
    WUF13_R::new(((self.bits >> 13) & 0x01) != 0)
  }
  #[doc = "Bit 14 - Wakeup flag for LLWU_Pn"]
  #[inline(always)]
  pub fn wuf14(&self) -> WUF14_R {
    WUF14_R::new(((self.bits >> 14) & 0x01) != 0)
  }
  #[doc = "Bit 15 - Wakeup flag for LLWU_Pn"]
  #[inline(always)]
  pub fn wuf15(&self) -> WUF15_R {
    WUF15_R::new(((self.bits >> 15) & 0x01) != 0)
  }
  #[doc = "Bit 16 - Wakeup flag for LLWU_Pn"]
  #[inline(always)]
  pub fn wuf16(&self) -> WUF16_R {
    WUF16_R::new(((self.bits >> 16) & 0x01) != 0)
  }
  #[doc = "Bit 17 - Wakeup flag for LLWU_Pn"]
  #[inline(always)]
  pub fn wuf17(&self) -> WUF17_R {
    WUF17_R::new(((self.bits >> 17) & 0x01) != 0)
  }
  #[doc = "Bit 18 - Wakeup flag for LLWU_Pn"]
  #[inline(always)]
  pub fn wuf18(&self) -> WUF18_R {
    WUF18_R::new(((self.bits >> 18) & 0x01) != 0)
  }
  #[doc = "Bit 19 - Wakeup flag for LLWU_Pn"]
  #[inline(always)]
  pub fn wuf19(&self) -> WUF19_R {
    WUF19_R::new(((self.bits >> 19) & 0x01) != 0)
  }
  #[doc = "Bit 20 - Wakeup flag for LLWU_Pn"]
  #[inline(always)]
  pub fn wuf20(&self) -> WUF20_R {
    WUF20_R::new(((self.bits >> 20) & 0x01) != 0)
  }
  #[doc = "Bit 21 - Wakeup flag for LLWU_Pn"]
  #[inline(always)]
  pub fn wuf21(&self) -> WUF21_R {
    WUF21_R::new(((self.bits >> 21) & 0x01) != 0)
  }
  #[doc = "Bit 22 - Wakeup flag for LLWU_Pn"]
  #[inline(always)]
  pub fn wuf22(&self) -> WUF22_R {
    WUF22_R::new(((self.bits >> 22) & 0x01) != 0)
  }
  #[doc = "Bit 23 - Wakeup flag for LLWU_Pn"]
  #[inline(always)]
  pub fn wuf23(&self) -> WUF23_R {
    WUF23_R::new(((self.bits >> 23) & 0x01) != 0)
  }
  #[doc = "Bit 24 - Wakeup flag for LLWU_Pn"]
  #[inline(always)]
  pub fn wuf24(&self) -> WUF24_R {
    WUF24_R::new(((self.bits >> 24) & 0x01) != 0)
  }
  #[doc = "Bit 25 - Wakeup flag for LLWU_Pn"]
  #[inline(always)]
  pub fn wuf25(&self) -> WUF25_R {
    WUF25_R::new(((self.bits >> 25) & 0x01) != 0)
  }
  #[doc = "Bit 26 - Wakeup flag for LLWU_Pn"]
  #[inline(always)]
  pub fn wuf26(&self) -> WUF26_R {
    WUF26_R::new(((self.bits >> 26) & 0x01) != 0)
  }
  #[doc = "Bit 27 - Wakeup flag for LLWU_Pn"]
  #[inline(always)]
  pub fn reserved27(&self) -> RESERVED27_R {
    RESERVED27_R::new(((self.bits >> 27) & 0x01) != 0)
  }
  #[doc = "Bit 28 - Wakeup flag for LLWU_Pn"]
  #[inline(always)]
  pub fn reserved28(&self) -> RESERVED28_R {
    RESERVED28_R::new(((self.bits >> 28) & 0x01) != 0)
  }
  #[doc = "Bit 29 - Wakeup flag for LLWU_Pn"]
  #[inline(always)]
  pub fn wuf29(&self) -> WUF29_R {
    WUF29_R::new(((self.bits >> 29) & 0x01) != 0)
  }
  #[doc = "Bit 30 - Wakeup flag for LLWU_Pn"]
  #[inline(always)]
  pub fn wuf30(&self) -> WUF30_R {
    WUF30_R::new(((self.bits >> 30) & 0x01) != 0)
  }
  #[doc = "Bit 31 - Wakeup flag for LLWU_Pn"]
  #[inline(always)]
  pub fn wuf31(&self) -> WUF31_R {
    WUF31_R::new(((self.bits >> 31) & 0x01) != 0)
  }
}
impl W {
  #[doc = "Bit 0 - Wakeup flag for LLWU_Pn"]
  #[inline(always)]
  pub fn wuf0(&mut self) -> WUF0_W {
    WUF0_W { w: self }
  }
  #[doc = "Bit 1 - Wakeup flag for LLWU_Pn"]
  #[inline(always)]
  pub fn wuf1(&mut self) -> WUF1_W {
    WUF1_W { w: self }
  }
  #[doc = "Bit 2 - Wakeup flag for LLWU_Pn"]
  #[inline(always)]
  pub fn wuf2(&mut self) -> WUF2_W {
    WUF2_W { w: self }
  }
  #[doc = "Bit 3 - Wakeup flag for LLWU_Pn"]
  #[inline(always)]
  pub fn wuf3(&mut self) -> WUF3_W {
    WUF3_W { w: self }
  }
  #[doc = "Bit 4 - Wakeup flag for LLWU_Pn"]
  #[inline(always)]
  pub fn wuf4(&mut self) -> WUF4_W {
    WUF4_W { w: self }
  }
  #[doc = "Bit 5 - Wakeup flag for LLWU_Pn"]
  #[inline(always)]
  pub fn wuf5(&mut self) -> WUF5_W {
    WUF5_W { w: self }
  }
  #[doc = "Bit 6 - Wakeup flag for LLWU_Pn"]
  #[inline(always)]
  pub fn wuf6(&mut self) -> WUF6_W {
    WUF6_W { w: self }
  }
  #[doc = "Bit 7 - Wakeup flag for LLWU_Pn"]
  #[inline(always)]
  pub fn wuf7(&mut self) -> WUF7_W {
    WUF7_W { w: self }
  }
  #[doc = "Bit 8 - Wakeup flag for LLWU_Pn"]
  #[inline(always)]
  pub fn wuf8(&mut self) -> WUF8_W {
    WUF8_W { w: self }
  }
  #[doc = "Bit 9 - Wakeup flag for LLWU_Pn"]
  #[inline(always)]
  pub fn wuf9(&mut self) -> WUF9_W {
    WUF9_W { w: self }
  }
  #[doc = "Bit 10 - Wakeup flag for LLWU_Pn"]
  #[inline(always)]
  pub fn wuf10(&mut self) -> WUF10_W {
    WUF10_W { w: self }
  }
  #[doc = "Bit 11 - Wakeup flag for LLWU_Pn"]
  #[inline(always)]
  pub fn wuf11(&mut self) -> WUF11_W {
    WUF11_W { w: self }
  }
  #[doc = "Bit 12 - Wakeup flag for LLWU_Pn"]
  #[inline(always)]
  pub fn wuf12(&mut self) -> WUF12_W {
    WUF12_W { w: self }
  }
  #[doc = "Bit 13 - Wakeup flag for LLWU_Pn"]
  #[inline(always)]
  pub fn wuf13(&mut self) -> WUF13_W {
    WUF13_W { w: self }
  }
  #[doc = "Bit 14 - Wakeup flag for LLWU_Pn"]
  #[inline(always)]
  pub fn wuf14(&mut self) -> WUF14_W {
    WUF14_W { w: self }
  }
  #[doc = "Bit 15 - Wakeup flag for LLWU_Pn"]
  #[inline(always)]
  pub fn wuf15(&mut self) -> WUF15_W {
    WUF15_W { w: self }
  }
  #[doc = "Bit 16 - Wakeup flag for LLWU_Pn"]
  #[inline(always)]
  pub fn wuf16(&mut self) -> WUF16_W {
    WUF16_W { w: self }
  }
  #[doc = "Bit 17 - Wakeup flag for LLWU_Pn"]
  #[inline(always)]
  pub fn wuf17(&mut self) -> WUF17_W {
    WUF17_W { w: self }
  }
  #[doc = "Bit 18 - Wakeup flag for LLWU_Pn"]
  #[inline(always)]
  pub fn wuf18(&mut self) -> WUF18_W {
    WUF18_W { w: self }
  }
  #[doc = "Bit 19 - Wakeup flag for LLWU_Pn"]
  #[inline(always)]
  pub fn wuf19(&mut self) -> WUF19_W {
    WUF19_W { w: self }
  }
  #[doc = "Bit 20 - Wakeup flag for LLWU_Pn"]
  #[inline(always)]
  pub fn wuf20(&mut self) -> WUF20_W {
    WUF20_W { w: self }
  }
  #[doc = "Bit 21 - Wakeup flag for LLWU_Pn"]
  #[inline(always)]
  pub fn wuf21(&mut self) -> WUF21_W {
    WUF21_W { w: self }
  }
  #[doc = "Bit 22 - Wakeup flag for LLWU_Pn"]
  #[inline(always)]
  pub fn wuf22(&mut self) -> WUF22_W {
    WUF22_W { w: self }
  }
  #[doc = "Bit 23 - Wakeup flag for LLWU_Pn"]
  #[inline(always)]
  pub fn wuf23(&mut self) -> WUF23_W {
    WUF23_W { w: self }
  }
  #[doc = "Bit 24 - Wakeup flag for LLWU_Pn"]
  #[inline(always)]
  pub fn wuf24(&mut self) -> WUF24_W {
    WUF24_W { w: self }
  }
  #[doc = "Bit 25 - Wakeup flag for LLWU_Pn"]
  #[inline(always)]
  pub fn wuf25(&mut self) -> WUF25_W {
    WUF25_W { w: self }
  }
  #[doc = "Bit 26 - Wakeup flag for LLWU_Pn"]
  #[inline(always)]
  pub fn wuf26(&mut self) -> WUF26_W {
    WUF26_W { w: self }
  }
  #[doc = "Bit 29 - Wakeup flag for LLWU_Pn"]
  #[inline(always)]
  pub fn wuf29(&mut self) -> WUF29_W {
    WUF29_W { w: self }
  }
  #[doc = "Bit 30 - Wakeup flag for LLWU_Pn"]
  #[inline(always)]
  pub fn wuf30(&mut self) -> WUF30_W {
    WUF30_W { w: self }
  }
  #[doc = "Bit 31 - Wakeup flag for LLWU_Pn"]
  #[inline(always)]
  pub fn wuf31(&mut self) -> WUF31_W {
    WUF31_W { w: self }
  }
}
