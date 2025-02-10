use super::MetaDataBlock;

pub struct Application<const N: usize> {
    app_id: AppId,
    pub app_data: [u8; N],
}

#[repr(u32)]
enum AppId {
    FlacFile = 0x4154_4348,
    BeSolo = 0x4253_4F4C,
    BugsPlayer = 0x4255_4753,
    GoldWaveCuePoints = 0x4375_6573,
    CueSplitter = 0x4669_6361,
    FacTools = 0x4674_6F6C,
    MotbMetaCzar = 0x4D4F_5442,
    Mp3StreamEditor = 0x4D50_5345,
    MusicMlMusicMetadataLanguage = 0x4D75_4D4C,
    SoundDevicesRiffChunkStorage = 0x5249_4646,
    SoundFontFlac = 0x5346_464C,
    SonyCreativeSoftware = 0x534F_4E59,
    Facsqueeze = 0x5351_455A,
    TwistedWave = 0x5474_5776,
    UitsEmbeddingTools = 0x5549_5453,
    FlacAiffChunkStorage = 0x6169_6666,
    FacImage = 0x696D_6167,
    ParseableEmbeddedExtensibleMetadata = 0x7065_656D,
    QflacStudio = 0x7166_7374,
    FlacRiffChunkStorage = 0x7269_6666,
    TagTuner = 0x7475_6E65,
    FlacWave64ChunkStorage = 0x7736_3420,
    XBAT = 0x7862_6174,
    Xmcd = 0x786D_6364,
    Other(u32),
}

impl AppId {
    fn id(&self) -> u32 {
        match self {
            Self::Other(n) => *n,
            _ => unsafe { *<*const _>::from(self).cast::<u32>() },
        }
    }
}

impl<const N: usize> Application<N> {
    pub fn new(app_id: AppId) -> Self {
        Self {
            app_id,
            app_data: [0; N],
        }
    }
}

impl<const N: usize> MetaDataBlock for Application<N>
where
    [(); N + 32]:,
{
    type Array = [u8; N + 32];
    fn to_bytes(&self) -> Self::Array {
        let mut ret = [0; N + 32];
        for (i, val) in self
            .app_id
            .id()
            .to_be_bytes()
            .into_iter()
            .chain(self.app_data.into_iter())
            .enumerate()
        {
            ret[i] = val;
        }
        ret
    }
}
