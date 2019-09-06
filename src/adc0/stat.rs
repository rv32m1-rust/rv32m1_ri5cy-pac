#[doc = "Reader of register STAT"]
pub type R = crate::R<u32, super::STAT>;
#[doc = "Writer for register STAT"]
pub type W = crate::W<u32, super::STAT>;
#[doc = "Register STAT `reset()`'s with value 0"]
impl crate::ResetValue for super::STAT {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "Result FIFO Ready Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDY_A {
  #[doc = "0: Result FIFO data level not above watermark level."]
  RDY_0,
  #[doc = "1: Result FIFO holding data above watermark level."]
  RDY_1,
}
impl From<RDY_A> for bool {
  #[inline(always)]
  fn from(variant: RDY_A) -> Self {
    match variant {
      RDY_A::RDY_0 => false,
      RDY_A::RDY_1 => true,
    }
  }
}
#[doc = "Reader of field `RDY`"]
pub type RDY_R = crate::R<bool, RDY_A>;
impl RDY_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> RDY_A {
    match self.bits {
      false => RDY_A::RDY_0,
      true => RDY_A::RDY_1,
    }
  }
  #[doc = "Checks if the value of the field is `RDY_0`"]
  #[inline(always)]
  pub fn is_rdy_0(&self) -> bool {
    *self == RDY_A::RDY_0
  }
  #[doc = "Checks if the value of the field is `RDY_1`"]
  #[inline(always)]
  pub fn is_rdy_1(&self) -> bool {
    *self == RDY_A::RDY_1
  }
}
#[doc = "Result FIFO Overflow Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FOF_A {
  #[doc = "0: No result FIFO overflow has occurred since the last time the flag was cleared."]
  FOF_0,
  #[doc = "1: At least one result FIFO overflow has occurred since the last time the flag was cleared."]
  FOF_1,
}
impl From<FOF_A> for bool {
  #[inline(always)]
  fn from(variant: FOF_A) -> Self {
    match variant {
      FOF_A::FOF_0 => false,
      FOF_A::FOF_1 => true,
    }
  }
}
#[doc = "Reader of field `FOF`"]
pub type FOF_R = crate::R<bool, FOF_A>;
impl FOF_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> FOF_A {
    match self.bits {
      false => FOF_A::FOF_0,
      true => FOF_A::FOF_1,
    }
  }
  #[doc = "Checks if the value of the field is `FOF_0`"]
  #[inline(always)]
  pub fn is_fof_0(&self) -> bool {
    *self == FOF_A::FOF_0
  }
  #[doc = "Checks if the value of the field is `FOF_1`"]
  #[inline(always)]
  pub fn is_fof_1(&self) -> bool {
    *self == FOF_A::FOF_1
  }
}
#[doc = "Write proxy for field `FOF`"]
pub struct FOF_W<'a> {
  w: &'a mut W,
}
impl<'a> FOF_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: FOF_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "No result FIFO overflow has occurred since the last time the flag was cleared."]
  #[inline(always)]
  pub fn fof_0(self) -> &'a mut W {
    self.variant(FOF_A::FOF_0)
  }
  #[doc = "At least one result FIFO overflow has occurred since the last time the flag was cleared."]
  #[inline(always)]
  pub fn fof_1(self) -> &'a mut W {
    self.variant(FOF_A::FOF_1)
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
#[doc = "Trigger Active\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRGACT_A {
  #[doc = "0: Command (sequence) associated with Trigger 0 currently being executed."]
  TRGACT_0,
  #[doc = "1: Command (sequence) associated with Trigger 1 currently being executed."]
  TRGACT_1,
  #[doc = "2: Command (sequence) associated with Trigger 2 currently being executed."]
  TRGACT_2,
  #[doc = "3: Command (sequence) associated with Trigger 3 currently being executed."]
  TRGACT_3,
}
impl From<TRGACT_A> for u8 {
  #[inline(always)]
  fn from(variant: TRGACT_A) -> Self {
    match variant {
      TRGACT_A::TRGACT_0 => 0,
      TRGACT_A::TRGACT_1 => 1,
      TRGACT_A::TRGACT_2 => 2,
      TRGACT_A::TRGACT_3 => 3,
    }
  }
}
#[doc = "Reader of field `TRGACT`"]
pub type TRGACT_R = crate::R<u8, TRGACT_A>;
impl TRGACT_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> TRGACT_A {
    match self.bits {
      0 => TRGACT_A::TRGACT_0,
      1 => TRGACT_A::TRGACT_1,
      2 => TRGACT_A::TRGACT_2,
      3 => TRGACT_A::TRGACT_3,
      _ => unreachable!(),
    }
  }
  #[doc = "Checks if the value of the field is `TRGACT_0`"]
  #[inline(always)]
  pub fn is_trgact_0(&self) -> bool {
    *self == TRGACT_A::TRGACT_0
  }
  #[doc = "Checks if the value of the field is `TRGACT_1`"]
  #[inline(always)]
  pub fn is_trgact_1(&self) -> bool {
    *self == TRGACT_A::TRGACT_1
  }
  #[doc = "Checks if the value of the field is `TRGACT_2`"]
  #[inline(always)]
  pub fn is_trgact_2(&self) -> bool {
    *self == TRGACT_A::TRGACT_2
  }
  #[doc = "Checks if the value of the field is `TRGACT_3`"]
  #[inline(always)]
  pub fn is_trgact_3(&self) -> bool {
    *self == TRGACT_A::TRGACT_3
  }
}
#[doc = "Command Active\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMDACT_A {
  #[doc = "0: No command is currently in progress."]
  CMDACT_0,
  #[doc = "1: Command 1 currently being executed."]
  CMDACT_1,
  #[doc = "2: Command 2 currently being executed."]
  CMDACT_2,
  #[doc = "3: Associated command number is currently being executed."]
  CMDACT_3,
  #[doc = "4: Associated command number is currently being executed."]
  CMDACT_4,
  #[doc = "5: Associated command number is currently being executed."]
  CMDACT_5,
  #[doc = "6: Associated command number is currently being executed."]
  CMDACT_6,
  #[doc = "7: Associated command number is currently being executed."]
  CMDACT_7,
  #[doc = "8: Associated command number is currently being executed."]
  CMDACT_8,
  #[doc = "9: Associated command number is currently being executed."]
  CMDACT_9,
}
impl From<CMDACT_A> for u8 {
  #[inline(always)]
  fn from(variant: CMDACT_A) -> Self {
    match variant {
      CMDACT_A::CMDACT_0 => 0,
      CMDACT_A::CMDACT_1 => 1,
      CMDACT_A::CMDACT_2 => 2,
      CMDACT_A::CMDACT_3 => 3,
      CMDACT_A::CMDACT_4 => 4,
      CMDACT_A::CMDACT_5 => 5,
      CMDACT_A::CMDACT_6 => 6,
      CMDACT_A::CMDACT_7 => 7,
      CMDACT_A::CMDACT_8 => 8,
      CMDACT_A::CMDACT_9 => 9,
    }
  }
}
#[doc = "Reader of field `CMDACT`"]
pub type CMDACT_R = crate::R<u8, CMDACT_A>;
impl CMDACT_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> crate::Variant<u8, CMDACT_A> {
    use crate::Variant::*;
    match self.bits {
      0 => Val(CMDACT_A::CMDACT_0),
      1 => Val(CMDACT_A::CMDACT_1),
      2 => Val(CMDACT_A::CMDACT_2),
      3 => Val(CMDACT_A::CMDACT_3),
      4 => Val(CMDACT_A::CMDACT_4),
      5 => Val(CMDACT_A::CMDACT_5),
      6 => Val(CMDACT_A::CMDACT_6),
      7 => Val(CMDACT_A::CMDACT_7),
      8 => Val(CMDACT_A::CMDACT_8),
      9 => Val(CMDACT_A::CMDACT_9),
      i => Res(i),
    }
  }
  #[doc = "Checks if the value of the field is `CMDACT_0`"]
  #[inline(always)]
  pub fn is_cmdact_0(&self) -> bool {
    *self == CMDACT_A::CMDACT_0
  }
  #[doc = "Checks if the value of the field is `CMDACT_1`"]
  #[inline(always)]
  pub fn is_cmdact_1(&self) -> bool {
    *self == CMDACT_A::CMDACT_1
  }
  #[doc = "Checks if the value of the field is `CMDACT_2`"]
  #[inline(always)]
  pub fn is_cmdact_2(&self) -> bool {
    *self == CMDACT_A::CMDACT_2
  }
  #[doc = "Checks if the value of the field is `CMDACT_3`"]
  #[inline(always)]
  pub fn is_cmdact_3(&self) -> bool {
    *self == CMDACT_A::CMDACT_3
  }
  #[doc = "Checks if the value of the field is `CMDACT_4`"]
  #[inline(always)]
  pub fn is_cmdact_4(&self) -> bool {
    *self == CMDACT_A::CMDACT_4
  }
  #[doc = "Checks if the value of the field is `CMDACT_5`"]
  #[inline(always)]
  pub fn is_cmdact_5(&self) -> bool {
    *self == CMDACT_A::CMDACT_5
  }
  #[doc = "Checks if the value of the field is `CMDACT_6`"]
  #[inline(always)]
  pub fn is_cmdact_6(&self) -> bool {
    *self == CMDACT_A::CMDACT_6
  }
  #[doc = "Checks if the value of the field is `CMDACT_7`"]
  #[inline(always)]
  pub fn is_cmdact_7(&self) -> bool {
    *self == CMDACT_A::CMDACT_7
  }
  #[doc = "Checks if the value of the field is `CMDACT_8`"]
  #[inline(always)]
  pub fn is_cmdact_8(&self) -> bool {
    *self == CMDACT_A::CMDACT_8
  }
  #[doc = "Checks if the value of the field is `CMDACT_9`"]
  #[inline(always)]
  pub fn is_cmdact_9(&self) -> bool {
    *self == CMDACT_A::CMDACT_9
  }
}
impl R {
  #[doc = "Bit 0 - Result FIFO Ready Flag"]
  #[inline(always)]
  pub fn rdy(&self) -> RDY_R {
    RDY_R::new((self.bits & 0x01) != 0)
  }
  #[doc = "Bit 1 - Result FIFO Overflow Flag"]
  #[inline(always)]
  pub fn fof(&self) -> FOF_R {
    FOF_R::new(((self.bits >> 1) & 0x01) != 0)
  }
  #[doc = "Bits 16:17 - Trigger Active"]
  #[inline(always)]
  pub fn trgact(&self) -> TRGACT_R {
    TRGACT_R::new(((self.bits >> 16) & 0x03) as u8)
  }
  #[doc = "Bits 24:27 - Command Active"]
  #[inline(always)]
  pub fn cmdact(&self) -> CMDACT_R {
    CMDACT_R::new(((self.bits >> 24) & 0x0f) as u8)
  }
}
impl W {
  #[doc = "Bit 1 - Result FIFO Overflow Flag"]
  #[inline(always)]
  pub fn fof(&mut self) -> FOF_W {
    FOF_W { w: self }
  }
}
