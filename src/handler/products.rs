use diesel;
use diesel::prelude::*;
use actix::*;
use actix_web::*;
use chrono::Utc;
use model::db::ConnDsl;
use model::response::{ProductsListMsgs, ProductOwnerListMsgs};
use model::products::{
    ProductsList,Products, ProductOwnerList,
    ProductOwner};

impl Handler<ProductsList> for ConnDsl {
    type Result = Result<ProductsListMsgs, Error>;

    fn handle(&mut self, products_list: ProductsList, _: &mut Self::Context) -> Self::Result {
        use apputils::schema::products::dsl::*;
        let conn = &self.0.get().map_err(error::ErrorInternalServerError)?;
        let fetched_products = products.load::<Products>(conn).map_err(error::ErrorInternalServerError)?;
        Ok(ProductsListMsgs { 
            status: 200,
            message : " products_list result.".to_string(),
            products_list: fetched_products,
        })
    }
}

impl Handler<ProductOwnerList> for ConnDsl {
    type Result = Result<ProductOwnerListMsgs, Error>;

    fn handle(&mut self, productowner_list: ProductOwnerList, _: &mut Self::Context) -> Self::Result {
        use apputils::schema::productowner::dsl::*;
        let conn = &self.0.get().map_err(error::ErrorInternalServerError)?;
        let fetched_pos = productowner.load::<ProductOwner>(conn).map_err(error::ErrorInternalServerError)?;
        Ok(ProductOwnerListMsgs { 
            status: 200,
            message : " productsowners_list result.".to_string(),
            productowner_list: fetched_pos,
        })
    }
}