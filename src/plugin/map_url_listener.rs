use crate::{async_manager, error::*};
use classicube_sys::{
    Net_Handler, Protocol, UNSAFE_GetString, OPCODE__OPCODE_ENV_SET_MAP_APPEARANCE,
    OPCODE__OPCODE_ENV_SET_MAP_URL,
};
use futures::{AsyncRead, AsyncReadExt, StreamExt, TryStreamExt};
use std::{
    cell::RefCell,
    io::{self, Read},
    slice,
};
use tokio_util::compat::TokioAsyncReadCompatExt;
use tracing::*;
use zip::read::read_zipfile_from_stream;

thread_local!(
    static OLD_APPEARANCE_HANDLER: RefCell<Net_Handler> = Default::default();
);

extern "C" fn appearance_handler(data: *mut u8) {
    {
        let data = unsafe { slice::from_raw_parts(data, 68) };
        let text = unsafe { UNSAFE_GetString(&data[..64]) }.to_string();
        got_url(text);
    }

    OLD_APPEARANCE_HANDLER.with(|cell| {
        let option = &*cell.borrow();
        let f = option.unwrap();
        unsafe {
            f(data);
        }
    });
}

fn init_appearance() {
    let old_handler = unsafe { Protocol.Handlers[OPCODE__OPCODE_ENV_SET_MAP_APPEARANCE as usize] };
    unsafe {
        Protocol.Handlers[OPCODE__OPCODE_ENV_SET_MAP_APPEARANCE as usize] =
            Some(appearance_handler);
    }

    OLD_APPEARANCE_HANDLER.with(|cell| {
        let option = &mut *cell.borrow_mut();
        *option = old_handler;
    });
}

thread_local!(
    static OLD_URL_HANDLER: RefCell<Net_Handler> = Default::default();
);

extern "C" fn url_handler(data: *mut u8) {
    {
        let data = unsafe { slice::from_raw_parts(data, 64) };
        let text = unsafe { UNSAFE_GetString(&data) }.to_string();
        got_url(text);
    }

    OLD_URL_HANDLER.with(|cell| {
        let option = &*cell.borrow();
        let f = option.unwrap();
        unsafe {
            f(data);
        }
    });
}

fn init_url() {
    let old_handler = unsafe { Protocol.Handlers[OPCODE__OPCODE_ENV_SET_MAP_URL as usize] };
    unsafe {
        Protocol.Handlers[OPCODE__OPCODE_ENV_SET_MAP_URL as usize] = Some(url_handler);
    }

    OLD_URL_HANDLER.with(|cell| {
        let option = &mut *cell.borrow_mut();
        *option = old_handler;
    });
}

pub fn init() {
    init_appearance();
    init_url();
}

pub fn free() {
    //
}

fn got_url(url: String) {
    if url.is_empty() {
        return;
    }

    debug!("map url {:?}", url);

    async_manager::spawn(async move {
        if let Err(e) = async move {
            let stream = reqwest::get(url)
                .await?
                .bytes_stream()
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
                .boxed();

            let async_reader = tokio_util::io::StreamReader::new(stream);
            let async_reader = tokio::io::BufReader::new(async_reader);
            let async_reader = TokioAsyncReadCompatExt::compat(async_reader);
            let mut reader = FuturesBlockOnReader { async_reader };

            tokio::task::spawn_blocking(move || {
                while let Some(zip_file) = read_zipfile_from_stream(&mut reader)? {
                    debug!(
                        "{}: {} bytes ({} bytes packed)",
                        zip_file.name(),
                        zip_file.size(),
                        zip_file.compressed_size()
                    );

                    // let mut buf = [0u8; 16];
                    // let n = zip_file.read(&mut buf)?;
                    // debug!("The first {} bytes are: {:?}", n, &buf[0..n]);
                }

                debug!("done");

                Ok::<_, Error>(())
            })
            .await??;

            Ok::<_, Error>(())
        }
        .await
        {
            warn!("{}", e);
        }
    });
}

struct FuturesBlockOnReader<R>
where
    R: AsyncRead,
{
    async_reader: R,
}

impl<R> Read for FuturesBlockOnReader<R>
where
    R: AsyncRead + Unpin,
{
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        futures::executor::block_on(self.async_reader.read(buf))
    }
}
