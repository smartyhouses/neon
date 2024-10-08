use crate::{
    context::RequestContext,
    virtual_file::owned_buffers_io::{io_buf_ext::FullSlice, write::OwnedAsyncWriter},
};
use tokio_epoll_uring::IoBuf;

pub struct Writer<W> {
    dst: W,
    bytes_amount: u64,
}

impl<W> Writer<W> {
    pub fn new(dst: W) -> Self {
        Self {
            dst,
            bytes_amount: 0,
        }
    }

    pub fn bytes_written(&self) -> u64 {
        self.bytes_amount
    }

    pub fn as_inner(&self) -> &W {
        &self.dst
    }

    /// Returns the wrapped `VirtualFile` object as well as the number
    /// of bytes that were written to it through this object.
    #[cfg_attr(target_os = "macos", allow(dead_code))]
    pub fn into_inner(self) -> (u64, W) {
        (self.bytes_amount, self.dst)
    }
}

impl<W> OwnedAsyncWriter for Writer<W>
where
    W: OwnedAsyncWriter,
{
    #[inline(always)]
    async fn write_all<Buf: IoBuf + Send>(
        &mut self,
        buf: FullSlice<Buf>,
        ctx: &RequestContext,
    ) -> std::io::Result<(usize, FullSlice<Buf>)> {
        let (nwritten, buf) = self.dst.write_all(buf, ctx).await?;
        self.bytes_amount += u64::try_from(nwritten).unwrap();
        Ok((nwritten, buf))
    }
}
