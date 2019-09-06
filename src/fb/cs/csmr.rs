#[doc = "Reader of register CSMR"]
pub type R = crate::R<u32, super::CSMR>;
#[doc = "Writer for register CSMR"]
pub type W = crate::W<u32, super::CSMR>;
#[doc = "Register CSMR `reset()`'s with value 0"]
impl crate::ResetValue for super::CSMR {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "Valid\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum V_A {
  #[doc = "0: Chip-select is invalid."]
  V_0,
  #[doc = "1: Chip-select is valid."]
  V_1,
}
impl From<V_A> for bool {
  #[inline(always)]
  fn from(variant: V_A) -> Self {
    match variant {
      V_A::V_0 => false,
      V_A::V_1 => true,
    }
  }
}
#[doc = "Reader of field `V`"]
pub type V_R = crate::R<bool, V_A>;
impl V_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> V_A {
    match self.bits {
      false => V_A::V_0,
      true => V_A::V_1,
    }
  }
  #[doc = "Checks if the value of the field is `V_0`"]
  #[inline(always)]
  pub fn is_v_0(&self) -> bool {
    *self == V_A::V_0
  }
  #[doc = "Checks if the value of the field is `V_1`"]
  #[inline(always)]
  pub fn is_v_1(&self) -> bool {
    *self == V_A::V_1
  }
}
#[doc = "Write proxy for field `V`"]
pub struct V_W<'a> {
  w: &'a mut W,
}
impl<'a> V_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: V_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Chip-select is invalid."]
  #[inline(always)]
  pub fn v_0(self) -> &'a mut W {
    self.variant(V_A::V_0)
  }
  #[doc = "Chip-select is valid."]
  #[inline(always)]
  pub fn v_1(self) -> &'a mut W {
    self.variant(V_A::V_1)
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
#[doc = "Write Protect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WP_A {
  #[doc = "0: Write accesses are allowed."]
  WP_0,
  #[doc = "1: Write accesses are not allowed. Attempting to write to the range of addresses for which the WP bit is set results in a bus error termination of the internal cycle and no external cycle."]
  WP_1,
}
impl From<WP_A> for bool {
  #[inline(always)]
  fn from(variant: WP_A) -> Self {
    match variant {
      WP_A::WP_0 => false,
      WP_A::WP_1 => true,
    }
  }
}
#[doc = "Reader of field `WP`"]
pub type WP_R = crate::R<bool, WP_A>;
impl WP_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> WP_A {
    match self.bits {
      false => WP_A::WP_0,
      true => WP_A::WP_1,
    }
  }
  #[doc = "Checks if the value of the field is `WP_0`"]
  #[inline(always)]
  pub fn is_wp_0(&self) -> bool {
    *self == WP_A::WP_0
  }
  #[doc = "Checks if the value of the field is `WP_1`"]
  #[inline(always)]
  pub fn is_wp_1(&self) -> bool {
    *self == WP_A::WP_1
  }
}
#[doc = "Write proxy for field `WP`"]
pub struct WP_W<'a> {
  w: &'a mut W,
}
impl<'a> WP_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WP_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Write accesses are allowed."]
  #[inline(always)]
  pub fn wp_0(self) -> &'a mut W {
    self.variant(WP_A::WP_0)
  }
  #[doc = "Write accesses are not allowed. Attempting to write to the range of addresses for which the WP bit is set results in a bus error termination of the internal cycle and no external cycle."]
  #[inline(always)]
  pub fn wp_1(self) -> &'a mut W {
    self.variant(WP_A::WP_1)
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
#[doc = "Base Address Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BAM_A {
  #[doc = "0: The corresponding address bit in CSAR is used in the chip-select decode."]
  BAM_0,
  #[doc = "1: The corresponding address bit in CSAR is a don't care in the chip-select decode."]
  BAM_1,
}
impl From<BAM_A> for u16 {
  #[inline(always)]
  fn from(variant: BAM_A) -> Self {
    match variant {
      BAM_A::BAM_0 => 0,
      BAM_A::BAM_1 => 1,
    }
  }
}
#[doc = "Reader of field `BAM`"]
pub type BAM_R = crate::R<u16, BAM_A>;
impl BAM_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> crate::Variant<u16, BAM_A> {
    use crate::Variant::*;
    match self.bits {
      0 => Val(BAM_A::BAM_0),
      1 => Val(BAM_A::BAM_1),
      i => Res(i),
    }
  }
  #[doc = "Checks if the value of the field is `BAM_0`"]
  #[inline(always)]
  pub fn is_bam_0(&self) -> bool {
    *self == BAM_A::BAM_0
  }
  #[doc = "Checks if the value of the field is `BAM_1`"]
  #[inline(always)]
  pub fn is_bam_1(&self) -> bool {
    *self == BAM_A::BAM_1
  }
}
#[doc = "Write proxy for field `BAM`"]
pub struct BAM_W<'a> {
  w: &'a mut W,
}
impl<'a> BAM_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: BAM_A) -> &'a mut W {
    unsafe { self.bits(variant.into()) }
  }
  #[doc = "The corresponding address bit in CSAR is used in the chip-select decode."]
  #[inline(always)]
  pub fn bam_0(self) -> &'a mut W {
    self.variant(BAM_A::BAM_0)
  }
  #[doc = "The corresponding address bit in CSAR is a don't care in the chip-select decode."]
  #[inline(always)]
  pub fn bam_1(self) -> &'a mut W {
    self.variant(BAM_A::BAM_1)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u16) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
    self.w
  }
}
impl R {
  #[doc = "Bit 0 - Valid"]
  #[inline(always)]
  pub fn v(&self) -> V_R {
    V_R::new((self.bits & 0x01) != 0)
  }
  #[doc = "Bit 8 - Write Protect"]
  #[inline(always)]
  pub fn wp(&self) -> WP_R {
    WP_R::new(((self.bits >> 8) & 0x01) != 0)
  }
  #[doc = "Bits 16:31 - Base Address Mask"]
  #[inline(always)]
  pub fn bam(&self) -> BAM_R {
    BAM_R::new(((self.bits >> 16) & 0xffff) as u16)
  }
}
impl W {
  #[doc = "Bit 0 - Valid"]
  #[inline(always)]
  pub fn v(&mut self) -> V_W {
    V_W { w: self }
  }
  #[doc = "Bit 8 - Write Protect"]
  #[inline(always)]
  pub fn wp(&mut self) -> WP_W {
    WP_W { w: self }
  }
  #[doc = "Bits 16:31 - Base Address Mask"]
  #[inline(always)]
  pub fn bam(&mut self) -> BAM_W {
    BAM_W { w: self }
  }
}
