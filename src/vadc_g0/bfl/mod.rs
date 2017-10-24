#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::BFL {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `BFL0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BFL0R {
    #[doc = "Passive state: result has not yet crossed the activation boundary, or selected gate signal is inactive, or this boundary flag is disabled"]
    VALUE1,
    #[doc = "Active state: result has crossed the activation boundary"]
    VALUE2,
}
impl BFL0R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            BFL0R::VALUE1 => false,
            BFL0R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BFL0R {
        match value {
            false => BFL0R::VALUE1,
            true => BFL0R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == BFL0R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == BFL0R::VALUE2
    }
}
#[doc = "Possible values of the field `BFL1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BFL1R {
    #[doc = "Passive state: result has not yet crossed the activation boundary, or selected gate signal is inactive, or this boundary flag is disabled"]
    VALUE1,
    #[doc = "Active state: result has crossed the activation boundary"]
    VALUE2,
}
impl BFL1R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            BFL1R::VALUE1 => false,
            BFL1R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BFL1R {
        match value {
            false => BFL1R::VALUE1,
            true => BFL1R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == BFL1R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == BFL1R::VALUE2
    }
}
#[doc = "Possible values of the field `BFL2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BFL2R {
    #[doc = "Passive state: result has not yet crossed the activation boundary, or selected gate signal is inactive, or this boundary flag is disabled"]
    VALUE1,
    #[doc = "Active state: result has crossed the activation boundary"]
    VALUE2,
}
impl BFL2R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            BFL2R::VALUE1 => false,
            BFL2R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BFL2R {
        match value {
            false => BFL2R::VALUE1,
            true => BFL2R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == BFL2R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == BFL2R::VALUE2
    }
}
#[doc = "Possible values of the field `BFL3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BFL3R {
    #[doc = "Passive state: result has not yet crossed the activation boundary, or selected gate signal is inactive, or this boundary flag is disabled"]
    VALUE1,
    #[doc = "Active state: result has crossed the activation boundary"]
    VALUE2,
}
impl BFL3R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            BFL3R::VALUE1 => false,
            BFL3R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BFL3R {
        match value {
            false => BFL3R::VALUE1,
            true => BFL3R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == BFL3R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == BFL3R::VALUE2
    }
}
#[doc = "Possible values of the field `BFE0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BFE0R {
    #[doc = "Output 0 on this channel"]
    VALUE1,
    #[doc = "Output BFLy on this channel"]
    VALUE2,
}
impl BFE0R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            BFE0R::VALUE1 => false,
            BFE0R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BFE0R {
        match value {
            false => BFE0R::VALUE1,
            true => BFE0R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == BFE0R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == BFE0R::VALUE2
    }
}
#[doc = "Possible values of the field `BFE1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BFE1R {
    #[doc = "Output 0 on this channel"]
    VALUE1,
    #[doc = "Output BFLy on this channel"]
    VALUE2,
}
impl BFE1R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            BFE1R::VALUE1 => false,
            BFE1R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BFE1R {
        match value {
            false => BFE1R::VALUE1,
            true => BFE1R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == BFE1R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == BFE1R::VALUE2
    }
}
#[doc = "Possible values of the field `BFE2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BFE2R {
    #[doc = "Output 0 on this channel"]
    VALUE1,
    #[doc = "Output BFLy on this channel"]
    VALUE2,
}
impl BFE2R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            BFE2R::VALUE1 => false,
            BFE2R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BFE2R {
        match value {
            false => BFE2R::VALUE1,
            true => BFE2R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == BFE2R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == BFE2R::VALUE2
    }
}
#[doc = "Possible values of the field `BFE3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BFE3R {
    #[doc = "Output 0 on this channel"]
    VALUE1,
    #[doc = "Output BFLy on this channel"]
    VALUE2,
}
impl BFE3R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            BFE3R::VALUE1 => false,
            BFE3R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BFE3R {
        match value {
            false => BFE3R::VALUE1,
            true => BFE3R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == BFE3R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == BFE3R::VALUE2
    }
}
#[doc = "Values that can be written to the field `BFE0`"]
pub enum BFE0W {
    #[doc = "Output 0 on this channel"]
    VALUE1,
    #[doc = "Output BFLy on this channel"]
    VALUE2,
}
impl BFE0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BFE0W::VALUE1 => false,
            BFE0W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BFE0W<'a> {
    w: &'a mut W,
}
impl<'a> _BFE0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BFE0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Output 0 on this channel"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(BFE0W::VALUE1)
    }
    #[doc = "Output BFLy on this channel"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(BFE0W::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BFE1`"]
