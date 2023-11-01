#[doc = "Register `DOEPTSIZ0` reader"]
pub type R = crate::R<DOEPTSIZ0_SPEC>;
#[doc = "Register `DOEPTSIZ0` writer"]
pub type W = crate::W<DOEPTSIZ0_SPEC>;
#[doc = "Field `XferSize` reader - Transfer Size"]
pub type XFER_SIZE_R = crate::FieldReader;
#[doc = "Field `XferSize` writer - Transfer Size"]
pub type XFER_SIZE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `PktCnt` reader - Packet Count"]
pub type PKT_CNT_R = crate::FieldReader;
#[doc = "Field `PktCnt` writer - Packet Count"]
pub type PKT_CNT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `SUPCnt` reader - SETUP Packet Count"]
pub type SUPCNT_R = crate::FieldReader<SUPCNT_A>;
#[doc = "SETUP Packet Count\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SUPCNT_A {
    #[doc = "1: 1 packet"]
    VALUE1 = 1,
    #[doc = "2: 2 packets"]
    VALUE2 = 2,
    #[doc = "3: 3 packets"]
    VALUE3 = 3,
}
impl From<SUPCNT_A> for u8 {
    #[inline(always)]
    fn from(variant: SUPCNT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SUPCNT_A {
    type Ux = u8;
}
impl SUPCNT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SUPCNT_A> {
        match self.bits {
            1 => Some(SUPCNT_A::VALUE1),
            2 => Some(SUPCNT_A::VALUE2),
            3 => Some(SUPCNT_A::VALUE3),
            _ => None,
        }
    }
    #[doc = "1 packet"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SUPCNT_A::VALUE1
    }
    #[doc = "2 packets"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SUPCNT_A::VALUE2
    }
    #[doc = "3 packets"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == SUPCNT_A::VALUE3
    }
}
#[doc = "Field `SUPCnt` writer - SETUP Packet Count"]
pub type SUPCNT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, SUPCNT_A>;
impl<'a, REG, const O: u8> SUPCNT_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1 packet"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SUPCNT_A::VALUE1)
    }
    #[doc = "2 packets"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SUPCNT_A::VALUE2)
    }
    #[doc = "3 packets"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(SUPCNT_A::VALUE3)
    }
}
impl R {
    #[doc = "Bits 0:6 - Transfer Size"]
    #[inline(always)]
    pub fn xfer_size(&self) -> XFER_SIZE_R {
        XFER_SIZE_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 19:20 - Packet Count"]
    #[inline(always)]
    pub fn pkt_cnt(&self) -> PKT_CNT_R {
        PKT_CNT_R::new(((self.bits >> 19) & 3) as u8)
    }
    #[doc = "Bits 29:30 - SETUP Packet Count"]
    #[inline(always)]
    pub fn supcnt(&self) -> SUPCNT_R {
        SUPCNT_R::new(((self.bits >> 29) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Transfer Size"]
    #[inline(always)]
    #[must_use]
    pub fn xfer_size(&mut self) -> XFER_SIZE_W<DOEPTSIZ0_SPEC, 0> {
        XFER_SIZE_W::new(self)
    }
    #[doc = "Bits 19:20 - Packet Count"]
    #[inline(always)]
    #[must_use]
    pub fn pkt_cnt(&mut self) -> PKT_CNT_W<DOEPTSIZ0_SPEC, 19> {
        PKT_CNT_W::new(self)
    }
    #[doc = "Bits 29:30 - SETUP Packet Count"]
    #[inline(always)]
    #[must_use]
    pub fn supcnt(&mut self) -> SUPCNT_W<DOEPTSIZ0_SPEC, 29> {
        SUPCNT_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Device OUT Endpoint 0 Transfer Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doeptsiz0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doeptsiz0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DOEPTSIZ0_SPEC;
impl crate::RegisterSpec for DOEPTSIZ0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doeptsiz0::R`](R) reader structure"]
impl crate::Readable for DOEPTSIZ0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`doeptsiz0::W`](W) writer structure"]
impl crate::Writable for DOEPTSIZ0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DOEPTSIZ0 to value 0"]
impl crate::Resettable for DOEPTSIZ0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
