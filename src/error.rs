use thiserror::Error;

#[derive(Error, Debug)]
pub enum BassError {
    #[error("The output is paused or stopped.")]
    OutputIsPausedOrStopped,
    #[error("The stream is not playable.")]
    StreamIsNotPlayable,
    #[error("The stream is not playing.")]
    StreamIsNotPlaying,
    #[error("The file couldn't be opened.")]
    FileCouldNotBeOpened,
    #[error("The file format isn't supported or recognised.")]
    InvalidFileFormat,
    #[error("The file doesn't contain audio or it contains audio and video.")]
    InvalidFileContent,
    #[error("The codec isn't supported.")]
    InvalidCodec,
    #[error("The sample format isn't supported.")]
    InvalidSampleFormat,
    #[error("There is too little free memory.")]
    InsufficientMemory,
    #[error("Couldn't initialize 3d support.")]
    CouldNotInitialize3DSupport,
    #[error("Internet connection isn't available.")]
    NoInternetConnection,
    #[error("The protocol isn't supported.")]
    InvalidProtocol,
    #[error("SSL support is not available.")]
    SslSupportNotAvailable,
    #[error("The file can't be streamed.")]
    UnstreamableFile,
    #[error("The server didn't respond to the request within the timeout period.")]
    TimeOut,
}
