#[doc = "Reader of register INT_MASK"]
pub type R = crate::R<u32, super::INT_MASK>;
#[doc = "Writer for register INT_MASK"]
pub type W = crate::W<u32, super::INT_MASK>;
#[doc = "Register INT_MASK `reset()`'s with value 0xffff"]
impl crate::ResetValue for super::INT_MASK {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0xffff
  }
}
#[doc = "Receive Data Threshold Interrupt Mask\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDT_IM_A {
  #[doc = "0: RDTF interrupt enabled"]
  RDT_IM_0,
  #[doc = "1: RDTF interrupt masked (default)"]
  RDT_IM_1,
}
impl From<RDT_IM_A> for bool {
  #[inline(always)]
  fn from(variant: RDT_IM_A) -> Self {
    match variant {
      RDT_IM_A::RDT_IM_0 => false,
      RDT_IM_A::RDT_IM_1 => true,
    }
  }
}
#[doc = "Reader of field `RDT_IM`"]
pub type RDT_IM_R = crate::R<bool, RDT_IM_A>;
impl RDT_IM_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> RDT_IM_A {
    match self.bits {
      false => RDT_IM_A::RDT_IM_0,
      true => RDT_IM_A::RDT_IM_1,
    }
  }
  #[doc = "Checks if the value of the field is `RDT_IM_0`"]
  #[inline(always)]
  pub fn is_rdt_im_0(&self) -> bool {
    *self == RDT_IM_A::RDT_IM_0
  }
  #[doc = "Checks if the value of the field is `RDT_IM_1`"]
  #[inline(always)]
  pub fn is_rdt_im_1(&self) -> bool {
    *self == RDT_IM_A::RDT_IM_1
  }
}
#[doc = "Write proxy for field `RDT_IM`"]
pub struct RDT_IM_W<'a> {
  w: &'a mut W,
}
impl<'a> RDT_IM_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: RDT_IM_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "RDTF interrupt enabled"]
  #[inline(always)]
  pub fn rdt_im_0(self) -> &'a mut W {
    self.variant(RDT_IM_A::RDT_IM_0)
  }
  #[doc = "RDTF interrupt masked (default)"]
  #[inline(always)]
  pub fn rdt_im_1(self) -> &'a mut W {
    self.variant(RDT_IM_A::RDT_IM_1)
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
#[doc = "Transmit Complete Interrupt Mask\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TC_IM_A {
  #[doc = "0: TCF interrupt enabled"]
  TC_IM_0,
  #[doc = "1: TCF interrupt masked (default)"]
  TC_IM_1,
}
impl From<TC_IM_A> for bool {
  #[inline(always)]
  fn from(variant: TC_IM_A) -> Self {
    match variant {
      TC_IM_A::TC_IM_0 => false,
      TC_IM_A::TC_IM_1 => true,
    }
  }
}
#[doc = "Reader of field `TC_IM`"]
pub type TC_IM_R = crate::R<bool, TC_IM_A>;
impl TC_IM_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> TC_IM_A {
    match self.bits {
      false => TC_IM_A::TC_IM_0,
      true => TC_IM_A::TC_IM_1,
    }
  }
  #[doc = "Checks if the value of the field is `TC_IM_0`"]
  #[inline(always)]
  pub fn is_tc_im_0(&self) -> bool {
    *self == TC_IM_A::TC_IM_0
  }
  #[doc = "Checks if the value of the field is `TC_IM_1`"]
  #[inline(always)]
  pub fn is_tc_im_1(&self) -> bool {
    *self == TC_IM_A::TC_IM_1
  }
}
#[doc = "Write proxy for field `TC_IM`"]
pub struct TC_IM_W<'a> {
  w: &'a mut W,
}
impl<'a> TC_IM_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: TC_IM_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "TCF interrupt enabled"]
  #[inline(always)]
  pub fn tc_im_0(self) -> &'a mut W {
    self.variant(TC_IM_A::TC_IM_0)
  }
  #[doc = "TCF interrupt masked (default)"]
  #[inline(always)]
  pub fn tc_im_1(self) -> &'a mut W {
    self.variant(TC_IM_A::TC_IM_1)
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
#[doc = "Receive FIFO Overflow Interrupt Mask\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RFO_IM_A {
  #[doc = "0: RFO interrupt enabled"]
  RFO_IM_0,
  #[doc = "1: RFO interrupt masked (default)"]
  RFO_IM_1,
}
impl From<RFO_IM_A> for bool {
  #[inline(always)]
  fn from(variant: RFO_IM_A) -> Self {
    match variant {
      RFO_IM_A::RFO_IM_0 => false,
      RFO_IM_A::RFO_IM_1 => true,
    }
  }
}
#[doc = "Reader of field `RFO_IM`"]
pub type RFO_IM_R = crate::R<bool, RFO_IM_A>;
impl RFO_IM_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> RFO_IM_A {
    match self.bits {
      false => RFO_IM_A::RFO_IM_0,
      true => RFO_IM_A::RFO_IM_1,
    }
  }
  #[doc = "Checks if the value of the field is `RFO_IM_0`"]
  #[inline(always)]
  pub fn is_rfo_im_0(&self) -> bool {
    *self == RFO_IM_A::RFO_IM_0
  }
  #[doc = "Checks if the value of the field is `RFO_IM_1`"]
  #[inline(always)]
  pub fn is_rfo_im_1(&self) -> bool {
    *self == RFO_IM_A::RFO_IM_1
  }
}
#[doc = "Write proxy for field `RFO_IM`"]
pub struct RFO_IM_W<'a> {
  w: &'a mut W,
}
impl<'a> RFO_IM_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: RFO_IM_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "RFO interrupt enabled"]
  #[inline(always)]
  pub fn rfo_im_0(self) -> &'a mut W {
    self.variant(RFO_IM_A::RFO_IM_0)
  }
  #[doc = "RFO interrupt masked (default)"]
  #[inline(always)]
  pub fn rfo_im_1(self) -> &'a mut W {
    self.variant(RFO_IM_A::RFO_IM_1)
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
#[doc = "Early Transmit Complete Interrupt Mask\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ETC_IM_A {
  #[doc = "0: ETC interrupt enabled"]
  ETC_IM_0,
  #[doc = "1: ETC interrupt masked (default)"]
  ETC_IM_1,
}
impl From<ETC_IM_A> for bool {
  #[inline(always)]
  fn from(variant: ETC_IM_A) -> Self {
    match variant {
      ETC_IM_A::ETC_IM_0 => false,
      ETC_IM_A::ETC_IM_1 => true,
    }
  }
}
#[doc = "Reader of field `ETC_IM`"]
pub type ETC_IM_R = crate::R<bool, ETC_IM_A>;
impl ETC_IM_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> ETC_IM_A {
    match self.bits {
      false => ETC_IM_A::ETC_IM_0,
      true => ETC_IM_A::ETC_IM_1,
    }
  }
  #[doc = "Checks if the value of the field is `ETC_IM_0`"]
  #[inline(always)]
  pub fn is_etc_im_0(&self) -> bool {
    *self == ETC_IM_A::ETC_IM_0
  }
  #[doc = "Checks if the value of the field is `ETC_IM_1`"]
  #[inline(always)]
  pub fn is_etc_im_1(&self) -> bool {
    *self == ETC_IM_A::ETC_IM_1
  }
}
#[doc = "Write proxy for field `ETC_IM`"]
pub struct ETC_IM_W<'a> {
  w: &'a mut W,
}
impl<'a> ETC_IM_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: ETC_IM_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "ETC interrupt enabled"]
  #[inline(always)]
  pub fn etc_im_0(self) -> &'a mut W {
    self.variant(ETC_IM_A::ETC_IM_0)
  }
  #[doc = "ETC interrupt masked (default)"]
  #[inline(always)]
  pub fn etc_im_1(self) -> &'a mut W {
    self.variant(ETC_IM_A::ETC_IM_1)
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
#[doc = "Transmit FIFO Empty Interrupt Mask\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TFE_IM_A {
  #[doc = "0: TFE interrupt enabled"]
  TFE_IM_0,
  #[doc = "1: TFE interrupt masked (default)"]
  TFE_IM_1,
}
impl From<TFE_IM_A> for bool {
  #[inline(always)]
  fn from(variant: TFE_IM_A) -> Self {
    match variant {
      TFE_IM_A::TFE_IM_0 => false,
      TFE_IM_A::TFE_IM_1 => true,
    }
  }
}
#[doc = "Reader of field `TFE_IM`"]
pub type TFE_IM_R = crate::R<bool, TFE_IM_A>;
impl TFE_IM_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> TFE_IM_A {
    match self.bits {
      false => TFE_IM_A::TFE_IM_0,
      true => TFE_IM_A::TFE_IM_1,
    }
  }
  #[doc = "Checks if the value of the field is `TFE_IM_0`"]
  #[inline(always)]
  pub fn is_tfe_im_0(&self) -> bool {
    *self == TFE_IM_A::TFE_IM_0
  }
  #[doc = "Checks if the value of the field is `TFE_IM_1`"]
  #[inline(always)]
  pub fn is_tfe_im_1(&self) -> bool {
    *self == TFE_IM_A::TFE_IM_1
  }
}
#[doc = "Write proxy for field `TFE_IM`"]
pub struct TFE_IM_W<'a> {
  w: &'a mut W,
}
impl<'a> TFE_IM_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: TFE_IM_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "TFE interrupt enabled"]
  #[inline(always)]
  pub fn tfe_im_0(self) -> &'a mut W {
    self.variant(TFE_IM_A::TFE_IM_0)
  }
  #[doc = "TFE interrupt masked (default)"]
  #[inline(always)]
  pub fn tfe_im_1(self) -> &'a mut W {
    self.variant(TFE_IM_A::TFE_IM_1)
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
#[doc = "Transmit NACK Threshold Interrupt Mask\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TNACK_IM_A {
  #[doc = "0: TNTE interrupt enabled"]
  TNACK_IM_0,
  #[doc = "1: TNTE interrupt masked (default)"]
  TNACK_IM_1,
}
impl From<TNACK_IM_A> for bool {
  #[inline(always)]
  fn from(variant: TNACK_IM_A) -> Self {
    match variant {
      TNACK_IM_A::TNACK_IM_0 => false,
      TNACK_IM_A::TNACK_IM_1 => true,
    }
  }
}
#[doc = "Reader of field `TNACK_IM`"]
pub type TNACK_IM_R = crate::R<bool, TNACK_IM_A>;
impl TNACK_IM_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> TNACK_IM_A {
    match self.bits {
      false => TNACK_IM_A::TNACK_IM_0,
      true => TNACK_IM_A::TNACK_IM_1,
    }
  }
  #[doc = "Checks if the value of the field is `TNACK_IM_0`"]
  #[inline(always)]
  pub fn is_tnack_im_0(&self) -> bool {
    *self == TNACK_IM_A::TNACK_IM_0
  }
  #[doc = "Checks if the value of the field is `TNACK_IM_1`"]
  #[inline(always)]
  pub fn is_tnack_im_1(&self) -> bool {
    *self == TNACK_IM_A::TNACK_IM_1
  }
}
#[doc = "Write proxy for field `TNACK_IM`"]
pub struct TNACK_IM_W<'a> {
  w: &'a mut W,
}
impl<'a> TNACK_IM_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: TNACK_IM_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "TNTE interrupt enabled"]
  #[inline(always)]
  pub fn tnack_im_0(self) -> &'a mut W {
    self.variant(TNACK_IM_A::TNACK_IM_0)
  }
  #[doc = "TNTE interrupt masked (default)"]
  #[inline(always)]
  pub fn tnack_im_1(self) -> &'a mut W {
    self.variant(TNACK_IM_A::TNACK_IM_1)
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
#[doc = "Transmit FIFO Full Interrupt Mask\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TFF_IM_A {
  #[doc = "0: TFF interrupt enabled"]
  TFF_IM_0,
  #[doc = "1: TFF interrupt masked (default)"]
  TFF_IM_1,
}
impl From<TFF_IM_A> for bool {
  #[inline(always)]
  fn from(variant: TFF_IM_A) -> Self {
    match variant {
      TFF_IM_A::TFF_IM_0 => false,
      TFF_IM_A::TFF_IM_1 => true,
    }
  }
}
#[doc = "Reader of field `TFF_IM`"]
pub type TFF_IM_R = crate::R<bool, TFF_IM_A>;
impl TFF_IM_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> TFF_IM_A {
    match self.bits {
      false => TFF_IM_A::TFF_IM_0,
      true => TFF_IM_A::TFF_IM_1,
    }
  }
  #[doc = "Checks if the value of the field is `TFF_IM_0`"]
  #[inline(always)]
  pub fn is_tff_im_0(&self) -> bool {
    *self == TFF_IM_A::TFF_IM_0
  }
  #[doc = "Checks if the value of the field is `TFF_IM_1`"]
  #[inline(always)]
  pub fn is_tff_im_1(&self) -> bool {
    *self == TFF_IM_A::TFF_IM_1
  }
}
#[doc = "Write proxy for field `TFF_IM`"]
pub struct TFF_IM_W<'a> {
  w: &'a mut W,
}
impl<'a> TFF_IM_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: TFF_IM_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "TFF interrupt enabled"]
  #[inline(always)]
  pub fn tff_im_0(self) -> &'a mut W {
    self.variant(TFF_IM_A::TFF_IM_0)
  }
  #[doc = "TFF interrupt masked (default)"]
  #[inline(always)]
  pub fn tff_im_1(self) -> &'a mut W {
    self.variant(TFF_IM_A::TFF_IM_1)
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
#[doc = "Transmit Data Threshold Interrupt Mask\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TDT_IM_A {
  #[doc = "0: TDTF interrupt enabled"]
  TDT_IM_0,
  #[doc = "1: TDTF interrupt masked (default)"]
  TDT_IM_1,
}
impl From<TDT_IM_A> for bool {
  #[inline(always)]
  fn from(variant: TDT_IM_A) -> Self {
    match variant {
      TDT_IM_A::TDT_IM_0 => false,
      TDT_IM_A::TDT_IM_1 => true,
    }
  }
}
#[doc = "Reader of field `TDT_IM`"]
pub type TDT_IM_R = crate::R<bool, TDT_IM_A>;
impl TDT_IM_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> TDT_IM_A {
    match self.bits {
      false => TDT_IM_A::TDT_IM_0,
      true => TDT_IM_A::TDT_IM_1,
    }
  }
  #[doc = "Checks if the value of the field is `TDT_IM_0`"]
  #[inline(always)]
  pub fn is_tdt_im_0(&self) -> bool {
    *self == TDT_IM_A::TDT_IM_0
  }
  #[doc = "Checks if the value of the field is `TDT_IM_1`"]
  #[inline(always)]
  pub fn is_tdt_im_1(&self) -> bool {
    *self == TDT_IM_A::TDT_IM_1
  }
}
#[doc = "Write proxy for field `TDT_IM`"]
pub struct TDT_IM_W<'a> {
  w: &'a mut W,
}
impl<'a> TDT_IM_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: TDT_IM_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "TDTF interrupt enabled"]
  #[inline(always)]
  pub fn tdt_im_0(self) -> &'a mut W {
    self.variant(TDT_IM_A::TDT_IM_0)
  }
  #[doc = "TDTF interrupt masked (default)"]
  #[inline(always)]
  pub fn tdt_im_1(self) -> &'a mut W {
    self.variant(TDT_IM_A::TDT_IM_1)
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
#[doc = "General Purpose Timer 0 Timeout Interrupt Mask\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPCNT0_IM_A {
  #[doc = "0: GPCNT0_TO interrupt enabled"]
  GPCNT0_IM_0,
  #[doc = "1: GPCNT0_TO interrupt masked (default)"]
  GPCNT0_IM_1,
}
impl From<GPCNT0_IM_A> for bool {
  #[inline(always)]
  fn from(variant: GPCNT0_IM_A) -> Self {
    match variant {
      GPCNT0_IM_A::GPCNT0_IM_0 => false,
      GPCNT0_IM_A::GPCNT0_IM_1 => true,
    }
  }
}
#[doc = "Reader of field `GPCNT0_IM`"]
pub type GPCNT0_IM_R = crate::R<bool, GPCNT0_IM_A>;
impl GPCNT0_IM_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> GPCNT0_IM_A {
    match self.bits {
      false => GPCNT0_IM_A::GPCNT0_IM_0,
      true => GPCNT0_IM_A::GPCNT0_IM_1,
    }
  }
  #[doc = "Checks if the value of the field is `GPCNT0_IM_0`"]
  #[inline(always)]
  pub fn is_gpcnt0_im_0(&self) -> bool {
    *self == GPCNT0_IM_A::GPCNT0_IM_0
  }
  #[doc = "Checks if the value of the field is `GPCNT0_IM_1`"]
  #[inline(always)]
  pub fn is_gpcnt0_im_1(&self) -> bool {
    *self == GPCNT0_IM_A::GPCNT0_IM_1
  }
}
#[doc = "Write proxy for field `GPCNT0_IM`"]
pub struct GPCNT0_IM_W<'a> {
  w: &'a mut W,
}
impl<'a> GPCNT0_IM_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: GPCNT0_IM_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "GPCNT0_TO interrupt enabled"]
  #[inline(always)]
  pub fn gpcnt0_im_0(self) -> &'a mut W {
    self.variant(GPCNT0_IM_A::GPCNT0_IM_0)
  }
  #[doc = "GPCNT0_TO interrupt masked (default)"]
  #[inline(always)]
  pub fn gpcnt0_im_1(self) -> &'a mut W {
    self.variant(GPCNT0_IM_A::GPCNT0_IM_1)
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
#[doc = "Character Wait Time Error Interrupt Mask\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CWT_ERR_IM_A {
  #[doc = "0: CWT_ERR interrupt enabled"]
  CWT_ERR_IM_0,
  #[doc = "1: CWT_ERR interrupt masked (default)"]
  CWT_ERR_IM_1,
}
impl From<CWT_ERR_IM_A> for bool {
  #[inline(always)]
  fn from(variant: CWT_ERR_IM_A) -> Self {
    match variant {
      CWT_ERR_IM_A::CWT_ERR_IM_0 => false,
      CWT_ERR_IM_A::CWT_ERR_IM_1 => true,
    }
  }
}
#[doc = "Reader of field `CWT_ERR_IM`"]
pub type CWT_ERR_IM_R = crate::R<bool, CWT_ERR_IM_A>;
impl CWT_ERR_IM_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> CWT_ERR_IM_A {
    match self.bits {
      false => CWT_ERR_IM_A::CWT_ERR_IM_0,
      true => CWT_ERR_IM_A::CWT_ERR_IM_1,
    }
  }
  #[doc = "Checks if the value of the field is `CWT_ERR_IM_0`"]
  #[inline(always)]
  pub fn is_cwt_err_im_0(&self) -> bool {
    *self == CWT_ERR_IM_A::CWT_ERR_IM_0
  }
  #[doc = "Checks if the value of the field is `CWT_ERR_IM_1`"]
  #[inline(always)]
  pub fn is_cwt_err_im_1(&self) -> bool {
    *self == CWT_ERR_IM_A::CWT_ERR_IM_1
  }
}
#[doc = "Write proxy for field `CWT_ERR_IM`"]
pub struct CWT_ERR_IM_W<'a> {
  w: &'a mut W,
}
impl<'a> CWT_ERR_IM_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: CWT_ERR_IM_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "CWT_ERR interrupt enabled"]
  #[inline(always)]
  pub fn cwt_err_im_0(self) -> &'a mut W {
    self.variant(CWT_ERR_IM_A::CWT_ERR_IM_0)
  }
  #[doc = "CWT_ERR interrupt masked (default)"]
  #[inline(always)]
  pub fn cwt_err_im_1(self) -> &'a mut W {
    self.variant(CWT_ERR_IM_A::CWT_ERR_IM_1)
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
#[doc = "Receiver NACK Threshold Interrupt Mask\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RNACK_IM_A {
  #[doc = "0: RTE interrupt enabled"]
  RNACK_IM_0,
  #[doc = "1: RTE interrupt masked (default)"]
  RNACK_IM_1,
}
impl From<RNACK_IM_A> for bool {
  #[inline(always)]
  fn from(variant: RNACK_IM_A) -> Self {
    match variant {
      RNACK_IM_A::RNACK_IM_0 => false,
      RNACK_IM_A::RNACK_IM_1 => true,
    }
  }
}
#[doc = "Reader of field `RNACK_IM`"]
pub type RNACK_IM_R = crate::R<bool, RNACK_IM_A>;
impl RNACK_IM_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> RNACK_IM_A {
    match self.bits {
      false => RNACK_IM_A::RNACK_IM_0,
      true => RNACK_IM_A::RNACK_IM_1,
    }
  }
  #[doc = "Checks if the value of the field is `RNACK_IM_0`"]
  #[inline(always)]
  pub fn is_rnack_im_0(&self) -> bool {
    *self == RNACK_IM_A::RNACK_IM_0
  }
  #[doc = "Checks if the value of the field is `RNACK_IM_1`"]
  #[inline(always)]
  pub fn is_rnack_im_1(&self) -> bool {
    *self == RNACK_IM_A::RNACK_IM_1
  }
}
#[doc = "Write proxy for field `RNACK_IM`"]
pub struct RNACK_IM_W<'a> {
  w: &'a mut W,
}
impl<'a> RNACK_IM_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: RNACK_IM_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "RTE interrupt enabled"]
  #[inline(always)]
  pub fn rnack_im_0(self) -> &'a mut W {
    self.variant(RNACK_IM_A::RNACK_IM_0)
  }
  #[doc = "RTE interrupt masked (default)"]
  #[inline(always)]
  pub fn rnack_im_1(self) -> &'a mut W {
    self.variant(RNACK_IM_A::RNACK_IM_1)
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
#[doc = "Block Wait Time Error Interrupt Mask\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BWT_ERR_IM_A {
  #[doc = "0: BWT_ERR interrupt enabled"]
  BWT_ERR_IM_0,
  #[doc = "1: BWT_ERR interrupt masked (default)"]
  BWT_ERR_IM_1,
}
impl From<BWT_ERR_IM_A> for bool {
  #[inline(always)]
  fn from(variant: BWT_ERR_IM_A) -> Self {
    match variant {
      BWT_ERR_IM_A::BWT_ERR_IM_0 => false,
      BWT_ERR_IM_A::BWT_ERR_IM_1 => true,
    }
  }
}
#[doc = "Reader of field `BWT_ERR_IM`"]
pub type BWT_ERR_IM_R = crate::R<bool, BWT_ERR_IM_A>;
impl BWT_ERR_IM_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> BWT_ERR_IM_A {
    match self.bits {
      false => BWT_ERR_IM_A::BWT_ERR_IM_0,
      true => BWT_ERR_IM_A::BWT_ERR_IM_1,
    }
  }
  #[doc = "Checks if the value of the field is `BWT_ERR_IM_0`"]
  #[inline(always)]
  pub fn is_bwt_err_im_0(&self) -> bool {
    *self == BWT_ERR_IM_A::BWT_ERR_IM_0
  }
  #[doc = "Checks if the value of the field is `BWT_ERR_IM_1`"]
  #[inline(always)]
  pub fn is_bwt_err_im_1(&self) -> bool {
    *self == BWT_ERR_IM_A::BWT_ERR_IM_1
  }
}
#[doc = "Write proxy for field `BWT_ERR_IM`"]
pub struct BWT_ERR_IM_W<'a> {
  w: &'a mut W,
}
impl<'a> BWT_ERR_IM_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: BWT_ERR_IM_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "BWT_ERR interrupt enabled"]
  #[inline(always)]
  pub fn bwt_err_im_0(self) -> &'a mut W {
    self.variant(BWT_ERR_IM_A::BWT_ERR_IM_0)
  }
  #[doc = "BWT_ERR interrupt masked (default)"]
  #[inline(always)]
  pub fn bwt_err_im_1(self) -> &'a mut W {
    self.variant(BWT_ERR_IM_A::BWT_ERR_IM_1)
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
#[doc = "Block Guard Time Error Interrupt\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BGT_ERR_IM_A {
  #[doc = "0: BGT_ERR interrupt enabled"]
  BGT_ERR_IM_0,
  #[doc = "1: BGT_ERR interrupt masked (default)"]
  BGT_ERR_IM_1,
}
impl From<BGT_ERR_IM_A> for bool {
  #[inline(always)]
  fn from(variant: BGT_ERR_IM_A) -> Self {
    match variant {
      BGT_ERR_IM_A::BGT_ERR_IM_0 => false,
      BGT_ERR_IM_A::BGT_ERR_IM_1 => true,
    }
  }
}
#[doc = "Reader of field `BGT_ERR_IM`"]
pub type BGT_ERR_IM_R = crate::R<bool, BGT_ERR_IM_A>;
impl BGT_ERR_IM_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> BGT_ERR_IM_A {
    match self.bits {
      false => BGT_ERR_IM_A::BGT_ERR_IM_0,
      true => BGT_ERR_IM_A::BGT_ERR_IM_1,
    }
  }
  #[doc = "Checks if the value of the field is `BGT_ERR_IM_0`"]
  #[inline(always)]
  pub fn is_bgt_err_im_0(&self) -> bool {
    *self == BGT_ERR_IM_A::BGT_ERR_IM_0
  }
  #[doc = "Checks if the value of the field is `BGT_ERR_IM_1`"]
  #[inline(always)]
  pub fn is_bgt_err_im_1(&self) -> bool {
    *self == BGT_ERR_IM_A::BGT_ERR_IM_1
  }
}
#[doc = "Write proxy for field `BGT_ERR_IM`"]
pub struct BGT_ERR_IM_W<'a> {
  w: &'a mut W,
}
impl<'a> BGT_ERR_IM_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: BGT_ERR_IM_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "BGT_ERR interrupt enabled"]
  #[inline(always)]
  pub fn bgt_err_im_0(self) -> &'a mut W {
    self.variant(BGT_ERR_IM_A::BGT_ERR_IM_0)
  }
  #[doc = "BGT_ERR interrupt masked (default)"]
  #[inline(always)]
  pub fn bgt_err_im_1(self) -> &'a mut W {
    self.variant(BGT_ERR_IM_A::BGT_ERR_IM_1)
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
#[doc = "General Purpose Counter 1 Timeout Interrupt Mask\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPCNT1_IM_A {
  #[doc = "0: GPCNT1_TO interrupt enabled"]
  GPCNT1_IM_0,
  #[doc = "1: GPCNT1_TO interrupt masked (default)"]
  GPCNT1_IM_1,
}
impl From<GPCNT1_IM_A> for bool {
  #[inline(always)]
  fn from(variant: GPCNT1_IM_A) -> Self {
    match variant {
      GPCNT1_IM_A::GPCNT1_IM_0 => false,
      GPCNT1_IM_A::GPCNT1_IM_1 => true,
    }
  }
}
#[doc = "Reader of field `GPCNT1_IM`"]
pub type GPCNT1_IM_R = crate::R<bool, GPCNT1_IM_A>;
impl GPCNT1_IM_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> GPCNT1_IM_A {
    match self.bits {
      false => GPCNT1_IM_A::GPCNT1_IM_0,
      true => GPCNT1_IM_A::GPCNT1_IM_1,
    }
  }
  #[doc = "Checks if the value of the field is `GPCNT1_IM_0`"]
  #[inline(always)]
  pub fn is_gpcnt1_im_0(&self) -> bool {
    *self == GPCNT1_IM_A::GPCNT1_IM_0
  }
  #[doc = "Checks if the value of the field is `GPCNT1_IM_1`"]
  #[inline(always)]
  pub fn is_gpcnt1_im_1(&self) -> bool {
    *self == GPCNT1_IM_A::GPCNT1_IM_1
  }
}
#[doc = "Write proxy for field `GPCNT1_IM`"]
pub struct GPCNT1_IM_W<'a> {
  w: &'a mut W,
}
impl<'a> GPCNT1_IM_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: GPCNT1_IM_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "GPCNT1_TO interrupt enabled"]
  #[inline(always)]
  pub fn gpcnt1_im_0(self) -> &'a mut W {
    self.variant(GPCNT1_IM_A::GPCNT1_IM_0)
  }
  #[doc = "GPCNT1_TO interrupt masked (default)"]
  #[inline(always)]
  pub fn gpcnt1_im_1(self) -> &'a mut W {
    self.variant(GPCNT1_IM_A::GPCNT1_IM_1)
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
#[doc = "Receive Data Interrupt Mask\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_DATA_IM_A {
  #[doc = "0: RX_DATA interrupt enabled"]
  RX_DATA_IM_0,
  #[doc = "1: RX_DATA interrupt masked (default)"]
  RX_DATA_IM_1,
}
impl From<RX_DATA_IM_A> for bool {
  #[inline(always)]
  fn from(variant: RX_DATA_IM_A) -> Self {
    match variant {
      RX_DATA_IM_A::RX_DATA_IM_0 => false,
      RX_DATA_IM_A::RX_DATA_IM_1 => true,
    }
  }
}
#[doc = "Reader of field `RX_DATA_IM`"]
pub type RX_DATA_IM_R = crate::R<bool, RX_DATA_IM_A>;
impl RX_DATA_IM_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> RX_DATA_IM_A {
    match self.bits {
      false => RX_DATA_IM_A::RX_DATA_IM_0,
      true => RX_DATA_IM_A::RX_DATA_IM_1,
    }
  }
  #[doc = "Checks if the value of the field is `RX_DATA_IM_0`"]
  #[inline(always)]
  pub fn is_rx_data_im_0(&self) -> bool {
    *self == RX_DATA_IM_A::RX_DATA_IM_0
  }
  #[doc = "Checks if the value of the field is `RX_DATA_IM_1`"]
  #[inline(always)]
  pub fn is_rx_data_im_1(&self) -> bool {
    *self == RX_DATA_IM_A::RX_DATA_IM_1
  }
}
#[doc = "Write proxy for field `RX_DATA_IM`"]
pub struct RX_DATA_IM_W<'a> {
  w: &'a mut W,
}
impl<'a> RX_DATA_IM_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: RX_DATA_IM_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "RX_DATA interrupt enabled"]
  #[inline(always)]
  pub fn rx_data_im_0(self) -> &'a mut W {
    self.variant(RX_DATA_IM_A::RX_DATA_IM_0)
  }
  #[doc = "RX_DATA interrupt masked (default)"]
  #[inline(always)]
  pub fn rx_data_im_1(self) -> &'a mut W {
    self.variant(RX_DATA_IM_A::RX_DATA_IM_1)
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
#[doc = "Parity Error Interrupt Mask\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEF_IM_A {
  #[doc = "0: PEF interrupt enabled"]
  PEF_IM_0,
  #[doc = "1: PEF interrupt masked (default)"]
  PEF_IM_1,
}
impl From<PEF_IM_A> for bool {
  #[inline(always)]
  fn from(variant: PEF_IM_A) -> Self {
    match variant {
      PEF_IM_A::PEF_IM_0 => false,
      PEF_IM_A::PEF_IM_1 => true,
    }
  }
}
#[doc = "Reader of field `PEF_IM`"]
pub type PEF_IM_R = crate::R<bool, PEF_IM_A>;
impl PEF_IM_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> PEF_IM_A {
    match self.bits {
      false => PEF_IM_A::PEF_IM_0,
      true => PEF_IM_A::PEF_IM_1,
    }
  }
  #[doc = "Checks if the value of the field is `PEF_IM_0`"]
  #[inline(always)]
  pub fn is_pef_im_0(&self) -> bool {
    *self == PEF_IM_A::PEF_IM_0
  }
  #[doc = "Checks if the value of the field is `PEF_IM_1`"]
  #[inline(always)]
  pub fn is_pef_im_1(&self) -> bool {
    *self == PEF_IM_A::PEF_IM_1
  }
}
#[doc = "Write proxy for field `PEF_IM`"]
pub struct PEF_IM_W<'a> {
  w: &'a mut W,
}
impl<'a> PEF_IM_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: PEF_IM_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "PEF interrupt enabled"]
  #[inline(always)]
  pub fn pef_im_0(self) -> &'a mut W {
    self.variant(PEF_IM_A::PEF_IM_0)
  }
  #[doc = "PEF interrupt masked (default)"]
  #[inline(always)]
  pub fn pef_im_1(self) -> &'a mut W {
    self.variant(PEF_IM_A::PEF_IM_1)
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
impl R {
  #[doc = "Bit 0 - Receive Data Threshold Interrupt Mask"]
  #[inline(always)]
  pub fn rdt_im(&self) -> RDT_IM_R {
    RDT_IM_R::new((self.bits & 0x01) != 0)
  }
  #[doc = "Bit 1 - Transmit Complete Interrupt Mask"]
  #[inline(always)]
  pub fn tc_im(&self) -> TC_IM_R {
    TC_IM_R::new(((self.bits >> 1) & 0x01) != 0)
  }
  #[doc = "Bit 2 - Receive FIFO Overflow Interrupt Mask"]
  #[inline(always)]
  pub fn rfo_im(&self) -> RFO_IM_R {
    RFO_IM_R::new(((self.bits >> 2) & 0x01) != 0)
  }
  #[doc = "Bit 3 - Early Transmit Complete Interrupt Mask"]
  #[inline(always)]
  pub fn etc_im(&self) -> ETC_IM_R {
    ETC_IM_R::new(((self.bits >> 3) & 0x01) != 0)
  }
  #[doc = "Bit 4 - Transmit FIFO Empty Interrupt Mask"]
  #[inline(always)]
  pub fn tfe_im(&self) -> TFE_IM_R {
    TFE_IM_R::new(((self.bits >> 4) & 0x01) != 0)
  }
  #[doc = "Bit 5 - Transmit NACK Threshold Interrupt Mask"]
  #[inline(always)]
  pub fn tnack_im(&self) -> TNACK_IM_R {
    TNACK_IM_R::new(((self.bits >> 5) & 0x01) != 0)
  }
  #[doc = "Bit 6 - Transmit FIFO Full Interrupt Mask"]
  #[inline(always)]
  pub fn tff_im(&self) -> TFF_IM_R {
    TFF_IM_R::new(((self.bits >> 6) & 0x01) != 0)
  }
  #[doc = "Bit 7 - Transmit Data Threshold Interrupt Mask"]
  #[inline(always)]
  pub fn tdt_im(&self) -> TDT_IM_R {
    TDT_IM_R::new(((self.bits >> 7) & 0x01) != 0)
  }
  #[doc = "Bit 8 - General Purpose Timer 0 Timeout Interrupt Mask"]
  #[inline(always)]
  pub fn gpcnt0_im(&self) -> GPCNT0_IM_R {
    GPCNT0_IM_R::new(((self.bits >> 8) & 0x01) != 0)
  }
  #[doc = "Bit 9 - Character Wait Time Error Interrupt Mask"]
  #[inline(always)]
  pub fn cwt_err_im(&self) -> CWT_ERR_IM_R {
    CWT_ERR_IM_R::new(((self.bits >> 9) & 0x01) != 0)
  }
  #[doc = "Bit 10 - Receiver NACK Threshold Interrupt Mask"]
  #[inline(always)]
  pub fn rnack_im(&self) -> RNACK_IM_R {
    RNACK_IM_R::new(((self.bits >> 10) & 0x01) != 0)
  }
  #[doc = "Bit 11 - Block Wait Time Error Interrupt Mask"]
  #[inline(always)]
  pub fn bwt_err_im(&self) -> BWT_ERR_IM_R {
    BWT_ERR_IM_R::new(((self.bits >> 11) & 0x01) != 0)
  }
  #[doc = "Bit 12 - Block Guard Time Error Interrupt"]
  #[inline(always)]
  pub fn bgt_err_im(&self) -> BGT_ERR_IM_R {
    BGT_ERR_IM_R::new(((self.bits >> 12) & 0x01) != 0)
  }
  #[doc = "Bit 13 - General Purpose Counter 1 Timeout Interrupt Mask"]
  #[inline(always)]
  pub fn gpcnt1_im(&self) -> GPCNT1_IM_R {
    GPCNT1_IM_R::new(((self.bits >> 13) & 0x01) != 0)
  }
  #[doc = "Bit 14 - Receive Data Interrupt Mask"]
  #[inline(always)]
  pub fn rx_data_im(&self) -> RX_DATA_IM_R {
    RX_DATA_IM_R::new(((self.bits >> 14) & 0x01) != 0)
  }
  #[doc = "Bit 15 - Parity Error Interrupt Mask"]
  #[inline(always)]
  pub fn pef_im(&self) -> PEF_IM_R {
    PEF_IM_R::new(((self.bits >> 15) & 0x01) != 0)
  }
}
impl W {
  #[doc = "Bit 0 - Receive Data Threshold Interrupt Mask"]
  #[inline(always)]
  pub fn rdt_im(&mut self) -> RDT_IM_W {
    RDT_IM_W { w: self }
  }
  #[doc = "Bit 1 - Transmit Complete Interrupt Mask"]
  #[inline(always)]
  pub fn tc_im(&mut self) -> TC_IM_W {
    TC_IM_W { w: self }
  }
  #[doc = "Bit 2 - Receive FIFO Overflow Interrupt Mask"]
  #[inline(always)]
  pub fn rfo_im(&mut self) -> RFO_IM_W {
    RFO_IM_W { w: self }
  }
  #[doc = "Bit 3 - Early Transmit Complete Interrupt Mask"]
  #[inline(always)]
  pub fn etc_im(&mut self) -> ETC_IM_W {
    ETC_IM_W { w: self }
  }
  #[doc = "Bit 4 - Transmit FIFO Empty Interrupt Mask"]
  #[inline(always)]
  pub fn tfe_im(&mut self) -> TFE_IM_W {
    TFE_IM_W { w: self }
  }
  #[doc = "Bit 5 - Transmit NACK Threshold Interrupt Mask"]
  #[inline(always)]
  pub fn tnack_im(&mut self) -> TNACK_IM_W {
    TNACK_IM_W { w: self }
  }
  #[doc = "Bit 6 - Transmit FIFO Full Interrupt Mask"]
  #[inline(always)]
  pub fn tff_im(&mut self) -> TFF_IM_W {
    TFF_IM_W { w: self }
  }
  #[doc = "Bit 7 - Transmit Data Threshold Interrupt Mask"]
  #[inline(always)]
  pub fn tdt_im(&mut self) -> TDT_IM_W {
    TDT_IM_W { w: self }
  }
  #[doc = "Bit 8 - General Purpose Timer 0 Timeout Interrupt Mask"]
  #[inline(always)]
  pub fn gpcnt0_im(&mut self) -> GPCNT0_IM_W {
    GPCNT0_IM_W { w: self }
  }
  #[doc = "Bit 9 - Character Wait Time Error Interrupt Mask"]
  #[inline(always)]
  pub fn cwt_err_im(&mut self) -> CWT_ERR_IM_W {
    CWT_ERR_IM_W { w: self }
  }
  #[doc = "Bit 10 - Receiver NACK Threshold Interrupt Mask"]
  #[inline(always)]
  pub fn rnack_im(&mut self) -> RNACK_IM_W {
    RNACK_IM_W { w: self }
  }
  #[doc = "Bit 11 - Block Wait Time Error Interrupt Mask"]
  #[inline(always)]
  pub fn bwt_err_im(&mut self) -> BWT_ERR_IM_W {
    BWT_ERR_IM_W { w: self }
  }
  #[doc = "Bit 12 - Block Guard Time Error Interrupt"]
  #[inline(always)]
  pub fn bgt_err_im(&mut self) -> BGT_ERR_IM_W {
    BGT_ERR_IM_W { w: self }
  }
  #[doc = "Bit 13 - General Purpose Counter 1 Timeout Interrupt Mask"]
  #[inline(always)]
  pub fn gpcnt1_im(&mut self) -> GPCNT1_IM_W {
    GPCNT1_IM_W { w: self }
  }
  #[doc = "Bit 14 - Receive Data Interrupt Mask"]
  #[inline(always)]
  pub fn rx_data_im(&mut self) -> RX_DATA_IM_W {
    RX_DATA_IM_W { w: self }
  }
  #[doc = "Bit 15 - Parity Error Interrupt Mask"]
  #[inline(always)]
  pub fn pef_im(&mut self) -> PEF_IM_W {
    PEF_IM_W { w: self }
  }
}
