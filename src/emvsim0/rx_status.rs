#[doc = "Reader of register RX_STATUS"]
pub type R = crate::R<u32, super::RX_STATUS>;
#[doc = "Writer for register RX_STATUS"]
pub type W = crate::W<u32, super::RX_STATUS>;
#[doc = "Register RX_STATUS `reset()`'s with value 0"]
impl crate::ResetValue for super::RX_STATUS {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "Receive FIFO Overflow Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RFO_A {
  #[doc = "0: No overrun error has occurred (default)"]
  RFO_0,
  #[doc = "1: A byte was received when the received FIFO was already full"]
  RFO_1,
}
impl From<RFO_A> for bool {
  #[inline(always)]
  fn from(variant: RFO_A) -> Self {
    match variant {
      RFO_A::RFO_0 => false,
      RFO_A::RFO_1 => true,
    }
  }
}
#[doc = "Reader of field `RFO`"]
pub type RFO_R = crate::R<bool, RFO_A>;
impl RFO_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> RFO_A {
    match self.bits {
      false => RFO_A::RFO_0,
      true => RFO_A::RFO_1,
    }
  }
  #[doc = "Checks if the value of the field is `RFO_0`"]
  #[inline(always)]
  pub fn is_rfo_0(&self) -> bool {
    *self == RFO_A::RFO_0
  }
  #[doc = "Checks if the value of the field is `RFO_1`"]
  #[inline(always)]
  pub fn is_rfo_1(&self) -> bool {
    *self == RFO_A::RFO_1
  }
}
#[doc = "Write proxy for field `RFO`"]
pub struct RFO_W<'a> {
  w: &'a mut W,
}
impl<'a> RFO_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: RFO_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "No overrun error has occurred (default)"]
  #[inline(always)]
  pub fn rfo_0(self) -> &'a mut W {
    self.variant(RFO_A::RFO_0)
  }
  #[doc = "A byte was received when the received FIFO was already full"]
  #[inline(always)]
  pub fn rfo_1(self) -> &'a mut W {
    self.variant(RFO_A::RFO_1)
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
#[doc = "Receive Data Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_DATA_A {
  #[doc = "0: No new byte is received"]
  RX_DATA_0,
  #[doc = "1: New byte is received ans stored in Receive FIFO"]
  RX_DATA_1,
}
impl From<RX_DATA_A> for bool {
  #[inline(always)]
  fn from(variant: RX_DATA_A) -> Self {
    match variant {
      RX_DATA_A::RX_DATA_0 => false,
      RX_DATA_A::RX_DATA_1 => true,
    }
  }
}
#[doc = "Reader of field `RX_DATA`"]
pub type RX_DATA_R = crate::R<bool, RX_DATA_A>;
impl RX_DATA_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> RX_DATA_A {
    match self.bits {
      false => RX_DATA_A::RX_DATA_0,
      true => RX_DATA_A::RX_DATA_1,
    }
  }
  #[doc = "Checks if the value of the field is `RX_DATA_0`"]
  #[inline(always)]
  pub fn is_rx_data_0(&self) -> bool {
    *self == RX_DATA_A::RX_DATA_0
  }
  #[doc = "Checks if the value of the field is `RX_DATA_1`"]
  #[inline(always)]
  pub fn is_rx_data_1(&self) -> bool {
    *self == RX_DATA_A::RX_DATA_1
  }
}
#[doc = "Write proxy for field `RX_DATA`"]
pub struct RX_DATA_W<'a> {
  w: &'a mut W,
}
impl<'a> RX_DATA_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: RX_DATA_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "No new byte is received"]
  #[inline(always)]
  pub fn rx_data_0(self) -> &'a mut W {
    self.variant(RX_DATA_A::RX_DATA_0)
  }
  #[doc = "New byte is received ans stored in Receive FIFO"]
  #[inline(always)]
  pub fn rx_data_1(self) -> &'a mut W {
    self.variant(RX_DATA_A::RX_DATA_1)
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
#[doc = "Receive Data Threshold Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDTF_A {
  #[doc = "0: Number of unread bytes in receive FIFO less than the value set by RDT\\[3:0\\] (default)."]
  RDTF_0,
  #[doc = "1: Number of unread bytes in receive FIFO greater or than equal to value set by RDT\\[3:0\\]."]
  RDTF_1,
}
impl From<RDTF_A> for bool {
  #[inline(always)]
  fn from(variant: RDTF_A) -> Self {
    match variant {
      RDTF_A::RDTF_0 => false,
      RDTF_A::RDTF_1 => true,
    }
  }
}
#[doc = "Reader of field `RDTF`"]
pub type RDTF_R = crate::R<bool, RDTF_A>;
impl RDTF_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> RDTF_A {
    match self.bits {
      false => RDTF_A::RDTF_0,
      true => RDTF_A::RDTF_1,
    }
  }
  #[doc = "Checks if the value of the field is `RDTF_0`"]
  #[inline(always)]
  pub fn is_rdtf_0(&self) -> bool {
    *self == RDTF_A::RDTF_0
  }
  #[doc = "Checks if the value of the field is `RDTF_1`"]
  #[inline(always)]
  pub fn is_rdtf_1(&self) -> bool {
    *self == RDTF_A::RDTF_1
  }
}
#[doc = "LRC Check OK Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LRC_OK_A {
  #[doc = "0: Current LRC value does not match remainder."]
  LRC_OK_0,
  #[doc = "1: Current calculated LRC value matches the expected result (i.e. zero)."]
  LRC_OK_1,
}
impl From<LRC_OK_A> for bool {
  #[inline(always)]
  fn from(variant: LRC_OK_A) -> Self {
    match variant {
      LRC_OK_A::LRC_OK_0 => false,
      LRC_OK_A::LRC_OK_1 => true,
    }
  }
}
#[doc = "Reader of field `LRC_OK`"]
pub type LRC_OK_R = crate::R<bool, LRC_OK_A>;
impl LRC_OK_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> LRC_OK_A {
    match self.bits {
      false => LRC_OK_A::LRC_OK_0,
      true => LRC_OK_A::LRC_OK_1,
    }
  }
  #[doc = "Checks if the value of the field is `LRC_OK_0`"]
  #[inline(always)]
  pub fn is_lrc_ok_0(&self) -> bool {
    *self == LRC_OK_A::LRC_OK_0
  }
  #[doc = "Checks if the value of the field is `LRC_OK_1`"]
  #[inline(always)]
  pub fn is_lrc_ok_1(&self) -> bool {
    *self == LRC_OK_A::LRC_OK_1
  }
}
#[doc = "CRC Check OK Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRC_OK_A {
  #[doc = "0: Current CRC value does not match remainder."]
  CRC_OK_0,
  #[doc = "1: Current calculated CRC value matches the expected result."]
  CRC_OK_1,
}
impl From<CRC_OK_A> for bool {
  #[inline(always)]
  fn from(variant: CRC_OK_A) -> Self {
    match variant {
      CRC_OK_A::CRC_OK_0 => false,
      CRC_OK_A::CRC_OK_1 => true,
    }
  }
}
#[doc = "Reader of field `CRC_OK`"]
pub type CRC_OK_R = crate::R<bool, CRC_OK_A>;
impl CRC_OK_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> CRC_OK_A {
    match self.bits {
      false => CRC_OK_A::CRC_OK_0,
      true => CRC_OK_A::CRC_OK_1,
    }
  }
  #[doc = "Checks if the value of the field is `CRC_OK_0`"]
  #[inline(always)]
  pub fn is_crc_ok_0(&self) -> bool {
    *self == CRC_OK_A::CRC_OK_0
  }
  #[doc = "Checks if the value of the field is `CRC_OK_1`"]
  #[inline(always)]
  pub fn is_crc_ok_1(&self) -> bool {
    *self == CRC_OK_A::CRC_OK_1
  }
}
#[doc = "Character Wait Time Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CWT_ERR_A {
  #[doc = "0: No CWT violation has occurred (default)."]
  CWT_ERR_0,
  #[doc = "1: Time between two consecutive characters has exceeded the value in CHAR_WAIT."]
  CWT_ERR_1,
}
impl From<CWT_ERR_A> for bool {
  #[inline(always)]
  fn from(variant: CWT_ERR_A) -> Self {
    match variant {
      CWT_ERR_A::CWT_ERR_0 => false,
      CWT_ERR_A::CWT_ERR_1 => true,
    }
  }
}
#[doc = "Reader of field `CWT_ERR`"]
pub type CWT_ERR_R = crate::R<bool, CWT_ERR_A>;
impl CWT_ERR_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> CWT_ERR_A {
    match self.bits {
      false => CWT_ERR_A::CWT_ERR_0,
      true => CWT_ERR_A::CWT_ERR_1,
    }
  }
  #[doc = "Checks if the value of the field is `CWT_ERR_0`"]
  #[inline(always)]
  pub fn is_cwt_err_0(&self) -> bool {
    *self == CWT_ERR_A::CWT_ERR_0
  }
  #[doc = "Checks if the value of the field is `CWT_ERR_1`"]
  #[inline(always)]
  pub fn is_cwt_err_1(&self) -> bool {
    *self == CWT_ERR_A::CWT_ERR_1
  }
}
#[doc = "Write proxy for field `CWT_ERR`"]
pub struct CWT_ERR_W<'a> {
  w: &'a mut W,
}
impl<'a> CWT_ERR_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: CWT_ERR_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "No CWT violation has occurred (default)."]
  #[inline(always)]
  pub fn cwt_err_0(self) -> &'a mut W {
    self.variant(CWT_ERR_A::CWT_ERR_0)
  }
  #[doc = "Time between two consecutive characters has exceeded the value in CHAR_WAIT."]
  #[inline(always)]
  pub fn cwt_err_1(self) -> &'a mut W {
    self.variant(CWT_ERR_A::CWT_ERR_1)
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
#[doc = "Received NACK Threshold Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTE_A {
  #[doc = "0: Number of NACKs generated by the receiver is less than the value programmed in RTH\\[3:0\\]"]
  RTE_0,
  #[doc = "1: Number of NACKs generated by the receiver is equal to the value programmed in RTH\\[3:0\\]"]
  RTE_1,
}
impl From<RTE_A> for bool {
  #[inline(always)]
  fn from(variant: RTE_A) -> Self {
    match variant {
      RTE_A::RTE_0 => false,
      RTE_A::RTE_1 => true,
    }
  }
}
#[doc = "Reader of field `RTE`"]
pub type RTE_R = crate::R<bool, RTE_A>;
impl RTE_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> RTE_A {
    match self.bits {
      false => RTE_A::RTE_0,
      true => RTE_A::RTE_1,
    }
  }
  #[doc = "Checks if the value of the field is `RTE_0`"]
  #[inline(always)]
  pub fn is_rte_0(&self) -> bool {
    *self == RTE_A::RTE_0
  }
  #[doc = "Checks if the value of the field is `RTE_1`"]
  #[inline(always)]
  pub fn is_rte_1(&self) -> bool {
    *self == RTE_A::RTE_1
  }
}
#[doc = "Write proxy for field `RTE`"]
pub struct RTE_W<'a> {
  w: &'a mut W,
}
impl<'a> RTE_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: RTE_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Number of NACKs generated by the receiver is less than the value programmed in RTH\\[3:0\\]"]
  #[inline(always)]
  pub fn rte_0(self) -> &'a mut W {
    self.variant(RTE_A::RTE_0)
  }
  #[doc = "Number of NACKs generated by the receiver is equal to the value programmed in RTH\\[3:0\\]"]
  #[inline(always)]
  pub fn rte_1(self) -> &'a mut W {
    self.variant(RTE_A::RTE_1)
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
#[doc = "Block Wait Time Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BWT_ERR_A {
  #[doc = "0: Block wait time not exceeded"]
  BWT_ERR_0,
  #[doc = "1: Block wait time was exceeded"]
  BWT_ERR_1,
}
impl From<BWT_ERR_A> for bool {
  #[inline(always)]
  fn from(variant: BWT_ERR_A) -> Self {
    match variant {
      BWT_ERR_A::BWT_ERR_0 => false,
      BWT_ERR_A::BWT_ERR_1 => true,
    }
  }
}
#[doc = "Reader of field `BWT_ERR`"]
pub type BWT_ERR_R = crate::R<bool, BWT_ERR_A>;
impl BWT_ERR_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> BWT_ERR_A {
    match self.bits {
      false => BWT_ERR_A::BWT_ERR_0,
      true => BWT_ERR_A::BWT_ERR_1,
    }
  }
  #[doc = "Checks if the value of the field is `BWT_ERR_0`"]
  #[inline(always)]
  pub fn is_bwt_err_0(&self) -> bool {
    *self == BWT_ERR_A::BWT_ERR_0
  }
  #[doc = "Checks if the value of the field is `BWT_ERR_1`"]
  #[inline(always)]
  pub fn is_bwt_err_1(&self) -> bool {
    *self == BWT_ERR_A::BWT_ERR_1
  }
}
#[doc = "Write proxy for field `BWT_ERR`"]
pub struct BWT_ERR_W<'a> {
  w: &'a mut W,
}
impl<'a> BWT_ERR_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: BWT_ERR_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Block wait time not exceeded"]
  #[inline(always)]
  pub fn bwt_err_0(self) -> &'a mut W {
    self.variant(BWT_ERR_A::BWT_ERR_0)
  }
  #[doc = "Block wait time was exceeded"]
  #[inline(always)]
  pub fn bwt_err_1(self) -> &'a mut W {
    self.variant(BWT_ERR_A::BWT_ERR_1)
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
#[doc = "Block Guard Time Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BGT_ERR_A {
  #[doc = "0: Block guard time was sufficient"]
  BGT_ERR_0,
  #[doc = "1: Block guard time was too small"]
  BGT_ERR_1,
}
impl From<BGT_ERR_A> for bool {
  #[inline(always)]
  fn from(variant: BGT_ERR_A) -> Self {
    match variant {
      BGT_ERR_A::BGT_ERR_0 => false,
      BGT_ERR_A::BGT_ERR_1 => true,
    }
  }
}
#[doc = "Reader of field `BGT_ERR`"]
pub type BGT_ERR_R = crate::R<bool, BGT_ERR_A>;
impl BGT_ERR_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> BGT_ERR_A {
    match self.bits {
      false => BGT_ERR_A::BGT_ERR_0,
      true => BGT_ERR_A::BGT_ERR_1,
    }
  }
  #[doc = "Checks if the value of the field is `BGT_ERR_0`"]
  #[inline(always)]
  pub fn is_bgt_err_0(&self) -> bool {
    *self == BGT_ERR_A::BGT_ERR_0
  }
  #[doc = "Checks if the value of the field is `BGT_ERR_1`"]
  #[inline(always)]
  pub fn is_bgt_err_1(&self) -> bool {
    *self == BGT_ERR_A::BGT_ERR_1
  }
}
#[doc = "Write proxy for field `BGT_ERR`"]
pub struct BGT_ERR_W<'a> {
  w: &'a mut W,
}
impl<'a> BGT_ERR_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: BGT_ERR_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Block guard time was sufficient"]
  #[inline(always)]
  pub fn bgt_err_0(self) -> &'a mut W {
    self.variant(BGT_ERR_A::BGT_ERR_0)
  }
  #[doc = "Block guard time was too small"]
  #[inline(always)]
  pub fn bgt_err_1(self) -> &'a mut W {
    self.variant(BGT_ERR_A::BGT_ERR_1)
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
#[doc = "Parity Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEF_A {
  #[doc = "0: No parity error detected"]
  PEF_0,
  #[doc = "1: Parity error detected"]
  PEF_1,
}
impl From<PEF_A> for bool {
  #[inline(always)]
  fn from(variant: PEF_A) -> Self {
    match variant {
      PEF_A::PEF_0 => false,
      PEF_A::PEF_1 => true,
    }
  }
}
#[doc = "Reader of field `PEF`"]
pub type PEF_R = crate::R<bool, PEF_A>;
impl PEF_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> PEF_A {
    match self.bits {
      false => PEF_A::PEF_0,
      true => PEF_A::PEF_1,
    }
  }
  #[doc = "Checks if the value of the field is `PEF_0`"]
  #[inline(always)]
  pub fn is_pef_0(&self) -> bool {
    *self == PEF_A::PEF_0
  }
  #[doc = "Checks if the value of the field is `PEF_1`"]
  #[inline(always)]
  pub fn is_pef_1(&self) -> bool {
    *self == PEF_A::PEF_1
  }
}
#[doc = "Write proxy for field `PEF`"]
pub struct PEF_W<'a> {
  w: &'a mut W,
}
impl<'a> PEF_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: PEF_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "No parity error detected"]
  #[inline(always)]
  pub fn pef_0(self) -> &'a mut W {
    self.variant(PEF_A::PEF_0)
  }
  #[doc = "Parity error detected"]
  #[inline(always)]
  pub fn pef_1(self) -> &'a mut W {
    self.variant(PEF_A::PEF_1)
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
#[doc = "Frame Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FEF_A {
  #[doc = "0: No frame error detected"]
  FEF_0,
  #[doc = "1: Frame error detected"]
  FEF_1,
}
impl From<FEF_A> for bool {
  #[inline(always)]
  fn from(variant: FEF_A) -> Self {
    match variant {
      FEF_A::FEF_0 => false,
      FEF_A::FEF_1 => true,
    }
  }
}
#[doc = "Reader of field `FEF`"]
pub type FEF_R = crate::R<bool, FEF_A>;
impl FEF_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> FEF_A {
    match self.bits {
      false => FEF_A::FEF_0,
      true => FEF_A::FEF_1,
    }
  }
  #[doc = "Checks if the value of the field is `FEF_0`"]
  #[inline(always)]
  pub fn is_fef_0(&self) -> bool {
    *self == FEF_A::FEF_0
  }
  #[doc = "Checks if the value of the field is `FEF_1`"]
  #[inline(always)]
  pub fn is_fef_1(&self) -> bool {
    *self == FEF_A::FEF_1
  }
}
#[doc = "Write proxy for field `FEF`"]
pub struct FEF_W<'a> {
  w: &'a mut W,
}
impl<'a> FEF_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: FEF_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "No frame error detected"]
  #[inline(always)]
  pub fn fef_0(self) -> &'a mut W {
    self.variant(FEF_A::FEF_0)
  }
  #[doc = "Frame error detected"]
  #[inline(always)]
  pub fn fef_1(self) -> &'a mut W {
    self.variant(FEF_A::FEF_1)
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
#[doc = "Reader of field `RX_WPTR`"]
pub type RX_WPTR_R = crate::R<u8, u8>;
#[doc = "Receive FIFO Byte Count\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_CNT_A {
  #[doc = "0: FIFO is emtpy"]
  RX_CNT_0,
}
impl From<RX_CNT_A> for u8 {
  #[inline(always)]
  fn from(variant: RX_CNT_A) -> Self {
    match variant {
      RX_CNT_A::RX_CNT_0 => 0,
    }
  }
}
#[doc = "Reader of field `RX_CNT`"]
pub type RX_CNT_R = crate::R<u8, RX_CNT_A>;
impl RX_CNT_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> crate::Variant<u8, RX_CNT_A> {
    use crate::Variant::*;
    match self.bits {
      0 => Val(RX_CNT_A::RX_CNT_0),
      i => Res(i),
    }
  }
  #[doc = "Checks if the value of the field is `RX_CNT_0`"]
  #[inline(always)]
  pub fn is_rx_cnt_0(&self) -> bool {
    *self == RX_CNT_A::RX_CNT_0
  }
}
impl R {
  #[doc = "Bit 0 - Receive FIFO Overflow Flag"]
  #[inline(always)]
  pub fn rfo(&self) -> RFO_R {
    RFO_R::new((self.bits & 0x01) != 0)
  }
  #[doc = "Bit 4 - Receive Data Interrupt Flag"]
  #[inline(always)]
  pub fn rx_data(&self) -> RX_DATA_R {
    RX_DATA_R::new(((self.bits >> 4) & 0x01) != 0)
  }
  #[doc = "Bit 5 - Receive Data Threshold Interrupt Flag"]
  #[inline(always)]
  pub fn rdtf(&self) -> RDTF_R {
    RDTF_R::new(((self.bits >> 5) & 0x01) != 0)
  }
  #[doc = "Bit 6 - LRC Check OK Flag"]
  #[inline(always)]
  pub fn lrc_ok(&self) -> LRC_OK_R {
    LRC_OK_R::new(((self.bits >> 6) & 0x01) != 0)
  }
  #[doc = "Bit 7 - CRC Check OK Flag"]
  #[inline(always)]
  pub fn crc_ok(&self) -> CRC_OK_R {
    CRC_OK_R::new(((self.bits >> 7) & 0x01) != 0)
  }
  #[doc = "Bit 8 - Character Wait Time Error Flag"]
  #[inline(always)]
  pub fn cwt_err(&self) -> CWT_ERR_R {
    CWT_ERR_R::new(((self.bits >> 8) & 0x01) != 0)
  }
  #[doc = "Bit 9 - Received NACK Threshold Error Flag"]
  #[inline(always)]
  pub fn rte(&self) -> RTE_R {
    RTE_R::new(((self.bits >> 9) & 0x01) != 0)
  }
  #[doc = "Bit 10 - Block Wait Time Error Flag"]
  #[inline(always)]
  pub fn bwt_err(&self) -> BWT_ERR_R {
    BWT_ERR_R::new(((self.bits >> 10) & 0x01) != 0)
  }
  #[doc = "Bit 11 - Block Guard Time Error Flag"]
  #[inline(always)]
  pub fn bgt_err(&self) -> BGT_ERR_R {
    BGT_ERR_R::new(((self.bits >> 11) & 0x01) != 0)
  }
  #[doc = "Bit 12 - Parity Error Flag"]
  #[inline(always)]
  pub fn pef(&self) -> PEF_R {
    PEF_R::new(((self.bits >> 12) & 0x01) != 0)
  }
  #[doc = "Bit 13 - Frame Error Flag"]
  #[inline(always)]
  pub fn fef(&self) -> FEF_R {
    FEF_R::new(((self.bits >> 13) & 0x01) != 0)
  }
  #[doc = "Bits 16:19 - Receive FIFO Write Pointer Value"]
  #[inline(always)]
  pub fn rx_wptr(&self) -> RX_WPTR_R {
    RX_WPTR_R::new(((self.bits >> 16) & 0x0f) as u8)
  }
  #[doc = "Bits 24:27 - Receive FIFO Byte Count"]
  #[inline(always)]
  pub fn rx_cnt(&self) -> RX_CNT_R {
    RX_CNT_R::new(((self.bits >> 24) & 0x0f) as u8)
  }
}
impl W {
  #[doc = "Bit 0 - Receive FIFO Overflow Flag"]
  #[inline(always)]
  pub fn rfo(&mut self) -> RFO_W {
    RFO_W { w: self }
  }
  #[doc = "Bit 4 - Receive Data Interrupt Flag"]
  #[inline(always)]
  pub fn rx_data(&mut self) -> RX_DATA_W {
    RX_DATA_W { w: self }
  }
  #[doc = "Bit 8 - Character Wait Time Error Flag"]
  #[inline(always)]
  pub fn cwt_err(&mut self) -> CWT_ERR_W {
    CWT_ERR_W { w: self }
  }
  #[doc = "Bit 9 - Received NACK Threshold Error Flag"]
  #[inline(always)]
  pub fn rte(&mut self) -> RTE_W {
    RTE_W { w: self }
  }
  #[doc = "Bit 10 - Block Wait Time Error Flag"]
  #[inline(always)]
  pub fn bwt_err(&mut self) -> BWT_ERR_W {
    BWT_ERR_W { w: self }
  }
  #[doc = "Bit 11 - Block Guard Time Error Flag"]
  #[inline(always)]
  pub fn bgt_err(&mut self) -> BGT_ERR_W {
    BGT_ERR_W { w: self }
  }
  #[doc = "Bit 12 - Parity Error Flag"]
  #[inline(always)]
  pub fn pef(&mut self) -> PEF_W {
    PEF_W { w: self }
  }
  #[doc = "Bit 13 - Frame Error Flag"]
  #[inline(always)]
  pub fn fef(&mut self) -> FEF_W {
    FEF_W { w: self }
  }
}
