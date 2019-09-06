#[doc = "Reader of register CC_CMD"]
pub type R = crate::R<u32, super::CC_CMD>;
#[doc = "Writer for register CC_CMD"]
pub type W = crate::W<u32, super::CC_CMD>;
#[doc = "Register CC_CMD `reset()`'s with value 0"]
impl crate::ResetValue for super::CC_CMD {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "Command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMD_AW {
  #[doc = "0: Use CR\\[DTCCFG\\] for task completion configuration"]
  CMD_0,
  #[doc = "1: Issue an interrupt request"]
  CMD_1,
  #[doc = "2: Assert Event Completion Signal"]
  CMD_2,
  #[doc = "4: Issue a DMA request"]
  CMD_4,
}
impl From<CMD_AW> for u8 {
  #[inline(always)]
  fn from(variant: CMD_AW) -> Self {
    match variant {
      CMD_AW::CMD_0 => 0,
      CMD_AW::CMD_1 => 1,
      CMD_AW::CMD_2 => 2,
      CMD_AW::CMD_4 => 4,
    }
  }
}
#[doc = "Write proxy for field `CMD`"]
pub struct CMD_W<'a> {
  w: &'a mut W,
}
impl<'a> CMD_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: CMD_AW) -> &'a mut W {
    unsafe { self.bits(variant.into()) }
  }
  #[doc = "Use CR\\[DTCCFG\\] for task completion configuration"]
  #[inline(always)]
  pub fn cmd_0(self) -> &'a mut W {
    self.variant(CMD_AW::CMD_0)
  }
  #[doc = "Issue an interrupt request"]
  #[inline(always)]
  pub fn cmd_1(self) -> &'a mut W {
    self.variant(CMD_AW::CMD_1)
  }
  #[doc = "Assert Event Completion Signal"]
  #[inline(always)]
  pub fn cmd_2(self) -> &'a mut W {
    self.variant(CMD_AW::CMD_2)
  }
  #[doc = "Issue a DMA request"]
  #[inline(always)]
  pub fn cmd_4(self) -> &'a mut W {
    self.variant(CMD_AW::CMD_4)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
    self.w
  }
}
impl R {}
impl W {
  #[doc = "Bits 16:18 - Command"]
  #[inline(always)]
  pub fn cmd(&mut self) -> CMD_W {
    CMD_W { w: self }
  }
}
