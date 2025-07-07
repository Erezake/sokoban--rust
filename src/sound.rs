use sdl2::mixer::{Chunk, Music, AUDIO_S16LSB, DEFAULT_CHANNELS, InitFlag};

pub struct Sounds<'a> {
    pub walk: Chunk,
    pub push: Chunk,
    pub win: Chunk,
    pub bgm: Music<'a>,
    pub pause: Chunk, // 开场BGM
}

impl<'a> Sounds<'a> {
    pub fn load() -> Result<Self, String> {
        sdl2::mixer::open_audio(44_100, AUDIO_S16LSB, DEFAULT_CHANNELS, 1024)?;
        sdl2::mixer::init(InitFlag::OGG | InitFlag::MP3);
        sdl2::mixer::allocate_channels(8);

        Ok(Sounds {
            walk: Chunk::from_file("assets/sound/walk.wav")?,
            push: Chunk::from_file("assets/sound/push.wav")?,
            win: Chunk::from_file("assets/sound/win.wav")?,
            bgm: Music::from_file("assets/sound/start_bgm.wav")?,
            pause: Chunk::from_file("assets/sound/pause.wav")?,
        })
    }
}