pub enum BFE1W {
    #[doc = "Output 0 on this channel"]
    VALUE1,
    #[doc = "Output BFLy on this channel"]
    VALUE2,
}
impl BFE1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BFE1W::VALUE1 => false,
            BFE1W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BFE1W<'a> {
    w: &'a mut W,
}
impl<'a> _BFE1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BFE1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Output 0 on this channel"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(BFE1W::VALUE1)
    }
    #[doc = "Output BFLy on this channel"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(BFE1W::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BFE2`"]
pub enum BFE2W {
    #[doc = "Output 0 on this channel"]
    VALUE1,
    #[doc = "Output BFLy on this channel"]
    VALUE2,
}
impl BFE2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BFE2W::VALUE1 => false,
            BFE2W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BFE2W<'a> {
    w: &'a mut W,
}
impl<'a> _BFE2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BFE2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Output 0 on this channel"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(BFE2W::VALUE1)
    }
    #[doc = "Output BFLy on this channel"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(BFE2W::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BFE3`"]
pub enum BFE3W {
    #[doc = "Output 0 on this channel"]
    VALUE1,
    #[doc = "Output BFLy on this channel"]
    VALUE2,
}
impl BFE3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BFE3W::VALUE1 => false,
            BFE3W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BFE3W<'a> {
    w: &'a mut W,
}
impl<'a> _BFE3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BFE3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Output 0 on this channel"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(BFE3W::VALUE1)
    }
    #[doc = "Output BFLy on this channel"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(BFE3W::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Boundar0 Flag y"]
    #[inline]
    pub fn bfl0(&self) -> BFL0R {
        BFL0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Boundar1 Flag y"]
    #[inline]
    pub fn bfl1(&self) -> BFL1R {
        BFL1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Boundar2 Flag y"]
    #[inline]
    pub fn bfl2(&self) -> BFL2R {
        BFL2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Boundar3 Flag y"]
    #[inline]
    pub fn bfl3(&self) -> BFL3R {
        BFL3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Enable Bit for Boundar0 Flag y"]
    #[inline]
    pub fn bfe0(&self) -> BFE0R {
        BFE0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Enable Bit for Boundar1 Flag y"]
    #[inline]
    pub fn bfe1(&self) -> BFE1R {
        BFE1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Enable Bit for Boundar2 Flag y"]
    #[inline]
    pub fn bfe2(&self) -> BFE2R {
        BFE2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Enable Bit for Boundar3 Flag y"]
    #[inline]
    pub fn bfe3(&self) -> BFE3R {
        BFE3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 16 - Enable Bit for Boundar0 Flag y"]
    #[inline]
    pub fn bfe0(&mut self) -> _BFE0W {
        _BFE0W { w: self }
    }
    #[doc = "Bit 17 - Enable Bit for Boundar1 Flag y"]
    #[inline]
    pub fn bfe1(&mut self) -> _BFE1W {
        _BFE1W { w: self }
    }
    #[doc = "Bit 18 - Enable Bit for Boundar2 Flag y"]
    #[inline]
    pub fn bfe2(&mut self) -> _BFE2W {
        _BFE2W { w: self }
    }
    #[doc = "Bit 19 - Enable Bit for Boundar3 Flag y"]
    #[inline]
    pub fn bfe3(&mut self) -> _BFE3W {
        _BFE3W { w: self }
    }
}