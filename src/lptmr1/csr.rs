#[doc = "Reader of register CSR"]
pub type R = crate::R<u32, super::CSR>;
#[doc = "Writer for register CSR"]
pub type W = crate::W<u32, super::CSR>;
#[doc = "Register CSR `reset()`'s with value 0"]
impl crate::ResetValue for super::CSR {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "Timer Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TEN_A {
  #[doc = "0: LPTMR is disabled and internal logic is reset."]
  TEN_0,
  #[doc = "1: LPTMR is enabled."]
  TEN_1,
}
impl From<TEN_A> for bool {
  #[inline(always)]
  fn from(variant: TEN_A) -> Self {
    match variant {
      TEN_A::TEN_0 => false,
      TEN_A::TEN_1 => true,
    }
  }
}
#[doc = "Reader of field `TEN`"]
pub type TEN_R = crate::R<bool, TEN_A>;
impl TEN_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> TEN_A {
    match self.bits {
      false => TEN_A::TEN_0,
      true => TEN_A::TEN_1,
    }
  }
  #[doc = "Checks if the value of the field is `TEN_0`"]
  #[inline(always)]
  pub fn is_ten_0(&self) -> bool {
    *self == TEN_A::TEN_0
  }
  #[doc = "Checks if the value of the field is `TEN_1`"]
  #[inline(always)]
  pub fn is_ten_1(&self) -> bool {
    *self == TEN_A::TEN_1
  }
}
#[doc = "Write proxy for field `TEN`"]
pub struct TEN_W<'a> {
  w: &'a mut W,
}
impl<'a> TEN_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: TEN_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "LPTMR is disabled and internal logic is reset."]
  #[inline(always)]
  pub fn ten_0(self) -> &'a mut W {
    self.variant(TEN_A::TEN_0)
  }
  #[doc = "LPTMR is enabled."]
  #[inline(always)]
  pub fn ten_1(self) -> &'a mut W {
    self.variant(TEN_A::TEN_1)
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
#[doc = "Timer Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMS_A {
  #[doc = "0: Time Counter mode."]
  TMS_0,
  #[doc = "1: Pulse Counter mode."]
  TMS_1,
}
impl From<TMS_A> for bool {
  #[inline(always)]
  fn from(variant: TMS_A) -> Self {
    match variant {
      TMS_A::TMS_0 => false,
      TMS_A::TMS_1 => true,
    }
  }
}
#[doc = "Reader of field `TMS`"]
pub type TMS_R = crate::R<bool, TMS_A>;
impl TMS_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> TMS_A {
    match self.bits {
      false => TMS_A::TMS_0,
      true => TMS_A::TMS_1,
    }
  }
  #[doc = "Checks if the value of the field is `TMS_0`"]
  #[inline(always)]
  pub fn is_tms_0(&self) -> bool {
    *self == TMS_A::TMS_0
  }
  #[doc = "Checks if the value of the field is `TMS_1`"]
  #[inline(always)]
  pub fn is_tms_1(&self) -> bool {
    *self == TMS_A::TMS_1
  }
}
#[doc = "Write proxy for field `TMS`"]
pub struct TMS_W<'a> {
  w: &'a mut W,
}
impl<'a> TMS_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: TMS_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Time Counter mode."]
  #[inline(always)]
  pub fn tms_0(self) -> &'a mut W {
    self.variant(TMS_A::TMS_0)
  }
  #[doc = "Pulse Counter mode."]
  #[inline(always)]
  pub fn tms_1(self) -> &'a mut W {
    self.variant(TMS_A::TMS_1)
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
#[doc = "Timer Free-Running Counter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TFC_A {
  #[doc = "0: CNR is reset whenever TCF is set."]
  TFC_0,
  #[doc = "1: CNR is reset on overflow."]
  TFC_1,
}
impl From<TFC_A> for bool {
  #[inline(always)]
  fn from(variant: TFC_A) -> Self {
    match variant {
      TFC_A::TFC_0 => false,
      TFC_A::TFC_1 => true,
    }
  }
}
#[doc = "Reader of field `TFC`"]
pub type TFC_R = crate::R<bool, TFC_A>;
impl TFC_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> TFC_A {
    match self.bits {
      false => TFC_A::TFC_0,
      true => TFC_A::TFC_1,
    }
  }
  #[doc = "Checks if the value of the field is `TFC_0`"]
  #[inline(always)]
  pub fn is_tfc_0(&self) -> bool {
    *self == TFC_A::TFC_0
  }
  #[doc = "Checks if the value of the field is `TFC_1`"]
  #[inline(always)]
  pub fn is_tfc_1(&self) -> bool {
    *self == TFC_A::TFC_1
  }
}
#[doc = "Write proxy for field `TFC`"]
pub struct TFC_W<'a> {
  w: &'a mut W,
}
impl<'a> TFC_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: TFC_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "CNR is reset whenever TCF is set."]
  #[inline(always)]
  pub fn tfc_0(self) -> &'a mut W {
    self.variant(TFC_A::TFC_0)
  }
  #[doc = "CNR is reset on overflow."]
  #[inline(always)]
  pub fn tfc_1(self) -> &'a mut W {
    self.variant(TFC_A::TFC_1)
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
#[doc = "Timer Pin Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TPP_A {
  #[doc = "0: Pulse Counter input source is active-high, and the CNR will increment on the rising-edge."]
  TPP_0,
  #[doc = "1: Pulse Counter input source is active-low, and the CNR will increment on the falling-edge."]
  TPP_1,
}
impl From<TPP_A> for bool {
  #[inline(always)]
  fn from(variant: TPP_A) -> Self {
    match variant {
      TPP_A::TPP_0 => false,
      TPP_A::TPP_1 => true,
    }
  }
}
#[doc = "Reader of field `TPP`"]
pub type TPP_R = crate::R<bool, TPP_A>;
impl TPP_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> TPP_A {
    match self.bits {
      false => TPP_A::TPP_0,
      true => TPP_A::TPP_1,
    }
  }
  #[doc = "Checks if the value of the field is `TPP_0`"]
  #[inline(always)]
  pub fn is_tpp_0(&self) -> bool {
    *self == TPP_A::TPP_0
  }
  #[doc = "Checks if the value of the field is `TPP_1`"]
  #[inline(always)]
  pub fn is_tpp_1(&self) -> bool {
    *self == TPP_A::TPP_1
  }
}
#[doc = "Write proxy for field `TPP`"]
pub struct TPP_W<'a> {
  w: &'a mut W,
}
impl<'a> TPP_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: TPP_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Pulse Counter input source is active-high, and the CNR will increment on the rising-edge."]
  #[inline(always)]
  pub fn tpp_0(self) -> &'a mut W {
    self.variant(TPP_A::TPP_0)
  }
  #[doc = "Pulse Counter input source is active-low, and the CNR will increment on the falling-edge."]
  #[inline(always)]
  pub fn tpp_1(self) -> &'a mut W {
    self.variant(TPP_A::TPP_1)
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
#[doc = "Timer Pin Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TPS_A {
  #[doc = "0: Pulse counter input 0 is selected."]
  TPS_0,
  #[doc = "1: Pulse counter input 1 is selected."]
  TPS_1,
  #[doc = "2: Pulse counter input 2 is selected."]
  TPS_2,
  #[doc = "3: Pulse counter input 3 is selected."]
  TPS_3,
}
impl From<TPS_A> for u8 {
  #[inline(always)]
  fn from(variant: TPS_A) -> Self {
    match variant {
      TPS_A::TPS_0 => 0,
      TPS_A::TPS_1 => 1,
      TPS_A::TPS_2 => 2,
      TPS_A::TPS_3 => 3,
    }
  }
}
#[doc = "Reader of field `TPS`"]
pub type TPS_R = crate::R<u8, TPS_A>;
impl TPS_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> TPS_A {
    match self.bits {
      0 => TPS_A::TPS_0,
      1 => TPS_A::TPS_1,
      2 => TPS_A::TPS_2,
      3 => TPS_A::TPS_3,
      _ => unreachable!(),
    }
  }
  #[doc = "Checks if the value of the field is `TPS_0`"]
  #[inline(always)]
  pub fn is_tps_0(&self) -> bool {
    *self == TPS_A::TPS_0
  }
  #[doc = "Checks if the value of the field is `TPS_1`"]
  #[inline(always)]
  pub fn is_tps_1(&self) -> bool {
    *self == TPS_A::TPS_1
  }
  #[doc = "Checks if the value of the field is `TPS_2`"]
  #[inline(always)]
  pub fn is_tps_2(&self) -> bool {
    *self == TPS_A::TPS_2
  }
  #[doc = "Checks if the value of the field is `TPS_3`"]
  #[inline(always)]
  pub fn is_tps_3(&self) -> bool {
    *self == TPS_A::TPS_3
  }
}
#[doc = "Write proxy for field `TPS`"]
pub struct TPS_W<'a> {
  w: &'a mut W,
}
impl<'a> TPS_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: TPS_A) -> &'a mut W {
    {
      self.bits(variant.into())
    }
  }
  #[doc = "Pulse counter input 0 is selected."]
  #[inline(always)]
  pub fn tps_0(self) -> &'a mut W {
    self.variant(TPS_A::TPS_0)
  }
  #[doc = "Pulse counter input 1 is selected."]
  #[inline(always)]
  pub fn tps_1(self) -> &'a mut W {
    self.variant(TPS_A::TPS_1)
  }
  #[doc = "Pulse counter input 2 is selected."]
  #[inline(always)]
  pub fn tps_2(self) -> &'a mut W {
    self.variant(TPS_A::TPS_2)
  }
  #[doc = "Pulse counter input 3 is selected."]
  #[inline(always)]
  pub fn tps_3(self) -> &'a mut W {
    self.variant(TPS_A::TPS_3)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
    self.w
  }
}
#[doc = "Timer Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIE_A {
  #[doc = "0: Timer interrupt disabled."]
  TIE_0,
  #[doc = "1: Timer interrupt enabled."]
  TIE_1,
}
impl From<TIE_A> for bool {
  #[inline(always)]
  fn from(variant: TIE_A) -> Self {
    match variant {
      TIE_A::TIE_0 => false,
      TIE_A::TIE_1 => true,
    }
  }
}
#[doc = "Reader of field `TIE`"]
pub type TIE_R = crate::R<bool, TIE_A>;
impl TIE_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> TIE_A {
    match self.bits {
      false => TIE_A::TIE_0,
      true => TIE_A::TIE_1,
    }
  }
  #[doc = "Checks if the value of the field is `TIE_0`"]
  #[inline(always)]
  pub fn is_tie_0(&self) -> bool {
    *self == TIE_A::TIE_0
  }
  #[doc = "Checks if the value of the field is `TIE_1`"]
  #[inline(always)]
  pub fn is_tie_1(&self) -> bool {
    *self == TIE_A::TIE_1
  }
}
#[doc = "Write proxy for field `TIE`"]
pub struct TIE_W<'a> {
  w: &'a mut W,
}
impl<'a> TIE_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: TIE_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Timer interrupt disabled."]
  #[inline(always)]
  pub fn tie_0(self) -> &'a mut W {
    self.variant(TIE_A::TIE_0)
  }
  #[doc = "Timer interrupt enabled."]
  #[inline(always)]
  pub fn tie_1(self) -> &'a mut W {
    self.variant(TIE_A::TIE_1)
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
#[doc = "Timer Compare Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCF_A {
  #[doc = "0: The value of CNR is not equal to CMR and increments."]
  TCF_0,
  #[doc = "1: The value of CNR is equal to CMR and increments."]
  TCF_1,
}
impl From<TCF_A> for bool {
  #[inline(always)]
  fn from(variant: TCF_A) -> Self {
    match variant {
      TCF_A::TCF_0 => false,
      TCF_A::TCF_1 => true,
    }
  }
}
#[doc = "Reader of field `TCF`"]
pub type TCF_R = crate::R<bool, TCF_A>;
impl TCF_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> TCF_A {
    match self.bits {
      false => TCF_A::TCF_0,
      true => TCF_A::TCF_1,
    }
  }
  #[doc = "Checks if the value of the field is `TCF_0`"]
  #[inline(always)]
  pub fn is_tcf_0(&self) -> bool {
    *self == TCF_A::TCF_0
  }
  #[doc = "Checks if the value of the field is `TCF_1`"]
  #[inline(always)]
  pub fn is_tcf_1(&self) -> bool {
    *self == TCF_A::TCF_1
  }
}
#[doc = "Write proxy for field `TCF`"]
pub struct TCF_W<'a> {
  w: &'a mut W,
}
impl<'a> TCF_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: TCF_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "The value of CNR is not equal to CMR and increments."]
  #[inline(always)]
  pub fn tcf_0(self) -> &'a mut W {
    self.variant(TCF_A::TCF_0)
  }
  #[doc = "The value of CNR is equal to CMR and increments."]
  #[inline(always)]
  pub fn tcf_1(self) -> &'a mut W {
    self.variant(TCF_A::TCF_1)
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
#[doc = "Timer DMA Request Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TDRE_A {
  #[doc = "0: Timer DMA Request disabled."]
  TDRE_0,
  #[doc = "1: Timer DMA Request enabled."]
  TDRE_1,
}
impl From<TDRE_A> for bool {
  #[inline(always)]
  fn from(variant: TDRE_A) -> Self {
    match variant {
      TDRE_A::TDRE_0 => false,
      TDRE_A::TDRE_1 => true,
    }
  }
}
#[doc = "Reader of field `TDRE`"]
pub type TDRE_R = crate::R<bool, TDRE_A>;
impl TDRE_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> TDRE_A {
    match self.bits {
      false => TDRE_A::TDRE_0,
      true => TDRE_A::TDRE_1,
    }
  }
  #[doc = "Checks if the value of the field is `TDRE_0`"]
  #[inline(always)]
  pub fn is_tdre_0(&self) -> bool {
    *self == TDRE_A::TDRE_0
  }
  #[doc = "Checks if the value of the field is `TDRE_1`"]
  #[inline(always)]
  pub fn is_tdre_1(&self) -> bool {
    *self == TDRE_A::TDRE_1
  }
}
#[doc = "Write proxy for field `TDRE`"]
pub struct TDRE_W<'a> {
  w: &'a mut W,
}
impl<'a> TDRE_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: TDRE_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Timer DMA Request disabled."]
  #[inline(always)]
  pub fn tdre_0(self) -> &'a mut W {
    self.variant(TDRE_A::TDRE_0)
  }
  #[doc = "Timer DMA Request enabled."]
  #[inline(always)]
  pub fn tdre_1(self) -> &'a mut W {
    self.variant(TDRE_A::TDRE_1)
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
impl R {
  #[doc = "Bit 0 - Timer Enable"]
  #[inline(always)]
  pub fn ten(&self) -> TEN_R {
    TEN_R::new((self.bits & 0x01) != 0)
  }
  #[doc = "Bit 1 - Timer Mode Select"]
  #[inline(always)]
  pub fn tms(&self) -> TMS_R {
    TMS_R::new(((self.bits >> 1) & 0x01) != 0)
  }
  #[doc = "Bit 2 - Timer Free-Running Counter"]
  #[inline(always)]
  pub fn tfc(&self) -> TFC_R {
    TFC_R::new(((self.bits >> 2) & 0x01) != 0)
  }
  #[doc = "Bit 3 - Timer Pin Polarity"]
  #[inline(always)]
  pub fn tpp(&self) -> TPP_R {
    TPP_R::new(((self.bits >> 3) & 0x01) != 0)
  }
  #[doc = "Bits 4:5 - Timer Pin Select"]
  #[inline(always)]
  pub fn tps(&self) -> TPS_R {
    TPS_R::new(((self.bits >> 4) & 0x03) as u8)
  }
  #[doc = "Bit 6 - Timer Interrupt Enable"]
  #[inline(always)]
  pub fn tie(&self) -> TIE_R {
    TIE_R::new(((self.bits >> 6) & 0x01) != 0)
  }
  #[doc = "Bit 7 - Timer Compare Flag"]
  #[inline(always)]
  pub fn tcf(&self) -> TCF_R {
    TCF_R::new(((self.bits >> 7) & 0x01) != 0)
  }
  #[doc = "Bit 8 - Timer DMA Request Enable"]
  #[inline(always)]
  pub fn tdre(&self) -> TDRE_R {
    TDRE_R::new(((self.bits >> 8) & 0x01) != 0)
  }
}
impl W {
  #[doc = "Bit 0 - Timer Enable"]
  #[inline(always)]
  pub fn ten(&mut self) -> TEN_W {
    TEN_W { w: self }
  }
  #[doc = "Bit 1 - Timer Mode Select"]
  #[inline(always)]
  pub fn tms(&mut self) -> TMS_W {
    TMS_W { w: self }
  }
  #[doc = "Bit 2 - Timer Free-Running Counter"]
  #[inline(always)]
  pub fn tfc(&mut self) -> TFC_W {
    TFC_W { w: self }
  }
  #[doc = "Bit 3 - Timer Pin Polarity"]
  #[inline(always)]
  pub fn tpp(&mut self) -> TPP_W {
    TPP_W { w: self }
  }
  #[doc = "Bits 4:5 - Timer Pin Select"]
  #[inline(always)]
  pub fn tps(&mut self) -> TPS_W {
    TPS_W { w: self }
  }
  #[doc = "Bit 6 - Timer Interrupt Enable"]
  #[inline(always)]
  pub fn tie(&mut self) -> TIE_W {
    TIE_W { w: self }
  }
  #[doc = "Bit 7 - Timer Compare Flag"]
  #[inline(always)]
  pub fn tcf(&mut self) -> TCF_W {
    TCF_W { w: self }
  }
  #[doc = "Bit 8 - Timer DMA Request Enable"]
  #[inline(always)]
  pub fn tdre(&mut self) -> TDRE_W {
    TDRE_W { w: self }
  }
}
