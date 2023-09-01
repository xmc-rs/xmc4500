#[doc = "Register `CFGL` reader"]
pub type R = crate::R<CFGL_SPEC>;
#[doc = "Register `CFGL` writer"]
pub type W = crate::W<CFGL_SPEC>;
#[doc = "Field `CH_PRIOR` reader - Channel priority"]
pub type CH_PRIOR_R = crate::FieldReader;
#[doc = "Field `CH_PRIOR` writer - Channel priority"]
pub type CH_PRIOR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `CH_SUSP` reader - Channel Suspend"]
pub type CH_SUSP_R = crate::BitReader<CH_SUSP_A>;
#[doc = "Channel Suspend\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH_SUSP_A {
    #[doc = "0: Not suspended."]
    VALUE1 = 0,
    #[doc = "1: Suspend DMA transfer from the source."]
    VALUE2 = 1,
}
impl From<CH_SUSP_A> for bool {
    #[inline(always)]
    fn from(variant: CH_SUSP_A) -> Self {
        variant as u8 != 0
    }
}
impl CH_SUSP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH_SUSP_A {
        match self.bits {
            false => CH_SUSP_A::VALUE1,
            true => CH_SUSP_A::VALUE2,
        }
    }
    #[doc = "Not suspended."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CH_SUSP_A::VALUE1
    }
    #[doc = "Suspend DMA transfer from the source."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CH_SUSP_A::VALUE2
    }
}
#[doc = "Field `CH_SUSP` writer - Channel Suspend"]
pub type CH_SUSP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CH_SUSP_A>;
impl<'a, REG, const O: u8> CH_SUSP_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not suspended."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CH_SUSP_A::VALUE1)
    }
    #[doc = "Suspend DMA transfer from the source."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CH_SUSP_A::VALUE2)
    }
}
#[doc = "Field `FIFO_EMPTY` reader - Indicates if there is data left in the channel FIFO"]
pub type FIFO_EMPTY_R = crate::BitReader<FIFO_EMPTY_A>;
#[doc = "Indicates if there is data left in the channel FIFO\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIFO_EMPTY_A {
    #[doc = "1: Channel FIFO empty"]
    VALUE1 = 1,
    #[doc = "0: Channel FIFO not empty"]
    VALUE2 = 0,
}
impl From<FIFO_EMPTY_A> for bool {
    #[inline(always)]
    fn from(variant: FIFO_EMPTY_A) -> Self {
        variant as u8 != 0
    }
}
impl FIFO_EMPTY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIFO_EMPTY_A {
        match self.bits {
            true => FIFO_EMPTY_A::VALUE1,
            false => FIFO_EMPTY_A::VALUE2,
        }
    }
    #[doc = "Channel FIFO empty"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == FIFO_EMPTY_A::VALUE1
    }
    #[doc = "Channel FIFO not empty"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == FIFO_EMPTY_A::VALUE2
    }
}
#[doc = "Field `HS_SEL_DST` reader - Destination Software or Hardware Handshaking Select"]
pub type HS_SEL_DST_R = crate::BitReader<HS_SEL_DST_A>;
#[doc = "Destination Software or Hardware Handshaking Select\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HS_SEL_DST_A {
    #[doc = "0: Hardware handshaking interface. Software-initiated transaction requests are ignored."]
    VALUE1 = 0,
    #[doc = "1: Software handshaking interface. Hardware- initiated transaction requests are ignored."]
    VALUE2 = 1,
}
impl From<HS_SEL_DST_A> for bool {
    #[inline(always)]
    fn from(variant: HS_SEL_DST_A) -> Self {
        variant as u8 != 0
    }
}
impl HS_SEL_DST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HS_SEL_DST_A {
        match self.bits {
            false => HS_SEL_DST_A::VALUE1,
            true => HS_SEL_DST_A::VALUE2,
        }
    }
    #[doc = "Hardware handshaking interface. Software-initiated transaction requests are ignored."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == HS_SEL_DST_A::VALUE1
    }
    #[doc = "Software handshaking interface. Hardware- initiated transaction requests are ignored."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == HS_SEL_DST_A::VALUE2
    }
}
#[doc = "Field `HS_SEL_DST` writer - Destination Software or Hardware Handshaking Select"]
pub type HS_SEL_DST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, HS_SEL_DST_A>;
impl<'a, REG, const O: u8> HS_SEL_DST_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Hardware handshaking interface. Software-initiated transaction requests are ignored."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(HS_SEL_DST_A::VALUE1)
    }
    #[doc = "Software handshaking interface. Hardware- initiated transaction requests are ignored."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(HS_SEL_DST_A::VALUE2)
    }
}
#[doc = "Field `HS_SEL_SRC` reader - Source Software or Hardware Handshaking Select"]
pub type HS_SEL_SRC_R = crate::BitReader<HS_SEL_SRC_A>;
#[doc = "Source Software or Hardware Handshaking Select\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HS_SEL_SRC_A {
    #[doc = "0: Hardware handshaking interface. Software-initiated transaction requests are ignored."]
    VALUE1 = 0,
    #[doc = "1: Software handshaking interface. Hardware-initiated transaction requests are ignored."]
    VALUE2 = 1,
}
impl From<HS_SEL_SRC_A> for bool {
    #[inline(always)]
    fn from(variant: HS_SEL_SRC_A) -> Self {
        variant as u8 != 0
    }
}
impl HS_SEL_SRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HS_SEL_SRC_A {
        match self.bits {
            false => HS_SEL_SRC_A::VALUE1,
            true => HS_SEL_SRC_A::VALUE2,
        }
    }
    #[doc = "Hardware handshaking interface. Software-initiated transaction requests are ignored."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == HS_SEL_SRC_A::VALUE1
    }
    #[doc = "Software handshaking interface. Hardware-initiated transaction requests are ignored."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == HS_SEL_SRC_A::VALUE2
    }
}
#[doc = "Field `HS_SEL_SRC` writer - Source Software or Hardware Handshaking Select"]
pub type HS_SEL_SRC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, HS_SEL_SRC_A>;
impl<'a, REG, const O: u8> HS_SEL_SRC_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Hardware handshaking interface. Software-initiated transaction requests are ignored."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(HS_SEL_SRC_A::VALUE1)
    }
    #[doc = "Software handshaking interface. Hardware-initiated transaction requests are ignored."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(HS_SEL_SRC_A::VALUE2)
    }
}
#[doc = "Field `LOCK_CH_L` reader - Channel Lock Level"]
pub type LOCK_CH_L_R = crate::FieldReader<LOCK_CH_L_A>;
#[doc = "Channel Lock Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LOCK_CH_L_A {
    #[doc = "0: Over complete DMA transfer"]
    VALUE1 = 0,
    #[doc = "1: Over complete DMA block transfer"]
    VALUE2 = 1,
    #[doc = "2: Over complete DMA transaction"]
    VALUE3 = 2,
}
impl From<LOCK_CH_L_A> for u8 {
    #[inline(always)]
    fn from(variant: LOCK_CH_L_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LOCK_CH_L_A {
    type Ux = u8;
}
impl LOCK_CH_L_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LOCK_CH_L_A> {
        match self.bits {
            0 => Some(LOCK_CH_L_A::VALUE1),
            1 => Some(LOCK_CH_L_A::VALUE2),
            2 => Some(LOCK_CH_L_A::VALUE3),
            _ => None,
        }
    }
    #[doc = "Over complete DMA transfer"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LOCK_CH_L_A::VALUE1
    }
    #[doc = "Over complete DMA block transfer"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LOCK_CH_L_A::VALUE2
    }
    #[doc = "Over complete DMA transaction"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == LOCK_CH_L_A::VALUE3
    }
}
#[doc = "Field `LOCK_CH_L` writer - Channel Lock Level"]
pub type LOCK_CH_L_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, LOCK_CH_L_A>;
impl<'a, REG, const O: u8> LOCK_CH_L_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Over complete DMA transfer"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(LOCK_CH_L_A::VALUE1)
    }
    #[doc = "Over complete DMA block transfer"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(LOCK_CH_L_A::VALUE2)
    }
    #[doc = "Over complete DMA transaction"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(LOCK_CH_L_A::VALUE3)
    }
}
#[doc = "Field `LOCK_B_L` reader - Bus Lock Level"]
pub type LOCK_B_L_R = crate::FieldReader<LOCK_B_L_A>;
#[doc = "Bus Lock Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LOCK_B_L_A {
    #[doc = "0: Over complete DMA transfer"]
    VALUE1 = 0,
    #[doc = "1: Over complete DMA block transfer"]
    VALUE2 = 1,
    #[doc = "2: Over complete DMA transaction"]
    VALUE3 = 2,
}
impl From<LOCK_B_L_A> for u8 {
    #[inline(always)]
    fn from(variant: LOCK_B_L_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LOCK_B_L_A {
    type Ux = u8;
}
impl LOCK_B_L_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LOCK_B_L_A> {
        match self.bits {
            0 => Some(LOCK_B_L_A::VALUE1),
            1 => Some(LOCK_B_L_A::VALUE2),
            2 => Some(LOCK_B_L_A::VALUE3),
            _ => None,
        }
    }
    #[doc = "Over complete DMA transfer"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LOCK_B_L_A::VALUE1
    }
    #[doc = "Over complete DMA block transfer"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LOCK_B_L_A::VALUE2
    }
    #[doc = "Over complete DMA transaction"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == LOCK_B_L_A::VALUE3
    }
}
#[doc = "Field `LOCK_B_L` writer - Bus Lock Level"]
pub type LOCK_B_L_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, LOCK_B_L_A>;
impl<'a, REG, const O: u8> LOCK_B_L_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Over complete DMA transfer"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(LOCK_B_L_A::VALUE1)
    }
    #[doc = "Over complete DMA block transfer"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(LOCK_B_L_A::VALUE2)
    }
    #[doc = "Over complete DMA transaction"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(LOCK_B_L_A::VALUE3)
    }
}
#[doc = "Field `LOCK_CH` reader - Channel Lock Bit"]
pub type LOCK_CH_R = crate::BitReader;
#[doc = "Field `LOCK_CH` writer - Channel Lock Bit"]
pub type LOCK_CH_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LOCK_B` reader - Bus Lock Bit"]
pub type LOCK_B_R = crate::BitReader;
#[doc = "Field `LOCK_B` writer - Bus Lock Bit"]
pub type LOCK_B_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DST_HS_POL` reader - Destination Handshaking Interface Polarity"]
pub type DST_HS_POL_R = crate::BitReader<DST_HS_POL_A>;
#[doc = "Destination Handshaking Interface Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DST_HS_POL_A {
    #[doc = "0: Active high"]
    VALUE1 = 0,
    #[doc = "1: Active low"]
    VALUE2 = 1,
}
impl From<DST_HS_POL_A> for bool {
    #[inline(always)]
    fn from(variant: DST_HS_POL_A) -> Self {
        variant as u8 != 0
    }
}
impl DST_HS_POL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DST_HS_POL_A {
        match self.bits {
            false => DST_HS_POL_A::VALUE1,
            true => DST_HS_POL_A::VALUE2,
        }
    }
    #[doc = "Active high"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DST_HS_POL_A::VALUE1
    }
    #[doc = "Active low"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DST_HS_POL_A::VALUE2
    }
}
#[doc = "Field `DST_HS_POL` writer - Destination Handshaking Interface Polarity"]
pub type DST_HS_POL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, DST_HS_POL_A>;
impl<'a, REG, const O: u8> DST_HS_POL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Active high"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(DST_HS_POL_A::VALUE1)
    }
    #[doc = "Active low"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(DST_HS_POL_A::VALUE2)
    }
}
#[doc = "Field `SRC_HS_POL` reader - Source Handshaking Interface Polarity"]
pub type SRC_HS_POL_R = crate::BitReader<SRC_HS_POL_A>;
#[doc = "Source Handshaking Interface Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRC_HS_POL_A {
    #[doc = "0: Active high"]
    VALUE1 = 0,
    #[doc = "1: Active low"]
    VALUE2 = 1,
}
impl From<SRC_HS_POL_A> for bool {
    #[inline(always)]
    fn from(variant: SRC_HS_POL_A) -> Self {
        variant as u8 != 0
    }
}
impl SRC_HS_POL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRC_HS_POL_A {
        match self.bits {
            false => SRC_HS_POL_A::VALUE1,
            true => SRC_HS_POL_A::VALUE2,
        }
    }
    #[doc = "Active high"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SRC_HS_POL_A::VALUE1
    }
    #[doc = "Active low"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SRC_HS_POL_A::VALUE2
    }
}
#[doc = "Field `SRC_HS_POL` writer - Source Handshaking Interface Polarity"]
pub type SRC_HS_POL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SRC_HS_POL_A>;
impl<'a, REG, const O: u8> SRC_HS_POL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Active high"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SRC_HS_POL_A::VALUE1)
    }
    #[doc = "Active low"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SRC_HS_POL_A::VALUE2)
    }
}
#[doc = "Field `MAX_ABRST` reader - Maximum AMBA Burst Length"]
pub type MAX_ABRST_R = crate::FieldReader<u16>;
#[doc = "Field `MAX_ABRST` writer - Maximum AMBA Burst Length"]
pub type MAX_ABRST_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 10, O, u16>;
impl R {
    #[doc = "Bits 5:7 - Channel priority"]
    #[inline(always)]
    pub fn ch_prior(&self) -> CH_PRIOR_R {
        CH_PRIOR_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 8 - Channel Suspend"]
    #[inline(always)]
    pub fn ch_susp(&self) -> CH_SUSP_R {
        CH_SUSP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Indicates if there is data left in the channel FIFO"]
    #[inline(always)]
    pub fn fifo_empty(&self) -> FIFO_EMPTY_R {
        FIFO_EMPTY_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Destination Software or Hardware Handshaking Select"]
    #[inline(always)]
    pub fn hs_sel_dst(&self) -> HS_SEL_DST_R {
        HS_SEL_DST_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Source Software or Hardware Handshaking Select"]
    #[inline(always)]
    pub fn hs_sel_src(&self) -> HS_SEL_SRC_R {
        HS_SEL_SRC_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Channel Lock Level"]
    #[inline(always)]
    pub fn lock_ch_l(&self) -> LOCK_CH_L_R {
        LOCK_CH_L_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Bus Lock Level"]
    #[inline(always)]
    pub fn lock_b_l(&self) -> LOCK_B_L_R {
        LOCK_B_L_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - Channel Lock Bit"]
    #[inline(always)]
    pub fn lock_ch(&self) -> LOCK_CH_R {
        LOCK_CH_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Bus Lock Bit"]
    #[inline(always)]
    pub fn lock_b(&self) -> LOCK_B_R {
        LOCK_B_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Destination Handshaking Interface Polarity"]
    #[inline(always)]
    pub fn dst_hs_pol(&self) -> DST_HS_POL_R {
        DST_HS_POL_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Source Handshaking Interface Polarity"]
    #[inline(always)]
    pub fn src_hs_pol(&self) -> SRC_HS_POL_R {
        SRC_HS_POL_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:29 - Maximum AMBA Burst Length"]
    #[inline(always)]
    pub fn max_abrst(&self) -> MAX_ABRST_R {
        MAX_ABRST_R::new(((self.bits >> 20) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 5:7 - Channel priority"]
    #[inline(always)]
    #[must_use]
    pub fn ch_prior(&mut self) -> CH_PRIOR_W<CFGL_SPEC, 5> {
        CH_PRIOR_W::new(self)
    }
    #[doc = "Bit 8 - Channel Suspend"]
    #[inline(always)]
    #[must_use]
    pub fn ch_susp(&mut self) -> CH_SUSP_W<CFGL_SPEC, 8> {
        CH_SUSP_W::new(self)
    }
    #[doc = "Bit 10 - Destination Software or Hardware Handshaking Select"]
    #[inline(always)]
    #[must_use]
    pub fn hs_sel_dst(&mut self) -> HS_SEL_DST_W<CFGL_SPEC, 10> {
        HS_SEL_DST_W::new(self)
    }
    #[doc = "Bit 11 - Source Software or Hardware Handshaking Select"]
    #[inline(always)]
    #[must_use]
    pub fn hs_sel_src(&mut self) -> HS_SEL_SRC_W<CFGL_SPEC, 11> {
        HS_SEL_SRC_W::new(self)
    }
    #[doc = "Bits 12:13 - Channel Lock Level"]
    #[inline(always)]
    #[must_use]
    pub fn lock_ch_l(&mut self) -> LOCK_CH_L_W<CFGL_SPEC, 12> {
        LOCK_CH_L_W::new(self)
    }
    #[doc = "Bits 14:15 - Bus Lock Level"]
    #[inline(always)]
    #[must_use]
    pub fn lock_b_l(&mut self) -> LOCK_B_L_W<CFGL_SPEC, 14> {
        LOCK_B_L_W::new(self)
    }
    #[doc = "Bit 16 - Channel Lock Bit"]
    #[inline(always)]
    #[must_use]
    pub fn lock_ch(&mut self) -> LOCK_CH_W<CFGL_SPEC, 16> {
        LOCK_CH_W::new(self)
    }
    #[doc = "Bit 17 - Bus Lock Bit"]
    #[inline(always)]
    #[must_use]
    pub fn lock_b(&mut self) -> LOCK_B_W<CFGL_SPEC, 17> {
        LOCK_B_W::new(self)
    }
    #[doc = "Bit 18 - Destination Handshaking Interface Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn dst_hs_pol(&mut self) -> DST_HS_POL_W<CFGL_SPEC, 18> {
        DST_HS_POL_W::new(self)
    }
    #[doc = "Bit 19 - Source Handshaking Interface Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn src_hs_pol(&mut self) -> SRC_HS_POL_W<CFGL_SPEC, 19> {
        SRC_HS_POL_W::new(self)
    }
    #[doc = "Bits 20:29 - Maximum AMBA Burst Length"]
    #[inline(always)]
    #[must_use]
    pub fn max_abrst(&mut self) -> MAX_ABRST_W<CFGL_SPEC, 20> {
        MAX_ABRST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Configuration Register Low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFGL_SPEC;
impl crate::RegisterSpec for CFGL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgl::R`](R) reader structure"]
impl crate::Readable for CFGL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cfgl::W`](W) writer structure"]
impl crate::Writable for CFGL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFGL to value 0x0e00"]
impl crate::Resettable for CFGL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0e00;
}
