#[doc = "Reader of register CSCR"]
pub type R = crate::R<u32, super::CSCR>;
#[doc = "Writer for register CSCR"]
pub type W = crate::W<u32, super::CSCR>;
#[doc = "Register CSCR `reset()`'s with value 0"]
impl crate::ResetValue for super::CSCR {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "Burst-Write Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BSTW_A {
  #[doc = "0: Disabled. Data exceeding the specified port size is broken into individual, port-sized, non-burst writes. For example, a 32-bit write to an 8-bit port takes four byte writes."]
  BSTW_0,
  #[doc = "1: Enabled. Enables burst write of data larger than the specified port size, including 32-bit writes to 8- and 16-bit ports, 16-bit writes to 8-bit ports, and line writes to 8-, 16-, and 32-bit ports."]
  BSTW_1,
}
impl From<BSTW_A> for bool {
  #[inline(always)]
  fn from(variant: BSTW_A) -> Self {
    match variant {
      BSTW_A::BSTW_0 => false,
      BSTW_A::BSTW_1 => true,
    }
  }
}
#[doc = "Reader of field `BSTW`"]
pub type BSTW_R = crate::R<bool, BSTW_A>;
impl BSTW_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> BSTW_A {
    match self.bits {
      false => BSTW_A::BSTW_0,
      true => BSTW_A::BSTW_1,
    }
  }
  #[doc = "Checks if the value of the field is `BSTW_0`"]
  #[inline(always)]
  pub fn is_bstw_0(&self) -> bool {
    *self == BSTW_A::BSTW_0
  }
  #[doc = "Checks if the value of the field is `BSTW_1`"]
  #[inline(always)]
  pub fn is_bstw_1(&self) -> bool {
    *self == BSTW_A::BSTW_1
  }
}
#[doc = "Write proxy for field `BSTW`"]
pub struct BSTW_W<'a> {
  w: &'a mut W,
}
impl<'a> BSTW_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: BSTW_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Disabled. Data exceeding the specified port size is broken into individual, port-sized, non-burst writes. For example, a 32-bit write to an 8-bit port takes four byte writes."]
  #[inline(always)]
  pub fn bstw_0(self) -> &'a mut W {
    self.variant(BSTW_A::BSTW_0)
  }
  #[doc = "Enabled. Enables burst write of data larger than the specified port size, including 32-bit writes to 8- and 16-bit ports, 16-bit writes to 8-bit ports, and line writes to 8-, 16-, and 32-bit ports."]
  #[inline(always)]
  pub fn bstw_1(self) -> &'a mut W {
    self.variant(BSTW_A::BSTW_1)
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
#[doc = "Burst-Read Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BSTR_A {
  #[doc = "0: Disabled. Data exceeding the specified port size is broken into individual, port-sized, non-burst reads. For example, a 32-bit read from an 8-bit port is broken into four 8-bit reads."]
  BSTR_0,
  #[doc = "1: Enabled. Enables data burst reads larger than the specified port size, including 32-bit reads from 8- and 16-bit ports, 16-bit reads from 8-bit ports, and line reads from 8-, 16-, and 32-bit ports."]
  BSTR_1,
}
impl From<BSTR_A> for bool {
  #[inline(always)]
  fn from(variant: BSTR_A) -> Self {
    match variant {
      BSTR_A::BSTR_0 => false,
      BSTR_A::BSTR_1 => true,
    }
  }
}
#[doc = "Reader of field `BSTR`"]
pub type BSTR_R = crate::R<bool, BSTR_A>;
impl BSTR_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> BSTR_A {
    match self.bits {
      false => BSTR_A::BSTR_0,
      true => BSTR_A::BSTR_1,
    }
  }
  #[doc = "Checks if the value of the field is `BSTR_0`"]
  #[inline(always)]
  pub fn is_bstr_0(&self) -> bool {
    *self == BSTR_A::BSTR_0
  }
  #[doc = "Checks if the value of the field is `BSTR_1`"]
  #[inline(always)]
  pub fn is_bstr_1(&self) -> bool {
    *self == BSTR_A::BSTR_1
  }
}
#[doc = "Write proxy for field `BSTR`"]
pub struct BSTR_W<'a> {
  w: &'a mut W,
}
impl<'a> BSTR_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: BSTR_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Disabled. Data exceeding the specified port size is broken into individual, port-sized, non-burst reads. For example, a 32-bit read from an 8-bit port is broken into four 8-bit reads."]
  #[inline(always)]
  pub fn bstr_0(self) -> &'a mut W {
    self.variant(BSTR_A::BSTR_0)
  }
  #[doc = "Enabled. Enables data burst reads larger than the specified port size, including 32-bit reads from 8- and 16-bit ports, 16-bit reads from 8-bit ports, and line reads from 8-, 16-, and 32-bit ports."]
  #[inline(always)]
  pub fn bstr_1(self) -> &'a mut W {
    self.variant(BSTR_A::BSTR_1)
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
#[doc = "Byte-Enable Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BEM_A {
  #[doc = "0: no description available"]
  BEM_0,
  #[doc = "1: no description available"]
  BEM_1,
}
impl From<BEM_A> for bool {
  #[inline(always)]
  fn from(variant: BEM_A) -> Self {
    match variant {
      BEM_A::BEM_0 => false,
      BEM_A::BEM_1 => true,
    }
  }
}
#[doc = "Reader of field `BEM`"]
pub type BEM_R = crate::R<bool, BEM_A>;
impl BEM_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> BEM_A {
    match self.bits {
      false => BEM_A::BEM_0,
      true => BEM_A::BEM_1,
    }
  }
  #[doc = "Checks if the value of the field is `BEM_0`"]
  #[inline(always)]
  pub fn is_bem_0(&self) -> bool {
    *self == BEM_A::BEM_0
  }
  #[doc = "Checks if the value of the field is `BEM_1`"]
  #[inline(always)]
  pub fn is_bem_1(&self) -> bool {
    *self == BEM_A::BEM_1
  }
}
#[doc = "Write proxy for field `BEM`"]
pub struct BEM_W<'a> {
  w: &'a mut W,
}
impl<'a> BEM_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: BEM_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "no description available"]
  #[inline(always)]
  pub fn bem_0(self) -> &'a mut W {
    self.variant(BEM_A::BEM_0)
  }
  #[doc = "no description available"]
  #[inline(always)]
  pub fn bem_1(self) -> &'a mut W {
    self.variant(BEM_A::BEM_1)
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
#[doc = "Port Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PS_A {
  #[doc = "0: 32-bit port size. Valid data is sampled and driven on FB_D\\[31:0\\]."]
  PS_0,
  #[doc = "1: no description available"]
  PS_1,
  #[doc = "2: no description available"]
  PS_2,
}
impl From<PS_A> for u8 {
  #[inline(always)]
  fn from(variant: PS_A) -> Self {
    match variant {
      PS_A::PS_0 => 0,
      PS_A::PS_1 => 1,
      PS_A::PS_2 => 2,
    }
  }
}
#[doc = "Reader of field `PS`"]
pub type PS_R = crate::R<u8, PS_A>;
impl PS_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> crate::Variant<u8, PS_A> {
    use crate::Variant::*;
    match self.bits {
      0 => Val(PS_A::PS_0),
      1 => Val(PS_A::PS_1),
      2 => Val(PS_A::PS_2),
      i => Res(i),
    }
  }
  #[doc = "Checks if the value of the field is `PS_0`"]
  #[inline(always)]
  pub fn is_ps_0(&self) -> bool {
    *self == PS_A::PS_0
  }
  #[doc = "Checks if the value of the field is `PS_1`"]
  #[inline(always)]
  pub fn is_ps_1(&self) -> bool {
    *self == PS_A::PS_1
  }
  #[doc = "Checks if the value of the field is `PS_2`"]
  #[inline(always)]
  pub fn is_ps_2(&self) -> bool {
    *self == PS_A::PS_2
  }
}
#[doc = "Write proxy for field `PS`"]
pub struct PS_W<'a> {
  w: &'a mut W,
}
impl<'a> PS_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: PS_A) -> &'a mut W {
    unsafe { self.bits(variant.into()) }
  }
  #[doc = "32-bit port size. Valid data is sampled and driven on FB_D\\[31:0\\]."]
  #[inline(always)]
  pub fn ps_0(self) -> &'a mut W {
    self.variant(PS_A::PS_0)
  }
  #[doc = "no description available"]
  #[inline(always)]
  pub fn ps_1(self) -> &'a mut W {
    self.variant(PS_A::PS_1)
  }
  #[doc = "no description available"]
  #[inline(always)]
  pub fn ps_2(self) -> &'a mut W {
    self.variant(PS_A::PS_2)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
    self.w
  }
}
#[doc = "Auto-Acknowledge Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AA_A {
  #[doc = "0: Disabled. No internal transfer acknowledge is asserted and the cycle is terminated externally."]
  AA_0,
  #[doc = "1: Enabled. Internal transfer acknowledge is asserted as specified by WS."]
  AA_1,
}
impl From<AA_A> for bool {
  #[inline(always)]
  fn from(variant: AA_A) -> Self {
    match variant {
      AA_A::AA_0 => false,
      AA_A::AA_1 => true,
    }
  }
}
#[doc = "Reader of field `AA`"]
pub type AA_R = crate::R<bool, AA_A>;
impl AA_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> AA_A {
    match self.bits {
      false => AA_A::AA_0,
      true => AA_A::AA_1,
    }
  }
  #[doc = "Checks if the value of the field is `AA_0`"]
  #[inline(always)]
  pub fn is_aa_0(&self) -> bool {
    *self == AA_A::AA_0
  }
  #[doc = "Checks if the value of the field is `AA_1`"]
  #[inline(always)]
  pub fn is_aa_1(&self) -> bool {
    *self == AA_A::AA_1
  }
}
#[doc = "Write proxy for field `AA`"]
pub struct AA_W<'a> {
  w: &'a mut W,
}
impl<'a> AA_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: AA_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Disabled. No internal transfer acknowledge is asserted and the cycle is terminated externally."]
  #[inline(always)]
  pub fn aa_0(self) -> &'a mut W {
    self.variant(AA_A::AA_0)
  }
  #[doc = "Enabled. Internal transfer acknowledge is asserted as specified by WS."]
  #[inline(always)]
  pub fn aa_1(self) -> &'a mut W {
    self.variant(AA_A::AA_1)
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
#[doc = "Byte-Lane Shift\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BLS_A {
  #[doc = "0: Not shifted. Data is left-aligned on FB_AD."]
  BLS_0,
  #[doc = "1: Shifted. Data is right-aligned on FB_AD."]
  BLS_1,
}
impl From<BLS_A> for bool {
  #[inline(always)]
  fn from(variant: BLS_A) -> Self {
    match variant {
      BLS_A::BLS_0 => false,
      BLS_A::BLS_1 => true,
    }
  }
}
#[doc = "Reader of field `BLS`"]
pub type BLS_R = crate::R<bool, BLS_A>;
impl BLS_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> BLS_A {
    match self.bits {
      false => BLS_A::BLS_0,
      true => BLS_A::BLS_1,
    }
  }
  #[doc = "Checks if the value of the field is `BLS_0`"]
  #[inline(always)]
  pub fn is_bls_0(&self) -> bool {
    *self == BLS_A::BLS_0
  }
  #[doc = "Checks if the value of the field is `BLS_1`"]
  #[inline(always)]
  pub fn is_bls_1(&self) -> bool {
    *self == BLS_A::BLS_1
  }
}
#[doc = "Write proxy for field `BLS`"]
pub struct BLS_W<'a> {
  w: &'a mut W,
}
impl<'a> BLS_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: BLS_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Not shifted. Data is left-aligned on FB_AD."]
  #[inline(always)]
  pub fn bls_0(self) -> &'a mut W {
    self.variant(BLS_A::BLS_0)
  }
  #[doc = "Shifted. Data is right-aligned on FB_AD."]
  #[inline(always)]
  pub fn bls_1(self) -> &'a mut W {
    self.variant(BLS_A::BLS_1)
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
#[doc = "Reader of field `WS`"]
pub type WS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WS`"]
pub struct WS_W<'a> {
  w: &'a mut W,
}
impl<'a> WS_W<'a> {
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x3f << 10)) | (((value as u32) & 0x3f) << 10);
    self.w
  }
}
#[doc = "Write Address Hold or Deselect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WRAH_A {
  #[doc = "0: no description available"]
  WRAH_0,
  #[doc = "1: 2 cycles"]
  WRAH_1,
  #[doc = "2: 3 cycles"]
  WRAH_2,
  #[doc = "3: no description available"]
  WRAH_3,
}
impl From<WRAH_A> for u8 {
  #[inline(always)]
  fn from(variant: WRAH_A) -> Self {
    match variant {
      WRAH_A::WRAH_0 => 0,
      WRAH_A::WRAH_1 => 1,
      WRAH_A::WRAH_2 => 2,
      WRAH_A::WRAH_3 => 3,
    }
  }
}
#[doc = "Reader of field `WRAH`"]
pub type WRAH_R = crate::R<u8, WRAH_A>;
impl WRAH_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> WRAH_A {
    match self.bits {
      0 => WRAH_A::WRAH_0,
      1 => WRAH_A::WRAH_1,
      2 => WRAH_A::WRAH_2,
      3 => WRAH_A::WRAH_3,
      _ => unreachable!(),
    }
  }
  #[doc = "Checks if the value of the field is `WRAH_0`"]
  #[inline(always)]
  pub fn is_wrah_0(&self) -> bool {
    *self == WRAH_A::WRAH_0
  }
  #[doc = "Checks if the value of the field is `WRAH_1`"]
  #[inline(always)]
  pub fn is_wrah_1(&self) -> bool {
    *self == WRAH_A::WRAH_1
  }
  #[doc = "Checks if the value of the field is `WRAH_2`"]
  #[inline(always)]
  pub fn is_wrah_2(&self) -> bool {
    *self == WRAH_A::WRAH_2
  }
  #[doc = "Checks if the value of the field is `WRAH_3`"]
  #[inline(always)]
  pub fn is_wrah_3(&self) -> bool {
    *self == WRAH_A::WRAH_3
  }
}
#[doc = "Write proxy for field `WRAH`"]
pub struct WRAH_W<'a> {
  w: &'a mut W,
}
impl<'a> WRAH_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WRAH_A) -> &'a mut W {
    {
      self.bits(variant.into())
    }
  }
  #[doc = "no description available"]
  #[inline(always)]
  pub fn wrah_0(self) -> &'a mut W {
    self.variant(WRAH_A::WRAH_0)
  }
  #[doc = "2 cycles"]
  #[inline(always)]
  pub fn wrah_1(self) -> &'a mut W {
    self.variant(WRAH_A::WRAH_1)
  }
  #[doc = "3 cycles"]
  #[inline(always)]
  pub fn wrah_2(self) -> &'a mut W {
    self.variant(WRAH_A::WRAH_2)
  }
  #[doc = "no description available"]
  #[inline(always)]
  pub fn wrah_3(self) -> &'a mut W {
    self.variant(WRAH_A::WRAH_3)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
    self.w
  }
}
#[doc = "Read Address Hold or Deselect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDAH_A {
  #[doc = "0: When AA is 1b, 1 cycle. When AA is 0b, 0 cycles."]
  RDAH_0,
  #[doc = "1: When AA is 1b, 2 cycles. When AA is 0b, 1 cycle."]
  RDAH_1,
  #[doc = "2: When AA is 1b, 3 cycles. When AA is 0b, 2 cycles."]
  RDAH_2,
  #[doc = "3: When AA is 1b, 4 cycles. When AA is 0b, 3 cycles."]
  RDAH_3,
}
impl From<RDAH_A> for u8 {
  #[inline(always)]
  fn from(variant: RDAH_A) -> Self {
    match variant {
      RDAH_A::RDAH_0 => 0,
      RDAH_A::RDAH_1 => 1,
      RDAH_A::RDAH_2 => 2,
      RDAH_A::RDAH_3 => 3,
    }
  }
}
#[doc = "Reader of field `RDAH`"]
pub type RDAH_R = crate::R<u8, RDAH_A>;
impl RDAH_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> RDAH_A {
    match self.bits {
      0 => RDAH_A::RDAH_0,
      1 => RDAH_A::RDAH_1,
      2 => RDAH_A::RDAH_2,
      3 => RDAH_A::RDAH_3,
      _ => unreachable!(),
    }
  }
  #[doc = "Checks if the value of the field is `RDAH_0`"]
  #[inline(always)]
  pub fn is_rdah_0(&self) -> bool {
    *self == RDAH_A::RDAH_0
  }
  #[doc = "Checks if the value of the field is `RDAH_1`"]
  #[inline(always)]
  pub fn is_rdah_1(&self) -> bool {
    *self == RDAH_A::RDAH_1
  }
  #[doc = "Checks if the value of the field is `RDAH_2`"]
  #[inline(always)]
  pub fn is_rdah_2(&self) -> bool {
    *self == RDAH_A::RDAH_2
  }
  #[doc = "Checks if the value of the field is `RDAH_3`"]
  #[inline(always)]
  pub fn is_rdah_3(&self) -> bool {
    *self == RDAH_A::RDAH_3
  }
}
#[doc = "Write proxy for field `RDAH`"]
pub struct RDAH_W<'a> {
  w: &'a mut W,
}
impl<'a> RDAH_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: RDAH_A) -> &'a mut W {
    {
      self.bits(variant.into())
    }
  }
  #[doc = "When AA is 1b, 1 cycle. When AA is 0b, 0 cycles."]
  #[inline(always)]
  pub fn rdah_0(self) -> &'a mut W {
    self.variant(RDAH_A::RDAH_0)
  }
  #[doc = "When AA is 1b, 2 cycles. When AA is 0b, 1 cycle."]
  #[inline(always)]
  pub fn rdah_1(self) -> &'a mut W {
    self.variant(RDAH_A::RDAH_1)
  }
  #[doc = "When AA is 1b, 3 cycles. When AA is 0b, 2 cycles."]
  #[inline(always)]
  pub fn rdah_2(self) -> &'a mut W {
    self.variant(RDAH_A::RDAH_2)
  }
  #[doc = "When AA is 1b, 4 cycles. When AA is 0b, 3 cycles."]
  #[inline(always)]
  pub fn rdah_3(self) -> &'a mut W {
    self.variant(RDAH_A::RDAH_3)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
    self.w
  }
}
#[doc = "Address Setup\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ASET_A {
  #[doc = "0: no description available"]
  ASET_0,
  #[doc = "1: no description available"]
  ASET_1,
  #[doc = "2: no description available"]
  ASET_2,
  #[doc = "3: no description available"]
  ASET_3,
}
impl From<ASET_A> for u8 {
  #[inline(always)]
  fn from(variant: ASET_A) -> Self {
    match variant {
      ASET_A::ASET_0 => 0,
      ASET_A::ASET_1 => 1,
      ASET_A::ASET_2 => 2,
      ASET_A::ASET_3 => 3,
    }
  }
}
#[doc = "Reader of field `ASET`"]
pub type ASET_R = crate::R<u8, ASET_A>;
impl ASET_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> ASET_A {
    match self.bits {
      0 => ASET_A::ASET_0,
      1 => ASET_A::ASET_1,
      2 => ASET_A::ASET_2,
      3 => ASET_A::ASET_3,
      _ => unreachable!(),
    }
  }
  #[doc = "Checks if the value of the field is `ASET_0`"]
  #[inline(always)]
  pub fn is_aset_0(&self) -> bool {
    *self == ASET_A::ASET_0
  }
  #[doc = "Checks if the value of the field is `ASET_1`"]
  #[inline(always)]
  pub fn is_aset_1(&self) -> bool {
    *self == ASET_A::ASET_1
  }
  #[doc = "Checks if the value of the field is `ASET_2`"]
  #[inline(always)]
  pub fn is_aset_2(&self) -> bool {
    *self == ASET_A::ASET_2
  }
  #[doc = "Checks if the value of the field is `ASET_3`"]
  #[inline(always)]
  pub fn is_aset_3(&self) -> bool {
    *self == ASET_A::ASET_3
  }
}
#[doc = "Write proxy for field `ASET`"]
pub struct ASET_W<'a> {
  w: &'a mut W,
}
impl<'a> ASET_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: ASET_A) -> &'a mut W {
    {
      self.bits(variant.into())
    }
  }
  #[doc = "no description available"]
  #[inline(always)]
  pub fn aset_0(self) -> &'a mut W {
    self.variant(ASET_A::ASET_0)
  }
  #[doc = "no description available"]
  #[inline(always)]
  pub fn aset_1(self) -> &'a mut W {
    self.variant(ASET_A::ASET_1)
  }
  #[doc = "no description available"]
  #[inline(always)]
  pub fn aset_2(self) -> &'a mut W {
    self.variant(ASET_A::ASET_2)
  }
  #[doc = "no description available"]
  #[inline(always)]
  pub fn aset_3(self) -> &'a mut W {
    self.variant(ASET_A::ASET_3)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
    self.w
  }
}
#[doc = "EXTS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTS_A {
  #[doc = "0: no description available"]
  EXTS_0,
  #[doc = "1: no description available"]
  EXTS_1,
}
impl From<EXTS_A> for bool {
  #[inline(always)]
  fn from(variant: EXTS_A) -> Self {
    match variant {
      EXTS_A::EXTS_0 => false,
      EXTS_A::EXTS_1 => true,
    }
  }
}
#[doc = "Reader of field `EXTS`"]
pub type EXTS_R = crate::R<bool, EXTS_A>;
impl EXTS_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> EXTS_A {
    match self.bits {
      false => EXTS_A::EXTS_0,
      true => EXTS_A::EXTS_1,
    }
  }
  #[doc = "Checks if the value of the field is `EXTS_0`"]
  #[inline(always)]
  pub fn is_exts_0(&self) -> bool {
    *self == EXTS_A::EXTS_0
  }
  #[doc = "Checks if the value of the field is `EXTS_1`"]
  #[inline(always)]
  pub fn is_exts_1(&self) -> bool {
    *self == EXTS_A::EXTS_1
  }
}
#[doc = "Write proxy for field `EXTS`"]
pub struct EXTS_W<'a> {
  w: &'a mut W,
}
impl<'a> EXTS_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: EXTS_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "no description available"]
  #[inline(always)]
  pub fn exts_0(self) -> &'a mut W {
    self.variant(EXTS_A::EXTS_0)
  }
  #[doc = "no description available"]
  #[inline(always)]
  pub fn exts_1(self) -> &'a mut W {
    self.variant(EXTS_A::EXTS_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
    self.w
  }
}
#[doc = "Secondary Wait State Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWSEN_A {
  #[doc = "0: Disabled. A number of wait states (specified by WS) are inserted before an internal transfer acknowledge is generated for all transfers."]
  SWSEN_0,
  #[doc = "1: Enabled. A number of wait states (specified by SWS) are inserted before an internal transfer acknowledge is generated for burst transfer secondary terminations."]
  SWSEN_1,
}
impl From<SWSEN_A> for bool {
  #[inline(always)]
  fn from(variant: SWSEN_A) -> Self {
    match variant {
      SWSEN_A::SWSEN_0 => false,
      SWSEN_A::SWSEN_1 => true,
    }
  }
}
#[doc = "Reader of field `SWSEN`"]
pub type SWSEN_R = crate::R<bool, SWSEN_A>;
impl SWSEN_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> SWSEN_A {
    match self.bits {
      false => SWSEN_A::SWSEN_0,
      true => SWSEN_A::SWSEN_1,
    }
  }
  #[doc = "Checks if the value of the field is `SWSEN_0`"]
  #[inline(always)]
  pub fn is_swsen_0(&self) -> bool {
    *self == SWSEN_A::SWSEN_0
  }
  #[doc = "Checks if the value of the field is `SWSEN_1`"]
  #[inline(always)]
  pub fn is_swsen_1(&self) -> bool {
    *self == SWSEN_A::SWSEN_1
  }
}
#[doc = "Write proxy for field `SWSEN`"]
pub struct SWSEN_W<'a> {
  w: &'a mut W,
}
impl<'a> SWSEN_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: SWSEN_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Disabled. A number of wait states (specified by WS) are inserted before an internal transfer acknowledge is generated for all transfers."]
  #[inline(always)]
  pub fn swsen_0(self) -> &'a mut W {
    self.variant(SWSEN_A::SWSEN_0)
  }
  #[doc = "Enabled. A number of wait states (specified by SWS) are inserted before an internal transfer acknowledge is generated for burst transfer secondary terminations."]
  #[inline(always)]
  pub fn swsen_1(self) -> &'a mut W {
    self.variant(SWSEN_A::SWSEN_1)
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
#[doc = "Reader of field `SWS`"]
pub type SWS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SWS`"]
pub struct SWS_W<'a> {
  w: &'a mut W,
}
impl<'a> SWS_W<'a> {
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x3f << 26)) | (((value as u32) & 0x3f) << 26);
    self.w
  }
}
impl R {
  #[doc = "Bit 3 - Burst-Write Enable"]
  #[inline(always)]
  pub fn bstw(&self) -> BSTW_R {
    BSTW_R::new(((self.bits >> 3) & 0x01) != 0)
  }
  #[doc = "Bit 4 - Burst-Read Enable"]
  #[inline(always)]
  pub fn bstr(&self) -> BSTR_R {
    BSTR_R::new(((self.bits >> 4) & 0x01) != 0)
  }
  #[doc = "Bit 5 - Byte-Enable Mode"]
  #[inline(always)]
  pub fn bem(&self) -> BEM_R {
    BEM_R::new(((self.bits >> 5) & 0x01) != 0)
  }
  #[doc = "Bits 6:7 - Port Size"]
  #[inline(always)]
  pub fn ps(&self) -> PS_R {
    PS_R::new(((self.bits >> 6) & 0x03) as u8)
  }
  #[doc = "Bit 8 - Auto-Acknowledge Enable"]
  #[inline(always)]
  pub fn aa(&self) -> AA_R {
    AA_R::new(((self.bits >> 8) & 0x01) != 0)
  }
  #[doc = "Bit 9 - Byte-Lane Shift"]
  #[inline(always)]
  pub fn bls(&self) -> BLS_R {
    BLS_R::new(((self.bits >> 9) & 0x01) != 0)
  }
  #[doc = "Bits 10:15 - Wait States"]
  #[inline(always)]
  pub fn ws(&self) -> WS_R {
    WS_R::new(((self.bits >> 10) & 0x3f) as u8)
  }
  #[doc = "Bits 16:17 - Write Address Hold or Deselect"]
  #[inline(always)]
  pub fn wrah(&self) -> WRAH_R {
    WRAH_R::new(((self.bits >> 16) & 0x03) as u8)
  }
  #[doc = "Bits 18:19 - Read Address Hold or Deselect"]
  #[inline(always)]
  pub fn rdah(&self) -> RDAH_R {
    RDAH_R::new(((self.bits >> 18) & 0x03) as u8)
  }
  #[doc = "Bits 20:21 - Address Setup"]
  #[inline(always)]
  pub fn aset(&self) -> ASET_R {
    ASET_R::new(((self.bits >> 20) & 0x03) as u8)
  }
  #[doc = "Bit 22 - EXTS"]
  #[inline(always)]
  pub fn exts(&self) -> EXTS_R {
    EXTS_R::new(((self.bits >> 22) & 0x01) != 0)
  }
  #[doc = "Bit 23 - Secondary Wait State Enable"]
  #[inline(always)]
  pub fn swsen(&self) -> SWSEN_R {
    SWSEN_R::new(((self.bits >> 23) & 0x01) != 0)
  }
  #[doc = "Bits 26:31 - Secondary Wait States"]
  #[inline(always)]
  pub fn sws(&self) -> SWS_R {
    SWS_R::new(((self.bits >> 26) & 0x3f) as u8)
  }
}
impl W {
  #[doc = "Bit 3 - Burst-Write Enable"]
  #[inline(always)]
  pub fn bstw(&mut self) -> BSTW_W {
    BSTW_W { w: self }
  }
  #[doc = "Bit 4 - Burst-Read Enable"]
  #[inline(always)]
  pub fn bstr(&mut self) -> BSTR_W {
    BSTR_W { w: self }
  }
  #[doc = "Bit 5 - Byte-Enable Mode"]
  #[inline(always)]
  pub fn bem(&mut self) -> BEM_W {
    BEM_W { w: self }
  }
  #[doc = "Bits 6:7 - Port Size"]
  #[inline(always)]
  pub fn ps(&mut self) -> PS_W {
    PS_W { w: self }
  }
  #[doc = "Bit 8 - Auto-Acknowledge Enable"]
  #[inline(always)]
  pub fn aa(&mut self) -> AA_W {
    AA_W { w: self }
  }
  #[doc = "Bit 9 - Byte-Lane Shift"]
  #[inline(always)]
  pub fn bls(&mut self) -> BLS_W {
    BLS_W { w: self }
  }
  #[doc = "Bits 10:15 - Wait States"]
  #[inline(always)]
  pub fn ws(&mut self) -> WS_W {
    WS_W { w: self }
  }
  #[doc = "Bits 16:17 - Write Address Hold or Deselect"]
  #[inline(always)]
  pub fn wrah(&mut self) -> WRAH_W {
    WRAH_W { w: self }
  }
  #[doc = "Bits 18:19 - Read Address Hold or Deselect"]
  #[inline(always)]
  pub fn rdah(&mut self) -> RDAH_W {
    RDAH_W { w: self }
  }
  #[doc = "Bits 20:21 - Address Setup"]
  #[inline(always)]
  pub fn aset(&mut self) -> ASET_W {
    ASET_W { w: self }
  }
  #[doc = "Bit 22 - EXTS"]
  #[inline(always)]
  pub fn exts(&mut self) -> EXTS_W {
    EXTS_W { w: self }
  }
  #[doc = "Bit 23 - Secondary Wait State Enable"]
  #[inline(always)]
  pub fn swsen(&mut self) -> SWSEN_W {
    SWSEN_W { w: self }
  }
  #[doc = "Bits 26:31 - Secondary Wait States"]
  #[inline(always)]
  pub fn sws(&mut self) -> SWS_W {
    SWS_W { w: self }
  }
}
