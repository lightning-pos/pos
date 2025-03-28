use chrono::Utc;
use diesel::{Connection, QueryDsl, RunQueryDsl, SelectableHelper};
use uuid::Uuid;

use crate::{
    core::{
        commands::{app_service::AppService, Command},
        models::sales::supplier_model::{
            Supplier, SupplierNewInput, SupplierUpdateChangeset, SupplierUpdateInput,
        },
        types::db_uuid::DbUuid,
    },
    error::Result,
    schema::suppliers,
};

// Commands
pub struct CreateSupplierCommand {
    pub supplier: SupplierNewInput,
}

pub struct UpdateSupplierCommand {
    pub supplier: SupplierUpdateInput,
}

pub struct DeleteSupplierCommand {
    pub id: DbUuid,
}

// Command Implementations
impl Command for CreateSupplierCommand {
    type Output = Supplier;

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        service.conn.transaction(|conn| {
            let now = Utc::now().naive_utc();
            let new_supplier = Supplier {
                id: Uuid::now_v7().into(),
                name: self.supplier.name.clone(),
                address: self.supplier.address.clone(),
                phone: self.supplier.phone.clone(),
                created_at: now,
                updated_at: now,
            };

            let res = diesel::insert_into(suppliers::table)
                .values(&new_supplier)
                .returning(Supplier::as_returning())
                .get_result(conn)?;

            Ok(res)
        })
    }
}

impl Command for UpdateSupplierCommand {
    type Output = Supplier;

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        service.conn.transaction(|conn| {
            let now = Utc::now().naive_utc();
            let supplier_id = self.supplier.id;

            let changeset = SupplierUpdateChangeset {
                id: supplier_id,
                name: self.supplier.name.clone(),
                address: self.supplier.address.clone(),
                phone: self.supplier.phone.clone(),
                updated_at: now,
            };

            let res = diesel::update(suppliers::table.find(supplier_id))
                .set(changeset)
                .returning(Supplier::as_returning())
                .get_result(conn)?;

            Ok(res)
        })
    }
}

impl Command for DeleteSupplierCommand {
    type Output = i32;

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        service.conn.transaction(|conn| {
            let res = diesel::delete(suppliers::table.find(self.id)).execute(conn)?;
            Ok(res as i32)
        })
    }
}
