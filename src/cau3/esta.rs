#[doc = "Reader of register ESTA"]
pub type R = crate::R<u32, super::ESTA>;
#[doc = "Error ID 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERRID1_A {
  #[doc = "1: Mode Error"]
  MODE_ERROR,
  #[doc = "2: PKHA N Register Size Error"]
  DATA_SIZE_ERROR,
  #[doc = "3: PKHA E Register Size Error"]
  KEY_SIZE_ERROR,
  #[doc = "4: PKHA A Register Size Error"]
  PKHA_A_SIZE_ERROR,
  #[doc = "5: PKHA B Register Size Error"]
  PKHA_B_SIZE_ERROR,
  #[doc = "6: PKHA C input (as contained in the PKHA B0 quadrant) is Zero"]
  DATA_OUT_OF_SEQ_ERROR,
  #[doc = "7: PKHA Divide by Zero Error"]
  PKHA_DIV_BY_0_ERROR,
  #[doc = "8: PKHA Modulus Even Error"]
  PKHA_MOD_EVEN_ERROR,
  #[doc = "15: Invalid Crypto Engine Selected"]
  INVALID_ENGINE_SEL_ERROR,
}
impl From<ERRID1_A> for u8 {
  #[inline(always)]
  fn from(variant: ERRID1_A) -> Self {
    match variant {
      ERRID1_A::MODE_ERROR => 1,
      ERRID1_A::DATA_SIZE_ERROR => 2,
      ERRID1_A::KEY_SIZE_ERROR => 3,
      ERRID1_A::PKHA_A_SIZE_ERROR => 4,
      ERRID1_A::PKHA_B_SIZE_ERROR => 5,
      ERRID1_A::DATA_OUT_OF_SEQ_ERROR => 6,
      ERRID1_A::PKHA_DIV_BY_0_ERROR => 7,
      ERRID1_A::PKHA_MOD_EVEN_ERROR => 8,
      ERRID1_A::INVALID_ENGINE_SEL_ERROR => 15,
    }
  }
}
#[doc = "Reader of field `ERRID1`"]
pub type ERRID1_R = crate::R<u8, ERRID1_A>;
impl ERRID1_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> crate::Variant<u8, ERRID1_A> {
    use crate::Variant::*;
    match self.bits {
      1 => Val(ERRID1_A::MODE_ERROR),
      2 => Val(ERRID1_A::DATA_SIZE_ERROR),
      3 => Val(ERRID1_A::KEY_SIZE_ERROR),
      4 => Val(ERRID1_A::PKHA_A_SIZE_ERROR),
      5 => Val(ERRID1_A::PKHA_B_SIZE_ERROR),
      6 => Val(ERRID1_A::DATA_OUT_OF_SEQ_ERROR),
      7 => Val(ERRID1_A::PKHA_DIV_BY_0_ERROR),
      8 => Val(ERRID1_A::PKHA_MOD_EVEN_ERROR),
      15 => Val(ERRID1_A::INVALID_ENGINE_SEL_ERROR),
      i => Res(i),
    }
  }
  #[doc = "Checks if the value of the field is `MODE_ERROR`"]
  #[inline(always)]
  pub fn is_mode_error(&self) -> bool {
    *self == ERRID1_A::MODE_ERROR
  }
  #[doc = "Checks if the value of the field is `DATA_SIZE_ERROR`"]
  #[inline(always)]
  pub fn is_data_size_error(&self) -> bool {
    *self == ERRID1_A::DATA_SIZE_ERROR
  }
  #[doc = "Checks if the value of the field is `KEY_SIZE_ERROR`"]
  #[inline(always)]
  pub fn is_key_size_error(&self) -> bool {
    *self == ERRID1_A::KEY_SIZE_ERROR
  }
  #[doc = "Checks if the value of the field is `PKHA_A_SIZE_ERROR`"]
  #[inline(always)]
  pub fn is_pkha_a_size_error(&self) -> bool {
    *self == ERRID1_A::PKHA_A_SIZE_ERROR
  }
  #[doc = "Checks if the value of the field is `PKHA_B_SIZE_ERROR`"]
  #[inline(always)]
  pub fn is_pkha_b_size_error(&self) -> bool {
    *self == ERRID1_A::PKHA_B_SIZE_ERROR
  }
  #[doc = "Checks if the value of the field is `DATA_OUT_OF_SEQ_ERROR`"]
  #[inline(always)]
  pub fn is_data_out_of_seq_error(&self) -> bool {
    *self == ERRID1_A::DATA_OUT_OF_SEQ_ERROR
  }
  #[doc = "Checks if the value of the field is `PKHA_DIV_BY_0_ERROR`"]
  #[inline(always)]
  pub fn is_pkha_div_by_0_error(&self) -> bool {
    *self == ERRID1_A::PKHA_DIV_BY_0_ERROR
  }
  #[doc = "Checks if the value of the field is `PKHA_MOD_EVEN_ERROR`"]
  #[inline(always)]
  pub fn is_pkha_mod_even_error(&self) -> bool {
    *self == ERRID1_A::PKHA_MOD_EVEN_ERROR
  }
  #[doc = "Checks if the value of the field is `INVALID_ENGINE_SEL_ERROR`"]
  #[inline(always)]
  pub fn is_invalid_engine_sel_error(&self) -> bool {
    *self == ERRID1_A::INVALID_ENGINE_SEL_ERROR
  }
}
#[doc = "algorithms\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CL1_A {
  #[doc = "0: General Error"]
  GEN_ERROR,
  #[doc = "8: Public Key"]
  PKHA_ERROR,
}
impl From<CL1_A> for u8 {
  #[inline(always)]
  fn from(variant: CL1_A) -> Self {
    match variant {
      CL1_A::GEN_ERROR => 0,
      CL1_A::PKHA_ERROR => 8,
    }
  }
}
#[doc = "Reader of field `CL1`"]
pub type CL1_R = crate::R<u8, CL1_A>;
impl CL1_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> crate::Variant<u8, CL1_A> {
    use crate::Variant::*;
    match self.bits {
      0 => Val(CL1_A::GEN_ERROR),
      8 => Val(CL1_A::PKHA_ERROR),
      i => Res(i),
    }
  }
  #[doc = "Checks if the value of the field is `GEN_ERROR`"]
  #[inline(always)]
  pub fn is_gen_error(&self) -> bool {
    *self == CL1_A::GEN_ERROR
  }
  #[doc = "Checks if the value of the field is `PKHA_ERROR`"]
  #[inline(always)]
  pub fn is_pkha_error(&self) -> bool {
    *self == CL1_A::PKHA_ERROR
  }
}
impl R {
  #[doc = "Bits 0:3 - Error ID 1"]
  #[inline(always)]
  pub fn errid1(&self) -> ERRID1_R {
    ERRID1_R::new((self.bits & 0x0f) as u8)
  }
  #[doc = "Bits 8:11 - algorithms"]
  #[inline(always)]
  pub fn cl1(&self) -> CL1_R {
    CL1_R::new(((self.bits >> 8) & 0x0f) as u8)
  }
}
