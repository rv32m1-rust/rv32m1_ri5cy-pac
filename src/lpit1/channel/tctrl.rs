#[doc = "Reader of register TCTRL"]
pub type R = crate::R<u32, super::TCTRL>;
#[doc = "Writer for register TCTRL"]
pub type W = crate::W<u32, super::TCTRL>;
#[doc = "Register TCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::TCTRL {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "Timer Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum T_EN_A {
  #[doc = "0: Timer Channel is disabled"]
  T_EN_0,
  #[doc = "1: Timer Channel is enabled"]
  T_EN_1,
}
impl From<T_EN_A> for bool {
  #[inline(always)]
  fn from(variant: T_EN_A) -> Self {
    match variant {
      T_EN_A::T_EN_0 => false,
      T_EN_A::T_EN_1 => true,
    }
  }
}
#[doc = "Reader of field `T_EN`"]
pub type T_EN_R = crate::R<bool, T_EN_A>;
impl T_EN_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> T_EN_A {
    match self.bits {
      false => T_EN_A::T_EN_0,
      true => T_EN_A::T_EN_1,
    }
  }
  #[doc = "Checks if the value of the field is `T_EN_0`"]
  #[inline(always)]
  pub fn is_t_en_0(&self) -> bool {
    *self == T_EN_A::T_EN_0
  }
  #[doc = "Checks if the value of the field is `T_EN_1`"]
  #[inline(always)]
  pub fn is_t_en_1(&self) -> bool {
    *self == T_EN_A::T_EN_1
  }
}
#[doc = "Write proxy for field `T_EN`"]
pub struct T_EN_W<'a> {
  w: &'a mut W,
}
impl<'a> T_EN_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: T_EN_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Timer Channel is disabled"]
  #[inline(always)]
  pub fn t_en_0(self) -> &'a mut W {
    self.variant(T_EN_A::T_EN_0)
  }
  #[doc = "Timer Channel is enabled"]
  #[inline(always)]
  pub fn t_en_1(self) -> &'a mut W {
    self.variant(T_EN_A::T_EN_1)
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
#[doc = "Chain Channel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHAIN_A {
  #[doc = "0: Channel Chaining is disabled. The channel timer runs independently."]
  CHAIN_0,
  #[doc = "1: Channel Chaining is enabled. The timer decrements on the previous channel's timeout."]
  CHAIN_1,
}
impl From<CHAIN_A> for bool {
  #[inline(always)]
  fn from(variant: CHAIN_A) -> Self {
    match variant {
      CHAIN_A::CHAIN_0 => false,
      CHAIN_A::CHAIN_1 => true,
    }
  }
}
#[doc = "Reader of field `CHAIN`"]
pub type CHAIN_R = crate::R<bool, CHAIN_A>;
impl CHAIN_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> CHAIN_A {
    match self.bits {
      false => CHAIN_A::CHAIN_0,
      true => CHAIN_A::CHAIN_1,
    }
  }
  #[doc = "Checks if the value of the field is `CHAIN_0`"]
  #[inline(always)]
  pub fn is_chain_0(&self) -> bool {
    *self == CHAIN_A::CHAIN_0
  }
  #[doc = "Checks if the value of the field is `CHAIN_1`"]
  #[inline(always)]
  pub fn is_chain_1(&self) -> bool {
    *self == CHAIN_A::CHAIN_1
  }
}
#[doc = "Write proxy for field `CHAIN`"]
pub struct CHAIN_W<'a> {
  w: &'a mut W,
}
impl<'a> CHAIN_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: CHAIN_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Channel Chaining is disabled. The channel timer runs independently."]
  #[inline(always)]
  pub fn chain_0(self) -> &'a mut W {
    self.variant(CHAIN_A::CHAIN_0)
  }
  #[doc = "Channel Chaining is enabled. The timer decrements on the previous channel's timeout."]
  #[inline(always)]
  pub fn chain_1(self) -> &'a mut W {
    self.variant(CHAIN_A::CHAIN_1)
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
#[doc = "Timer Operation Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODE_A {
  #[doc = "0: 32-bit Periodic Counter"]
  MODE_0,
  #[doc = "1: Dual 16-bit Periodic Counter"]
  MODE_1,
  #[doc = "2: 32-bit Trigger Accumulator"]
  MODE_2,
  #[doc = "3: 32-bit Trigger Input Capture"]
  MODE_3,
}
impl From<MODE_A> for u8 {
  #[inline(always)]
  fn from(variant: MODE_A) -> Self {
    match variant {
      MODE_A::MODE_0 => 0,
      MODE_A::MODE_1 => 1,
      MODE_A::MODE_2 => 2,
      MODE_A::MODE_3 => 3,
    }
  }
}
#[doc = "Reader of field `MODE`"]
pub type MODE_R = crate::R<u8, MODE_A>;
impl MODE_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> MODE_A {
    match self.bits {
      0 => MODE_A::MODE_0,
      1 => MODE_A::MODE_1,
      2 => MODE_A::MODE_2,
      3 => MODE_A::MODE_3,
      _ => unreachable!(),
    }
  }
  #[doc = "Checks if the value of the field is `MODE_0`"]
  #[inline(always)]
  pub fn is_mode_0(&self) -> bool {
    *self == MODE_A::MODE_0
  }
  #[doc = "Checks if the value of the field is `MODE_1`"]
  #[inline(always)]
  pub fn is_mode_1(&self) -> bool {
    *self == MODE_A::MODE_1
  }
  #[doc = "Checks if the value of the field is `MODE_2`"]
  #[inline(always)]
  pub fn is_mode_2(&self) -> bool {
    *self == MODE_A::MODE_2
  }
  #[doc = "Checks if the value of the field is `MODE_3`"]
  #[inline(always)]
  pub fn is_mode_3(&self) -> bool {
    *self == MODE_A::MODE_3
  }
}
#[doc = "Write proxy for field `MODE`"]
pub struct MODE_W<'a> {
  w: &'a mut W,
}
impl<'a> MODE_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: MODE_A) -> &'a mut W {
    {
      self.bits(variant.into())
    }
  }
  #[doc = "32-bit Periodic Counter"]
  #[inline(always)]
  pub fn mode_0(self) -> &'a mut W {
    self.variant(MODE_A::MODE_0)
  }
  #[doc = "Dual 16-bit Periodic Counter"]
  #[inline(always)]
  pub fn mode_1(self) -> &'a mut W {
    self.variant(MODE_A::MODE_1)
  }
  #[doc = "32-bit Trigger Accumulator"]
  #[inline(always)]
  pub fn mode_2(self) -> &'a mut W {
    self.variant(MODE_A::MODE_2)
  }
  #[doc = "32-bit Trigger Input Capture"]
  #[inline(always)]
  pub fn mode_3(self) -> &'a mut W {
    self.variant(MODE_A::MODE_3)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
    self.w
  }
}
#[doc = "Timer Start On Trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSOT_A {
  #[doc = "0: Timer starts to decrement immediately based on the restart condition (controlled by the Timer Stop On Interrupt bit (TSOI))"]
  TSOT_0,
  #[doc = "1: Timer starts to decrement when a rising edge on a selected trigger is detected"]
  TSOT_1,
}
impl From<TSOT_A> for bool {
  #[inline(always)]
  fn from(variant: TSOT_A) -> Self {
    match variant {
      TSOT_A::TSOT_0 => false,
      TSOT_A::TSOT_1 => true,
    }
  }
}
#[doc = "Reader of field `TSOT`"]
pub type TSOT_R = crate::R<bool, TSOT_A>;
impl TSOT_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> TSOT_A {
    match self.bits {
      false => TSOT_A::TSOT_0,
      true => TSOT_A::TSOT_1,
    }
  }
  #[doc = "Checks if the value of the field is `TSOT_0`"]
  #[inline(always)]
  pub fn is_tsot_0(&self) -> bool {
    *self == TSOT_A::TSOT_0
  }
  #[doc = "Checks if the value of the field is `TSOT_1`"]
  #[inline(always)]
  pub fn is_tsot_1(&self) -> bool {
    *self == TSOT_A::TSOT_1
  }
}
#[doc = "Write proxy for field `TSOT`"]
pub struct TSOT_W<'a> {
  w: &'a mut W,
}
impl<'a> TSOT_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: TSOT_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Timer starts to decrement immediately based on the restart condition (controlled by the Timer Stop On Interrupt bit (TSOI))"]
  #[inline(always)]
  pub fn tsot_0(self) -> &'a mut W {
    self.variant(TSOT_A::TSOT_0)
  }
  #[doc = "Timer starts to decrement when a rising edge on a selected trigger is detected"]
  #[inline(always)]
  pub fn tsot_1(self) -> &'a mut W {
    self.variant(TSOT_A::TSOT_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
    self.w
  }
}
#[doc = "Timer Stop On Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSOI_A {
  #[doc = "0: The channel timer does not stop after timeout"]
  TSOI_0,
  #[doc = "1: The channel timer will stop after a timeout, and the channel timer will restart based on Timer Start On Trigger bit (TSOT). When TSOT = 0, the channel timer will restart after a rising edge on the Timer Enable bit (T_EN) is detected (which means that the timer channel is disabled and then enabled). When TSOT = 1, the channel timer will restart after a rising edge on the selected trigger is detected."]
  TSOI_1,
}
impl From<TSOI_A> for bool {
  #[inline(always)]
  fn from(variant: TSOI_A) -> Self {
    match variant {
      TSOI_A::TSOI_0 => false,
      TSOI_A::TSOI_1 => true,
    }
  }
}
#[doc = "Reader of field `TSOI`"]
pub type TSOI_R = crate::R<bool, TSOI_A>;
impl TSOI_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> TSOI_A {
    match self.bits {
      false => TSOI_A::TSOI_0,
      true => TSOI_A::TSOI_1,
    }
  }
  #[doc = "Checks if the value of the field is `TSOI_0`"]
  #[inline(always)]
  pub fn is_tsoi_0(&self) -> bool {
    *self == TSOI_A::TSOI_0
  }
  #[doc = "Checks if the value of the field is `TSOI_1`"]
  #[inline(always)]
  pub fn is_tsoi_1(&self) -> bool {
    *self == TSOI_A::TSOI_1
  }
}
#[doc = "Write proxy for field `TSOI`"]
pub struct TSOI_W<'a> {
  w: &'a mut W,
}
impl<'a> TSOI_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: TSOI_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "The channel timer does not stop after timeout"]
  #[inline(always)]
  pub fn tsoi_0(self) -> &'a mut W {
    self.variant(TSOI_A::TSOI_0)
  }
  #[doc = "The channel timer will stop after a timeout, and the channel timer will restart based on Timer Start On Trigger bit (TSOT). When TSOT = 0, the channel timer will restart after a rising edge on the Timer Enable bit (T_EN) is detected (which means that the timer channel is disabled and then enabled). When TSOT = 1, the channel timer will restart after a rising edge on the selected trigger is detected."]
  #[inline(always)]
  pub fn tsoi_1(self) -> &'a mut W {
    self.variant(TSOI_A::TSOI_1)
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
#[doc = "Timer Reload On Trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TROT_A {
  #[doc = "0: Timer will not reload on the selected trigger"]
  TROT_0,
  #[doc = "1: Timer will reload on the selected trigger"]
  TROT_1,
}
impl From<TROT_A> for bool {
  #[inline(always)]
  fn from(variant: TROT_A) -> Self {
    match variant {
      TROT_A::TROT_0 => false,
      TROT_A::TROT_1 => true,
    }
  }
}
#[doc = "Reader of field `TROT`"]
pub type TROT_R = crate::R<bool, TROT_A>;
impl TROT_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> TROT_A {
    match self.bits {
      false => TROT_A::TROT_0,
      true => TROT_A::TROT_1,
    }
  }
  #[doc = "Checks if the value of the field is `TROT_0`"]
  #[inline(always)]
  pub fn is_trot_0(&self) -> bool {
    *self == TROT_A::TROT_0
  }
  #[doc = "Checks if the value of the field is `TROT_1`"]
  #[inline(always)]
  pub fn is_trot_1(&self) -> bool {
    *self == TROT_A::TROT_1
  }
}
#[doc = "Write proxy for field `TROT`"]
pub struct TROT_W<'a> {
  w: &'a mut W,
}
impl<'a> TROT_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: TROT_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Timer will not reload on the selected trigger"]
  #[inline(always)]
  pub fn trot_0(self) -> &'a mut W {
    self.variant(TROT_A::TROT_0)
  }
  #[doc = "Timer will reload on the selected trigger"]
  #[inline(always)]
  pub fn trot_1(self) -> &'a mut W {
    self.variant(TROT_A::TROT_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
    self.w
  }
}
#[doc = "Trigger Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRG_SRC_A {
  #[doc = "0: Selects external triggers"]
  TRG_SRC_0,
  #[doc = "1: Selects internal triggers"]
  TRG_SRC_1,
}
impl From<TRG_SRC_A> for bool {
  #[inline(always)]
  fn from(variant: TRG_SRC_A) -> Self {
    match variant {
      TRG_SRC_A::TRG_SRC_0 => false,
      TRG_SRC_A::TRG_SRC_1 => true,
    }
  }
}
#[doc = "Reader of field `TRG_SRC`"]
pub type TRG_SRC_R = crate::R<bool, TRG_SRC_A>;
impl TRG_SRC_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> TRG_SRC_A {
    match self.bits {
      false => TRG_SRC_A::TRG_SRC_0,
      true => TRG_SRC_A::TRG_SRC_1,
    }
  }
  #[doc = "Checks if the value of the field is `TRG_SRC_0`"]
  #[inline(always)]
  pub fn is_trg_src_0(&self) -> bool {
    *self == TRG_SRC_A::TRG_SRC_0
  }
  #[doc = "Checks if the value of the field is `TRG_SRC_1`"]
  #[inline(always)]
  pub fn is_trg_src_1(&self) -> bool {
    *self == TRG_SRC_A::TRG_SRC_1
  }
}
#[doc = "Write proxy for field `TRG_SRC`"]
pub struct TRG_SRC_W<'a> {
  w: &'a mut W,
}
impl<'a> TRG_SRC_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: TRG_SRC_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Selects external triggers"]
  #[inline(always)]
  pub fn trg_src_0(self) -> &'a mut W {
    self.variant(TRG_SRC_A::TRG_SRC_0)
  }
  #[doc = "Selects internal triggers"]
  #[inline(always)]
  pub fn trg_src_1(self) -> &'a mut W {
    self.variant(TRG_SRC_A::TRG_SRC_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
    self.w
  }
}
#[doc = "Trigger Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRG_SEL_A {
  #[doc = "0: Timer channel 0 - 3 trigger source is selected"]
  TRG_SEL_0,
  #[doc = "1: Timer channel 0 - 3 trigger source is selected"]
  TRG_SEL_1,
  #[doc = "2: Timer channel 0 - 3 trigger source is selected"]
  TRG_SEL_2,
  #[doc = "3: Timer channel 0 - 3 trigger source is selected"]
  TRG_SEL_3,
}
impl From<TRG_SEL_A> for u8 {
  #[inline(always)]
  fn from(variant: TRG_SEL_A) -> Self {
    match variant {
      TRG_SEL_A::TRG_SEL_0 => 0,
      TRG_SEL_A::TRG_SEL_1 => 1,
      TRG_SEL_A::TRG_SEL_2 => 2,
      TRG_SEL_A::TRG_SEL_3 => 3,
    }
  }
}
#[doc = "Reader of field `TRG_SEL`"]
pub type TRG_SEL_R = crate::R<u8, TRG_SEL_A>;
impl TRG_SEL_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> crate::Variant<u8, TRG_SEL_A> {
    use crate::Variant::*;
    match self.bits {
      0 => Val(TRG_SEL_A::TRG_SEL_0),
      1 => Val(TRG_SEL_A::TRG_SEL_1),
      2 => Val(TRG_SEL_A::TRG_SEL_2),
      3 => Val(TRG_SEL_A::TRG_SEL_3),
      i => Res(i),
    }
  }
  #[doc = "Checks if the value of the field is `TRG_SEL_0`"]
  #[inline(always)]
  pub fn is_trg_sel_0(&self) -> bool {
    *self == TRG_SEL_A::TRG_SEL_0
  }
  #[doc = "Checks if the value of the field is `TRG_SEL_1`"]
  #[inline(always)]
  pub fn is_trg_sel_1(&self) -> bool {
    *self == TRG_SEL_A::TRG_SEL_1
  }
  #[doc = "Checks if the value of the field is `TRG_SEL_2`"]
  #[inline(always)]
  pub fn is_trg_sel_2(&self) -> bool {
    *self == TRG_SEL_A::TRG_SEL_2
  }
  #[doc = "Checks if the value of the field is `TRG_SEL_3`"]
  #[inline(always)]
  pub fn is_trg_sel_3(&self) -> bool {
    *self == TRG_SEL_A::TRG_SEL_3
  }
}
#[doc = "Write proxy for field `TRG_SEL`"]
pub struct TRG_SEL_W<'a> {
  w: &'a mut W,
}
impl<'a> TRG_SEL_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: TRG_SEL_A) -> &'a mut W {
    unsafe { self.bits(variant.into()) }
  }
  #[doc = "Timer channel 0 - 3 trigger source is selected"]
  #[inline(always)]
  pub fn trg_sel_0(self) -> &'a mut W {
    self.variant(TRG_SEL_A::TRG_SEL_0)
  }
  #[doc = "Timer channel 0 - 3 trigger source is selected"]
  #[inline(always)]
  pub fn trg_sel_1(self) -> &'a mut W {
    self.variant(TRG_SEL_A::TRG_SEL_1)
  }
  #[doc = "Timer channel 0 - 3 trigger source is selected"]
  #[inline(always)]
  pub fn trg_sel_2(self) -> &'a mut W {
    self.variant(TRG_SEL_A::TRG_SEL_2)
  }
  #[doc = "Timer channel 0 - 3 trigger source is selected"]
  #[inline(always)]
  pub fn trg_sel_3(self) -> &'a mut W {
    self.variant(TRG_SEL_A::TRG_SEL_3)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
    self.w
  }
}
impl R {
  #[doc = "Bit 0 - Timer Enable"]
  #[inline(always)]
  pub fn t_en(&self) -> T_EN_R {
    T_EN_R::new((self.bits & 0x01) != 0)
  }
  #[doc = "Bit 1 - Chain Channel"]
  #[inline(always)]
  pub fn chain(&self) -> CHAIN_R {
    CHAIN_R::new(((self.bits >> 1) & 0x01) != 0)
  }
  #[doc = "Bits 2:3 - Timer Operation Mode"]
  #[inline(always)]
  pub fn mode(&self) -> MODE_R {
    MODE_R::new(((self.bits >> 2) & 0x03) as u8)
  }
  #[doc = "Bit 16 - Timer Start On Trigger"]
  #[inline(always)]
  pub fn tsot(&self) -> TSOT_R {
    TSOT_R::new(((self.bits >> 16) & 0x01) != 0)
  }
  #[doc = "Bit 17 - Timer Stop On Interrupt"]
  #[inline(always)]
  pub fn tsoi(&self) -> TSOI_R {
    TSOI_R::new(((self.bits >> 17) & 0x01) != 0)
  }
  #[doc = "Bit 18 - Timer Reload On Trigger"]
  #[inline(always)]
  pub fn trot(&self) -> TROT_R {
    TROT_R::new(((self.bits >> 18) & 0x01) != 0)
  }
  #[doc = "Bit 23 - Trigger Source"]
  #[inline(always)]
  pub fn trg_src(&self) -> TRG_SRC_R {
    TRG_SRC_R::new(((self.bits >> 23) & 0x01) != 0)
  }
  #[doc = "Bits 24:27 - Trigger Select"]
  #[inline(always)]
  pub fn trg_sel(&self) -> TRG_SEL_R {
    TRG_SEL_R::new(((self.bits >> 24) & 0x0f) as u8)
  }
}
impl W {
  #[doc = "Bit 0 - Timer Enable"]
  #[inline(always)]
  pub fn t_en(&mut self) -> T_EN_W {
    T_EN_W { w: self }
  }
  #[doc = "Bit 1 - Chain Channel"]
  #[inline(always)]
  pub fn chain(&mut self) -> CHAIN_W {
    CHAIN_W { w: self }
  }
  #[doc = "Bits 2:3 - Timer Operation Mode"]
  #[inline(always)]
  pub fn mode(&mut self) -> MODE_W {
    MODE_W { w: self }
  }
  #[doc = "Bit 16 - Timer Start On Trigger"]
  #[inline(always)]
  pub fn tsot(&mut self) -> TSOT_W {
    TSOT_W { w: self }
  }
  #[doc = "Bit 17 - Timer Stop On Interrupt"]
  #[inline(always)]
  pub fn tsoi(&mut self) -> TSOI_W {
    TSOI_W { w: self }
  }
  #[doc = "Bit 18 - Timer Reload On Trigger"]
  #[inline(always)]
  pub fn trot(&mut self) -> TROT_W {
    TROT_W { w: self }
  }
  #[doc = "Bit 23 - Trigger Source"]
  #[inline(always)]
  pub fn trg_src(&mut self) -> TRG_SRC_W {
    TRG_SRC_W { w: self }
  }
  #[doc = "Bits 24:27 - Trigger Select"]
  #[inline(always)]
  pub fn trg_sel(&mut self) -> TRG_SEL_W {
    TRG_SEL_W { w: self }
  }
}
