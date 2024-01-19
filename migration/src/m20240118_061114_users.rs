use sea_orm_migration::prelude::*;

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
    Fullname,
    #[sea_orm(iden = "created_at")]
    CreatedAt,
    #[sea_orm(iden = "updated_at")]
    UpdatedAt,
    #[sea_orm(iden = "phone_number")]
    PhoneNumber,
    Email,
    Password,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        // todo!();

        let _ = manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Users::Id)
                            .uuid()
                            .default(PgFunc::gen_random_uuid())
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Users::Fullname).string().not_null())
                    .col(
                        ColumnDef::new(Users::CreatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Users::UpdatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(ColumnDef::new(Users::PhoneNumber).string().not_null())
                    .col(ColumnDef::new(Users::Email).string().not_null())
                    .col(ColumnDef::new(Users::Password).string().not_null())
                    .to_owned(),
            )
            .await;

        let db = manager.get_connection();
        let _ = db.execute_unprepared(
            "
        CREATE OR REPLACE FUNCTION update_updated_at_column()   
        RETURNS TRIGGER AS $$
        BEGIN
            NEW.updated_at = now();
            RETURN NEW;   
        END;
        $$ language 'plpgsql';",
        );

        let _ = db.execute_unprepared(
            "
            CREATE TRIGGER update_users_updated_at
            BEFORE UPDATE ON users 
            FOR EACH ROW EXECUTE PROCEDURE update_updated)at_column();
        ",
        );

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        // todo!();

        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await
    }
}
