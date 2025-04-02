use chrono::Utc;
use diesel::{Connection, ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use uuid::Uuid;

use crate::{
    core::{
        commands::{app_service::AppService, Command},
        models::sales::sales_charge_type_model::*,
        types::db_uuid::DbUuid,
    },
    error::Result,
    schema::sales_charge_types,
};

// Commands
pub struct CreateSalesChargeTypeCommand {
    pub charge_type: SalesChargeTypeNewInput,
}

pub struct UpdateSalesChargeTypeCommand {
    pub charge_type: SalesChargeTypeUpdateInput,
}

// Command Implementations
impl Command for CreateSalesChargeTypeCommand {
    type Output = SalesChargeType;

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        service.conn.transaction(|conn| {
            let now = Utc::now().naive_utc();
            let new_charge_type = SalesChargeType {
                id: Uuid::now_v7().into(),
                name: self.charge_type.name.clone(),
                description: self.charge_type.description.clone(),
                created_at: now,
                updated_at: now,
            };

            let res = diesel::insert_into(sales_charge_types::table)
                .values(&new_charge_type)
                .returning(SalesChargeType::as_returning())
                .get_result(conn)?;

            Ok(res)
        })
    }
}

impl Command for UpdateSalesChargeTypeCommand {
    type Output = SalesChargeType;

    fn exec(&self, service: &mut AppService) -> Result<Self::Output> {
        service.conn.transaction(|conn| {
            let now = Utc::now().naive_utc();
            let changeset = self.charge_type.clone().into_changeset(now);

            let res = diesel::update(sales_charge_types::table)
                .filter(sales_charge_types::id.eq(self.charge_type.id))
                .set(changeset)
                .returning(SalesChargeType::as_returning())
                .get_result(conn)?;

            Ok(res)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::Error;

    #[test]
    fn test_create_sales_charge_type() {
        let mut service = AppService::new(":memory:");

        let input = SalesChargeTypeNewInput {
            name: "Service Charge".to_string(),
            description: Some("A charge for service".to_string()),
        };

        let cmd = CreateSalesChargeTypeCommand { charge_type: input };
        let result = cmd.exec(&mut service).unwrap();

        assert_eq!(result.name, "Service Charge");
        assert_eq!(result.description, Some("A charge for service".to_string()));
    }

    #[test]
    fn test_update_sales_charge_type() {
        let mut service = AppService::new(":memory:");

        // Create first
        let input = SalesChargeTypeNewInput {
            name: "Initial Charge".to_string(),
            description: None,
        };
        let cmd = CreateSalesChargeTypeCommand { charge_type: input };
        let created = cmd.exec(&mut service).unwrap();

        // Update
        let update_input = SalesChargeTypeUpdateInput {
            id: created.id,
            name: Some("Updated Charge".to_string()),
            description: Some(Some("Updated Description".to_string())), // Set description
        };
        let update_cmd = UpdateSalesChargeTypeCommand {
            charge_type: update_input,
        };
        let updated = update_cmd.exec(&mut service).unwrap();

        assert_eq!(updated.id, created.id);
        assert_eq!(updated.name, "Updated Charge");
        assert_eq!(updated.description, Some("Updated Description".to_string()));

        // Update again - remove description
        let update_input2 = SalesChargeTypeUpdateInput {
            id: created.id,
            name: None,              // Keep name the same
            description: Some(None), // Remove description
        };
        let update_cmd2 = UpdateSalesChargeTypeCommand {
            charge_type: update_input2,
        };
        let updated2 = update_cmd2.exec(&mut service).unwrap();

        assert_eq!(updated2.name, "Updated Charge");
        assert!(updated2.description.is_none());
    }

    #[test]
    fn test_update_non_existent_charge_type() {
        let mut service = AppService::new(":memory:");

        let update_input = SalesChargeTypeUpdateInput {
            id: Uuid::now_v7().into(),
            name: Some("Doesn't Matter".to_string()),
            description: None,
        };
        let update_cmd = UpdateSalesChargeTypeCommand {
            charge_type: update_input,
        };
        let result = update_cmd.exec(&mut service);

        assert!(matches!(result, Err(Error::DieselError(_))));
    }
}
