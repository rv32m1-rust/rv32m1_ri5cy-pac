#[doc = "Reader of register PARAM"]
pub type R = crate::R<u32, super::PARAM>;
#[doc = "DAC resolution\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DAC_RES_A {
  #[doc = "0: 4 bit DAC"]
  DAC_RES_0,
  #[doc = "1: 6 bit DAC"]
  DAC_RES_1,
  #[doc = "2: 8 bit DAC"]
  DAC_RES_2,
  #[doc = "3: 10 bit DAC"]
  DAC_RES_3,
  #[doc = "4: 12 bit DAC"]
  DAC_RES_4,
  #[doc = "5: 14 bit DAC"]
  DAC_RES_5,
  #[doc = "6: 16 bit DAC"]
  DAC_RES_6,
}
impl From<DAC_RES_A> for u8 {
  #[inline(always)]
  fn from(variant: DAC_RES_A) -> Self {
    match variant {
      DAC_RES_A::DAC_RES_0 => 0,
      DAC_RES_A::DAC_RES_1 => 1,
      DAC_RES_A::DAC_RES_2 => 2,
      DAC_RES_A::DAC_RES_3 => 3,
      DAC_RES_A::DAC_RES_4 => 4,
      DAC_RES_A::DAC_RES_5 => 5,
      DAC_RES_A::DAC_RES_6 => 6,
    }
  }
}
#[doc = "Reader of field `DAC_RES`"]
pub type DAC_RES_R = crate::R<u8, DAC_RES_A>;
impl DAC_RES_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> crate::Variant<u8, DAC_RES_A> {
    use crate::Variant::*;
    match self.bits {
      0 => Val(DAC_RES_A::DAC_RES_0),
      1 => Val(DAC_RES_A::DAC_RES_1),
      2 => Val(DAC_RES_A::DAC_RES_2),
      3 => Val(DAC_RES_A::DAC_RES_3),
      4 => Val(DAC_RES_A::DAC_RES_4),
      5 => Val(DAC_RES_A::DAC_RES_5),
      6 => Val(DAC_RES_A::DAC_RES_6),
      i => Res(i),
    }
  }
  #[doc = "Checks if the value of the field is `DAC_RES_0`"]
  #[inline(always)]
  pub fn is_dac_res_0(&self) -> bool {
    *self == DAC_RES_A::DAC_RES_0
  }
  #[doc = "Checks if the value of the field is `DAC_RES_1`"]
  #[inline(always)]
  pub fn is_dac_res_1(&self) -> bool {
    *self == DAC_RES_A::DAC_RES_1
  }
  #[doc = "Checks if the value of the field is `DAC_RES_2`"]
  #[inline(always)]
  pub fn is_dac_res_2(&self) -> bool {
    *self == DAC_RES_A::DAC_RES_2
  }
  #[doc = "Checks if the value of the field is `DAC_RES_3`"]
  #[inline(always)]
  pub fn is_dac_res_3(&self) -> bool {
    *self == DAC_RES_A::DAC_RES_3
  }
  #[doc = "Checks if the value of the field is `DAC_RES_4`"]
  #[inline(always)]
  pub fn is_dac_res_4(&self) -> bool {
    *self == DAC_RES_A::DAC_RES_4
  }
  #[doc = "Checks if the value of the field is `DAC_RES_5`"]
  #[inline(always)]
  pub fn is_dac_res_5(&self) -> bool {
    *self == DAC_RES_A::DAC_RES_5
  }
  #[doc = "Checks if the value of the field is `DAC_RES_6`"]
  #[inline(always)]
  pub fn is_dac_res_6(&self) -> bool {
    *self == DAC_RES_A::DAC_RES_6
  }
}
impl R {
  #[doc = "Bits 0:3 - DAC resolution"]
  #[inline(always)]
  pub fn dac_res(&self) -> DAC_RES_R {
    DAC_RES_R::new((self.bits & 0x0f) as u8)
  }
}
