#[doc = "Reader of register TCTRL[%s]"]
pub type R = crate::R<u32, super::TCTRL>;
#[doc = "Writer for register TCTRL[%s]"]
pub type W = crate::W<u32, super::TCTRL>;
#[doc = "Register TCTRL[%s] `reset()`'s with value 0"]
impl crate::ResetValue for super::TCTRL {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "Trigger enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HTEN_A {
  #[doc = "0: Hardware trigger source disabled"]
  HTEN_0,
  #[doc = "1: Hardware trigger source enabled"]
  HTEN_1,
}
impl From<HTEN_A> for bool {
  #[inline(always)]
  fn from(variant: HTEN_A) -> Self {
    match variant {
      HTEN_A::HTEN_0 => false,
      HTEN_A::HTEN_1 => true,
    }
  }
}
#[doc = "Reader of field `HTEN`"]
pub type HTEN_R = crate::R<bool, HTEN_A>;
impl HTEN_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> HTEN_A {
    match self.bits {
      false => HTEN_A::HTEN_0,
      true => HTEN_A::HTEN_1,
    }
  }
  #[doc = "Checks if the value of the field is `HTEN_0`"]
  #[inline(always)]
  pub fn is_hten_0(&self) -> bool {
    *self == HTEN_A::HTEN_0
  }
  #[doc = "Checks if the value of the field is `HTEN_1`"]
  #[inline(always)]
  pub fn is_hten_1(&self) -> bool {
    *self == HTEN_A::HTEN_1
  }
}
#[doc = "Write proxy for field `HTEN`"]
pub struct HTEN_W<'a> {
  w: &'a mut W,
}
impl<'a> HTEN_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: HTEN_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Hardware trigger source disabled"]
  #[inline(always)]
  pub fn hten_0(self) -> &'a mut W {
    self.variant(HTEN_A::HTEN_0)
  }
  #[doc = "Hardware trigger source enabled"]
  #[inline(always)]
  pub fn hten_1(self) -> &'a mut W {
    self.variant(HTEN_A::HTEN_1)
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
#[doc = "Trigger priority setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TPRI_A {
  #[doc = "0: Set to highest priority, Level 1"]
  TPRI_0,
  #[doc = "1: Set to corresponding priority level"]
  TPRI_1,
  #[doc = "2: Set to corresponding priority level"]
  TPRI_2,
  #[doc = "3: Set to lowest priority, Level 4"]
  TPRI_3,
}
impl From<TPRI_A> for u8 {
  #[inline(always)]
  fn from(variant: TPRI_A) -> Self {
    match variant {
      TPRI_A::TPRI_0 => 0,
      TPRI_A::TPRI_1 => 1,
      TPRI_A::TPRI_2 => 2,
      TPRI_A::TPRI_3 => 3,
    }
  }
}
#[doc = "Reader of field `TPRI`"]
pub type TPRI_R = crate::R<u8, TPRI_A>;
impl TPRI_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> TPRI_A {
    match self.bits {
      0 => TPRI_A::TPRI_0,
      1 => TPRI_A::TPRI_1,
      2 => TPRI_A::TPRI_2,
      3 => TPRI_A::TPRI_3,
      _ => unreachable!(),
    }
  }
  #[doc = "Checks if the value of the field is `TPRI_0`"]
  #[inline(always)]
  pub fn is_tpri_0(&self) -> bool {
    *self == TPRI_A::TPRI_0
  }
  #[doc = "Checks if the value of the field is `TPRI_1`"]
  #[inline(always)]
  pub fn is_tpri_1(&self) -> bool {
    *self == TPRI_A::TPRI_1
  }
  #[doc = "Checks if the value of the field is `TPRI_2`"]
  #[inline(always)]
  pub fn is_tpri_2(&self) -> bool {
    *self == TPRI_A::TPRI_2
  }
  #[doc = "Checks if the value of the field is `TPRI_3`"]
  #[inline(always)]
  pub fn is_tpri_3(&self) -> bool {
    *self == TPRI_A::TPRI_3
  }
}
#[doc = "Write proxy for field `TPRI`"]
pub struct TPRI_W<'a> {
  w: &'a mut W,
}
impl<'a> TPRI_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: TPRI_A) -> &'a mut W {
    {
      self.bits(variant.into())
    }
  }
  #[doc = "Set to highest priority, Level 1"]
  #[inline(always)]
  pub fn tpri_0(self) -> &'a mut W {
    self.variant(TPRI_A::TPRI_0)
  }
  #[doc = "Set to corresponding priority level"]
  #[inline(always)]
  pub fn tpri_1(self) -> &'a mut W {
    self.variant(TPRI_A::TPRI_1)
  }
  #[doc = "Set to corresponding priority level"]
  #[inline(always)]
  pub fn tpri_2(self) -> &'a mut W {
    self.variant(TPRI_A::TPRI_2)
  }
  #[doc = "Set to lowest priority, Level 4"]
  #[inline(always)]
  pub fn tpri_3(self) -> &'a mut W {
    self.variant(TPRI_A::TPRI_3)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
    self.w
  }
}
#[doc = "Reader of field `TDLY`"]
pub type TDLY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TDLY`"]
pub struct TDLY_W<'a> {
  w: &'a mut W,
}
impl<'a> TDLY_W<'a> {
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
    self.w
  }
}
#[doc = "Trigger command select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCMD_A {
  #[doc = "0: Not a valid selection from the command buffer. Trigger event is ignored."]
  TCMD_0,
  #[doc = "1: CMD1 is executed"]
  TCMD_1,
  #[doc = "2: Corresponding CMD is executed"]
  TCMD_2,
  #[doc = "3: Corresponding CMD is executed"]
  TCMD_3,
  #[doc = "4: Corresponding CMD is executed"]
  TCMD_4,
  #[doc = "5: Corresponding CMD is executed"]
  TCMD_5,
  #[doc = "6: Corresponding CMD is executed"]
  TCMD_6,
  #[doc = "7: Corresponding CMD is executed"]
  TCMD_7,
  #[doc = "8: Corresponding CMD is executed"]
  TCMD_8,
  #[doc = "9: Corresponding CMD is executed"]
  TCMD_9,
  #[doc = "15: CMD15 is executed"]
  TCMD_15,
}
impl From<TCMD_A> for u8 {
  #[inline(always)]
  fn from(variant: TCMD_A) -> Self {
    match variant {
      TCMD_A::TCMD_0 => 0,
      TCMD_A::TCMD_1 => 1,
      TCMD_A::TCMD_2 => 2,
      TCMD_A::TCMD_3 => 3,
      TCMD_A::TCMD_4 => 4,
      TCMD_A::TCMD_5 => 5,
      TCMD_A::TCMD_6 => 6,
      TCMD_A::TCMD_7 => 7,
      TCMD_A::TCMD_8 => 8,
      TCMD_A::TCMD_9 => 9,
      TCMD_A::TCMD_15 => 15,
    }
  }
}
#[doc = "Reader of field `TCMD`"]
pub type TCMD_R = crate::R<u8, TCMD_A>;
impl TCMD_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> crate::Variant<u8, TCMD_A> {
    use crate::Variant::*;
    match self.bits {
      0 => Val(TCMD_A::TCMD_0),
      1 => Val(TCMD_A::TCMD_1),
      2 => Val(TCMD_A::TCMD_2),
      3 => Val(TCMD_A::TCMD_3),
      4 => Val(TCMD_A::TCMD_4),
      5 => Val(TCMD_A::TCMD_5),
      6 => Val(TCMD_A::TCMD_6),
      7 => Val(TCMD_A::TCMD_7),
      8 => Val(TCMD_A::TCMD_8),
      9 => Val(TCMD_A::TCMD_9),
      15 => Val(TCMD_A::TCMD_15),
      i => Res(i),
    }
  }
  #[doc = "Checks if the value of the field is `TCMD_0`"]
  #[inline(always)]
  pub fn is_tcmd_0(&self) -> bool {
    *self == TCMD_A::TCMD_0
  }
  #[doc = "Checks if the value of the field is `TCMD_1`"]
  #[inline(always)]
  pub fn is_tcmd_1(&self) -> bool {
    *self == TCMD_A::TCMD_1
  }
  #[doc = "Checks if the value of the field is `TCMD_2`"]
  #[inline(always)]
  pub fn is_tcmd_2(&self) -> bool {
    *self == TCMD_A::TCMD_2
  }
  #[doc = "Checks if the value of the field is `TCMD_3`"]
  #[inline(always)]
  pub fn is_tcmd_3(&self) -> bool {
    *self == TCMD_A::TCMD_3
  }
  #[doc = "Checks if the value of the field is `TCMD_4`"]
  #[inline(always)]
  pub fn is_tcmd_4(&self) -> bool {
    *self == TCMD_A::TCMD_4
  }
  #[doc = "Checks if the value of the field is `TCMD_5`"]
  #[inline(always)]
  pub fn is_tcmd_5(&self) -> bool {
    *self == TCMD_A::TCMD_5
  }
  #[doc = "Checks if the value of the field is `TCMD_6`"]
  #[inline(always)]
  pub fn is_tcmd_6(&self) -> bool {
    *self == TCMD_A::TCMD_6
  }
  #[doc = "Checks if the value of the field is `TCMD_7`"]
  #[inline(always)]
  pub fn is_tcmd_7(&self) -> bool {
    *self == TCMD_A::TCMD_7
  }
  #[doc = "Checks if the value of the field is `TCMD_8`"]
  #[inline(always)]
  pub fn is_tcmd_8(&self) -> bool {
    *self == TCMD_A::TCMD_8
  }
  #[doc = "Checks if the value of the field is `TCMD_9`"]
  #[inline(always)]
  pub fn is_tcmd_9(&self) -> bool {
    *self == TCMD_A::TCMD_9
  }
  #[doc = "Checks if the value of the field is `TCMD_15`"]
  #[inline(always)]
  pub fn is_tcmd_15(&self) -> bool {
    *self == TCMD_A::TCMD_15
  }
}
#[doc = "Write proxy for field `TCMD`"]
pub struct TCMD_W<'a> {
  w: &'a mut W,
}
impl<'a> TCMD_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: TCMD_A) -> &'a mut W {
    unsafe { self.bits(variant.into()) }
  }
  #[doc = "Not a valid selection from the command buffer. Trigger event is ignored."]
  #[inline(always)]
  pub fn tcmd_0(self) -> &'a mut W {
    self.variant(TCMD_A::TCMD_0)
  }
  #[doc = "CMD1 is executed"]
  #[inline(always)]
  pub fn tcmd_1(self) -> &'a mut W {
    self.variant(TCMD_A::TCMD_1)
  }
  #[doc = "Corresponding CMD is executed"]
  #[inline(always)]
  pub fn tcmd_2(self) -> &'a mut W {
    self.variant(TCMD_A::TCMD_2)
  }
  #[doc = "Corresponding CMD is executed"]
  #[inline(always)]
  pub fn tcmd_3(self) -> &'a mut W {
    self.variant(TCMD_A::TCMD_3)
  }
  #[doc = "Corresponding CMD is executed"]
  #[inline(always)]
  pub fn tcmd_4(self) -> &'a mut W {
    self.variant(TCMD_A::TCMD_4)
  }
  #[doc = "Corresponding CMD is executed"]
  #[inline(always)]
  pub fn tcmd_5(self) -> &'a mut W {
    self.variant(TCMD_A::TCMD_5)
  }
  #[doc = "Corresponding CMD is executed"]
  #[inline(always)]
  pub fn tcmd_6(self) -> &'a mut W {
    self.variant(TCMD_A::TCMD_6)
  }
  #[doc = "Corresponding CMD is executed"]
  #[inline(always)]
  pub fn tcmd_7(self) -> &'a mut W {
    self.variant(TCMD_A::TCMD_7)
  }
  #[doc = "Corresponding CMD is executed"]
  #[inline(always)]
  pub fn tcmd_8(self) -> &'a mut W {
    self.variant(TCMD_A::TCMD_8)
  }
  #[doc = "Corresponding CMD is executed"]
  #[inline(always)]
  pub fn tcmd_9(self) -> &'a mut W {
    self.variant(TCMD_A::TCMD_9)
  }
  #[doc = "CMD15 is executed"]
  #[inline(always)]
  pub fn tcmd_15(self) -> &'a mut W {
    self.variant(TCMD_A::TCMD_15)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
    self.w
  }
}
impl R {
  #[doc = "Bit 0 - Trigger enable"]
  #[inline(always)]
  pub fn hten(&self) -> HTEN_R {
    HTEN_R::new((self.bits & 0x01) != 0)
  }
  #[doc = "Bits 8:9 - Trigger priority setting"]
  #[inline(always)]
  pub fn tpri(&self) -> TPRI_R {
    TPRI_R::new(((self.bits >> 8) & 0x03) as u8)
  }
  #[doc = "Bits 16:19 - Trigger delay select"]
  #[inline(always)]
  pub fn tdly(&self) -> TDLY_R {
    TDLY_R::new(((self.bits >> 16) & 0x0f) as u8)
  }
  #[doc = "Bits 24:27 - Trigger command select"]
  #[inline(always)]
  pub fn tcmd(&self) -> TCMD_R {
    TCMD_R::new(((self.bits >> 24) & 0x0f) as u8)
  }
}
impl W {
  #[doc = "Bit 0 - Trigger enable"]
  #[inline(always)]
  pub fn hten(&mut self) -> HTEN_W {
    HTEN_W { w: self }
  }
  #[doc = "Bits 8:9 - Trigger priority setting"]
  #[inline(always)]
  pub fn tpri(&mut self) -> TPRI_W {
    TPRI_W { w: self }
  }
  #[doc = "Bits 16:19 - Trigger delay select"]
  #[inline(always)]
  pub fn tdly(&mut self) -> TDLY_W {
    TDLY_W { w: self }
  }
  #[doc = "Bits 24:27 - Trigger command select"]
  #[inline(always)]
  pub fn tcmd(&mut self) -> TCMD_W {
    TCMD_W { w: self }
  }
}
