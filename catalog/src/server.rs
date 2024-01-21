use crate::{
    catalog::{Catalog, Item},
    catalog_proto::{
        catalog_service_server::{CatalogService, CatalogServiceServer},
        ListItemsRequest, ListItemsResponse,
    },
};
use log::info;
use tonic::{transport::Server, Request, Response, Status};

mod catalog;

pub mod catalog_proto {
    tonic::include_proto!("proto.catalog.v1");
}

#[derive(Default)]
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
    let catalog_service = MyCatalogService::default();

    Server::builder()
        .add_service(CatalogServiceServer::new(catalog_service))
        .serve(addr)
        .await?;

    Ok(())
}
