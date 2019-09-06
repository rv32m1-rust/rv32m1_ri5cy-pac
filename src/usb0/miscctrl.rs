#[doc = "Reader of register MISCCTRL"]
pub type R = crate::R<u8, super::MISCCTRL>;
#[doc = "Writer for register MISCCTRL"]
pub type W = crate::W<u8, super::MISCCTRL>;
#[doc = "Register MISCCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::MISCCTRL {
  type Type = u8;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "Dynamic SOF Threshold Compare mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOFDYNTHLD_A {
  #[doc = "0: SOF_TOK interrupt is set when byte times SOF threshold is reached."]
  SOFDYNTHLD_0,
  #[doc = "1: SOF_TOK interrupt is set when 8 byte times SOF threshold is reached or overstepped."]
  SOFDYNTHLD_1,
}
impl From<SOFDYNTHLD_A> for bool {
  #[inline(always)]
  fn from(variant: SOFDYNTHLD_A) -> Self {
    match variant {
      SOFDYNTHLD_A::SOFDYNTHLD_0 => false,
      SOFDYNTHLD_A::SOFDYNTHLD_1 => true,
    }
  }
}
#[doc = "Reader of field `SOFDYNTHLD`"]
pub type SOFDYNTHLD_R = crate::R<bool, SOFDYNTHLD_A>;
impl SOFDYNTHLD_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> SOFDYNTHLD_A {
    match self.bits {
      false => SOFDYNTHLD_A::SOFDYNTHLD_0,
      true => SOFDYNTHLD_A::SOFDYNTHLD_1,
    }
  }
  #[doc = "Checks if the value of the field is `SOFDYNTHLD_0`"]
  #[inline(always)]
  pub fn is_sofdynthld_0(&self) -> bool {
    *self == SOFDYNTHLD_A::SOFDYNTHLD_0
  }
  #[doc = "Checks if the value of the field is `SOFDYNTHLD_1`"]
  #[inline(always)]
  pub fn is_sofdynthld_1(&self) -> bool {
    *self == SOFDYNTHLD_A::SOFDYNTHLD_1
  }
}
#[doc = "Write proxy for field `SOFDYNTHLD`"]
pub struct SOFDYNTHLD_W<'a> {
  w: &'a mut W,
}
impl<'a> SOFDYNTHLD_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: SOFDYNTHLD_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "SOF_TOK interrupt is set when byte times SOF threshold is reached."]
  #[inline(always)]
  pub fn sofdynthld_0(self) -> &'a mut W {
    self.variant(SOFDYNTHLD_A::SOFDYNTHLD_0)
  }
  #[doc = "SOF_TOK interrupt is set when 8 byte times SOF threshold is reached or overstepped."]
  #[inline(always)]
  pub fn sofdynthld_1(self) -> &'a mut W {
    self.variant(SOFDYNTHLD_A::SOFDYNTHLD_1)
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
#[doc = "SOF_TOK Interrupt Generation Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOFBUSSET_A {
  #[doc = "0: SOF_TOK interrupt is set according to SOF threshold value."]
  SOFBUSSET_0,
  #[doc = "1: SOF_TOK interrupt is set when SOF counter reaches 0."]
  SOFBUSSET_1,
}
impl From<SOFBUSSET_A> for bool {
  #[inline(always)]
  fn from(variant: SOFBUSSET_A) -> Self {
    match variant {
      SOFBUSSET_A::SOFBUSSET_0 => false,
      SOFBUSSET_A::SOFBUSSET_1 => true,
    }
  }
}
#[doc = "Reader of field `SOFBUSSET`"]
pub type SOFBUSSET_R = crate::R<bool, SOFBUSSET_A>;
impl SOFBUSSET_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> SOFBUSSET_A {
    match self.bits {
      false => SOFBUSSET_A::SOFBUSSET_0,
      true => SOFBUSSET_A::SOFBUSSET_1,
    }
  }
  #[doc = "Checks if the value of the field is `SOFBUSSET_0`"]
  #[inline(always)]
  pub fn is_sofbusset_0(&self) -> bool {
    *self == SOFBUSSET_A::SOFBUSSET_0
  }
  #[doc = "Checks if the value of the field is `SOFBUSSET_1`"]
  #[inline(always)]
  pub fn is_sofbusset_1(&self) -> bool {
    *self == SOFBUSSET_A::SOFBUSSET_1
  }
}
#[doc = "Write proxy for field `SOFBUSSET`"]
pub struct SOFBUSSET_W<'a> {
  w: &'a mut W,
}
impl<'a> SOFBUSSET_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: SOFBUSSET_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "SOF_TOK interrupt is set according to SOF threshold value."]
  #[inline(always)]
  pub fn sofbusset_0(self) -> &'a mut W {
    self.variant(SOFBUSSET_A::SOFBUSSET_0)
  }
  #[doc = "SOF_TOK interrupt is set when SOF counter reaches 0."]
  #[inline(always)]
  pub fn sofbusset_1(self) -> &'a mut W {
    self.variant(SOFBUSSET_A::SOFBUSSET_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u8) & 0x01) << 1);
    self.w
  }
}
#[doc = "OWN Error Detect for ISO IN / ISO OUT Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OWNERRISODIS_A {
  #[doc = "0: OWN error detect for ISO IN / ISO OUT is not disabled."]
  OWNERRISODIS_0,
  #[doc = "1: OWN error detect for ISO IN / ISO OUT is disabled."]
  OWNERRISODIS_1,
}
impl From<OWNERRISODIS_A> for bool {
  #[inline(always)]
  fn from(variant: OWNERRISODIS_A) -> Self {
    match variant {
      OWNERRISODIS_A::OWNERRISODIS_0 => false,
      OWNERRISODIS_A::OWNERRISODIS_1 => true,
    }
  }
}
#[doc = "Reader of field `OWNERRISODIS`"]
pub type OWNERRISODIS_R = crate::R<bool, OWNERRISODIS_A>;
impl OWNERRISODIS_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> OWNERRISODIS_A {
    match self.bits {
      false => OWNERRISODIS_A::OWNERRISODIS_0,
      true => OWNERRISODIS_A::OWNERRISODIS_1,
    }
  }
  #[doc = "Checks if the value of the field is `OWNERRISODIS_0`"]
  #[inline(always)]
  pub fn is_ownerrisodis_0(&self) -> bool {
    *self == OWNERRISODIS_A::OWNERRISODIS_0
  }
  #[doc = "Checks if the value of the field is `OWNERRISODIS_1`"]
  #[inline(always)]
  pub fn is_ownerrisodis_1(&self) -> bool {
    *self == OWNERRISODIS_A::OWNERRISODIS_1
  }
}
#[doc = "Write proxy for field `OWNERRISODIS`"]
pub struct OWNERRISODIS_W<'a> {
  w: &'a mut W,
}
impl<'a> OWNERRISODIS_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: OWNERRISODIS_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "OWN error detect for ISO IN / ISO OUT is not disabled."]
  #[inline(always)]
  pub fn ownerrisodis_0(self) -> &'a mut W {
    self.variant(OWNERRISODIS_A::OWNERRISODIS_0)
  }
  #[doc = "OWN error detect for ISO IN / ISO OUT is disabled."]
  #[inline(always)]
  pub fn ownerrisodis_1(self) -> &'a mut W {
    self.variant(OWNERRISODIS_A::OWNERRISODIS_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u8) & 0x01) << 2);
    self.w
  }
}
#[doc = "VREGIN Rising Edge Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VREDG_EN_A {
  #[doc = "0: VREGIN rising edge interrupt disabled."]
  VREDG_EN_0,
  #[doc = "1: VREGIN rising edge interrupt enabled."]
  VREDG_EN_1,
}
impl From<VREDG_EN_A> for bool {
  #[inline(always)]
  fn from(variant: VREDG_EN_A) -> Self {
    match variant {
      VREDG_EN_A::VREDG_EN_0 => false,
      VREDG_EN_A::VREDG_EN_1 => true,
    }
  }
}
#[doc = "Reader of field `VREDG_EN`"]
pub type VREDG_EN_R = crate::R<bool, VREDG_EN_A>;
impl VREDG_EN_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> VREDG_EN_A {
    match self.bits {
      false => VREDG_EN_A::VREDG_EN_0,
      true => VREDG_EN_A::VREDG_EN_1,
    }
  }
  #[doc = "Checks if the value of the field is `VREDG_EN_0`"]
  #[inline(always)]
  pub fn is_vredg_en_0(&self) -> bool {
    *self == VREDG_EN_A::VREDG_EN_0
  }
  #[doc = "Checks if the value of the field is `VREDG_EN_1`"]
  #[inline(always)]
  pub fn is_vredg_en_1(&self) -> bool {
    *self == VREDG_EN_A::VREDG_EN_1
  }
}
#[doc = "Write proxy for field `VREDG_EN`"]
pub struct VREDG_EN_W<'a> {
  w: &'a mut W,
}
impl<'a> VREDG_EN_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: VREDG_EN_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "VREGIN rising edge interrupt disabled."]
  #[inline(always)]
  pub fn vredg_en_0(self) -> &'a mut W {
    self.variant(VREDG_EN_A::VREDG_EN_0)
  }
  #[doc = "VREGIN rising edge interrupt enabled."]
  #[inline(always)]
  pub fn vredg_en_1(self) -> &'a mut W {
    self.variant(VREDG_EN_A::VREDG_EN_1)
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
#[doc = "VREGIN Falling Edge Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VFEDG_EN_A {
  #[doc = "0: VREGIN falling edge interrupt disabled."]
  VFEDG_EN_0,
  #[doc = "1: VREGIN falling edge interrupt enabled."]
  VFEDG_EN_1,
}
impl From<VFEDG_EN_A> for bool {
  #[inline(always)]
  fn from(variant: VFEDG_EN_A) -> Self {
    match variant {
      VFEDG_EN_A::VFEDG_EN_0 => false,
      VFEDG_EN_A::VFEDG_EN_1 => true,
    }
  }
}
#[doc = "Reader of field `VFEDG_EN`"]
pub type VFEDG_EN_R = crate::R<bool, VFEDG_EN_A>;
impl VFEDG_EN_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> VFEDG_EN_A {
    match self.bits {
      false => VFEDG_EN_A::VFEDG_EN_0,
      true => VFEDG_EN_A::VFEDG_EN_1,
    }
  }
  #[doc = "Checks if the value of the field is `VFEDG_EN_0`"]
  #[inline(always)]
  pub fn is_vfedg_en_0(&self) -> bool {
    *self == VFEDG_EN_A::VFEDG_EN_0
  }
  #[doc = "Checks if the value of the field is `VFEDG_EN_1`"]
  #[inline(always)]
  pub fn is_vfedg_en_1(&self) -> bool {
    *self == VFEDG_EN_A::VFEDG_EN_1
  }
}
#[doc = "Write proxy for field `VFEDG_EN`"]
pub struct VFEDG_EN_W<'a> {
  w: &'a mut W,
}
impl<'a> VFEDG_EN_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: VFEDG_EN_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "VREGIN falling edge interrupt disabled."]
  #[inline(always)]
  pub fn vfedg_en_0(self) -> &'a mut W {
    self.variant(VFEDG_EN_A::VFEDG_EN_0)
  }
  #[doc = "VREGIN falling edge interrupt enabled."]
  #[inline(always)]
  pub fn vfedg_en_1(self) -> &'a mut W {
    self.variant(VFEDG_EN_A::VFEDG_EN_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u8) & 0x01) << 4);
    self.w
  }
}
#[doc = "USB Peripheral mode Stall Adjust Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STL_ADJ_EN_A {
  #[doc = "0: If USB_ENDPTn\\[END_STALL\\] = 1, both IN and OUT directions for the associated endpoint will be stalled"]
  STL_ADJ_EN_0,
  #[doc = "1: If USB_ENDPTn\\[END_STALL\\] = 1, the USB_STALL_xx_DIS registers control which directions for the associated endpoint will be stalled."]
  STL_ADJ_EN_1,
}
impl From<STL_ADJ_EN_A> for bool {
  #[inline(always)]
  fn from(variant: STL_ADJ_EN_A) -> Self {
    match variant {
      STL_ADJ_EN_A::STL_ADJ_EN_0 => false,
      STL_ADJ_EN_A::STL_ADJ_EN_1 => true,
    }
  }
}
#[doc = "Reader of field `STL_ADJ_EN`"]
pub type STL_ADJ_EN_R = crate::R<bool, STL_ADJ_EN_A>;
impl STL_ADJ_EN_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> STL_ADJ_EN_A {
    match self.bits {
      false => STL_ADJ_EN_A::STL_ADJ_EN_0,
      true => STL_ADJ_EN_A::STL_ADJ_EN_1,
    }
  }
  #[doc = "Checks if the value of the field is `STL_ADJ_EN_0`"]
  #[inline(always)]
  pub fn is_stl_adj_en_0(&self) -> bool {
    *self == STL_ADJ_EN_A::STL_ADJ_EN_0
  }
  #[doc = "Checks if the value of the field is `STL_ADJ_EN_1`"]
  #[inline(always)]
  pub fn is_stl_adj_en_1(&self) -> bool {
    *self == STL_ADJ_EN_A::STL_ADJ_EN_1
  }
}
#[doc = "Write proxy for field `STL_ADJ_EN`"]
pub struct STL_ADJ_EN_W<'a> {
  w: &'a mut W,
}
impl<'a> STL_ADJ_EN_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: STL_ADJ_EN_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "If USB_ENDPTn\\[END_STALL\\] = 1, both IN and OUT directions for the associated endpoint will be stalled"]
  #[inline(always)]
  pub fn stl_adj_en_0(self) -> &'a mut W {
    self.variant(STL_ADJ_EN_A::STL_ADJ_EN_0)
  }
  #[doc = "If USB_ENDPTn\\[END_STALL\\] = 1, the USB_STALL_xx_DIS registers control which directions for the associated endpoint will be stalled."]
  #[inline(always)]
  pub fn stl_adj_en_1(self) -> &'a mut W {
    self.variant(STL_ADJ_EN_A::STL_ADJ_EN_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u8) & 0x01) << 7);
    self.w
  }
}
impl R {
  #[doc = "Bit 0 - Dynamic SOF Threshold Compare mode"]
  #[inline(always)]
  pub fn sofdynthld(&self) -> SOFDYNTHLD_R {
    SOFDYNTHLD_R::new((self.bits & 0x01) != 0)
  }
  #[doc = "Bit 1 - SOF_TOK Interrupt Generation Mode Select"]
  #[inline(always)]
  pub fn sofbusset(&self) -> SOFBUSSET_R {
    SOFBUSSET_R::new(((self.bits >> 1) & 0x01) != 0)
  }
  #[doc = "Bit 2 - OWN Error Detect for ISO IN / ISO OUT Disable"]
  #[inline(always)]
  pub fn ownerrisodis(&self) -> OWNERRISODIS_R {
    OWNERRISODIS_R::new(((self.bits >> 2) & 0x01) != 0)
  }
  #[doc = "Bit 3 - VREGIN Rising Edge Interrupt Enable"]
  #[inline(always)]
  pub fn vredg_en(&self) -> VREDG_EN_R {
    VREDG_EN_R::new(((self.bits >> 3) & 0x01) != 0)
  }
  #[doc = "Bit 4 - VREGIN Falling Edge Interrupt Enable"]
  #[inline(always)]
  pub fn vfedg_en(&self) -> VFEDG_EN_R {
    VFEDG_EN_R::new(((self.bits >> 4) & 0x01) != 0)
  }
  #[doc = "Bit 7 - USB Peripheral mode Stall Adjust Enable"]
  #[inline(always)]
  pub fn stl_adj_en(&self) -> STL_ADJ_EN_R {
    STL_ADJ_EN_R::new(((self.bits >> 7) & 0x01) != 0)
  }
}
impl W {
  #[doc = "Bit 0 - Dynamic SOF Threshold Compare mode"]
  #[inline(always)]
  pub fn sofdynthld(&mut self) -> SOFDYNTHLD_W {
    SOFDYNTHLD_W { w: self }
  }
  #[doc = "Bit 1 - SOF_TOK Interrupt Generation Mode Select"]
  #[inline(always)]
  pub fn sofbusset(&mut self) -> SOFBUSSET_W {
    SOFBUSSET_W { w: self }
  }
  #[doc = "Bit 2 - OWN Error Detect for ISO IN / ISO OUT Disable"]
  #[inline(always)]
  pub fn ownerrisodis(&mut self) -> OWNERRISODIS_W {
    OWNERRISODIS_W { w: self }
  }
  #[doc = "Bit 3 - VREGIN Rising Edge Interrupt Enable"]
  #[inline(always)]
  pub fn vredg_en(&mut self) -> VREDG_EN_W {
    VREDG_EN_W { w: self }
  }
  #[doc = "Bit 4 - VREGIN Falling Edge Interrupt Enable"]
  #[inline(always)]
  pub fn vfedg_en(&mut self) -> VFEDG_EN_W {
    VFEDG_EN_W { w: self }
  }
  #[doc = "Bit 7 - USB Peripheral mode Stall Adjust Enable"]
  #[inline(always)]
  pub fn stl_adj_en(&mut self) -> STL_ADJ_EN_W {
    STL_ADJ_EN_W { w: self }
  }
}
