#[doc = "Reader of register PE1"]
pub type R = crate::R<u32, super::PE1>;
#[doc = "Writer for register PE1"]
pub type W = crate::W<u32, super::PE1>;
#[doc = "Register PE1 `reset()`'s with value 0"]
impl crate::ResetValue for super::PE1 {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "Wakeup pin enable for LLWU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUPE0_A {
  #[doc = "0: External input pin disabled as wakeup input"]
  WUPE0_0,
  #[doc = "1: External input pin enabled with rising edge detection when configured as interrupt/DMA request or high level detection when configured as trigger request"]
  WUPE0_1,
  #[doc = "2: External input pin enabled with falling edge detection when configured as interrupt/DMA request or low level detection when configured as trigger request"]
  WUPE0_2,
  #[doc = "3: External input pin enabled with any change detection when configured as interrupt/DMA request"]
  WUPE0_3,
}
impl From<WUPE0_A> for u8 {
  #[inline(always)]
  fn from(variant: WUPE0_A) -> Self {
    match variant {
      WUPE0_A::WUPE0_0 => 0,
      WUPE0_A::WUPE0_1 => 1,
      WUPE0_A::WUPE0_2 => 2,
      WUPE0_A::WUPE0_3 => 3,
    }
  }
}
#[doc = "Reader of field `WUPE0`"]
pub type WUPE0_R = crate::R<u8, WUPE0_A>;
impl WUPE0_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> WUPE0_A {
    match self.bits {
      0 => WUPE0_A::WUPE0_0,
      1 => WUPE0_A::WUPE0_1,
      2 => WUPE0_A::WUPE0_2,
      3 => WUPE0_A::WUPE0_3,
      _ => unreachable!(),
    }
  }
  #[doc = "Checks if the value of the field is `WUPE0_0`"]
  #[inline(always)]
  pub fn is_wupe0_0(&self) -> bool {
    *self == WUPE0_A::WUPE0_0
  }
  #[doc = "Checks if the value of the field is `WUPE0_1`"]
  #[inline(always)]
  pub fn is_wupe0_1(&self) -> bool {
    *self == WUPE0_A::WUPE0_1
  }
  #[doc = "Checks if the value of the field is `WUPE0_2`"]
  #[inline(always)]
  pub fn is_wupe0_2(&self) -> bool {
    *self == WUPE0_A::WUPE0_2
  }
  #[doc = "Checks if the value of the field is `WUPE0_3`"]
  #[inline(always)]
  pub fn is_wupe0_3(&self) -> bool {
    *self == WUPE0_A::WUPE0_3
  }
}
#[doc = "Write proxy for field `WUPE0`"]
pub struct WUPE0_W<'a> {
  w: &'a mut W,
}
impl<'a> WUPE0_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUPE0_A) -> &'a mut W {
    {
      self.bits(variant.into())
    }
  }
  #[doc = "External input pin disabled as wakeup input"]
  #[inline(always)]
  pub fn wupe0_0(self) -> &'a mut W {
    self.variant(WUPE0_A::WUPE0_0)
  }
  #[doc = "External input pin enabled with rising edge detection when configured as interrupt/DMA request or high level detection when configured as trigger request"]
  #[inline(always)]
  pub fn wupe0_1(self) -> &'a mut W {
    self.variant(WUPE0_A::WUPE0_1)
  }
  #[doc = "External input pin enabled with falling edge detection when configured as interrupt/DMA request or low level detection when configured as trigger request"]
  #[inline(always)]
  pub fn wupe0_2(self) -> &'a mut W {
    self.variant(WUPE0_A::WUPE0_2)
  }
  #[doc = "External input pin enabled with any change detection when configured as interrupt/DMA request"]
  #[inline(always)]
  pub fn wupe0_3(self) -> &'a mut W {
    self.variant(WUPE0_A::WUPE0_3)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
    self.w
  }
}
#[doc = "Wakeup pin enable for LLWU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUPE1_A {
  #[doc = "0: External input pin disabled as wakeup input"]
  WUPE1_0,
  #[doc = "1: External input pin enabled with rising edge detection when configured as interrupt/DMA request or high level detection when configured as trigger request"]
  WUPE1_1,
  #[doc = "2: External input pin enabled with falling edge detection when configured as interrupt/DMA request or low level detection when configured as trigger request"]
  WUPE1_2,
  #[doc = "3: External input pin enabled with any change detection when configured as interrupt/DMA request"]
  WUPE1_3,
}
impl From<WUPE1_A> for u8 {
  #[inline(always)]
  fn from(variant: WUPE1_A) -> Self {
    match variant {
      WUPE1_A::WUPE1_0 => 0,
      WUPE1_A::WUPE1_1 => 1,
      WUPE1_A::WUPE1_2 => 2,
      WUPE1_A::WUPE1_3 => 3,
    }
  }
}
#[doc = "Reader of field `WUPE1`"]
pub type WUPE1_R = crate::R<u8, WUPE1_A>;
impl WUPE1_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> WUPE1_A {
    match self.bits {
      0 => WUPE1_A::WUPE1_0,
      1 => WUPE1_A::WUPE1_1,
      2 => WUPE1_A::WUPE1_2,
      3 => WUPE1_A::WUPE1_3,
      _ => unreachable!(),
    }
  }
  #[doc = "Checks if the value of the field is `WUPE1_0`"]
  #[inline(always)]
  pub fn is_wupe1_0(&self) -> bool {
    *self == WUPE1_A::WUPE1_0
  }
  #[doc = "Checks if the value of the field is `WUPE1_1`"]
  #[inline(always)]
  pub fn is_wupe1_1(&self) -> bool {
    *self == WUPE1_A::WUPE1_1
  }
  #[doc = "Checks if the value of the field is `WUPE1_2`"]
  #[inline(always)]
  pub fn is_wupe1_2(&self) -> bool {
    *self == WUPE1_A::WUPE1_2
  }
  #[doc = "Checks if the value of the field is `WUPE1_3`"]
  #[inline(always)]
  pub fn is_wupe1_3(&self) -> bool {
    *self == WUPE1_A::WUPE1_3
  }
}
#[doc = "Write proxy for field `WUPE1`"]
pub struct WUPE1_W<'a> {
  w: &'a mut W,
}
impl<'a> WUPE1_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUPE1_A) -> &'a mut W {
    {
      self.bits(variant.into())
    }
  }
  #[doc = "External input pin disabled as wakeup input"]
  #[inline(always)]
  pub fn wupe1_0(self) -> &'a mut W {
    self.variant(WUPE1_A::WUPE1_0)
  }
  #[doc = "External input pin enabled with rising edge detection when configured as interrupt/DMA request or high level detection when configured as trigger request"]
  #[inline(always)]
  pub fn wupe1_1(self) -> &'a mut W {
    self.variant(WUPE1_A::WUPE1_1)
  }
  #[doc = "External input pin enabled with falling edge detection when configured as interrupt/DMA request or low level detection when configured as trigger request"]
  #[inline(always)]
  pub fn wupe1_2(self) -> &'a mut W {
    self.variant(WUPE1_A::WUPE1_2)
  }
  #[doc = "External input pin enabled with any change detection when configured as interrupt/DMA request"]
  #[inline(always)]
  pub fn wupe1_3(self) -> &'a mut W {
    self.variant(WUPE1_A::WUPE1_3)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
    self.w
  }
}
#[doc = "Wakeup pin enable for LLWU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUPE2_A {
  #[doc = "0: External input pin disabled as wakeup input"]
  WUPE2_0,
  #[doc = "1: External input pin enabled with rising edge detection when configured as interrupt/DMA request or high level detection when configured as trigger request"]
  WUPE2_1,
  #[doc = "2: External input pin enabled with falling edge detection when configured as interrupt/DMA request or low level detection when configured as trigger request"]
  WUPE2_2,
  #[doc = "3: External input pin enabled with any change detection when configured as interrupt/DMA request"]
  WUPE2_3,
}
impl From<WUPE2_A> for u8 {
  #[inline(always)]
  fn from(variant: WUPE2_A) -> Self {
    match variant {
      WUPE2_A::WUPE2_0 => 0,
      WUPE2_A::WUPE2_1 => 1,
      WUPE2_A::WUPE2_2 => 2,
      WUPE2_A::WUPE2_3 => 3,
    }
  }
}
#[doc = "Reader of field `WUPE2`"]
pub type WUPE2_R = crate::R<u8, WUPE2_A>;
impl WUPE2_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> WUPE2_A {
    match self.bits {
      0 => WUPE2_A::WUPE2_0,
      1 => WUPE2_A::WUPE2_1,
      2 => WUPE2_A::WUPE2_2,
      3 => WUPE2_A::WUPE2_3,
      _ => unreachable!(),
    }
  }
  #[doc = "Checks if the value of the field is `WUPE2_0`"]
  #[inline(always)]
  pub fn is_wupe2_0(&self) -> bool {
    *self == WUPE2_A::WUPE2_0
  }
  #[doc = "Checks if the value of the field is `WUPE2_1`"]
  #[inline(always)]
  pub fn is_wupe2_1(&self) -> bool {
    *self == WUPE2_A::WUPE2_1
  }
  #[doc = "Checks if the value of the field is `WUPE2_2`"]
  #[inline(always)]
  pub fn is_wupe2_2(&self) -> bool {
    *self == WUPE2_A::WUPE2_2
  }
  #[doc = "Checks if the value of the field is `WUPE2_3`"]
  #[inline(always)]
  pub fn is_wupe2_3(&self) -> bool {
    *self == WUPE2_A::WUPE2_3
  }
}
#[doc = "Write proxy for field `WUPE2`"]
pub struct WUPE2_W<'a> {
  w: &'a mut W,
}
impl<'a> WUPE2_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUPE2_A) -> &'a mut W {
    {
      self.bits(variant.into())
    }
  }
  #[doc = "External input pin disabled as wakeup input"]
  #[inline(always)]
  pub fn wupe2_0(self) -> &'a mut W {
    self.variant(WUPE2_A::WUPE2_0)
  }
  #[doc = "External input pin enabled with rising edge detection when configured as interrupt/DMA request or high level detection when configured as trigger request"]
  #[inline(always)]
  pub fn wupe2_1(self) -> &'a mut W {
    self.variant(WUPE2_A::WUPE2_1)
  }
  #[doc = "External input pin enabled with falling edge detection when configured as interrupt/DMA request or low level detection when configured as trigger request"]
  #[inline(always)]
  pub fn wupe2_2(self) -> &'a mut W {
    self.variant(WUPE2_A::WUPE2_2)
  }
  #[doc = "External input pin enabled with any change detection when configured as interrupt/DMA request"]
  #[inline(always)]
  pub fn wupe2_3(self) -> &'a mut W {
    self.variant(WUPE2_A::WUPE2_3)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
    self.w
  }
}
#[doc = "Wakeup pin enable for LLWU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUPE3_A {
  #[doc = "0: External input pin disabled as wakeup input"]
  WUPE3_0,
  #[doc = "1: External input pin enabled with rising edge detection when configured as interrupt/DMA request or high level detection when configured as trigger request"]
  WUPE3_1,
  #[doc = "2: External input pin enabled with falling edge detection when configured as interrupt/DMA request or low level detection when configured as trigger request"]
  WUPE3_2,
  #[doc = "3: External input pin enabled with any change detection when configured as interrupt/DMA request"]
  WUPE3_3,
}
impl From<WUPE3_A> for u8 {
  #[inline(always)]
  fn from(variant: WUPE3_A) -> Self {
    match variant {
      WUPE3_A::WUPE3_0 => 0,
      WUPE3_A::WUPE3_1 => 1,
      WUPE3_A::WUPE3_2 => 2,
      WUPE3_A::WUPE3_3 => 3,
    }
  }
}
#[doc = "Reader of field `WUPE3`"]
pub type WUPE3_R = crate::R<u8, WUPE3_A>;
impl WUPE3_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> WUPE3_A {
    match self.bits {
      0 => WUPE3_A::WUPE3_0,
      1 => WUPE3_A::WUPE3_1,
      2 => WUPE3_A::WUPE3_2,
      3 => WUPE3_A::WUPE3_3,
      _ => unreachable!(),
    }
  }
  #[doc = "Checks if the value of the field is `WUPE3_0`"]
  #[inline(always)]
  pub fn is_wupe3_0(&self) -> bool {
    *self == WUPE3_A::WUPE3_0
  }
  #[doc = "Checks if the value of the field is `WUPE3_1`"]
  #[inline(always)]
  pub fn is_wupe3_1(&self) -> bool {
    *self == WUPE3_A::WUPE3_1
  }
  #[doc = "Checks if the value of the field is `WUPE3_2`"]
  #[inline(always)]
  pub fn is_wupe3_2(&self) -> bool {
    *self == WUPE3_A::WUPE3_2
  }
  #[doc = "Checks if the value of the field is `WUPE3_3`"]
  #[inline(always)]
  pub fn is_wupe3_3(&self) -> bool {
    *self == WUPE3_A::WUPE3_3
  }
}
#[doc = "Write proxy for field `WUPE3`"]
pub struct WUPE3_W<'a> {
  w: &'a mut W,
}
impl<'a> WUPE3_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUPE3_A) -> &'a mut W {
    {
      self.bits(variant.into())
    }
  }
  #[doc = "External input pin disabled as wakeup input"]
  #[inline(always)]
  pub fn wupe3_0(self) -> &'a mut W {
    self.variant(WUPE3_A::WUPE3_0)
  }
  #[doc = "External input pin enabled with rising edge detection when configured as interrupt/DMA request or high level detection when configured as trigger request"]
  #[inline(always)]
  pub fn wupe3_1(self) -> &'a mut W {
    self.variant(WUPE3_A::WUPE3_1)
  }
  #[doc = "External input pin enabled with falling edge detection when configured as interrupt/DMA request or low level detection when configured as trigger request"]
  #[inline(always)]
  pub fn wupe3_2(self) -> &'a mut W {
    self.variant(WUPE3_A::WUPE3_2)
  }
  #[doc = "External input pin enabled with any change detection when configured as interrupt/DMA request"]
  #[inline(always)]
  pub fn wupe3_3(self) -> &'a mut W {
    self.variant(WUPE3_A::WUPE3_3)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
    self.w
  }
}
#[doc = "Wakeup pin enable for LLWU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUPE4_A {
  #[doc = "0: External input pin disabled as wakeup input"]
  WUPE4_0,
  #[doc = "1: External input pin enabled with rising edge detection when configured as interrupt/DMA request or high level detection when configured as trigger request"]
  WUPE4_1,
  #[doc = "2: External input pin enabled with falling edge detection when configured as interrupt/DMA request or low level detection when configured as trigger request"]
  WUPE4_2,
  #[doc = "3: External input pin enabled with any change detection when configured as interrupt/DMA request"]
  WUPE4_3,
}
impl From<WUPE4_A> for u8 {
  #[inline(always)]
  fn from(variant: WUPE4_A) -> Self {
    match variant {
      WUPE4_A::WUPE4_0 => 0,
      WUPE4_A::WUPE4_1 => 1,
      WUPE4_A::WUPE4_2 => 2,
      WUPE4_A::WUPE4_3 => 3,
    }
  }
}
#[doc = "Reader of field `WUPE4`"]
pub type WUPE4_R = crate::R<u8, WUPE4_A>;
impl WUPE4_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> WUPE4_A {
    match self.bits {
      0 => WUPE4_A::WUPE4_0,
      1 => WUPE4_A::WUPE4_1,
      2 => WUPE4_A::WUPE4_2,
      3 => WUPE4_A::WUPE4_3,
      _ => unreachable!(),
    }
  }
  #[doc = "Checks if the value of the field is `WUPE4_0`"]
  #[inline(always)]
  pub fn is_wupe4_0(&self) -> bool {
    *self == WUPE4_A::WUPE4_0
  }
  #[doc = "Checks if the value of the field is `WUPE4_1`"]
  #[inline(always)]
  pub fn is_wupe4_1(&self) -> bool {
    *self == WUPE4_A::WUPE4_1
  }
  #[doc = "Checks if the value of the field is `WUPE4_2`"]
  #[inline(always)]
  pub fn is_wupe4_2(&self) -> bool {
    *self == WUPE4_A::WUPE4_2
  }
  #[doc = "Checks if the value of the field is `WUPE4_3`"]
  #[inline(always)]
  pub fn is_wupe4_3(&self) -> bool {
    *self == WUPE4_A::WUPE4_3
  }
}
#[doc = "Write proxy for field `WUPE4`"]
pub struct WUPE4_W<'a> {
  w: &'a mut W,
}
impl<'a> WUPE4_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUPE4_A) -> &'a mut W {
    {
      self.bits(variant.into())
    }
  }
  #[doc = "External input pin disabled as wakeup input"]
  #[inline(always)]
  pub fn wupe4_0(self) -> &'a mut W {
    self.variant(WUPE4_A::WUPE4_0)
  }
  #[doc = "External input pin enabled with rising edge detection when configured as interrupt/DMA request or high level detection when configured as trigger request"]
  #[inline(always)]
  pub fn wupe4_1(self) -> &'a mut W {
    self.variant(WUPE4_A::WUPE4_1)
  }
  #[doc = "External input pin enabled with falling edge detection when configured as interrupt/DMA request or low level detection when configured as trigger request"]
  #[inline(always)]
  pub fn wupe4_2(self) -> &'a mut W {
    self.variant(WUPE4_A::WUPE4_2)
  }
  #[doc = "External input pin enabled with any change detection when configured as interrupt/DMA request"]
  #[inline(always)]
  pub fn wupe4_3(self) -> &'a mut W {
    self.variant(WUPE4_A::WUPE4_3)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
    self.w
  }
}
#[doc = "Wakeup pin enable for LLWU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUPE5_A {
  #[doc = "0: External input pin disabled as wakeup input"]
  WUPE5_0,
  #[doc = "1: External input pin enabled with rising edge detection when configured as interrupt/DMA request or high level detection when configured as trigger request"]
  WUPE5_1,
  #[doc = "2: External input pin enabled with falling edge detection when configured as interrupt/DMA request or low level detection when configured as trigger request"]
  WUPE5_2,
  #[doc = "3: External input pin enabled with any change detection when configured as interrupt/DMA request"]
  WUPE5_3,
}
impl From<WUPE5_A> for u8 {
  #[inline(always)]
  fn from(variant: WUPE5_A) -> Self {
    match variant {
      WUPE5_A::WUPE5_0 => 0,
      WUPE5_A::WUPE5_1 => 1,
      WUPE5_A::WUPE5_2 => 2,
      WUPE5_A::WUPE5_3 => 3,
    }
  }
}
#[doc = "Reader of field `WUPE5`"]
pub type WUPE5_R = crate::R<u8, WUPE5_A>;
impl WUPE5_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> WUPE5_A {
    match self.bits {
      0 => WUPE5_A::WUPE5_0,
      1 => WUPE5_A::WUPE5_1,
      2 => WUPE5_A::WUPE5_2,
      3 => WUPE5_A::WUPE5_3,
      _ => unreachable!(),
    }
  }
  #[doc = "Checks if the value of the field is `WUPE5_0`"]
  #[inline(always)]
  pub fn is_wupe5_0(&self) -> bool {
    *self == WUPE5_A::WUPE5_0
  }
  #[doc = "Checks if the value of the field is `WUPE5_1`"]
  #[inline(always)]
  pub fn is_wupe5_1(&self) -> bool {
    *self == WUPE5_A::WUPE5_1
  }
  #[doc = "Checks if the value of the field is `WUPE5_2`"]
  #[inline(always)]
  pub fn is_wupe5_2(&self) -> bool {
    *self == WUPE5_A::WUPE5_2
  }
  #[doc = "Checks if the value of the field is `WUPE5_3`"]
  #[inline(always)]
  pub fn is_wupe5_3(&self) -> bool {
    *self == WUPE5_A::WUPE5_3
  }
}
#[doc = "Write proxy for field `WUPE5`"]
pub struct WUPE5_W<'a> {
  w: &'a mut W,
}
impl<'a> WUPE5_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUPE5_A) -> &'a mut W {
    {
      self.bits(variant.into())
    }
  }
  #[doc = "External input pin disabled as wakeup input"]
  #[inline(always)]
  pub fn wupe5_0(self) -> &'a mut W {
    self.variant(WUPE5_A::WUPE5_0)
  }
  #[doc = "External input pin enabled with rising edge detection when configured as interrupt/DMA request or high level detection when configured as trigger request"]
  #[inline(always)]
  pub fn wupe5_1(self) -> &'a mut W {
    self.variant(WUPE5_A::WUPE5_1)
  }
  #[doc = "External input pin enabled with falling edge detection when configured as interrupt/DMA request or low level detection when configured as trigger request"]
  #[inline(always)]
  pub fn wupe5_2(self) -> &'a mut W {
    self.variant(WUPE5_A::WUPE5_2)
  }
  #[doc = "External input pin enabled with any change detection when configured as interrupt/DMA request"]
  #[inline(always)]
  pub fn wupe5_3(self) -> &'a mut W {
    self.variant(WUPE5_A::WUPE5_3)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
    self.w
  }
}
#[doc = "Wakeup pin enable for LLWU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUPE6_A {
  #[doc = "0: External input pin disabled as wakeup input"]
  WUPE6_0,
  #[doc = "1: External input pin enabled with rising edge detection when configured as interrupt/DMA request or high level detection when configured as trigger request"]
  WUPE6_1,
  #[doc = "2: External input pin enabled with falling edge detection when configured as interrupt/DMA request or low level detection when configured as trigger request"]
  WUPE6_2,
  #[doc = "3: External input pin enabled with any change detection when configured as interrupt/DMA request"]
  WUPE6_3,
}
impl From<WUPE6_A> for u8 {
  #[inline(always)]
  fn from(variant: WUPE6_A) -> Self {
    match variant {
      WUPE6_A::WUPE6_0 => 0,
      WUPE6_A::WUPE6_1 => 1,
      WUPE6_A::WUPE6_2 => 2,
      WUPE6_A::WUPE6_3 => 3,
    }
  }
}
#[doc = "Reader of field `WUPE6`"]
pub type WUPE6_R = crate::R<u8, WUPE6_A>;
impl WUPE6_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> WUPE6_A {
    match self.bits {
      0 => WUPE6_A::WUPE6_0,
      1 => WUPE6_A::WUPE6_1,
      2 => WUPE6_A::WUPE6_2,
      3 => WUPE6_A::WUPE6_3,
      _ => unreachable!(),
    }
  }
  #[doc = "Checks if the value of the field is `WUPE6_0`"]
  #[inline(always)]
  pub fn is_wupe6_0(&self) -> bool {
    *self == WUPE6_A::WUPE6_0
  }
  #[doc = "Checks if the value of the field is `WUPE6_1`"]
  #[inline(always)]
  pub fn is_wupe6_1(&self) -> bool {
    *self == WUPE6_A::WUPE6_1
  }
  #[doc = "Checks if the value of the field is `WUPE6_2`"]
  #[inline(always)]
  pub fn is_wupe6_2(&self) -> bool {
    *self == WUPE6_A::WUPE6_2
  }
  #[doc = "Checks if the value of the field is `WUPE6_3`"]
  #[inline(always)]
  pub fn is_wupe6_3(&self) -> bool {
    *self == WUPE6_A::WUPE6_3
  }
}
#[doc = "Write proxy for field `WUPE6`"]
pub struct WUPE6_W<'a> {
  w: &'a mut W,
}
impl<'a> WUPE6_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUPE6_A) -> &'a mut W {
    {
      self.bits(variant.into())
    }
  }
  #[doc = "External input pin disabled as wakeup input"]
  #[inline(always)]
  pub fn wupe6_0(self) -> &'a mut W {
    self.variant(WUPE6_A::WUPE6_0)
  }
  #[doc = "External input pin enabled with rising edge detection when configured as interrupt/DMA request or high level detection when configured as trigger request"]
  #[inline(always)]
  pub fn wupe6_1(self) -> &'a mut W {
    self.variant(WUPE6_A::WUPE6_1)
  }
  #[doc = "External input pin enabled with falling edge detection when configured as interrupt/DMA request or low level detection when configured as trigger request"]
  #[inline(always)]
  pub fn wupe6_2(self) -> &'a mut W {
    self.variant(WUPE6_A::WUPE6_2)
  }
  #[doc = "External input pin enabled with any change detection when configured as interrupt/DMA request"]
  #[inline(always)]
  pub fn wupe6_3(self) -> &'a mut W {
    self.variant(WUPE6_A::WUPE6_3)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
    self.w
  }
}
#[doc = "Wakeup pin enable for LLWU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUPE7_A {
  #[doc = "0: External input pin disabled as wakeup input"]
  WUPE7_0,
  #[doc = "1: External input pin enabled with rising edge detection when configured as interrupt/DMA request or high level detection when configured as trigger request"]
  WUPE7_1,
  #[doc = "2: External input pin enabled with falling edge detection when configured as interrupt/DMA request or low level detection when configured as trigger request"]
  WUPE7_2,
  #[doc = "3: External input pin enabled with any change detection when configured as interrupt/DMA request"]
  WUPE7_3,
}
impl From<WUPE7_A> for u8 {
  #[inline(always)]
  fn from(variant: WUPE7_A) -> Self {
    match variant {
      WUPE7_A::WUPE7_0 => 0,
      WUPE7_A::WUPE7_1 => 1,
      WUPE7_A::WUPE7_2 => 2,
      WUPE7_A::WUPE7_3 => 3,
    }
  }
}
#[doc = "Reader of field `WUPE7`"]
pub type WUPE7_R = crate::R<u8, WUPE7_A>;
impl WUPE7_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> WUPE7_A {
    match self.bits {
      0 => WUPE7_A::WUPE7_0,
      1 => WUPE7_A::WUPE7_1,
      2 => WUPE7_A::WUPE7_2,
      3 => WUPE7_A::WUPE7_3,
      _ => unreachable!(),
    }
  }
  #[doc = "Checks if the value of the field is `WUPE7_0`"]
  #[inline(always)]
  pub fn is_wupe7_0(&self) -> bool {
    *self == WUPE7_A::WUPE7_0
  }
  #[doc = "Checks if the value of the field is `WUPE7_1`"]
  #[inline(always)]
  pub fn is_wupe7_1(&self) -> bool {
    *self == WUPE7_A::WUPE7_1
  }
  #[doc = "Checks if the value of the field is `WUPE7_2`"]
  #[inline(always)]
  pub fn is_wupe7_2(&self) -> bool {
    *self == WUPE7_A::WUPE7_2
  }
  #[doc = "Checks if the value of the field is `WUPE7_3`"]
  #[inline(always)]
  pub fn is_wupe7_3(&self) -> bool {
    *self == WUPE7_A::WUPE7_3
  }
}
#[doc = "Write proxy for field `WUPE7`"]
pub struct WUPE7_W<'a> {
  w: &'a mut W,
}
impl<'a> WUPE7_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUPE7_A) -> &'a mut W {
    {
      self.bits(variant.into())
    }
  }
  #[doc = "External input pin disabled as wakeup input"]
  #[inline(always)]
  pub fn wupe7_0(self) -> &'a mut W {
    self.variant(WUPE7_A::WUPE7_0)
  }
  #[doc = "External input pin enabled with rising edge detection when configured as interrupt/DMA request or high level detection when configured as trigger request"]
  #[inline(always)]
  pub fn wupe7_1(self) -> &'a mut W {
    self.variant(WUPE7_A::WUPE7_1)
  }
  #[doc = "External input pin enabled with falling edge detection when configured as interrupt/DMA request or low level detection when configured as trigger request"]
  #[inline(always)]
  pub fn wupe7_2(self) -> &'a mut W {
    self.variant(WUPE7_A::WUPE7_2)
  }
  #[doc = "External input pin enabled with any change detection when configured as interrupt/DMA request"]
  #[inline(always)]
  pub fn wupe7_3(self) -> &'a mut W {
    self.variant(WUPE7_A::WUPE7_3)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
    self.w
  }
}
#[doc = "Wakeup pin enable for LLWU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUPE8_A {
  #[doc = "0: External input pin disabled as wakeup input"]
  WUPE8_0,
  #[doc = "1: External input pin enabled with rising edge detection when configured as interrupt/DMA request or high level detection when configured as trigger request"]
  WUPE8_1,
  #[doc = "2: External input pin enabled with falling edge detection when configured as interrupt/DMA request or low level detection when configured as trigger request"]
  WUPE8_2,
  #[doc = "3: External input pin enabled with any change detection when configured as interrupt/DMA request"]
  WUPE8_3,
}
impl From<WUPE8_A> for u8 {
  #[inline(always)]
  fn from(variant: WUPE8_A) -> Self {
    match variant {
      WUPE8_A::WUPE8_0 => 0,
      WUPE8_A::WUPE8_1 => 1,
      WUPE8_A::WUPE8_2 => 2,
      WUPE8_A::WUPE8_3 => 3,
    }
  }
}
#[doc = "Reader of field `WUPE8`"]
pub type WUPE8_R = crate::R<u8, WUPE8_A>;
impl WUPE8_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> WUPE8_A {
    match self.bits {
      0 => WUPE8_A::WUPE8_0,
      1 => WUPE8_A::WUPE8_1,
      2 => WUPE8_A::WUPE8_2,
      3 => WUPE8_A::WUPE8_3,
      _ => unreachable!(),
    }
  }
  #[doc = "Checks if the value of the field is `WUPE8_0`"]
  #[inline(always)]
  pub fn is_wupe8_0(&self) -> bool {
    *self == WUPE8_A::WUPE8_0
  }
  #[doc = "Checks if the value of the field is `WUPE8_1`"]
  #[inline(always)]
  pub fn is_wupe8_1(&self) -> bool {
    *self == WUPE8_A::WUPE8_1
  }
  #[doc = "Checks if the value of the field is `WUPE8_2`"]
  #[inline(always)]
  pub fn is_wupe8_2(&self) -> bool {
    *self == WUPE8_A::WUPE8_2
  }
  #[doc = "Checks if the value of the field is `WUPE8_3`"]
  #[inline(always)]
  pub fn is_wupe8_3(&self) -> bool {
    *self == WUPE8_A::WUPE8_3
  }
}
#[doc = "Write proxy for field `WUPE8`"]
pub struct WUPE8_W<'a> {
  w: &'a mut W,
}
impl<'a> WUPE8_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUPE8_A) -> &'a mut W {
    {
      self.bits(variant.into())
    }
  }
  #[doc = "External input pin disabled as wakeup input"]
  #[inline(always)]
  pub fn wupe8_0(self) -> &'a mut W {
    self.variant(WUPE8_A::WUPE8_0)
  }
  #[doc = "External input pin enabled with rising edge detection when configured as interrupt/DMA request or high level detection when configured as trigger request"]
  #[inline(always)]
  pub fn wupe8_1(self) -> &'a mut W {
    self.variant(WUPE8_A::WUPE8_1)
  }
  #[doc = "External input pin enabled with falling edge detection when configured as interrupt/DMA request or low level detection when configured as trigger request"]
  #[inline(always)]
  pub fn wupe8_2(self) -> &'a mut W {
    self.variant(WUPE8_A::WUPE8_2)
  }
  #[doc = "External input pin enabled with any change detection when configured as interrupt/DMA request"]
  #[inline(always)]
  pub fn wupe8_3(self) -> &'a mut W {
    self.variant(WUPE8_A::WUPE8_3)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
    self.w
  }
}
#[doc = "Wakeup pin enable for LLWU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUPE9_A {
  #[doc = "0: External input pin disabled as wakeup input"]
  WUPE9_0,
  #[doc = "1: External input pin enabled with rising edge detection when configured as interrupt/DMA request or high level detection when configured as trigger request"]
  WUPE9_1,
  #[doc = "2: External input pin enabled with falling edge detection when configured as interrupt/DMA request or low level detection when configured as trigger request"]
  WUPE9_2,
  #[doc = "3: External input pin enabled with any change detection when configured as interrupt/DMA request"]
  WUPE9_3,
}
impl From<WUPE9_A> for u8 {
  #[inline(always)]
  fn from(variant: WUPE9_A) -> Self {
    match variant {
      WUPE9_A::WUPE9_0 => 0,
      WUPE9_A::WUPE9_1 => 1,
      WUPE9_A::WUPE9_2 => 2,
      WUPE9_A::WUPE9_3 => 3,
    }
  }
}
#[doc = "Reader of field `WUPE9`"]
pub type WUPE9_R = crate::R<u8, WUPE9_A>;
impl WUPE9_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> WUPE9_A {
    match self.bits {
      0 => WUPE9_A::WUPE9_0,
      1 => WUPE9_A::WUPE9_1,
      2 => WUPE9_A::WUPE9_2,
      3 => WUPE9_A::WUPE9_3,
      _ => unreachable!(),
    }
  }
  #[doc = "Checks if the value of the field is `WUPE9_0`"]
  #[inline(always)]
  pub fn is_wupe9_0(&self) -> bool {
    *self == WUPE9_A::WUPE9_0
  }
  #[doc = "Checks if the value of the field is `WUPE9_1`"]
  #[inline(always)]
  pub fn is_wupe9_1(&self) -> bool {
    *self == WUPE9_A::WUPE9_1
  }
  #[doc = "Checks if the value of the field is `WUPE9_2`"]
  #[inline(always)]
  pub fn is_wupe9_2(&self) -> bool {
    *self == WUPE9_A::WUPE9_2
  }
  #[doc = "Checks if the value of the field is `WUPE9_3`"]
  #[inline(always)]
  pub fn is_wupe9_3(&self) -> bool {
    *self == WUPE9_A::WUPE9_3
  }
}
#[doc = "Write proxy for field `WUPE9`"]
pub struct WUPE9_W<'a> {
  w: &'a mut W,
}
impl<'a> WUPE9_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUPE9_A) -> &'a mut W {
    {
      self.bits(variant.into())
    }
  }
  #[doc = "External input pin disabled as wakeup input"]
  #[inline(always)]
  pub fn wupe9_0(self) -> &'a mut W {
    self.variant(WUPE9_A::WUPE9_0)
  }
  #[doc = "External input pin enabled with rising edge detection when configured as interrupt/DMA request or high level detection when configured as trigger request"]
  #[inline(always)]
  pub fn wupe9_1(self) -> &'a mut W {
    self.variant(WUPE9_A::WUPE9_1)
  }
  #[doc = "External input pin enabled with falling edge detection when configured as interrupt/DMA request or low level detection when configured as trigger request"]
  #[inline(always)]
  pub fn wupe9_2(self) -> &'a mut W {
    self.variant(WUPE9_A::WUPE9_2)
  }
  #[doc = "External input pin enabled with any change detection when configured as interrupt/DMA request"]
  #[inline(always)]
  pub fn wupe9_3(self) -> &'a mut W {
    self.variant(WUPE9_A::WUPE9_3)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
    self.w
  }
}
#[doc = "Wakeup pin enable for LLWU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUPE10_A {
  #[doc = "0: External input pin disabled as wakeup input"]
  WUPE10_0,
  #[doc = "1: External input pin enabled with rising edge detection when configured as interrupt/DMA request or high level detection when configured as trigger request"]
  WUPE10_1,
  #[doc = "2: External input pin enabled with falling edge detection when configured as interrupt/DMA request or low level detection when configured as trigger request"]
  WUPE10_2,
  #[doc = "3: External input pin enabled with any change detection when configured as interrupt/DMA request"]
  WUPE10_3,
}
impl From<WUPE10_A> for u8 {
  #[inline(always)]
  fn from(variant: WUPE10_A) -> Self {
    match variant {
      WUPE10_A::WUPE10_0 => 0,
      WUPE10_A::WUPE10_1 => 1,
      WUPE10_A::WUPE10_2 => 2,
      WUPE10_A::WUPE10_3 => 3,
    }
  }
}
#[doc = "Reader of field `WUPE10`"]
pub type WUPE10_R = crate::R<u8, WUPE10_A>;
impl WUPE10_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> WUPE10_A {
    match self.bits {
      0 => WUPE10_A::WUPE10_0,
      1 => WUPE10_A::WUPE10_1,
      2 => WUPE10_A::WUPE10_2,
      3 => WUPE10_A::WUPE10_3,
      _ => unreachable!(),
    }
  }
  #[doc = "Checks if the value of the field is `WUPE10_0`"]
  #[inline(always)]
  pub fn is_wupe10_0(&self) -> bool {
    *self == WUPE10_A::WUPE10_0
  }
  #[doc = "Checks if the value of the field is `WUPE10_1`"]
  #[inline(always)]
  pub fn is_wupe10_1(&self) -> bool {
    *self == WUPE10_A::WUPE10_1
  }
  #[doc = "Checks if the value of the field is `WUPE10_2`"]
  #[inline(always)]
  pub fn is_wupe10_2(&self) -> bool {
    *self == WUPE10_A::WUPE10_2
  }
  #[doc = "Checks if the value of the field is `WUPE10_3`"]
  #[inline(always)]
  pub fn is_wupe10_3(&self) -> bool {
    *self == WUPE10_A::WUPE10_3
  }
}
#[doc = "Write proxy for field `WUPE10`"]
pub struct WUPE10_W<'a> {
  w: &'a mut W,
}
impl<'a> WUPE10_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUPE10_A) -> &'a mut W {
    {
      self.bits(variant.into())
    }
  }
  #[doc = "External input pin disabled as wakeup input"]
  #[inline(always)]
  pub fn wupe10_0(self) -> &'a mut W {
    self.variant(WUPE10_A::WUPE10_0)
  }
  #[doc = "External input pin enabled with rising edge detection when configured as interrupt/DMA request or high level detection when configured as trigger request"]
  #[inline(always)]
  pub fn wupe10_1(self) -> &'a mut W {
    self.variant(WUPE10_A::WUPE10_1)
  }
  #[doc = "External input pin enabled with falling edge detection when configured as interrupt/DMA request or low level detection when configured as trigger request"]
  #[inline(always)]
  pub fn wupe10_2(self) -> &'a mut W {
    self.variant(WUPE10_A::WUPE10_2)
  }
  #[doc = "External input pin enabled with any change detection when configured as interrupt/DMA request"]
  #[inline(always)]
  pub fn wupe10_3(self) -> &'a mut W {
    self.variant(WUPE10_A::WUPE10_3)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
    self.w
  }
}
#[doc = "Wakeup pin enable for LLWU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUPE11_A {
  #[doc = "0: External input pin disabled as wakeup input"]
  WUPE11_0,
  #[doc = "1: External input pin enabled with rising edge detection when configured as interrupt/DMA request or high level detection when configured as trigger request"]
  WUPE11_1,
  #[doc = "2: External input pin enabled with falling edge detection when configured as interrupt/DMA request or low level detection when configured as trigger request"]
  WUPE11_2,
  #[doc = "3: External input pin enabled with any change detection when configured as interrupt/DMA request"]
  WUPE11_3,
}
impl From<WUPE11_A> for u8 {
  #[inline(always)]
  fn from(variant: WUPE11_A) -> Self {
    match variant {
      WUPE11_A::WUPE11_0 => 0,
      WUPE11_A::WUPE11_1 => 1,
      WUPE11_A::WUPE11_2 => 2,
      WUPE11_A::WUPE11_3 => 3,
    }
  }
}
#[doc = "Reader of field `WUPE11`"]
pub type WUPE11_R = crate::R<u8, WUPE11_A>;
impl WUPE11_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> WUPE11_A {
    match self.bits {
      0 => WUPE11_A::WUPE11_0,
      1 => WUPE11_A::WUPE11_1,
      2 => WUPE11_A::WUPE11_2,
      3 => WUPE11_A::WUPE11_3,
      _ => unreachable!(),
    }
  }
  #[doc = "Checks if the value of the field is `WUPE11_0`"]
  #[inline(always)]
  pub fn is_wupe11_0(&self) -> bool {
    *self == WUPE11_A::WUPE11_0
  }
  #[doc = "Checks if the value of the field is `WUPE11_1`"]
  #[inline(always)]
  pub fn is_wupe11_1(&self) -> bool {
    *self == WUPE11_A::WUPE11_1
  }
  #[doc = "Checks if the value of the field is `WUPE11_2`"]
  #[inline(always)]
  pub fn is_wupe11_2(&self) -> bool {
    *self == WUPE11_A::WUPE11_2
  }
  #[doc = "Checks if the value of the field is `WUPE11_3`"]
  #[inline(always)]
  pub fn is_wupe11_3(&self) -> bool {
    *self == WUPE11_A::WUPE11_3
  }
}
#[doc = "Write proxy for field `WUPE11`"]
pub struct WUPE11_W<'a> {
  w: &'a mut W,
}
impl<'a> WUPE11_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUPE11_A) -> &'a mut W {
    {
      self.bits(variant.into())
    }
  }
  #[doc = "External input pin disabled as wakeup input"]
  #[inline(always)]
  pub fn wupe11_0(self) -> &'a mut W {
    self.variant(WUPE11_A::WUPE11_0)
  }
  #[doc = "External input pin enabled with rising edge detection when configured as interrupt/DMA request or high level detection when configured as trigger request"]
  #[inline(always)]
  pub fn wupe11_1(self) -> &'a mut W {
    self.variant(WUPE11_A::WUPE11_1)
  }
  #[doc = "External input pin enabled with falling edge detection when configured as interrupt/DMA request or low level detection when configured as trigger request"]
  #[inline(always)]
  pub fn wupe11_2(self) -> &'a mut W {
    self.variant(WUPE11_A::WUPE11_2)
  }
  #[doc = "External input pin enabled with any change detection when configured as interrupt/DMA request"]
  #[inline(always)]
  pub fn wupe11_3(self) -> &'a mut W {
    self.variant(WUPE11_A::WUPE11_3)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
    self.w
  }
}
#[doc = "Wakeup pin enable for LLWU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUPE12_A {
  #[doc = "0: External input pin disabled as wakeup input"]
  WUPE12_0,
  #[doc = "1: External input pin enabled with rising edge detection when configured as interrupt/DMA request or high level detection when configured as trigger request"]
  WUPE12_1,
  #[doc = "2: External input pin enabled with falling edge detection when configured as interrupt/DMA request or low level detection when configured as trigger request"]
  WUPE12_2,
  #[doc = "3: External input pin enabled with any change detection when configured as interrupt/DMA request"]
  WUPE12_3,
}
impl From<WUPE12_A> for u8 {
  #[inline(always)]
  fn from(variant: WUPE12_A) -> Self {
    match variant {
      WUPE12_A::WUPE12_0 => 0,
      WUPE12_A::WUPE12_1 => 1,
      WUPE12_A::WUPE12_2 => 2,
      WUPE12_A::WUPE12_3 => 3,
    }
  }
}
#[doc = "Reader of field `WUPE12`"]
pub type WUPE12_R = crate::R<u8, WUPE12_A>;
impl WUPE12_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> WUPE12_A {
    match self.bits {
      0 => WUPE12_A::WUPE12_0,
      1 => WUPE12_A::WUPE12_1,
      2 => WUPE12_A::WUPE12_2,
      3 => WUPE12_A::WUPE12_3,
      _ => unreachable!(),
    }
  }
  #[doc = "Checks if the value of the field is `WUPE12_0`"]
  #[inline(always)]
  pub fn is_wupe12_0(&self) -> bool {
    *self == WUPE12_A::WUPE12_0
  }
  #[doc = "Checks if the value of the field is `WUPE12_1`"]
  #[inline(always)]
  pub fn is_wupe12_1(&self) -> bool {
    *self == WUPE12_A::WUPE12_1
  }
  #[doc = "Checks if the value of the field is `WUPE12_2`"]
  #[inline(always)]
  pub fn is_wupe12_2(&self) -> bool {
    *self == WUPE12_A::WUPE12_2
  }
  #[doc = "Checks if the value of the field is `WUPE12_3`"]
  #[inline(always)]
  pub fn is_wupe12_3(&self) -> bool {
    *self == WUPE12_A::WUPE12_3
  }
}
#[doc = "Write proxy for field `WUPE12`"]
pub struct WUPE12_W<'a> {
  w: &'a mut W,
}
impl<'a> WUPE12_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUPE12_A) -> &'a mut W {
    {
      self.bits(variant.into())
    }
  }
  #[doc = "External input pin disabled as wakeup input"]
  #[inline(always)]
  pub fn wupe12_0(self) -> &'a mut W {
    self.variant(WUPE12_A::WUPE12_0)
  }
  #[doc = "External input pin enabled with rising edge detection when configured as interrupt/DMA request or high level detection when configured as trigger request"]
  #[inline(always)]
  pub fn wupe12_1(self) -> &'a mut W {
    self.variant(WUPE12_A::WUPE12_1)
  }
  #[doc = "External input pin enabled with falling edge detection when configured as interrupt/DMA request or low level detection when configured as trigger request"]
  #[inline(always)]
  pub fn wupe12_2(self) -> &'a mut W {
    self.variant(WUPE12_A::WUPE12_2)
  }
  #[doc = "External input pin enabled with any change detection when configured as interrupt/DMA request"]
  #[inline(always)]
  pub fn wupe12_3(self) -> &'a mut W {
    self.variant(WUPE12_A::WUPE12_3)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
    self.w
  }
}
#[doc = "Wakeup pin enable for LLWU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUPE13_A {
  #[doc = "0: External input pin disabled as wakeup input"]
  WUPE13_0,
  #[doc = "1: External input pin enabled with rising edge detection when configured as interrupt/DMA request or high level detection when configured as trigger request"]
  WUPE13_1,
  #[doc = "2: External input pin enabled with falling edge detection when configured as interrupt/DMA request or low level detection when configured as trigger request"]
  WUPE13_2,
  #[doc = "3: External input pin enabled with any change detection when configured as interrupt/DMA request"]
  WUPE13_3,
}
impl From<WUPE13_A> for u8 {
  #[inline(always)]
  fn from(variant: WUPE13_A) -> Self {
    match variant {
      WUPE13_A::WUPE13_0 => 0,
      WUPE13_A::WUPE13_1 => 1,
      WUPE13_A::WUPE13_2 => 2,
      WUPE13_A::WUPE13_3 => 3,
    }
  }
}
#[doc = "Reader of field `WUPE13`"]
pub type WUPE13_R = crate::R<u8, WUPE13_A>;
impl WUPE13_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> WUPE13_A {
    match self.bits {
      0 => WUPE13_A::WUPE13_0,
      1 => WUPE13_A::WUPE13_1,
      2 => WUPE13_A::WUPE13_2,
      3 => WUPE13_A::WUPE13_3,
      _ => unreachable!(),
    }
  }
  #[doc = "Checks if the value of the field is `WUPE13_0`"]
  #[inline(always)]
  pub fn is_wupe13_0(&self) -> bool {
    *self == WUPE13_A::WUPE13_0
  }
  #[doc = "Checks if the value of the field is `WUPE13_1`"]
  #[inline(always)]
  pub fn is_wupe13_1(&self) -> bool {
    *self == WUPE13_A::WUPE13_1
  }
  #[doc = "Checks if the value of the field is `WUPE13_2`"]
  #[inline(always)]
  pub fn is_wupe13_2(&self) -> bool {
    *self == WUPE13_A::WUPE13_2
  }
  #[doc = "Checks if the value of the field is `WUPE13_3`"]
  #[inline(always)]
  pub fn is_wupe13_3(&self) -> bool {
    *self == WUPE13_A::WUPE13_3
  }
}
#[doc = "Write proxy for field `WUPE13`"]
pub struct WUPE13_W<'a> {
  w: &'a mut W,
}
impl<'a> WUPE13_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUPE13_A) -> &'a mut W {
    {
      self.bits(variant.into())
    }
  }
  #[doc = "External input pin disabled as wakeup input"]
  #[inline(always)]
  pub fn wupe13_0(self) -> &'a mut W {
    self.variant(WUPE13_A::WUPE13_0)
  }
  #[doc = "External input pin enabled with rising edge detection when configured as interrupt/DMA request or high level detection when configured as trigger request"]
  #[inline(always)]
  pub fn wupe13_1(self) -> &'a mut W {
    self.variant(WUPE13_A::WUPE13_1)
  }
  #[doc = "External input pin enabled with falling edge detection when configured as interrupt/DMA request or low level detection when configured as trigger request"]
  #[inline(always)]
  pub fn wupe13_2(self) -> &'a mut W {
    self.variant(WUPE13_A::WUPE13_2)
  }
  #[doc = "External input pin enabled with any change detection when configured as interrupt/DMA request"]
  #[inline(always)]
  pub fn wupe13_3(self) -> &'a mut W {
    self.variant(WUPE13_A::WUPE13_3)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x03 << 26)) | (((value as u32) & 0x03) << 26);
    self.w
  }
}
#[doc = "Wakeup pin enable for LLWU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUPE14_A {
  #[doc = "0: External input pin disabled as wakeup input"]
  WUPE14_0,
  #[doc = "1: External input pin enabled with rising edge detection when configured as interrupt/DMA request or high level detection when configured as trigger request"]
  WUPE14_1,
  #[doc = "2: External input pin enabled with falling edge detection when configured as interrupt/DMA request or low level detection when configured as trigger request"]
  WUPE14_2,
  #[doc = "3: External input pin enabled with any change detection when configured as interrupt/DMA request"]
  WUPE14_3,
}
impl From<WUPE14_A> for u8 {
  #[inline(always)]
  fn from(variant: WUPE14_A) -> Self {
    match variant {
      WUPE14_A::WUPE14_0 => 0,
      WUPE14_A::WUPE14_1 => 1,
      WUPE14_A::WUPE14_2 => 2,
      WUPE14_A::WUPE14_3 => 3,
    }
  }
}
#[doc = "Reader of field `WUPE14`"]
pub type WUPE14_R = crate::R<u8, WUPE14_A>;
impl WUPE14_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> WUPE14_A {
    match self.bits {
      0 => WUPE14_A::WUPE14_0,
      1 => WUPE14_A::WUPE14_1,
      2 => WUPE14_A::WUPE14_2,
      3 => WUPE14_A::WUPE14_3,
      _ => unreachable!(),
    }
  }
  #[doc = "Checks if the value of the field is `WUPE14_0`"]
  #[inline(always)]
  pub fn is_wupe14_0(&self) -> bool {
    *self == WUPE14_A::WUPE14_0
  }
  #[doc = "Checks if the value of the field is `WUPE14_1`"]
  #[inline(always)]
  pub fn is_wupe14_1(&self) -> bool {
    *self == WUPE14_A::WUPE14_1
  }
  #[doc = "Checks if the value of the field is `WUPE14_2`"]
  #[inline(always)]
  pub fn is_wupe14_2(&self) -> bool {
    *self == WUPE14_A::WUPE14_2
  }
  #[doc = "Checks if the value of the field is `WUPE14_3`"]
  #[inline(always)]
  pub fn is_wupe14_3(&self) -> bool {
    *self == WUPE14_A::WUPE14_3
  }
}
#[doc = "Write proxy for field `WUPE14`"]
pub struct WUPE14_W<'a> {
  w: &'a mut W,
}
impl<'a> WUPE14_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUPE14_A) -> &'a mut W {
    {
      self.bits(variant.into())
    }
  }
  #[doc = "External input pin disabled as wakeup input"]
  #[inline(always)]
  pub fn wupe14_0(self) -> &'a mut W {
    self.variant(WUPE14_A::WUPE14_0)
  }
  #[doc = "External input pin enabled with rising edge detection when configured as interrupt/DMA request or high level detection when configured as trigger request"]
  #[inline(always)]
  pub fn wupe14_1(self) -> &'a mut W {
    self.variant(WUPE14_A::WUPE14_1)
  }
  #[doc = "External input pin enabled with falling edge detection when configured as interrupt/DMA request or low level detection when configured as trigger request"]
  #[inline(always)]
  pub fn wupe14_2(self) -> &'a mut W {
    self.variant(WUPE14_A::WUPE14_2)
  }
  #[doc = "External input pin enabled with any change detection when configured as interrupt/DMA request"]
  #[inline(always)]
  pub fn wupe14_3(self) -> &'a mut W {
    self.variant(WUPE14_A::WUPE14_3)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
    self.w
  }
}
#[doc = "Wakeup pin enable for LLWU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUPE15_A {
  #[doc = "0: External input pin disabled as wakeup input"]
  WUPE15_0,
  #[doc = "1: External input pin enabled with rising edge detection when configured as interrupt/DMA request or high level detection when configured as trigger request"]
  WUPE15_1,
  #[doc = "2: External input pin enabled with falling edge detection when configured as interrupt/DMA request or low level detection when configured as trigger request"]
  WUPE15_2,
  #[doc = "3: External input pin enabled with any change detection when configured as interrupt/DMA request"]
  WUPE15_3,
}
impl From<WUPE15_A> for u8 {
  #[inline(always)]
  fn from(variant: WUPE15_A) -> Self {
    match variant {
      WUPE15_A::WUPE15_0 => 0,
      WUPE15_A::WUPE15_1 => 1,
      WUPE15_A::WUPE15_2 => 2,
      WUPE15_A::WUPE15_3 => 3,
    }
  }
}
#[doc = "Reader of field `WUPE15`"]
pub type WUPE15_R = crate::R<u8, WUPE15_A>;
impl WUPE15_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> WUPE15_A {
    match self.bits {
      0 => WUPE15_A::WUPE15_0,
      1 => WUPE15_A::WUPE15_1,
      2 => WUPE15_A::WUPE15_2,
      3 => WUPE15_A::WUPE15_3,
      _ => unreachable!(),
    }
  }
  #[doc = "Checks if the value of the field is `WUPE15_0`"]
  #[inline(always)]
  pub fn is_wupe15_0(&self) -> bool {
    *self == WUPE15_A::WUPE15_0
  }
  #[doc = "Checks if the value of the field is `WUPE15_1`"]
  #[inline(always)]
  pub fn is_wupe15_1(&self) -> bool {
    *self == WUPE15_A::WUPE15_1
  }
  #[doc = "Checks if the value of the field is `WUPE15_2`"]
  #[inline(always)]
  pub fn is_wupe15_2(&self) -> bool {
    *self == WUPE15_A::WUPE15_2
  }
  #[doc = "Checks if the value of the field is `WUPE15_3`"]
  #[inline(always)]
  pub fn is_wupe15_3(&self) -> bool {
    *self == WUPE15_A::WUPE15_3
  }
}
#[doc = "Write proxy for field `WUPE15`"]
pub struct WUPE15_W<'a> {
  w: &'a mut W,
}
impl<'a> WUPE15_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUPE15_A) -> &'a mut W {
    {
      self.bits(variant.into())
    }
  }
  #[doc = "External input pin disabled as wakeup input"]
  #[inline(always)]
  pub fn wupe15_0(self) -> &'a mut W {
    self.variant(WUPE15_A::WUPE15_0)
  }
  #[doc = "External input pin enabled with rising edge detection when configured as interrupt/DMA request or high level detection when configured as trigger request"]
  #[inline(always)]
  pub fn wupe15_1(self) -> &'a mut W {
    self.variant(WUPE15_A::WUPE15_1)
  }
  #[doc = "External input pin enabled with falling edge detection when configured as interrupt/DMA request or low level detection when configured as trigger request"]
  #[inline(always)]
  pub fn wupe15_2(self) -> &'a mut W {
    self.variant(WUPE15_A::WUPE15_2)
  }
  #[doc = "External input pin enabled with any change detection when configured as interrupt/DMA request"]
  #[inline(always)]
  pub fn wupe15_3(self) -> &'a mut W {
    self.variant(WUPE15_A::WUPE15_3)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
    self.w
  }
}
impl R {
  #[doc = "Bits 0:1 - Wakeup pin enable for LLWU_Pn"]
  #[inline(always)]
  pub fn wupe0(&self) -> WUPE0_R {
    WUPE0_R::new((self.bits & 0x03) as u8)
  }
  #[doc = "Bits 2:3 - Wakeup pin enable for LLWU_Pn"]
  #[inline(always)]
  pub fn wupe1(&self) -> WUPE1_R {
    WUPE1_R::new(((self.bits >> 2) & 0x03) as u8)
  }
  #[doc = "Bits 4:5 - Wakeup pin enable for LLWU_Pn"]
  #[inline(always)]
  pub fn wupe2(&self) -> WUPE2_R {
    WUPE2_R::new(((self.bits >> 4) & 0x03) as u8)
  }
  #[doc = "Bits 6:7 - Wakeup pin enable for LLWU_Pn"]
  #[inline(always)]
  pub fn wupe3(&self) -> WUPE3_R {
    WUPE3_R::new(((self.bits >> 6) & 0x03) as u8)
  }
  #[doc = "Bits 8:9 - Wakeup pin enable for LLWU_Pn"]
  #[inline(always)]
  pub fn wupe4(&self) -> WUPE4_R {
    WUPE4_R::new(((self.bits >> 8) & 0x03) as u8)
  }
  #[doc = "Bits 10:11 - Wakeup pin enable for LLWU_Pn"]
  #[inline(always)]
  pub fn wupe5(&self) -> WUPE5_R {
    WUPE5_R::new(((self.bits >> 10) & 0x03) as u8)
  }
  #[doc = "Bits 12:13 - Wakeup pin enable for LLWU_Pn"]
  #[inline(always)]
  pub fn wupe6(&self) -> WUPE6_R {
    WUPE6_R::new(((self.bits >> 12) & 0x03) as u8)
  }
  #[doc = "Bits 14:15 - Wakeup pin enable for LLWU_Pn"]
  #[inline(always)]
  pub fn wupe7(&self) -> WUPE7_R {
    WUPE7_R::new(((self.bits >> 14) & 0x03) as u8)
  }
  #[doc = "Bits 16:17 - Wakeup pin enable for LLWU_Pn"]
  #[inline(always)]
  pub fn wupe8(&self) -> WUPE8_R {
    WUPE8_R::new(((self.bits >> 16) & 0x03) as u8)
  }
  #[doc = "Bits 18:19 - Wakeup pin enable for LLWU_Pn"]
  #[inline(always)]
  pub fn wupe9(&self) -> WUPE9_R {
    WUPE9_R::new(((self.bits >> 18) & 0x03) as u8)
  }
  #[doc = "Bits 20:21 - Wakeup pin enable for LLWU_Pn"]
  #[inline(always)]
  pub fn wupe10(&self) -> WUPE10_R {
    WUPE10_R::new(((self.bits >> 20) & 0x03) as u8)
  }
  #[doc = "Bits 22:23 - Wakeup pin enable for LLWU_Pn"]
  #[inline(always)]
  pub fn wupe11(&self) -> WUPE11_R {
    WUPE11_R::new(((self.bits >> 22) & 0x03) as u8)
  }
  #[doc = "Bits 24:25 - Wakeup pin enable for LLWU_Pn"]
  #[inline(always)]
  pub fn wupe12(&self) -> WUPE12_R {
    WUPE12_R::new(((self.bits >> 24) & 0x03) as u8)
  }
  #[doc = "Bits 26:27 - Wakeup pin enable for LLWU_Pn"]
  #[inline(always)]
  pub fn wupe13(&self) -> WUPE13_R {
    WUPE13_R::new(((self.bits >> 26) & 0x03) as u8)
  }
  #[doc = "Bits 28:29 - Wakeup pin enable for LLWU_Pn"]
  #[inline(always)]
  pub fn wupe14(&self) -> WUPE14_R {
    WUPE14_R::new(((self.bits >> 28) & 0x03) as u8)
  }
  #[doc = "Bits 30:31 - Wakeup pin enable for LLWU_Pn"]
  #[inline(always)]
  pub fn wupe15(&self) -> WUPE15_R {
    WUPE15_R::new(((self.bits >> 30) & 0x03) as u8)
  }
}
impl W {
  #[doc = "Bits 0:1 - Wakeup pin enable for LLWU_Pn"]
  #[inline(always)]
  pub fn wupe0(&mut self) -> WUPE0_W {
    WUPE0_W { w: self }
  }
  #[doc = "Bits 2:3 - Wakeup pin enable for LLWU_Pn"]
  #[inline(always)]
  pub fn wupe1(&mut self) -> WUPE1_W {
    WUPE1_W { w: self }
  }
  #[doc = "Bits 4:5 - Wakeup pin enable for LLWU_Pn"]
  #[inline(always)]
  pub fn wupe2(&mut self) -> WUPE2_W {
    WUPE2_W { w: self }
  }
  #[doc = "Bits 6:7 - Wakeup pin enable for LLWU_Pn"]
  #[inline(always)]
  pub fn wupe3(&mut self) -> WUPE3_W {
    WUPE3_W { w: self }
  }
  #[doc = "Bits 8:9 - Wakeup pin enable for LLWU_Pn"]
  #[inline(always)]
  pub fn wupe4(&mut self) -> WUPE4_W {
    WUPE4_W { w: self }
  }
  #[doc = "Bits 10:11 - Wakeup pin enable for LLWU_Pn"]
  #[inline(always)]
  pub fn wupe5(&mut self) -> WUPE5_W {
    WUPE5_W { w: self }
  }
  #[doc = "Bits 12:13 - Wakeup pin enable for LLWU_Pn"]
  #[inline(always)]
  pub fn wupe6(&mut self) -> WUPE6_W {
    WUPE6_W { w: self }
  }
  #[doc = "Bits 14:15 - Wakeup pin enable for LLWU_Pn"]
  #[inline(always)]
  pub fn wupe7(&mut self) -> WUPE7_W {
    WUPE7_W { w: self }
  }
  #[doc = "Bits 16:17 - Wakeup pin enable for LLWU_Pn"]
  #[inline(always)]
  pub fn wupe8(&mut self) -> WUPE8_W {
    WUPE8_W { w: self }
  }
  #[doc = "Bits 18:19 - Wakeup pin enable for LLWU_Pn"]
  #[inline(always)]
  pub fn wupe9(&mut self) -> WUPE9_W {
    WUPE9_W { w: self }
  }
  #[doc = "Bits 20:21 - Wakeup pin enable for LLWU_Pn"]
  #[inline(always)]
  pub fn wupe10(&mut self) -> WUPE10_W {
    WUPE10_W { w: self }
  }
  #[doc = "Bits 22:23 - Wakeup pin enable for LLWU_Pn"]
  #[inline(always)]
  pub fn wupe11(&mut self) -> WUPE11_W {
    WUPE11_W { w: self }
  }
  #[doc = "Bits 24:25 - Wakeup pin enable for LLWU_Pn"]
  #[inline(always)]
  pub fn wupe12(&mut self) -> WUPE12_W {
    WUPE12_W { w: self }
  }
  #[doc = "Bits 26:27 - Wakeup pin enable for LLWU_Pn"]
  #[inline(always)]
  pub fn wupe13(&mut self) -> WUPE13_W {
    WUPE13_W { w: self }
  }
  #[doc = "Bits 28:29 - Wakeup pin enable for LLWU_Pn"]
  #[inline(always)]
  pub fn wupe14(&mut self) -> WUPE14_W {
    WUPE14_W { w: self }
  }
  #[doc = "Bits 30:31 - Wakeup pin enable for LLWU_Pn"]
  #[inline(always)]
  pub fn wupe15(&mut self) -> WUPE15_W {
    WUPE15_W { w: self }
  }
}
