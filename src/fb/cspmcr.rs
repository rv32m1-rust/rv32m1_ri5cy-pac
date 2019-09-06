#[doc = "Reader of register CSPMCR"]
pub type R = crate::R<u32, super::CSPMCR>;
#[doc = "Writer for register CSPMCR"]
pub type W = crate::W<u32, super::CSPMCR>;
#[doc = "Register CSPMCR `reset()`'s with value 0"]
impl crate::ResetValue for super::CSPMCR {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "FlexBus Signal Group 5 Multiplex control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GROUP5_A {
  #[doc = "0: no description available"]
  GROUP5_0,
  #[doc = "1: no description available"]
  GROUP5_1,
  #[doc = "2: no description available"]
  GROUP5_2,
}
impl From<GROUP5_A> for u8 {
  #[inline(always)]
  fn from(variant: GROUP5_A) -> Self {
    match variant {
      GROUP5_A::GROUP5_0 => 0,
      GROUP5_A::GROUP5_1 => 1,
      GROUP5_A::GROUP5_2 => 2,
    }
  }
}
#[doc = "Reader of field `GROUP5`"]
pub type GROUP5_R = crate::R<u8, GROUP5_A>;
impl GROUP5_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> crate::Variant<u8, GROUP5_A> {
    use crate::Variant::*;
    match self.bits {
      0 => Val(GROUP5_A::GROUP5_0),
      1 => Val(GROUP5_A::GROUP5_1),
      2 => Val(GROUP5_A::GROUP5_2),
      i => Res(i),
    }
  }
  #[doc = "Checks if the value of the field is `GROUP5_0`"]
  #[inline(always)]
  pub fn is_group5_0(&self) -> bool {
    *self == GROUP5_A::GROUP5_0
  }
  #[doc = "Checks if the value of the field is `GROUP5_1`"]
  #[inline(always)]
  pub fn is_group5_1(&self) -> bool {
    *self == GROUP5_A::GROUP5_1
  }
  #[doc = "Checks if the value of the field is `GROUP5_2`"]
  #[inline(always)]
  pub fn is_group5_2(&self) -> bool {
    *self == GROUP5_A::GROUP5_2
  }
}
#[doc = "Write proxy for field `GROUP5`"]
pub struct GROUP5_W<'a> {
  w: &'a mut W,
}
impl<'a> GROUP5_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: GROUP5_A) -> &'a mut W {
    unsafe { self.bits(variant.into()) }
  }
  #[doc = "no description available"]
  #[inline(always)]
  pub fn group5_0(self) -> &'a mut W {
    self.variant(GROUP5_A::GROUP5_0)
  }
  #[doc = "no description available"]
  #[inline(always)]
  pub fn group5_1(self) -> &'a mut W {
    self.variant(GROUP5_A::GROUP5_1)
  }
  #[doc = "no description available"]
  #[inline(always)]
  pub fn group5_2(self) -> &'a mut W {
    self.variant(GROUP5_A::GROUP5_2)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
    self.w
  }
}
#[doc = "FlexBus Signal Group 4 Multiplex control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GROUP4_A {
  #[doc = "0: no description available"]
  GROUP4_0,
  #[doc = "1: no description available"]
  GROUP4_1,
  #[doc = "2: no description available"]
  GROUP4_2,
}
impl From<GROUP4_A> for u8 {
  #[inline(always)]
  fn from(variant: GROUP4_A) -> Self {
    match variant {
      GROUP4_A::GROUP4_0 => 0,
      GROUP4_A::GROUP4_1 => 1,
      GROUP4_A::GROUP4_2 => 2,
    }
  }
}
#[doc = "Reader of field `GROUP4`"]
pub type GROUP4_R = crate::R<u8, GROUP4_A>;
impl GROUP4_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> crate::Variant<u8, GROUP4_A> {
    use crate::Variant::*;
    match self.bits {
      0 => Val(GROUP4_A::GROUP4_0),
      1 => Val(GROUP4_A::GROUP4_1),
      2 => Val(GROUP4_A::GROUP4_2),
      i => Res(i),
    }
  }
  #[doc = "Checks if the value of the field is `GROUP4_0`"]
  #[inline(always)]
  pub fn is_group4_0(&self) -> bool {
    *self == GROUP4_A::GROUP4_0
  }
  #[doc = "Checks if the value of the field is `GROUP4_1`"]
  #[inline(always)]
  pub fn is_group4_1(&self) -> bool {
    *self == GROUP4_A::GROUP4_1
  }
  #[doc = "Checks if the value of the field is `GROUP4_2`"]
  #[inline(always)]
  pub fn is_group4_2(&self) -> bool {
    *self == GROUP4_A::GROUP4_2
  }
}
#[doc = "Write proxy for field `GROUP4`"]
pub struct GROUP4_W<'a> {
  w: &'a mut W,
}
impl<'a> GROUP4_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: GROUP4_A) -> &'a mut W {
    unsafe { self.bits(variant.into()) }
  }
  #[doc = "no description available"]
  #[inline(always)]
  pub fn group4_0(self) -> &'a mut W {
    self.variant(GROUP4_A::GROUP4_0)
  }
  #[doc = "no description available"]
  #[inline(always)]
  pub fn group4_1(self) -> &'a mut W {
    self.variant(GROUP4_A::GROUP4_1)
  }
  #[doc = "no description available"]
  #[inline(always)]
  pub fn group4_2(self) -> &'a mut W {
    self.variant(GROUP4_A::GROUP4_2)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
    self.w
  }
}
#[doc = "FlexBus Signal Group 3 Multiplex control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GROUP3_A {
  #[doc = "0: no description available"]
  GROUP3_0,
  #[doc = "1: FB_TSIZ1"]
  GROUP3_1,
  #[doc = "2: no description available"]
  GROUP3_2,
}
impl From<GROUP3_A> for u8 {
  #[inline(always)]
  fn from(variant: GROUP3_A) -> Self {
    match variant {
      GROUP3_A::GROUP3_0 => 0,
      GROUP3_A::GROUP3_1 => 1,
      GROUP3_A::GROUP3_2 => 2,
    }
  }
}
#[doc = "Reader of field `GROUP3`"]
pub type GROUP3_R = crate::R<u8, GROUP3_A>;
impl GROUP3_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> crate::Variant<u8, GROUP3_A> {
    use crate::Variant::*;
    match self.bits {
      0 => Val(GROUP3_A::GROUP3_0),
      1 => Val(GROUP3_A::GROUP3_1),
      2 => Val(GROUP3_A::GROUP3_2),
      i => Res(i),
    }
  }
  #[doc = "Checks if the value of the field is `GROUP3_0`"]
  #[inline(always)]
  pub fn is_group3_0(&self) -> bool {
    *self == GROUP3_A::GROUP3_0
  }
  #[doc = "Checks if the value of the field is `GROUP3_1`"]
  #[inline(always)]
  pub fn is_group3_1(&self) -> bool {
    *self == GROUP3_A::GROUP3_1
  }
  #[doc = "Checks if the value of the field is `GROUP3_2`"]
  #[inline(always)]
  pub fn is_group3_2(&self) -> bool {
    *self == GROUP3_A::GROUP3_2
  }
}
#[doc = "Write proxy for field `GROUP3`"]
pub struct GROUP3_W<'a> {
  w: &'a mut W,
}
impl<'a> GROUP3_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: GROUP3_A) -> &'a mut W {
    unsafe { self.bits(variant.into()) }
  }
  #[doc = "no description available"]
  #[inline(always)]
  pub fn group3_0(self) -> &'a mut W {
    self.variant(GROUP3_A::GROUP3_0)
  }
  #[doc = "FB_TSIZ1"]
  #[inline(always)]
  pub fn group3_1(self) -> &'a mut W {
    self.variant(GROUP3_A::GROUP3_1)
  }
  #[doc = "no description available"]
  #[inline(always)]
  pub fn group3_2(self) -> &'a mut W {
    self.variant(GROUP3_A::GROUP3_2)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
    self.w
  }
}
#[doc = "FlexBus Signal Group 2 Multiplex control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GROUP2_A {
  #[doc = "0: no description available"]
  GROUP2_0,
  #[doc = "1: FB_TSIZ0"]
  GROUP2_1,
  #[doc = "2: no description available"]
  GROUP2_2,
}
impl From<GROUP2_A> for u8 {
  #[inline(always)]
  fn from(variant: GROUP2_A) -> Self {
    match variant {
      GROUP2_A::GROUP2_0 => 0,
      GROUP2_A::GROUP2_1 => 1,
      GROUP2_A::GROUP2_2 => 2,
    }
  }
}
#[doc = "Reader of field `GROUP2`"]
pub type GROUP2_R = crate::R<u8, GROUP2_A>;
impl GROUP2_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> crate::Variant<u8, GROUP2_A> {
    use crate::Variant::*;
    match self.bits {
      0 => Val(GROUP2_A::GROUP2_0),
      1 => Val(GROUP2_A::GROUP2_1),
      2 => Val(GROUP2_A::GROUP2_2),
      i => Res(i),
    }
  }
  #[doc = "Checks if the value of the field is `GROUP2_0`"]
  #[inline(always)]
  pub fn is_group2_0(&self) -> bool {
    *self == GROUP2_A::GROUP2_0
  }
  #[doc = "Checks if the value of the field is `GROUP2_1`"]
  #[inline(always)]
  pub fn is_group2_1(&self) -> bool {
    *self == GROUP2_A::GROUP2_1
  }
  #[doc = "Checks if the value of the field is `GROUP2_2`"]
  #[inline(always)]
  pub fn is_group2_2(&self) -> bool {
    *self == GROUP2_A::GROUP2_2
  }
}
#[doc = "Write proxy for field `GROUP2`"]
pub struct GROUP2_W<'a> {
  w: &'a mut W,
}
impl<'a> GROUP2_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: GROUP2_A) -> &'a mut W {
    unsafe { self.bits(variant.into()) }
  }
  #[doc = "no description available"]
  #[inline(always)]
  pub fn group2_0(self) -> &'a mut W {
    self.variant(GROUP2_A::GROUP2_0)
  }
  #[doc = "FB_TSIZ0"]
  #[inline(always)]
  pub fn group2_1(self) -> &'a mut W {
    self.variant(GROUP2_A::GROUP2_1)
  }
  #[doc = "no description available"]
  #[inline(always)]
  pub fn group2_2(self) -> &'a mut W {
    self.variant(GROUP2_A::GROUP2_2)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
    self.w
  }
}
#[doc = "FlexBus Signal Group 1 Multiplex control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GROUP1_A {
  #[doc = "0: FB_ALE"]
  GROUP1_0,
  #[doc = "1: no description available"]
  GROUP1_1,
  #[doc = "2: no description available"]
  GROUP1_2,
}
impl From<GROUP1_A> for u8 {
  #[inline(always)]
  fn from(variant: GROUP1_A) -> Self {
    match variant {
      GROUP1_A::GROUP1_0 => 0,
      GROUP1_A::GROUP1_1 => 1,
      GROUP1_A::GROUP1_2 => 2,
    }
  }
}
#[doc = "Reader of field `GROUP1`"]
pub type GROUP1_R = crate::R<u8, GROUP1_A>;
impl GROUP1_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> crate::Variant<u8, GROUP1_A> {
    use crate::Variant::*;
    match self.bits {
      0 => Val(GROUP1_A::GROUP1_0),
      1 => Val(GROUP1_A::GROUP1_1),
      2 => Val(GROUP1_A::GROUP1_2),
      i => Res(i),
    }
  }
  #[doc = "Checks if the value of the field is `GROUP1_0`"]
  #[inline(always)]
  pub fn is_group1_0(&self) -> bool {
    *self == GROUP1_A::GROUP1_0
  }
  #[doc = "Checks if the value of the field is `GROUP1_1`"]
  #[inline(always)]
  pub fn is_group1_1(&self) -> bool {
    *self == GROUP1_A::GROUP1_1
  }
  #[doc = "Checks if the value of the field is `GROUP1_2`"]
  #[inline(always)]
  pub fn is_group1_2(&self) -> bool {
    *self == GROUP1_A::GROUP1_2
  }
}
#[doc = "Write proxy for field `GROUP1`"]
pub struct GROUP1_W<'a> {
  w: &'a mut W,
}
impl<'a> GROUP1_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: GROUP1_A) -> &'a mut W {
    unsafe { self.bits(variant.into()) }
  }
  #[doc = "FB_ALE"]
  #[inline(always)]
  pub fn group1_0(self) -> &'a mut W {
    self.variant(GROUP1_A::GROUP1_0)
  }
  #[doc = "no description available"]
  #[inline(always)]
  pub fn group1_1(self) -> &'a mut W {
    self.variant(GROUP1_A::GROUP1_1)
  }
  #[doc = "no description available"]
  #[inline(always)]
  pub fn group1_2(self) -> &'a mut W {
    self.variant(GROUP1_A::GROUP1_2)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
    self.w
  }
}
impl R {
  #[doc = "Bits 12:15 - FlexBus Signal Group 5 Multiplex control"]
  #[inline(always)]
  pub fn group5(&self) -> GROUP5_R {
    GROUP5_R::new(((self.bits >> 12) & 0x0f) as u8)
  }
  #[doc = "Bits 16:19 - FlexBus Signal Group 4 Multiplex control"]
  #[inline(always)]
  pub fn group4(&self) -> GROUP4_R {
    GROUP4_R::new(((self.bits >> 16) & 0x0f) as u8)
  }
  #[doc = "Bits 20:23 - FlexBus Signal Group 3 Multiplex control"]
  #[inline(always)]
  pub fn group3(&self) -> GROUP3_R {
    GROUP3_R::new(((self.bits >> 20) & 0x0f) as u8)
  }
  #[doc = "Bits 24:27 - FlexBus Signal Group 2 Multiplex control"]
  #[inline(always)]
  pub fn group2(&self) -> GROUP2_R {
    GROUP2_R::new(((self.bits >> 24) & 0x0f) as u8)
  }
  #[doc = "Bits 28:31 - FlexBus Signal Group 1 Multiplex control"]
  #[inline(always)]
  pub fn group1(&self) -> GROUP1_R {
    GROUP1_R::new(((self.bits >> 28) & 0x0f) as u8)
  }
}
impl W {
  #[doc = "Bits 12:15 - FlexBus Signal Group 5 Multiplex control"]
  #[inline(always)]
  pub fn group5(&mut self) -> GROUP5_W {
    GROUP5_W { w: self }
  }
  #[doc = "Bits 16:19 - FlexBus Signal Group 4 Multiplex control"]
  #[inline(always)]
  pub fn group4(&mut self) -> GROUP4_W {
    GROUP4_W { w: self }
  }
  #[doc = "Bits 20:23 - FlexBus Signal Group 3 Multiplex control"]
  #[inline(always)]
  pub fn group3(&mut self) -> GROUP3_W {
    GROUP3_W { w: self }
  }
  #[doc = "Bits 24:27 - FlexBus Signal Group 2 Multiplex control"]
  #[inline(always)]
  pub fn group2(&mut self) -> GROUP2_W {
    GROUP2_W { w: self }
  }
  #[doc = "Bits 28:31 - FlexBus Signal Group 1 Multiplex control"]
  #[inline(always)]
  pub fn group1(&mut self) -> GROUP1_W {
    GROUP1_W { w: self }
  }
}
