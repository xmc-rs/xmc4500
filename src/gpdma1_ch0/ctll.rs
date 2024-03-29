#[doc = "Register `CTLL` reader"]
pub type R = crate::R<CtllSpec>;
#[doc = "Register `CTLL` writer"]
pub type W = crate::W<CtllSpec>;
#[doc = "Field `INT_EN` reader - Interrupt Enable Bit"]
pub type IntEnR = crate::BitReader;
#[doc = "Field `INT_EN` writer - Interrupt Enable Bit"]
pub type IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DST_TR_WIDTH` reader - Destination Transfer Width"]
pub type DstTrWidthR = crate::FieldReader;
#[doc = "Field `DST_TR_WIDTH` writer - Destination Transfer Width"]
pub type DstTrWidthW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SRC_TR_WIDTH` reader - Source Transfer Width"]
pub type SrcTrWidthR = crate::FieldReader;
#[doc = "Field `SRC_TR_WIDTH` writer - Source Transfer Width"]
pub type SrcTrWidthW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Destination Address Increment\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dinc {
    #[doc = "0: Increment"]
    Value1 = 0,
    #[doc = "1: Decrement"]
    Value2 = 1,
    #[doc = "2: No change"]
    Value3 = 2,
}
impl From<Dinc> for u8 {
    #[inline(always)]
    fn from(variant: Dinc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dinc {
    type Ux = u8;
}
#[doc = "Field `DINC` reader - Destination Address Increment"]
pub type DincR = crate::FieldReader<Dinc>;
impl DincR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Dinc> {
        match self.bits {
            0 => Some(Dinc::Value1),
            1 => Some(Dinc::Value2),
            2 => Some(Dinc::Value3),
            _ => None,
        }
    }
    #[doc = "Increment"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Dinc::Value1
    }
    #[doc = "Decrement"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Dinc::Value2
    }
    #[doc = "No change"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Dinc::Value3
    }
}
#[doc = "Field `DINC` writer - Destination Address Increment"]
pub type DincW<'a, REG> = crate::FieldWriter<'a, REG, 2, Dinc>;
impl<'a, REG> DincW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Increment"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Dinc::Value1)
    }
    #[doc = "Decrement"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Dinc::Value2)
    }
    #[doc = "No change"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Dinc::Value3)
    }
}
#[doc = "Source Address Increment\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sinc {
    #[doc = "0: Increment"]
    Value1 = 0,
    #[doc = "1: Decrement"]
    Value2 = 1,
    #[doc = "2: No change"]
    Value3 = 2,
}
impl From<Sinc> for u8 {
    #[inline(always)]
    fn from(variant: Sinc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sinc {
    type Ux = u8;
}
#[doc = "Field `SINC` reader - Source Address Increment"]
pub type SincR = crate::FieldReader<Sinc>;
impl SincR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Sinc> {
        match self.bits {
            0 => Some(Sinc::Value1),
            1 => Some(Sinc::Value2),
            2 => Some(Sinc::Value3),
            _ => None,
        }
    }
    #[doc = "Increment"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Sinc::Value1
    }
    #[doc = "Decrement"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Sinc::Value2
    }
    #[doc = "No change"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Sinc::Value3
    }
}
#[doc = "Field `SINC` writer - Source Address Increment"]
pub type SincW<'a, REG> = crate::FieldWriter<'a, REG, 2, Sinc>;
impl<'a, REG> SincW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Increment"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Sinc::Value1)
    }
    #[doc = "Decrement"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Sinc::Value2)
    }
    #[doc = "No change"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Sinc::Value3)
    }
}
#[doc = "Field `DEST_MSIZE` reader - Destination Burst Transaction Length"]
pub type DestMsizeR = crate::FieldReader;
#[doc = "Field `DEST_MSIZE` writer - Destination Burst Transaction Length"]
pub type DestMsizeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SRC_MSIZE` reader - Source Burst Transaction Length"]
pub type SrcMsizeR = crate::FieldReader;
#[doc = "Field `SRC_MSIZE` writer - Source Burst Transaction Length"]
pub type SrcMsizeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TT_FC` reader - Transfer Type and Flow Control"]
pub type TtFcR = crate::FieldReader;
#[doc = "Field `TT_FC` writer - Transfer Type and Flow Control"]
pub type TtFcW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - Interrupt Enable Bit"]
    #[inline(always)]
    pub fn int_en(&self) -> IntEnR {
        IntEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - Destination Transfer Width"]
    #[inline(always)]
    pub fn dst_tr_width(&self) -> DstTrWidthR {
        DstTrWidthR::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bits 4:6 - Source Transfer Width"]
    #[inline(always)]
    pub fn src_tr_width(&self) -> SrcTrWidthR {
        SrcTrWidthR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 7:8 - Destination Address Increment"]
    #[inline(always)]
    pub fn dinc(&self) -> DincR {
        DincR::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bits 9:10 - Source Address Increment"]
    #[inline(always)]
    pub fn sinc(&self) -> SincR {
        SincR::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bits 11:13 - Destination Burst Transaction Length"]
    #[inline(always)]
    pub fn dest_msize(&self) -> DestMsizeR {
        DestMsizeR::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bits 14:16 - Source Burst Transaction Length"]
    #[inline(always)]
    pub fn src_msize(&self) -> SrcMsizeR {
        SrcMsizeR::new(((self.bits >> 14) & 7) as u8)
    }
    #[doc = "Bits 20:22 - Transfer Type and Flow Control"]
    #[inline(always)]
    pub fn tt_fc(&self) -> TtFcR {
        TtFcR::new(((self.bits >> 20) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt Enable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn int_en(&mut self) -> IntEnW<CtllSpec> {
        IntEnW::new(self, 0)
    }
    #[doc = "Bits 1:3 - Destination Transfer Width"]
    #[inline(always)]
    #[must_use]
    pub fn dst_tr_width(&mut self) -> DstTrWidthW<CtllSpec> {
        DstTrWidthW::new(self, 1)
    }
    #[doc = "Bits 4:6 - Source Transfer Width"]
    #[inline(always)]
    #[must_use]
    pub fn src_tr_width(&mut self) -> SrcTrWidthW<CtllSpec> {
        SrcTrWidthW::new(self, 4)
    }
    #[doc = "Bits 7:8 - Destination Address Increment"]
    #[inline(always)]
    #[must_use]
    pub fn dinc(&mut self) -> DincW<CtllSpec> {
        DincW::new(self, 7)
    }
    #[doc = "Bits 9:10 - Source Address Increment"]
    #[inline(always)]
    #[must_use]
    pub fn sinc(&mut self) -> SincW<CtllSpec> {
        SincW::new(self, 9)
    }
    #[doc = "Bits 11:13 - Destination Burst Transaction Length"]
    #[inline(always)]
    #[must_use]
    pub fn dest_msize(&mut self) -> DestMsizeW<CtllSpec> {
        DestMsizeW::new(self, 11)
    }
    #[doc = "Bits 14:16 - Source Burst Transaction Length"]
    #[inline(always)]
    #[must_use]
    pub fn src_msize(&mut self) -> SrcMsizeW<CtllSpec> {
        SrcMsizeW::new(self, 14)
    }
    #[doc = "Bits 20:22 - Transfer Type and Flow Control"]
    #[inline(always)]
    #[must_use]
    pub fn tt_fc(&mut self) -> TtFcW<CtllSpec> {
        TtFcW::new(self, 20)
    }
}
#[doc = "Control Register Low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctll::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctll::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtllSpec;
impl crate::RegisterSpec for CtllSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctll::R`](R) reader structure"]
impl crate::Readable for CtllSpec {}
#[doc = "`write(|w| ..)` method takes [`ctll::W`](W) writer structure"]
impl crate::Writable for CtllSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLL to value 0x0030_4801"]
impl crate::Resettable for CtllSpec {
    const RESET_VALUE: u32 = 0x0030_4801;
}
