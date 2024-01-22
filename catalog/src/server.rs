use crate::{
    catalog::{Catalog, Item},
    catalog_proto::{
        catalog_service_server::{CatalogService, CatalogServiceServer},
        ListItemsRequest, ListItemsResponse,
    },
};
use log::info;
use std::path::PathBuf;
use tonic::{transport::Server, Code, Request, Response, Status};

mod catalog;

pub mod catalog_proto {
    tonic::include_proto!("proto.catalog.v1");
}

pub struct MyCatalogService {
    pub catalog: Catalog,
}

#[tonic::async_trait]
impl CatalogService for MyCatalogService {
    async fn list_items(
        &self,
        request: Request<ListItemsRequest>,
    ) -> Result<Response<ListItemsResponse>, Status> {
        info!("got a request from {:?}", request.remote_addr());

        let reply = ListItemsResponse {
            items: self
                .catalog
                .list_items()
                .map_err(|err| Status::new(Code::Unavailable, err.to_string()))?
                .iter()
                .cloned()
                .map(Item::into)
                .collect(),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;

    let path = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR")?).join("catalog.sqlite");
    let catalog_service = MyCatalogService {
        catalog: Catalog::new(path.as_path(), "catalog1"),
    };

    Server::builder()
        .add_service(CatalogServiceServer::new(catalog_service))
        .serve(addr)
        .await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::catalog::Catalog;
    use rusqlite::Connection;
    use std::path::PathBuf;

    #[test]
    fn fetch_items_from_db() {
        let path =
            PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap()).join("catalog.sqlite");
        let conn = Connection::open(&path).unwrap();
        let sql = "CREATE TABLE IF NOT EXISTS catalog1 (
        id PRIMARY KEY,
        title TEXT NOT NULL,
        price REAL NOT NULL,
        item_count INTEGER
 );";
        conn.execute(sql, []).unwrap();
        let catalog = Catalog::new(path.as_path(), "catalog1");
        catalog.list_items().unwrap();
    }
}
