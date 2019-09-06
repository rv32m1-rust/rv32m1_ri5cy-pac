#[doc = "Reader of register PRS3"]
pub type R = crate::R<u32, super::PRS3>;
#[doc = "Writer for register PRS3"]
pub type W = crate::W<u32, super::PRS3>;
#[doc = "Register PRS3 `reset()`'s with value 0x0050_1234"]
impl crate::ResetValue for super::PRS3 {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0x0050_1234
  }
}
#[doc = "Master 0 Priority. Sets the arbitration priority for this port on the associated slave port.\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum M0_A {
  #[doc = "0: This master has level 1, or highest, priority when accessing the slave port."]
  M0_0,
  #[doc = "1: This master has level 2 priority when accessing the slave port."]
  M0_1,
  #[doc = "2: This master has level 3 priority when accessing the slave port."]
  M0_2,
  #[doc = "3: This master has level 4 priority when accessing the slave port."]
  M0_3,
  #[doc = "4: This master has level 5 priority when accessing the slave port."]
  M0_4,
  #[doc = "5: This master has level 6 priority when accessing the slave port."]
  M0_5,
  #[doc = "6: This master has level 7 priority when accessing the slave port."]
  M0_6,
  #[doc = "7: This master has level 8, or lowest, priority when accessing the slave port."]
  M0_7,
}
impl From<M0_A> for u8 {
  #[inline(always)]
  fn from(variant: M0_A) -> Self {
    match variant {
      M0_A::M0_0 => 0,
      M0_A::M0_1 => 1,
      M0_A::M0_2 => 2,
      M0_A::M0_3 => 3,
      M0_A::M0_4 => 4,
      M0_A::M0_5 => 5,
      M0_A::M0_6 => 6,
      M0_A::M0_7 => 7,
    }
  }
}
#[doc = "Reader of field `M0`"]
pub type M0_R = crate::R<u8, M0_A>;
impl M0_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> M0_A {
    match self.bits {
      0 => M0_A::M0_0,
      1 => M0_A::M0_1,
      2 => M0_A::M0_2,
      3 => M0_A::M0_3,
      4 => M0_A::M0_4,
      5 => M0_A::M0_5,
      6 => M0_A::M0_6,
      7 => M0_A::M0_7,
      _ => unreachable!(),
    }
  }
  #[doc = "Checks if the value of the field is `M0_0`"]
  #[inline(always)]
  pub fn is_m0_0(&self) -> bool {
    *self == M0_A::M0_0
  }
  #[doc = "Checks if the value of the field is `M0_1`"]
  #[inline(always)]
  pub fn is_m0_1(&self) -> bool {
    *self == M0_A::M0_1
  }
  #[doc = "Checks if the value of the field is `M0_2`"]
  #[inline(always)]
  pub fn is_m0_2(&self) -> bool {
    *self == M0_A::M0_2
  }
  #[doc = "Checks if the value of the field is `M0_3`"]
  #[inline(always)]
  pub fn is_m0_3(&self) -> bool {
    *self == M0_A::M0_3
  }
  #[doc = "Checks if the value of the field is `M0_4`"]
  #[inline(always)]
  pub fn is_m0_4(&self) -> bool {
    *self == M0_A::M0_4
  }
  #[doc = "Checks if the value of the field is `M0_5`"]
  #[inline(always)]
  pub fn is_m0_5(&self) -> bool {
    *self == M0_A::M0_5
  }
  #[doc = "Checks if the value of the field is `M0_6`"]
  #[inline(always)]
  pub fn is_m0_6(&self) -> bool {
    *self == M0_A::M0_6
  }
  #[doc = "Checks if the value of the field is `M0_7`"]
  #[inline(always)]
  pub fn is_m0_7(&self) -> bool {
    *self == M0_A::M0_7
  }
}
#[doc = "Write proxy for field `M0`"]
pub struct M0_W<'a> {
  w: &'a mut W,
}
impl<'a> M0_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: M0_A) -> &'a mut W {
    {
      self.bits(variant.into())
    }
  }
  #[doc = "This master has level 1, or highest, priority when accessing the slave port."]
  #[inline(always)]
  pub fn m0_0(self) -> &'a mut W {
    self.variant(M0_A::M0_0)
  }
  #[doc = "This master has level 2 priority when accessing the slave port."]
  #[inline(always)]
  pub fn m0_1(self) -> &'a mut W {
    self.variant(M0_A::M0_1)
  }
  #[doc = "This master has level 3 priority when accessing the slave port."]
  #[inline(always)]
  pub fn m0_2(self) -> &'a mut W {
    self.variant(M0_A::M0_2)
  }
  #[doc = "This master has level 4 priority when accessing the slave port."]
  #[inline(always)]
  pub fn m0_3(self) -> &'a mut W {
    self.variant(M0_A::M0_3)
  }
  #[doc = "This master has level 5 priority when accessing the slave port."]
  #[inline(always)]
  pub fn m0_4(self) -> &'a mut W {
    self.variant(M0_A::M0_4)
  }
  #[doc = "This master has level 6 priority when accessing the slave port."]
  #[inline(always)]
  pub fn m0_5(self) -> &'a mut W {
    self.variant(M0_A::M0_5)
  }
  #[doc = "This master has level 7 priority when accessing the slave port."]
  #[inline(always)]
  pub fn m0_6(self) -> &'a mut W {
    self.variant(M0_A::M0_6)
  }
  #[doc = "This master has level 8, or lowest, priority when accessing the slave port."]
  #[inline(always)]
  pub fn m0_7(self) -> &'a mut W {
    self.variant(M0_A::M0_7)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
    self.w
  }
}
#[doc = "Master 1 Priority. Sets the arbitration priority for this port on the associated slave port.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum M1_A {
  #[doc = "0: This master has level 1, or highest, priority when accessing the slave port."]
  M1_0,
  #[doc = "1: This master has level 2 priority when accessing the slave port."]
  M1_1,
  #[doc = "2: This master has level 3 priority when accessing the slave port."]
  M1_2,
  #[doc = "3: This master has level 4 priority when accessing the slave port."]
  M1_3,
  #[doc = "4: This master has level 5 priority when accessing the slave port."]
  M1_4,
  #[doc = "5: This master has level 6 priority when accessing the slave port."]
  M1_5,
  #[doc = "6: This master has level 7 priority when accessing the slave port."]
  M1_6,
  #[doc = "7: This master has level 8, or lowest, priority when accessing the slave port."]
  M1_7,
}
impl From<M1_A> for u8 {
  #[inline(always)]
  fn from(variant: M1_A) -> Self {
    match variant {
      M1_A::M1_0 => 0,
      M1_A::M1_1 => 1,
      M1_A::M1_2 => 2,
      M1_A::M1_3 => 3,
      M1_A::M1_4 => 4,
      M1_A::M1_5 => 5,
      M1_A::M1_6 => 6,
      M1_A::M1_7 => 7,
    }
  }
}
#[doc = "Reader of field `M1`"]
pub type M1_R = crate::R<u8, M1_A>;
impl M1_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> M1_A {
    match self.bits {
      0 => M1_A::M1_0,
      1 => M1_A::M1_1,
      2 => M1_A::M1_2,
      3 => M1_A::M1_3,
      4 => M1_A::M1_4,
      5 => M1_A::M1_5,
      6 => M1_A::M1_6,
      7 => M1_A::M1_7,
      _ => unreachable!(),
    }
  }
  #[doc = "Checks if the value of the field is `M1_0`"]
  #[inline(always)]
  pub fn is_m1_0(&self) -> bool {
    *self == M1_A::M1_0
  }
  #[doc = "Checks if the value of the field is `M1_1`"]
  #[inline(always)]
  pub fn is_m1_1(&self) -> bool {
    *self == M1_A::M1_1
  }
  #[doc = "Checks if the value of the field is `M1_2`"]
  #[inline(always)]
  pub fn is_m1_2(&self) -> bool {
    *self == M1_A::M1_2
  }
  #[doc = "Checks if the value of the field is `M1_3`"]
  #[inline(always)]
  pub fn is_m1_3(&self) -> bool {
    *self == M1_A::M1_3
  }
  #[doc = "Checks if the value of the field is `M1_4`"]
  #[inline(always)]
  pub fn is_m1_4(&self) -> bool {
    *self == M1_A::M1_4
  }
  #[doc = "Checks if the value of the field is `M1_5`"]
  #[inline(always)]
  pub fn is_m1_5(&self) -> bool {
    *self == M1_A::M1_5
  }
  #[doc = "Checks if the value of the field is `M1_6`"]
  #[inline(always)]
  pub fn is_m1_6(&self) -> bool {
    *self == M1_A::M1_6
  }
  #[doc = "Checks if the value of the field is `M1_7`"]
  #[inline(always)]
  pub fn is_m1_7(&self) -> bool {
    *self == M1_A::M1_7
  }
}
#[doc = "Write proxy for field `M1`"]
pub struct M1_W<'a> {
  w: &'a mut W,
}
impl<'a> M1_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: M1_A) -> &'a mut W {
    {
      self.bits(variant.into())
    }
  }
  #[doc = "This master has level 1, or highest, priority when accessing the slave port."]
  #[inline(always)]
  pub fn m1_0(self) -> &'a mut W {
    self.variant(M1_A::M1_0)
  }
  #[doc = "This master has level 2 priority when accessing the slave port."]
  #[inline(always)]
  pub fn m1_1(self) -> &'a mut W {
    self.variant(M1_A::M1_1)
  }
  #[doc = "This master has level 3 priority when accessing the slave port."]
  #[inline(always)]
  pub fn m1_2(self) -> &'a mut W {
    self.variant(M1_A::M1_2)
  }
  #[doc = "This master has level 4 priority when accessing the slave port."]
  #[inline(always)]
  pub fn m1_3(self) -> &'a mut W {
    self.variant(M1_A::M1_3)
  }
  #[doc = "This master has level 5 priority when accessing the slave port."]
  #[inline(always)]
  pub fn m1_4(self) -> &'a mut W {
    self.variant(M1_A::M1_4)
  }
  #[doc = "This master has level 6 priority when accessing the slave port."]
  #[inline(always)]
  pub fn m1_5(self) -> &'a mut W {
    self.variant(M1_A::M1_5)
  }
  #[doc = "This master has level 7 priority when accessing the slave port."]
  #[inline(always)]
  pub fn m1_6(self) -> &'a mut W {
    self.variant(M1_A::M1_6)
  }
  #[doc = "This master has level 8, or lowest, priority when accessing the slave port."]
  #[inline(always)]
  pub fn m1_7(self) -> &'a mut W {
    self.variant(M1_A::M1_7)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
    self.w
  }
}
#[doc = "Master 2 Priority. Sets the arbitration priority for this port on the associated slave port.\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum M2_A {
  #[doc = "0: This master has level 1, or highest, priority when accessing the slave port."]
  M2_0,
  #[doc = "1: This master has level 2 priority when accessing the slave port."]
  M2_1,
  #[doc = "2: This master has level 3 priority when accessing the slave port."]
  M2_2,
  #[doc = "3: This master has level 4 priority when accessing the slave port."]
  M2_3,
  #[doc = "4: This master has level 5 priority when accessing the slave port."]
  M2_4,
  #[doc = "5: This master has level 6 priority when accessing the slave port."]
  M2_5,
  #[doc = "6: This master has level 7 priority when accessing the slave port."]
  M2_6,
  #[doc = "7: This master has level 8, or lowest, priority when accessing the slave port."]
  M2_7,
}
impl From<M2_A> for u8 {
  #[inline(always)]
  fn from(variant: M2_A) -> Self {
    match variant {
      M2_A::M2_0 => 0,
      M2_A::M2_1 => 1,
      M2_A::M2_2 => 2,
      M2_A::M2_3 => 3,
      M2_A::M2_4 => 4,
      M2_A::M2_5 => 5,
      M2_A::M2_6 => 6,
      M2_A::M2_7 => 7,
    }
  }
}
#[doc = "Reader of field `M2`"]
pub type M2_R = crate::R<u8, M2_A>;
impl M2_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> M2_A {
    match self.bits {
      0 => M2_A::M2_0,
      1 => M2_A::M2_1,
      2 => M2_A::M2_2,
      3 => M2_A::M2_3,
      4 => M2_A::M2_4,
      5 => M2_A::M2_5,
      6 => M2_A::M2_6,
      7 => M2_A::M2_7,
      _ => unreachable!(),
    }
  }
  #[doc = "Checks if the value of the field is `M2_0`"]
  #[inline(always)]
  pub fn is_m2_0(&self) -> bool {
    *self == M2_A::M2_0
  }
  #[doc = "Checks if the value of the field is `M2_1`"]
  #[inline(always)]
  pub fn is_m2_1(&self) -> bool {
    *self == M2_A::M2_1
  }
  #[doc = "Checks if the value of the field is `M2_2`"]
  #[inline(always)]
  pub fn is_m2_2(&self) -> bool {
    *self == M2_A::M2_2
  }
  #[doc = "Checks if the value of the field is `M2_3`"]
  #[inline(always)]
  pub fn is_m2_3(&self) -> bool {
    *self == M2_A::M2_3
  }
  #[doc = "Checks if the value of the field is `M2_4`"]
  #[inline(always)]
  pub fn is_m2_4(&self) -> bool {
    *self == M2_A::M2_4
  }
  #[doc = "Checks if the value of the field is `M2_5`"]
  #[inline(always)]
  pub fn is_m2_5(&self) -> bool {
    *self == M2_A::M2_5
  }
  #[doc = "Checks if the value of the field is `M2_6`"]
  #[inline(always)]
  pub fn is_m2_6(&self) -> bool {
    *self == M2_A::M2_6
  }
  #[doc = "Checks if the value of the field is `M2_7`"]
  #[inline(always)]
  pub fn is_m2_7(&self) -> bool {
    *self == M2_A::M2_7
  }
}
#[doc = "Write proxy for field `M2`"]
pub struct M2_W<'a> {
  w: &'a mut W,
}
impl<'a> M2_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: M2_A) -> &'a mut W {
    {
      self.bits(variant.into())
    }
  }
  #[doc = "This master has level 1, or highest, priority when accessing the slave port."]
  #[inline(always)]
  pub fn m2_0(self) -> &'a mut W {
    self.variant(M2_A::M2_0)
  }
  #[doc = "This master has level 2 priority when accessing the slave port."]
  #[inline(always)]
  pub fn m2_1(self) -> &'a mut W {
    self.variant(M2_A::M2_1)
  }
  #[doc = "This master has level 3 priority when accessing the slave port."]
  #[inline(always)]
  pub fn m2_2(self) -> &'a mut W {
    self.variant(M2_A::M2_2)
  }
  #[doc = "This master has level 4 priority when accessing the slave port."]
  #[inline(always)]
  pub fn m2_3(self) -> &'a mut W {
    self.variant(M2_A::M2_3)
  }
  #[doc = "This master has level 5 priority when accessing the slave port."]
  #[inline(always)]
  pub fn m2_4(self) -> &'a mut W {
    self.variant(M2_A::M2_4)
  }
  #[doc = "This master has level 6 priority when accessing the slave port."]
  #[inline(always)]
  pub fn m2_5(self) -> &'a mut W {
    self.variant(M2_A::M2_5)
  }
  #[doc = "This master has level 7 priority when accessing the slave port."]
  #[inline(always)]
  pub fn m2_6(self) -> &'a mut W {
    self.variant(M2_A::M2_6)
  }
  #[doc = "This master has level 8, or lowest, priority when accessing the slave port."]
  #[inline(always)]
  pub fn m2_7(self) -> &'a mut W {
    self.variant(M2_A::M2_7)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
    self.w
  }
}
#[doc = "Master 3 Priority. Sets the arbitration priority for this port on the associated slave port.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum M3_A {
  #[doc = "0: This master has level 1, or highest, priority when accessing the slave port."]
  M3_0,
  #[doc = "1: This master has level 2 priority when accessing the slave port."]
  M3_1,
  #[doc = "2: This master has level 3 priority when accessing the slave port."]
  M3_2,
  #[doc = "3: This master has level 4 priority when accessing the slave port."]
  M3_3,
  #[doc = "4: This master has level 5 priority when accessing the slave port."]
  M3_4,
  #[doc = "5: This master has level 6 priority when accessing the slave port."]
  M3_5,
  #[doc = "6: This master has level 7 priority when accessing the slave port."]
  M3_6,
  #[doc = "7: This master has level 8, or lowest, priority when accessing the slave port."]
  M3_7,
}
impl From<M3_A> for u8 {
  #[inline(always)]
  fn from(variant: M3_A) -> Self {
    match variant {
      M3_A::M3_0 => 0,
      M3_A::M3_1 => 1,
      M3_A::M3_2 => 2,
      M3_A::M3_3 => 3,
      M3_A::M3_4 => 4,
      M3_A::M3_5 => 5,
      M3_A::M3_6 => 6,
      M3_A::M3_7 => 7,
    }
  }
}
#[doc = "Reader of field `M3`"]
pub type M3_R = crate::R<u8, M3_A>;
impl M3_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> M3_A {
    match self.bits {
      0 => M3_A::M3_0,
      1 => M3_A::M3_1,
      2 => M3_A::M3_2,
      3 => M3_A::M3_3,
      4 => M3_A::M3_4,
      5 => M3_A::M3_5,
      6 => M3_A::M3_6,
      7 => M3_A::M3_7,
      _ => unreachable!(),
    }
  }
  #[doc = "Checks if the value of the field is `M3_0`"]
  #[inline(always)]
  pub fn is_m3_0(&self) -> bool {
    *self == M3_A::M3_0
  }
  #[doc = "Checks if the value of the field is `M3_1`"]
  #[inline(always)]
  pub fn is_m3_1(&self) -> bool {
    *self == M3_A::M3_1
  }
  #[doc = "Checks if the value of the field is `M3_2`"]
  #[inline(always)]
  pub fn is_m3_2(&self) -> bool {
    *self == M3_A::M3_2
  }
  #[doc = "Checks if the value of the field is `M3_3`"]
  #[inline(always)]
  pub fn is_m3_3(&self) -> bool {
    *self == M3_A::M3_3
  }
  #[doc = "Checks if the value of the field is `M3_4`"]
  #[inline(always)]
  pub fn is_m3_4(&self) -> bool {
    *self == M3_A::M3_4
  }
  #[doc = "Checks if the value of the field is `M3_5`"]
  #[inline(always)]
  pub fn is_m3_5(&self) -> bool {
    *self == M3_A::M3_5
  }
  #[doc = "Checks if the value of the field is `M3_6`"]
  #[inline(always)]
  pub fn is_m3_6(&self) -> bool {
    *self == M3_A::M3_6
  }
  #[doc = "Checks if the value of the field is `M3_7`"]
  #[inline(always)]
  pub fn is_m3_7(&self) -> bool {
    *self == M3_A::M3_7
  }
}
#[doc = "Write proxy for field `M3`"]
pub struct M3_W<'a> {
  w: &'a mut W,
}
impl<'a> M3_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: M3_A) -> &'a mut W {
    {
      self.bits(variant.into())
    }
  }
  #[doc = "This master has level 1, or highest, priority when accessing the slave port."]
  #[inline(always)]
  pub fn m3_0(self) -> &'a mut W {
    self.variant(M3_A::M3_0)
  }
  #[doc = "This master has level 2 priority when accessing the slave port."]
  #[inline(always)]
  pub fn m3_1(self) -> &'a mut W {
    self.variant(M3_A::M3_1)
  }
  #[doc = "This master has level 3 priority when accessing the slave port."]
  #[inline(always)]
  pub fn m3_2(self) -> &'a mut W {
    self.variant(M3_A::M3_2)
  }
  #[doc = "This master has level 4 priority when accessing the slave port."]
  #[inline(always)]
  pub fn m3_3(self) -> &'a mut W {
    self.variant(M3_A::M3_3)
  }
  #[doc = "This master has level 5 priority when accessing the slave port."]
  #[inline(always)]
  pub fn m3_4(self) -> &'a mut W {
    self.variant(M3_A::M3_4)
  }
  #[doc = "This master has level 6 priority when accessing the slave port."]
  #[inline(always)]
  pub fn m3_5(self) -> &'a mut W {
    self.variant(M3_A::M3_5)
  }
  #[doc = "This master has level 7 priority when accessing the slave port."]
  #[inline(always)]
  pub fn m3_6(self) -> &'a mut W {
    self.variant(M3_A::M3_6)
  }
  #[doc = "This master has level 8, or lowest, priority when accessing the slave port."]
  #[inline(always)]
  pub fn m3_7(self) -> &'a mut W {
    self.variant(M3_A::M3_7)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u32) & 0x07) << 12);
    self.w
  }
}
#[doc = "Master 4 Priority. Sets the arbitration priority for this port on the associated slave port.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum M4_A {
  #[doc = "0: This master has level 1, or highest, priority when accessing the slave port."]
  M4_0,
  #[doc = "1: This master has level 2 priority when accessing the slave port."]
  M4_1,
  #[doc = "2: This master has level 3 priority when accessing the slave port."]
  M4_2,
  #[doc = "3: This master has level 4 priority when accessing the slave port."]
  M4_3,
  #[doc = "4: This master has level 5 priority when accessing the slave port."]
  M4_4,
  #[doc = "5: This master has level 6 priority when accessing the slave port."]
  M4_5,
  #[doc = "6: This master has level 7 priority when accessing the slave port."]
  M4_6,
  #[doc = "7: This master has level 8, or lowest, priority when accessing the slave port."]
  M4_7,
}
impl From<M4_A> for u8 {
  #[inline(always)]
  fn from(variant: M4_A) -> Self {
    match variant {
      M4_A::M4_0 => 0,
      M4_A::M4_1 => 1,
      M4_A::M4_2 => 2,
      M4_A::M4_3 => 3,
      M4_A::M4_4 => 4,
      M4_A::M4_5 => 5,
      M4_A::M4_6 => 6,
      M4_A::M4_7 => 7,
    }
  }
}
#[doc = "Reader of field `M4`"]
pub type M4_R = crate::R<u8, M4_A>;
impl M4_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> M4_A {
    match self.bits {
      0 => M4_A::M4_0,
      1 => M4_A::M4_1,
      2 => M4_A::M4_2,
      3 => M4_A::M4_3,
      4 => M4_A::M4_4,
      5 => M4_A::M4_5,
      6 => M4_A::M4_6,
      7 => M4_A::M4_7,
      _ => unreachable!(),
    }
  }
  #[doc = "Checks if the value of the field is `M4_0`"]
  #[inline(always)]
  pub fn is_m4_0(&self) -> bool {
    *self == M4_A::M4_0
  }
  #[doc = "Checks if the value of the field is `M4_1`"]
  #[inline(always)]
  pub fn is_m4_1(&self) -> bool {
    *self == M4_A::M4_1
  }
  #[doc = "Checks if the value of the field is `M4_2`"]
  #[inline(always)]
  pub fn is_m4_2(&self) -> bool {
    *self == M4_A::M4_2
  }
  #[doc = "Checks if the value of the field is `M4_3`"]
  #[inline(always)]
  pub fn is_m4_3(&self) -> bool {
    *self == M4_A::M4_3
  }
  #[doc = "Checks if the value of the field is `M4_4`"]
  #[inline(always)]
  pub fn is_m4_4(&self) -> bool {
    *self == M4_A::M4_4
  }
  #[doc = "Checks if the value of the field is `M4_5`"]
  #[inline(always)]
  pub fn is_m4_5(&self) -> bool {
    *self == M4_A::M4_5
  }
  #[doc = "Checks if the value of the field is `M4_6`"]
  #[inline(always)]
  pub fn is_m4_6(&self) -> bool {
    *self == M4_A::M4_6
  }
  #[doc = "Checks if the value of the field is `M4_7`"]
  #[inline(always)]
  pub fn is_m4_7(&self) -> bool {
    *self == M4_A::M4_7
  }
}
#[doc = "Write proxy for field `M4`"]
pub struct M4_W<'a> {
  w: &'a mut W,
}
impl<'a> M4_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: M4_A) -> &'a mut W {
    {
      self.bits(variant.into())
    }
  }
  #[doc = "This master has level 1, or highest, priority when accessing the slave port."]
  #[inline(always)]
  pub fn m4_0(self) -> &'a mut W {
    self.variant(M4_A::M4_0)
  }
  #[doc = "This master has level 2 priority when accessing the slave port."]
  #[inline(always)]
  pub fn m4_1(self) -> &'a mut W {
    self.variant(M4_A::M4_1)
  }
  #[doc = "This master has level 3 priority when accessing the slave port."]
  #[inline(always)]
  pub fn m4_2(self) -> &'a mut W {
    self.variant(M4_A::M4_2)
  }
  #[doc = "This master has level 4 priority when accessing the slave port."]
  #[inline(always)]
  pub fn m4_3(self) -> &'a mut W {
    self.variant(M4_A::M4_3)
  }
  #[doc = "This master has level 5 priority when accessing the slave port."]
  #[inline(always)]
  pub fn m4_4(self) -> &'a mut W {
    self.variant(M4_A::M4_4)
  }
  #[doc = "This master has level 6 priority when accessing the slave port."]
  #[inline(always)]
  pub fn m4_5(self) -> &'a mut W {
    self.variant(M4_A::M4_5)
  }
  #[doc = "This master has level 7 priority when accessing the slave port."]
  #[inline(always)]
  pub fn m4_6(self) -> &'a mut W {
    self.variant(M4_A::M4_6)
  }
  #[doc = "This master has level 8, or lowest, priority when accessing the slave port."]
  #[inline(always)]
  pub fn m4_7(self) -> &'a mut W {
    self.variant(M4_A::M4_7)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
    self.w
  }
}
#[doc = "Master 5 Priority. Sets the arbitration priority for this port on the associated slave port.\n\nValue on reset: 5"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum M5_A {
  #[doc = "0: This master has level 1, or highest, priority when accessing the slave port."]
  M5_0,
  #[doc = "1: This master has level 2 priority when accessing the slave port."]
  M5_1,
  #[doc = "2: This master has level 3 priority when accessing the slave port."]
  M5_2,
  #[doc = "3: This master has level 4 priority when accessing the slave port."]
  M5_3,
  #[doc = "4: This master has level 5 priority when accessing the slave port."]
  M5_4,
  #[doc = "5: This master has level 6 priority when accessing the slave port."]
  M5_5,
  #[doc = "6: This master has level 7 priority when accessing the slave port."]
  M5_6,
  #[doc = "7: This master has level 8, or lowest, priority when accessing the slave port."]
  M5_7,
}
impl From<M5_A> for u8 {
  #[inline(always)]
  fn from(variant: M5_A) -> Self {
    match variant {
      M5_A::M5_0 => 0,
      M5_A::M5_1 => 1,
      M5_A::M5_2 => 2,
      M5_A::M5_3 => 3,
      M5_A::M5_4 => 4,
      M5_A::M5_5 => 5,
      M5_A::M5_6 => 6,
      M5_A::M5_7 => 7,
    }
  }
}
#[doc = "Reader of field `M5`"]
pub type M5_R = crate::R<u8, M5_A>;
impl M5_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> M5_A {
    match self.bits {
      0 => M5_A::M5_0,
      1 => M5_A::M5_1,
      2 => M5_A::M5_2,
      3 => M5_A::M5_3,
      4 => M5_A::M5_4,
      5 => M5_A::M5_5,
      6 => M5_A::M5_6,
      7 => M5_A::M5_7,
      _ => unreachable!(),
    }
  }
  #[doc = "Checks if the value of the field is `M5_0`"]
  #[inline(always)]
  pub fn is_m5_0(&self) -> bool {
    *self == M5_A::M5_0
  }
  #[doc = "Checks if the value of the field is `M5_1`"]
  #[inline(always)]
  pub fn is_m5_1(&self) -> bool {
    *self == M5_A::M5_1
  }
  #[doc = "Checks if the value of the field is `M5_2`"]
  #[inline(always)]
  pub fn is_m5_2(&self) -> bool {
    *self == M5_A::M5_2
  }
  #[doc = "Checks if the value of the field is `M5_3`"]
  #[inline(always)]
  pub fn is_m5_3(&self) -> bool {
    *self == M5_A::M5_3
  }
  #[doc = "Checks if the value of the field is `M5_4`"]
  #[inline(always)]
  pub fn is_m5_4(&self) -> bool {
    *self == M5_A::M5_4
  }
  #[doc = "Checks if the value of the field is `M5_5`"]
  #[inline(always)]
  pub fn is_m5_5(&self) -> bool {
    *self == M5_A::M5_5
  }
  #[doc = "Checks if the value of the field is `M5_6`"]
  #[inline(always)]
  pub fn is_m5_6(&self) -> bool {
    *self == M5_A::M5_6
  }
  #[doc = "Checks if the value of the field is `M5_7`"]
  #[inline(always)]
  pub fn is_m5_7(&self) -> bool {
    *self == M5_A::M5_7
  }
}
#[doc = "Write proxy for field `M5`"]
pub struct M5_W<'a> {
  w: &'a mut W,
}
impl<'a> M5_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: M5_A) -> &'a mut W {
    {
      self.bits(variant.into())
    }
  }
  #[doc = "This master has level 1, or highest, priority when accessing the slave port."]
  #[inline(always)]
  pub fn m5_0(self) -> &'a mut W {
    self.variant(M5_A::M5_0)
  }
  #[doc = "This master has level 2 priority when accessing the slave port."]
  #[inline(always)]
  pub fn m5_1(self) -> &'a mut W {
    self.variant(M5_A::M5_1)
  }
  #[doc = "This master has level 3 priority when accessing the slave port."]
  #[inline(always)]
  pub fn m5_2(self) -> &'a mut W {
    self.variant(M5_A::M5_2)
  }
  #[doc = "This master has level 4 priority when accessing the slave port."]
  #[inline(always)]
  pub fn m5_3(self) -> &'a mut W {
    self.variant(M5_A::M5_3)
  }
  #[doc = "This master has level 5 priority when accessing the slave port."]
  #[inline(always)]
  pub fn m5_4(self) -> &'a mut W {
    self.variant(M5_A::M5_4)
  }
  #[doc = "This master has level 6 priority when accessing the slave port."]
  #[inline(always)]
  pub fn m5_5(self) -> &'a mut W {
    self.variant(M5_A::M5_5)
  }
  #[doc = "This master has level 7 priority when accessing the slave port."]
  #[inline(always)]
  pub fn m5_6(self) -> &'a mut W {
    self.variant(M5_A::M5_6)
  }
  #[doc = "This master has level 8, or lowest, priority when accessing the slave port."]
  #[inline(always)]
  pub fn m5_7(self) -> &'a mut W {
    self.variant(M5_A::M5_7)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x07 << 20)) | (((value as u32) & 0x07) << 20);
    self.w
  }
}
impl R {
  #[doc = "Bits 0:2 - Master 0 Priority. Sets the arbitration priority for this port on the associated slave port."]
  #[inline(always)]
  pub fn m0(&self) -> M0_R {
    M0_R::new((self.bits & 0x07) as u8)
  }
  #[doc = "Bits 4:6 - Master 1 Priority. Sets the arbitration priority for this port on the associated slave port."]
  #[inline(always)]
  pub fn m1(&self) -> M1_R {
    M1_R::new(((self.bits >> 4) & 0x07) as u8)
  }
  #[doc = "Bits 8:10 - Master 2 Priority. Sets the arbitration priority for this port on the associated slave port."]
  #[inline(always)]
  pub fn m2(&self) -> M2_R {
    M2_R::new(((self.bits >> 8) & 0x07) as u8)
  }
  #[doc = "Bits 12:14 - Master 3 Priority. Sets the arbitration priority for this port on the associated slave port."]
  #[inline(always)]
  pub fn m3(&self) -> M3_R {
    M3_R::new(((self.bits >> 12) & 0x07) as u8)
  }
  #[doc = "Bits 16:18 - Master 4 Priority. Sets the arbitration priority for this port on the associated slave port."]
  #[inline(always)]
  pub fn m4(&self) -> M4_R {
    M4_R::new(((self.bits >> 16) & 0x07) as u8)
  }
  #[doc = "Bits 20:22 - Master 5 Priority. Sets the arbitration priority for this port on the associated slave port."]
  #[inline(always)]
  pub fn m5(&self) -> M5_R {
    M5_R::new(((self.bits >> 20) & 0x07) as u8)
  }
}
impl W {
  #[doc = "Bits 0:2 - Master 0 Priority. Sets the arbitration priority for this port on the associated slave port."]
  #[inline(always)]
  pub fn m0(&mut self) -> M0_W {
    M0_W { w: self }
  }
  #[doc = "Bits 4:6 - Master 1 Priority. Sets the arbitration priority for this port on the associated slave port."]
  #[inline(always)]
  pub fn m1(&mut self) -> M1_W {
    M1_W { w: self }
  }
  #[doc = "Bits 8:10 - Master 2 Priority. Sets the arbitration priority for this port on the associated slave port."]
  #[inline(always)]
  pub fn m2(&mut self) -> M2_W {
    M2_W { w: self }
  }
  #[doc = "Bits 12:14 - Master 3 Priority. Sets the arbitration priority for this port on the associated slave port."]
  #[inline(always)]
  pub fn m3(&mut self) -> M3_W {
    M3_W { w: self }
  }
  #[doc = "Bits 16:18 - Master 4 Priority. Sets the arbitration priority for this port on the associated slave port."]
  #[inline(always)]
  pub fn m4(&mut self) -> M4_W {
    M4_W { w: self }
  }
  #[doc = "Bits 20:22 - Master 5 Priority. Sets the arbitration priority for this port on the associated slave port."]
  #[inline(always)]
  pub fn m5(&mut self) -> M5_W {
    M5_W { w: self }
  }
}
