use steel::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
pub enum BlackgoldMintInstruction {
    Init = 0,
    MintBlackgold = 1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Init {}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct MintBlackgold {
    // No of tokens to mint in this request (max 2 allowed per request as per the architecture)
    pub amount: [u8; 8],
}

instruction!(BlackgoldMintInstruction, Init);
instruction!(BlackgoldMintInstruction, MintBlackgold);
