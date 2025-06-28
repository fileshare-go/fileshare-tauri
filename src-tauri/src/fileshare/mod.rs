use std::{fs, os::unix::fs::MetadataExt, path::Path};

use tokio_stream::wrappers::ReceiverStream;

use crate::{
    gen,
    loader::{self, ChunkLoader},
    util,
};

pub async fn upload(path: impl AsRef<Path>) -> Result<(), String> {
    println!("{:?}", path.as_ref().to_str());
    let mut client =
        gen::upload_service_client::UploadServiceClient::connect("http://127.0.0.1:60011")
            .await
            .unwrap();
    let filename = path.as_ref().to_string_lossy().to_string();
    let file = fs::File::open(path.as_ref()).unwrap();
    let file_size = file.metadata().unwrap().size() as i64;
    let sha256 = util::calculate_sha256(path.as_ref()).unwrap();
    let meta = gen::FileMeta {
        filename,
        file_size,
        sha256: sha256.clone(),
    };
    let upload_request = gen::UploadRequest {
        file_size,
        meta: Some(meta),
    };

    let resp = client
        .pre_upload(tonic::Request::new(upload_request.into()))
        .await
        .unwrap()
        .into_inner();

    let mut loader = loader::SerialLoader::new(path, resp.chunk_size);
    dbg!(&resp.chunk_list);

    let (tx, rx) = tokio::sync::mpsc::channel(20);
    tokio::spawn(async move {
        for idx in resp.chunk_list {
            match loader.load(idx) {
                Ok(buf) => {
                    println!("{idx} size: {}", buf.len());
                    let file_chunk = gen::FileChunk {
                        chunk_index: idx,
                        data: buf,
                        sha256: sha256.clone(),
                    };
                    if let Err(e) = tx.send(file_chunk).await {
                        dbg!(e);
                        break;
                    }
                }
                Err(e) => {
                    dbg!(e);
                }
            }
        }
    });

    let stream = ReceiverStream::new(rx);
    client.upload(stream).await.unwrap();

    Ok(())
}
