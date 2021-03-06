#[doc = "Reader of register SSRS"]
pub type R = crate::R<u32, super::SSRS>;
#[doc = "Writer for register SSRS"]
pub type W = crate::W<u32, super::SSRS>;
#[doc = "Register SSRS `reset()`'s with value 0x06"]
impl crate::ResetValue for super::SSRS {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0x06
  }
}
#[doc = "Wakeup Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAKEUP_A {
  #[doc = "0: Reset not generated by wakeup from VLLS mode."]
  WAKEUP_0,
  #[doc = "1: Reset generated by wakeup from VLLS mode."]
  WAKEUP_1,
}
impl From<WAKEUP_A> for bool {
  #[inline(always)]
  fn from(variant: WAKEUP_A) -> Self {
    match variant {
      WAKEUP_A::WAKEUP_0 => false,
      WAKEUP_A::WAKEUP_1 => true,
    }
  }
}
#[doc = "Reader of field `WAKEUP`"]
pub type WAKEUP_R = crate::R<bool, WAKEUP_A>;
impl WAKEUP_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> WAKEUP_A {
    match self.bits {
      false => WAKEUP_A::WAKEUP_0,
      true => WAKEUP_A::WAKEUP_1,
    }
  }
  #[doc = "Checks if the value of the field is `WAKEUP_0`"]
  #[inline(always)]
  pub fn is_wakeup_0(&self) -> bool {
    *self == WAKEUP_A::WAKEUP_0
  }
  #[doc = "Checks if the value of the field is `WAKEUP_1`"]
  #[inline(always)]
  pub fn is_wakeup_1(&self) -> bool {
    *self == WAKEUP_A::WAKEUP_1
  }
}
#[doc = "Write proxy for field `WAKEUP`"]
pub struct WAKEUP_W<'a> {
  w: &'a mut W,
}
impl<'a> WAKEUP_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WAKEUP_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Reset not generated by wakeup from VLLS mode."]
  #[inline(always)]
  pub fn wakeup_0(self) -> &'a mut W {
    self.variant(WAKEUP_A::WAKEUP_0)
  }
  #[doc = "Reset generated by wakeup from VLLS mode."]
  #[inline(always)]
  pub fn wakeup_1(self) -> &'a mut W {
    self.variant(WAKEUP_A::WAKEUP_1)
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
#[doc = "POR Reset\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POR_A {
  #[doc = "0: Reset not generated by POR."]
  POR_0,
  #[doc = "1: Reset generated by POR."]
  POR_1,
}
impl From<POR_A> for bool {
  #[inline(always)]
  fn from(variant: POR_A) -> Self {
    match variant {
      POR_A::POR_0 => false,
      POR_A::POR_1 => true,
    }
  }
}
#[doc = "Reader of field `POR`"]
pub type POR_R = crate::R<bool, POR_A>;
impl POR_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> POR_A {
    match self.bits {
      false => POR_A::POR_0,
      true => POR_A::POR_1,
    }
  }
  #[doc = "Checks if the value of the field is `POR_0`"]
  #[inline(always)]
  pub fn is_por_0(&self) -> bool {
    *self == POR_A::POR_0
  }
  #[doc = "Checks if the value of the field is `POR_1`"]
  #[inline(always)]
  pub fn is_por_1(&self) -> bool {
    *self == POR_A::POR_1
  }
}
#[doc = "Write proxy for field `POR`"]
pub struct POR_W<'a> {
  w: &'a mut W,
}
impl<'a> POR_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: POR_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Reset not generated by POR."]
  #[inline(always)]
  pub fn por_0(self) -> &'a mut W {
    self.variant(POR_A::POR_0)
  }
  #[doc = "Reset generated by POR."]
  #[inline(always)]
  pub fn por_1(self) -> &'a mut W {
    self.variant(POR_A::POR_1)
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
#[doc = "LVD Reset\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LVD_A {
  #[doc = "0: Reset not generated by LVD."]
  LVD_0,
  #[doc = "1: Reset generated by LVD."]
  LVD_1,
}
impl From<LVD_A> for bool {
  #[inline(always)]
  fn from(variant: LVD_A) -> Self {
    match variant {
      LVD_A::LVD_0 => false,
      LVD_A::LVD_1 => true,
    }
  }
}
#[doc = "Reader of field `LVD`"]
pub type LVD_R = crate::R<bool, LVD_A>;
impl LVD_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> LVD_A {
    match self.bits {
      false => LVD_A::LVD_0,
      true => LVD_A::LVD_1,
    }
  }
  #[doc = "Checks if the value of the field is `LVD_0`"]
  #[inline(always)]
  pub fn is_lvd_0(&self) -> bool {
    *self == LVD_A::LVD_0
  }
  #[doc = "Checks if the value of the field is `LVD_1`"]
  #[inline(always)]
  pub fn is_lvd_1(&self) -> bool {
    *self == LVD_A::LVD_1
  }
}
#[doc = "Write proxy for field `LVD`"]
pub struct LVD_W<'a> {
  w: &'a mut W,
}
impl<'a> LVD_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: LVD_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Reset not generated by LVD."]
  #[inline(always)]
  pub fn lvd_0(self) -> &'a mut W {
    self.variant(LVD_A::LVD_0)
  }
  #[doc = "Reset generated by LVD."]
  #[inline(always)]
  pub fn lvd_1(self) -> &'a mut W {
    self.variant(LVD_A::LVD_1)
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
#[doc = "HVD Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HVD_A {
  #[doc = "0: Reset not generated by HVD."]
  HVD_0,
  #[doc = "1: Reset generated by HVD."]
  HVD_1,
}
impl From<HVD_A> for bool {
  #[inline(always)]
  fn from(variant: HVD_A) -> Self {
    match variant {
      HVD_A::HVD_0 => false,
      HVD_A::HVD_1 => true,
    }
  }
}
#[doc = "Reader of field `HVD`"]
pub type HVD_R = crate::R<bool, HVD_A>;
impl HVD_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> HVD_A {
    match self.bits {
      false => HVD_A::HVD_0,
      true => HVD_A::HVD_1,
    }
  }
  #[doc = "Checks if the value of the field is `HVD_0`"]
  #[inline(always)]
  pub fn is_hvd_0(&self) -> bool {
    *self == HVD_A::HVD_0
  }
  #[doc = "Checks if the value of the field is `HVD_1`"]
  #[inline(always)]
  pub fn is_hvd_1(&self) -> bool {
    *self == HVD_A::HVD_1
  }
}
#[doc = "Write proxy for field `HVD`"]
pub struct HVD_W<'a> {
  w: &'a mut W,
}
impl<'a> HVD_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: HVD_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Reset not generated by HVD."]
  #[inline(always)]
  pub fn hvd_0(self) -> &'a mut W {
    self.variant(HVD_A::HVD_0)
  }
  #[doc = "Reset generated by HVD."]
  #[inline(always)]
  pub fn hvd_1(self) -> &'a mut W {
    self.variant(HVD_A::HVD_1)
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
#[doc = "Warm Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WARM_A {
  #[doc = "0: Reset not generated by system reset source."]
  WARM_0,
  #[doc = "1: Reset generated by system reset source."]
  WARM_1,
}
impl From<WARM_A> for bool {
  #[inline(always)]
  fn from(variant: WARM_A) -> Self {
    match variant {
      WARM_A::WARM_0 => false,
      WARM_A::WARM_1 => true,
    }
  }
}
#[doc = "Reader of field `WARM`"]
pub type WARM_R = crate::R<bool, WARM_A>;
impl WARM_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> WARM_A {
    match self.bits {
      false => WARM_A::WARM_0,
      true => WARM_A::WARM_1,
    }
  }
  #[doc = "Checks if the value of the field is `WARM_0`"]
  #[inline(always)]
  pub fn is_warm_0(&self) -> bool {
    *self == WARM_A::WARM_0
  }
  #[doc = "Checks if the value of the field is `WARM_1`"]
  #[inline(always)]
  pub fn is_warm_1(&self) -> bool {
    *self == WARM_A::WARM_1
  }
}
#[doc = "Write proxy for field `WARM`"]
pub struct WARM_W<'a> {
  w: &'a mut W,
}
impl<'a> WARM_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WARM_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Reset not generated by system reset source."]
  #[inline(always)]
  pub fn warm_0(self) -> &'a mut W {
    self.variant(WARM_A::WARM_0)
  }
  #[doc = "Reset generated by system reset source."]
  #[inline(always)]
  pub fn warm_1(self) -> &'a mut W {
    self.variant(WARM_A::WARM_1)
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
#[doc = "Fatal Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FATAL_A {
  #[doc = "0: Reset was not generated by a fatal reset source."]
  FATAL_0,
  #[doc = "1: Reset was generated by a fatal reset source."]
  FATAL_1,
}
impl From<FATAL_A> for bool {
  #[inline(always)]
  fn from(variant: FATAL_A) -> Self {
    match variant {
      FATAL_A::FATAL_0 => false,
      FATAL_A::FATAL_1 => true,
    }
  }
}
#[doc = "Reader of field `FATAL`"]
pub type FATAL_R = crate::R<bool, FATAL_A>;
impl FATAL_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> FATAL_A {
    match self.bits {
      false => FATAL_A::FATAL_0,
      true => FATAL_A::FATAL_1,
    }
  }
  #[doc = "Checks if the value of the field is `FATAL_0`"]
  #[inline(always)]
  pub fn is_fatal_0(&self) -> bool {
    *self == FATAL_A::FATAL_0
  }
  #[doc = "Checks if the value of the field is `FATAL_1`"]
  #[inline(always)]
  pub fn is_fatal_1(&self) -> bool {
    *self == FATAL_A::FATAL_1
  }
}
#[doc = "Write proxy for field `FATAL`"]
pub struct FATAL_W<'a> {
  w: &'a mut W,
}
impl<'a> FATAL_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: FATAL_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Reset was not generated by a fatal reset source."]
  #[inline(always)]
  pub fn fatal_0(self) -> &'a mut W {
    self.variant(FATAL_A::FATAL_0)
  }
  #[doc = "Reset was generated by a fatal reset source."]
  #[inline(always)]
  pub fn fatal_1(self) -> &'a mut W {
    self.variant(FATAL_A::FATAL_1)
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
#[doc = "Pin Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN_A {
  #[doc = "0: Reset was not generated from the RESET_B pin."]
  PIN_0,
  #[doc = "1: Reset was generated from the RESET_B pin."]
  PIN_1,
}
impl From<PIN_A> for bool {
  #[inline(always)]
  fn from(variant: PIN_A) -> Self {
    match variant {
      PIN_A::PIN_0 => false,
      PIN_A::PIN_1 => true,
    }
  }
}
#[doc = "Reader of field `PIN`"]
pub type PIN_R = crate::R<bool, PIN_A>;
impl PIN_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> PIN_A {
    match self.bits {
      false => PIN_A::PIN_0,
      true => PIN_A::PIN_1,
    }
  }
  #[doc = "Checks if the value of the field is `PIN_0`"]
  #[inline(always)]
  pub fn is_pin_0(&self) -> bool {
    *self == PIN_A::PIN_0
  }
  #[doc = "Checks if the value of the field is `PIN_1`"]
  #[inline(always)]
  pub fn is_pin_1(&self) -> bool {
    *self == PIN_A::PIN_1
  }
}
#[doc = "Write proxy for field `PIN`"]
pub struct PIN_W<'a> {
  w: &'a mut W,
}
impl<'a> PIN_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: PIN_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Reset was not generated from the RESET_B pin."]
  #[inline(always)]
  pub fn pin_0(self) -> &'a mut W {
    self.variant(PIN_A::PIN_0)
  }
  #[doc = "Reset was generated from the RESET_B pin."]
  #[inline(always)]
  pub fn pin_1(self) -> &'a mut W {
    self.variant(PIN_A::PIN_1)
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
#[doc = "MDM Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MDM_A {
  #[doc = "0: Reset was not generated from the MDM reset request."]
  MDM_0,
  #[doc = "1: Reset was generated from the MDM reset request."]
  MDM_1,
}
impl From<MDM_A> for bool {
  #[inline(always)]
  fn from(variant: MDM_A) -> Self {
    match variant {
      MDM_A::MDM_0 => false,
      MDM_A::MDM_1 => true,
    }
  }
}
#[doc = "Reader of field `MDM`"]
pub type MDM_R = crate::R<bool, MDM_A>;
impl MDM_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> MDM_A {
    match self.bits {
      false => MDM_A::MDM_0,
      true => MDM_A::MDM_1,
    }
  }
  #[doc = "Checks if the value of the field is `MDM_0`"]
  #[inline(always)]
  pub fn is_mdm_0(&self) -> bool {
    *self == MDM_A::MDM_0
  }
  #[doc = "Checks if the value of the field is `MDM_1`"]
  #[inline(always)]
  pub fn is_mdm_1(&self) -> bool {
    *self == MDM_A::MDM_1
  }
}
#[doc = "Write proxy for field `MDM`"]
pub struct MDM_W<'a> {
  w: &'a mut W,
}
impl<'a> MDM_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: MDM_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Reset was not generated from the MDM reset request."]
  #[inline(always)]
  pub fn mdm_0(self) -> &'a mut W {
    self.variant(MDM_A::MDM_0)
  }
  #[doc = "Reset was generated from the MDM reset request."]
  #[inline(always)]
  pub fn mdm_1(self) -> &'a mut W {
    self.variant(MDM_A::MDM_1)
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
#[doc = "Reset Timeout\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSTACK_A {
  #[doc = "0: Reset not generated from Reset Controller Timeout."]
  RSTACK_0,
  #[doc = "1: Reset generated from Reset Controller Timeout."]
  RSTACK_1,
}
impl From<RSTACK_A> for bool {
  #[inline(always)]
  fn from(variant: RSTACK_A) -> Self {
    match variant {
      RSTACK_A::RSTACK_0 => false,
      RSTACK_A::RSTACK_1 => true,
    }
  }
}
#[doc = "Reader of field `RSTACK`"]
pub type RSTACK_R = crate::R<bool, RSTACK_A>;
impl RSTACK_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> RSTACK_A {
    match self.bits {
      false => RSTACK_A::RSTACK_0,
      true => RSTACK_A::RSTACK_1,
    }
  }
  #[doc = "Checks if the value of the field is `RSTACK_0`"]
  #[inline(always)]
  pub fn is_rstack_0(&self) -> bool {
    *self == RSTACK_A::RSTACK_0
  }
  #[doc = "Checks if the value of the field is `RSTACK_1`"]
  #[inline(always)]
  pub fn is_rstack_1(&self) -> bool {
    *self == RSTACK_A::RSTACK_1
  }
}
#[doc = "Write proxy for field `RSTACK`"]
pub struct RSTACK_W<'a> {
  w: &'a mut W,
}
impl<'a> RSTACK_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: RSTACK_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Reset not generated from Reset Controller Timeout."]
  #[inline(always)]
  pub fn rstack_0(self) -> &'a mut W {
    self.variant(RSTACK_A::RSTACK_0)
  }
  #[doc = "Reset generated from Reset Controller Timeout."]
  #[inline(always)]
  pub fn rstack_1(self) -> &'a mut W {
    self.variant(RSTACK_A::RSTACK_1)
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
#[doc = "Stop Timeout Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STOPACK_A {
  #[doc = "0: Reset not generated by Stop Controller Timeout."]
  STOPACK_0,
  #[doc = "1: Reset generated by Stop Controller Timeout."]
  STOPACK_1,
}
impl From<STOPACK_A> for bool {
  #[inline(always)]
  fn from(variant: STOPACK_A) -> Self {
    match variant {
      STOPACK_A::STOPACK_0 => false,
      STOPACK_A::STOPACK_1 => true,
    }
  }
}
#[doc = "Reader of field `STOPACK`"]
pub type STOPACK_R = crate::R<bool, STOPACK_A>;
impl STOPACK_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> STOPACK_A {
    match self.bits {
      false => STOPACK_A::STOPACK_0,
      true => STOPACK_A::STOPACK_1,
    }
  }
  #[doc = "Checks if the value of the field is `STOPACK_0`"]
  #[inline(always)]
  pub fn is_stopack_0(&self) -> bool {
    *self == STOPACK_A::STOPACK_0
  }
  #[doc = "Checks if the value of the field is `STOPACK_1`"]
  #[inline(always)]
  pub fn is_stopack_1(&self) -> bool {
    *self == STOPACK_A::STOPACK_1
  }
}
#[doc = "Write proxy for field `STOPACK`"]
pub struct STOPACK_W<'a> {
  w: &'a mut W,
}
impl<'a> STOPACK_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: STOPACK_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Reset not generated by Stop Controller Timeout."]
  #[inline(always)]
  pub fn stopack_0(self) -> &'a mut W {
    self.variant(STOPACK_A::STOPACK_0)
  }
  #[doc = "Reset generated by Stop Controller Timeout."]
  #[inline(always)]
  pub fn stopack_1(self) -> &'a mut W {
    self.variant(STOPACK_A::STOPACK_1)
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
#[doc = "SCG Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCG_A {
  #[doc = "0: Reset is not generated from an SCG loss of lock or loss of clock."]
  SCG_0,
  #[doc = "1: Reset is generated from an SCG loss of lock or loss of clock."]
  SCG_1,
}
impl From<SCG_A> for bool {
  #[inline(always)]
  fn from(variant: SCG_A) -> Self {
    match variant {
      SCG_A::SCG_0 => false,
      SCG_A::SCG_1 => true,
    }
  }
}
#[doc = "Reader of field `SCG`"]
pub type SCG_R = crate::R<bool, SCG_A>;
impl SCG_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> SCG_A {
    match self.bits {
      false => SCG_A::SCG_0,
      true => SCG_A::SCG_1,
    }
  }
  #[doc = "Checks if the value of the field is `SCG_0`"]
  #[inline(always)]
  pub fn is_scg_0(&self) -> bool {
    *self == SCG_A::SCG_0
  }
  #[doc = "Checks if the value of the field is `SCG_1`"]
  #[inline(always)]
  pub fn is_scg_1(&self) -> bool {
    *self == SCG_A::SCG_1
  }
}
#[doc = "Write proxy for field `SCG`"]
pub struct SCG_W<'a> {
  w: &'a mut W,
}
impl<'a> SCG_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: SCG_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Reset is not generated from an SCG loss of lock or loss of clock."]
  #[inline(always)]
  pub fn scg_0(self) -> &'a mut W {
    self.variant(SCG_A::SCG_0)
  }
  #[doc = "Reset is generated from an SCG loss of lock or loss of clock."]
  #[inline(always)]
  pub fn scg_1(self) -> &'a mut W {
    self.variant(SCG_A::SCG_1)
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
#[doc = "Watchdog Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDOG_A {
  #[doc = "0: Reset is not generated from the WatchDog timeout."]
  WDOG_0,
  #[doc = "1: Reset is generated from the WatchDog timeout."]
  WDOG_1,
}
impl From<WDOG_A> for bool {
  #[inline(always)]
  fn from(variant: WDOG_A) -> Self {
    match variant {
      WDOG_A::WDOG_0 => false,
      WDOG_A::WDOG_1 => true,
    }
  }
}
#[doc = "Reader of field `WDOG`"]
pub type WDOG_R = crate::R<bool, WDOG_A>;
impl WDOG_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> WDOG_A {
    match self.bits {
      false => WDOG_A::WDOG_0,
      true => WDOG_A::WDOG_1,
    }
  }
  #[doc = "Checks if the value of the field is `WDOG_0`"]
  #[inline(always)]
  pub fn is_wdog_0(&self) -> bool {
    *self == WDOG_A::WDOG_0
  }
  #[doc = "Checks if the value of the field is `WDOG_1`"]
  #[inline(always)]
  pub fn is_wdog_1(&self) -> bool {
    *self == WDOG_A::WDOG_1
  }
}
#[doc = "Write proxy for field `WDOG`"]
pub struct WDOG_W<'a> {
  w: &'a mut W,
}
impl<'a> WDOG_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WDOG_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Reset is not generated from the WatchDog timeout."]
  #[inline(always)]
  pub fn wdog_0(self) -> &'a mut W {
    self.variant(WDOG_A::WDOG_0)
  }
  #[doc = "Reset is generated from the WatchDog timeout."]
  #[inline(always)]
  pub fn wdog_1(self) -> &'a mut W {
    self.variant(WDOG_A::WDOG_1)
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
#[doc = "Software Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SW_A {
  #[doc = "0: Reset not generated by software request from core."]
  SW_0,
  #[doc = "1: Reset generated by software request from core."]
  SW_1,
}
impl From<SW_A> for bool {
  #[inline(always)]
  fn from(variant: SW_A) -> Self {
    match variant {
      SW_A::SW_0 => false,
      SW_A::SW_1 => true,
    }
  }
}
#[doc = "Reader of field `SW`"]
pub type SW_R = crate::R<bool, SW_A>;
impl SW_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> SW_A {
    match self.bits {
      false => SW_A::SW_0,
      true => SW_A::SW_1,
    }
  }
  #[doc = "Checks if the value of the field is `SW_0`"]
  #[inline(always)]
  pub fn is_sw_0(&self) -> bool {
    *self == SW_A::SW_0
  }
  #[doc = "Checks if the value of the field is `SW_1`"]
  #[inline(always)]
  pub fn is_sw_1(&self) -> bool {
    *self == SW_A::SW_1
  }
}
#[doc = "Write proxy for field `SW`"]
pub struct SW_W<'a> {
  w: &'a mut W,
}
impl<'a> SW_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: SW_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Reset not generated by software request from core."]
  #[inline(always)]
  pub fn sw_0(self) -> &'a mut W {
    self.variant(SW_A::SW_0)
  }
  #[doc = "Reset generated by software request from core."]
  #[inline(always)]
  pub fn sw_1(self) -> &'a mut W {
    self.variant(SW_A::SW_1)
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
#[doc = "Lockup Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCKUP_A {
  #[doc = "0: Reset not generated by core lockup."]
  LOCKUP_0,
  #[doc = "1: Reset generated by core lockup."]
  LOCKUP_1,
}
impl From<LOCKUP_A> for bool {
  #[inline(always)]
  fn from(variant: LOCKUP_A) -> Self {
    match variant {
      LOCKUP_A::LOCKUP_0 => false,
      LOCKUP_A::LOCKUP_1 => true,
    }
  }
}
#[doc = "Reader of field `LOCKUP`"]
pub type LOCKUP_R = crate::R<bool, LOCKUP_A>;
impl LOCKUP_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> LOCKUP_A {
    match self.bits {
      false => LOCKUP_A::LOCKUP_0,
      true => LOCKUP_A::LOCKUP_1,
    }
  }
  #[doc = "Checks if the value of the field is `LOCKUP_0`"]
  #[inline(always)]
  pub fn is_lockup_0(&self) -> bool {
    *self == LOCKUP_A::LOCKUP_0
  }
  #[doc = "Checks if the value of the field is `LOCKUP_1`"]
  #[inline(always)]
  pub fn is_lockup_1(&self) -> bool {
    *self == LOCKUP_A::LOCKUP_1
  }
}
#[doc = "Write proxy for field `LOCKUP`"]
pub struct LOCKUP_W<'a> {
  w: &'a mut W,
}
impl<'a> LOCKUP_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: LOCKUP_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Reset not generated by core lockup."]
  #[inline(always)]
  pub fn lockup_0(self) -> &'a mut W {
    self.variant(LOCKUP_A::LOCKUP_0)
  }
  #[doc = "Reset generated by core lockup."]
  #[inline(always)]
  pub fn lockup_1(self) -> &'a mut W {
    self.variant(LOCKUP_A::LOCKUP_1)
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
#[doc = "Core1 Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CORE1_A {
  #[doc = "0: Reset not generated from Core1 reset source."]
  CORE1_0,
  #[doc = "1: Reset generated from Core1 reset source."]
  CORE1_1,
}
impl From<CORE1_A> for bool {
  #[inline(always)]
  fn from(variant: CORE1_A) -> Self {
    match variant {
      CORE1_A::CORE1_0 => false,
      CORE1_A::CORE1_1 => true,
    }
  }
}
#[doc = "Reader of field `CORE1`"]
pub type CORE1_R = crate::R<bool, CORE1_A>;
impl CORE1_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> CORE1_A {
    match self.bits {
      false => CORE1_A::CORE1_0,
      true => CORE1_A::CORE1_1,
    }
  }
  #[doc = "Checks if the value of the field is `CORE1_0`"]
  #[inline(always)]
  pub fn is_core1_0(&self) -> bool {
    *self == CORE1_A::CORE1_0
  }
  #[doc = "Checks if the value of the field is `CORE1_1`"]
  #[inline(always)]
  pub fn is_core1_1(&self) -> bool {
    *self == CORE1_A::CORE1_1
  }
}
#[doc = "Write proxy for field `CORE1`"]
pub struct CORE1_W<'a> {
  w: &'a mut W,
}
impl<'a> CORE1_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: CORE1_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Reset not generated from Core1 reset source."]
  #[inline(always)]
  pub fn core1_0(self) -> &'a mut W {
    self.variant(CORE1_A::CORE1_0)
  }
  #[doc = "Reset generated from Core1 reset source."]
  #[inline(always)]
  pub fn core1_1(self) -> &'a mut W {
    self.variant(CORE1_A::CORE1_1)
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
#[doc = "JTAG System Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JTAG_A {
  #[doc = "0: Reset not generated by JTAG system reset."]
  JTAG_0,
  #[doc = "1: Reset generated by JTAG system reset."]
  JTAG_1,
}
impl From<JTAG_A> for bool {
  #[inline(always)]
  fn from(variant: JTAG_A) -> Self {
    match variant {
      JTAG_A::JTAG_0 => false,
      JTAG_A::JTAG_1 => true,
    }
  }
}
#[doc = "Reader of field `JTAG`"]
pub type JTAG_R = crate::R<bool, JTAG_A>;
impl JTAG_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> JTAG_A {
    match self.bits {
      false => JTAG_A::JTAG_0,
      true => JTAG_A::JTAG_1,
    }
  }
  #[doc = "Checks if the value of the field is `JTAG_0`"]
  #[inline(always)]
  pub fn is_jtag_0(&self) -> bool {
    *self == JTAG_A::JTAG_0
  }
  #[doc = "Checks if the value of the field is `JTAG_1`"]
  #[inline(always)]
  pub fn is_jtag_1(&self) -> bool {
    *self == JTAG_A::JTAG_1
  }
}
#[doc = "Write proxy for field `JTAG`"]
pub struct JTAG_W<'a> {
  w: &'a mut W,
}
impl<'a> JTAG_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: JTAG_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Reset not generated by JTAG system reset."]
  #[inline(always)]
  pub fn jtag_0(self) -> &'a mut W {
    self.variant(JTAG_A::JTAG_0)
  }
  #[doc = "Reset generated by JTAG system reset."]
  #[inline(always)]
  pub fn jtag_1(self) -> &'a mut W {
    self.variant(JTAG_A::JTAG_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
    self.w
  }
}
impl R {
  #[doc = "Bit 0 - Wakeup Reset"]
  #[inline(always)]
  pub fn wakeup(&self) -> WAKEUP_R {
    WAKEUP_R::new((self.bits & 0x01) != 0)
  }
  #[doc = "Bit 1 - POR Reset"]
  #[inline(always)]
  pub fn por(&self) -> POR_R {
    POR_R::new(((self.bits >> 1) & 0x01) != 0)
  }
  #[doc = "Bit 2 - LVD Reset"]
  #[inline(always)]
  pub fn lvd(&self) -> LVD_R {
    LVD_R::new(((self.bits >> 2) & 0x01) != 0)
  }
  #[doc = "Bit 3 - HVD Reset"]
  #[inline(always)]
  pub fn hvd(&self) -> HVD_R {
    HVD_R::new(((self.bits >> 3) & 0x01) != 0)
  }
  #[doc = "Bit 4 - Warm Reset"]
  #[inline(always)]
  pub fn warm(&self) -> WARM_R {
    WARM_R::new(((self.bits >> 4) & 0x01) != 0)
  }
  #[doc = "Bit 5 - Fatal Reset"]
  #[inline(always)]
  pub fn fatal(&self) -> FATAL_R {
    FATAL_R::new(((self.bits >> 5) & 0x01) != 0)
  }
  #[doc = "Bit 8 - Pin Reset"]
  #[inline(always)]
  pub fn pin(&self) -> PIN_R {
    PIN_R::new(((self.bits >> 8) & 0x01) != 0)
  }
  #[doc = "Bit 9 - MDM Reset"]
  #[inline(always)]
  pub fn mdm(&self) -> MDM_R {
    MDM_R::new(((self.bits >> 9) & 0x01) != 0)
  }
  #[doc = "Bit 10 - Reset Timeout"]
  #[inline(always)]
  pub fn rstack(&self) -> RSTACK_R {
    RSTACK_R::new(((self.bits >> 10) & 0x01) != 0)
  }
  #[doc = "Bit 11 - Stop Timeout Reset"]
  #[inline(always)]
  pub fn stopack(&self) -> STOPACK_R {
    STOPACK_R::new(((self.bits >> 11) & 0x01) != 0)
  }
  #[doc = "Bit 12 - SCG Reset"]
  #[inline(always)]
  pub fn scg(&self) -> SCG_R {
    SCG_R::new(((self.bits >> 12) & 0x01) != 0)
  }
  #[doc = "Bit 13 - Watchdog Reset"]
  #[inline(always)]
  pub fn wdog(&self) -> WDOG_R {
    WDOG_R::new(((self.bits >> 13) & 0x01) != 0)
  }
  #[doc = "Bit 14 - Software Reset"]
  #[inline(always)]
  pub fn sw(&self) -> SW_R {
    SW_R::new(((self.bits >> 14) & 0x01) != 0)
  }
  #[doc = "Bit 15 - Lockup Reset"]
  #[inline(always)]
  pub fn lockup(&self) -> LOCKUP_R {
    LOCKUP_R::new(((self.bits >> 15) & 0x01) != 0)
  }
  #[doc = "Bit 17 - Core1 Reset"]
  #[inline(always)]
  pub fn core1(&self) -> CORE1_R {
    CORE1_R::new(((self.bits >> 17) & 0x01) != 0)
  }
  #[doc = "Bit 28 - JTAG System Reset"]
  #[inline(always)]
  pub fn jtag(&self) -> JTAG_R {
    JTAG_R::new(((self.bits >> 28) & 0x01) != 0)
  }
}
impl W {
  #[doc = "Bit 0 - Wakeup Reset"]
  #[inline(always)]
  pub fn wakeup(&mut self) -> WAKEUP_W {
    WAKEUP_W { w: self }
  }
  #[doc = "Bit 1 - POR Reset"]
  #[inline(always)]
  pub fn por(&mut self) -> POR_W {
    POR_W { w: self }
  }
  #[doc = "Bit 2 - LVD Reset"]
  #[inline(always)]
  pub fn lvd(&mut self) -> LVD_W {
    LVD_W { w: self }
  }
  #[doc = "Bit 3 - HVD Reset"]
  #[inline(always)]
  pub fn hvd(&mut self) -> HVD_W {
    HVD_W { w: self }
  }
  #[doc = "Bit 4 - Warm Reset"]
  #[inline(always)]
  pub fn warm(&mut self) -> WARM_W {
    WARM_W { w: self }
  }
  #[doc = "Bit 5 - Fatal Reset"]
  #[inline(always)]
  pub fn fatal(&mut self) -> FATAL_W {
    FATAL_W { w: self }
  }
  #[doc = "Bit 8 - Pin Reset"]
  #[inline(always)]
  pub fn pin(&mut self) -> PIN_W {
    PIN_W { w: self }
  }
  #[doc = "Bit 9 - MDM Reset"]
  #[inline(always)]
  pub fn mdm(&mut self) -> MDM_W {
    MDM_W { w: self }
  }
  #[doc = "Bit 10 - Reset Timeout"]
  #[inline(always)]
  pub fn rstack(&mut self) -> RSTACK_W {
    RSTACK_W { w: self }
  }
  #[doc = "Bit 11 - Stop Timeout Reset"]
  #[inline(always)]
  pub fn stopack(&mut self) -> STOPACK_W {
    STOPACK_W { w: self }
  }
  #[doc = "Bit 12 - SCG Reset"]
  #[inline(always)]
  pub fn scg(&mut self) -> SCG_W {
    SCG_W { w: self }
  }
  #[doc = "Bit 13 - Watchdog Reset"]
  #[inline(always)]
  pub fn wdog(&mut self) -> WDOG_W {
    WDOG_W { w: self }
  }
  #[doc = "Bit 14 - Software Reset"]
  #[inline(always)]
  pub fn sw(&mut self) -> SW_W {
    SW_W { w: self }
  }
  #[doc = "Bit 15 - Lockup Reset"]
  #[inline(always)]
  pub fn lockup(&mut self) -> LOCKUP_W {
    LOCKUP_W { w: self }
  }
  #[doc = "Bit 17 - Core1 Reset"]
  #[inline(always)]
  pub fn core1(&mut self) -> CORE1_W {
    CORE1_W { w: self }
  }
  #[doc = "Bit 28 - JTAG System Reset"]
  #[inline(always)]
  pub fn jtag(&mut self) -> JTAG_W {
    JTAG_W { w: self }
  }
}
