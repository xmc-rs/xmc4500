#[doc = "Register `MAC_ADDRESS0_LOW` reader"]
pub type R = crate::R<MAC_ADDRESS0_LOW_SPEC>;
#[doc = "Register `MAC_ADDRESS0_LOW` writer"]
pub type W = crate::W<MAC_ADDRESS0_LOW_SPEC>;
#[doc = "Field `ADDRLO` reader - MAC Address0 \\[31:0\\]"]
pub type ADDRLO_R = crate::FieldReader<u32>;
#[doc = "Field `ADDRLO` writer - MAC Address0 \\[31:0\\]"]
pub type ADDRLO_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - MAC Address0 \\[31:0\\]"]
    #[inline(always)]
    pub fn addrlo(&self) -> ADDRLO_R {
        ADDRLO_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - MAC Address0 \\[31:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn addrlo(&mut self) -> ADDRLO_W<MAC_ADDRESS0_LOW_SPEC, 0> {
        ADDRLO_W::new(self)
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
#[doc = "MAC Address0 Low Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mac_address0_low::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mac_address0_low::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MAC_ADDRESS0_LOW_SPEC;
impl crate::RegisterSpec for MAC_ADDRESS0_LOW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mac_address0_low::R`](R) reader structure"]
impl crate::Readable for MAC_ADDRESS0_LOW_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mac_address0_low::W`](W) writer structure"]
impl crate::Writable for MAC_ADDRESS0_LOW_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MAC_ADDRESS0_LOW to value 0xffff_ffff"]
impl crate::Resettable for MAC_ADDRESS0_LOW_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
