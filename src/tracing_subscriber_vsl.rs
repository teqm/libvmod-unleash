use std::io;
use std::io::Write;
use tracing::{Level, Metadata};
use tracing_subscriber::fmt::MakeWriter;
use varnish::vcl::ctx::{log, LogTag};
use varnish_sys::VSL_tag_e_SLT_VCL_Log;

pub struct MakeVSLWriter;

impl<'a> MakeWriter<'a> for MakeVSLWriter {
    type Writer = VSLWriter;

    fn make_writer(&'a self) -> Self::Writer {
        Default::default()
    }

    fn make_writer_for(&'a self, meta: &Metadata<'_>) -> Self::Writer {
        VSLWriter {
            level: *meta.level(),
            buf: Vec::new(),
        }
    }
}

pub struct VSLWriter {
    level: Level,
    buf: Vec<u8>,
}

impl Write for VSLWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.buf.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        let msg = std::str::from_utf8(&self.buf)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        let log_tag = match self.level {
            Level::ERROR => LogTag::Error,
            _ => LogTag::Any(VSL_tag_e_SLT_VCL_Log),
        };

        log(log_tag, format!("unleash: {}", msg).as_str());

        Ok(())
    }
}

impl Drop for VSLWriter {
    fn drop(&mut self) {
        let _ = self.flush();
    }
}

impl Default for VSLWriter {
    fn default() -> Self {
        VSLWriter {
            level: Level::INFO,
            buf: Vec::new(),
        }
    }
}

pub fn set_as_global_default(level: Level) {
    let subscriber = tracing_subscriber::fmt()
        .with_writer(MakeVSLWriter)
        .with_ansi(false)
        .with_max_level(level)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
}
