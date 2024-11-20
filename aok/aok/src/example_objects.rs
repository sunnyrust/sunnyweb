use super::AokModel;

/// The Product Table.
///
/// Start off a query on this table using. [all()](#method.all) or [where_col()](#method.where_col).
///
#[derive(Debug, sqlx::FromRow, AokModel)]
#[aok_path(crate)]
#[aok(db(Postgres, Mssql, Mysql, Sqlite))]
#[aok(schema = "examples", table = "products")]
#[aok(HasMany(product_orders, super::example_objects::ProductOrders, "product_id"))]
pub struct Product {
    #[aok(primary_key)]
    #[sqlx(rename = "ID")]
    pub id: i32,
    pub active: Option<bool>,
    pub description: Option<String>,
    pub name: String,
}

/// The Order Table.
///
/// Start off a query on this table using. [all()](#method.all) or [where_col()](#method.where_col).
///
/// ```
/// use aok::example_objects::Order;
///
/// async fn run_query<C>(conn: &C) -> aok::errors::Result<()>
///     where C: aok::connection::Connection<sqlx::Postgres>
/// {
///   let q = Order::where_col(|o| o.id.equal(3) )
///       .map_query(|o| o.product_orders )
///       .map_query(|po| po.product )
///       .where_col(|p| p.active.equal(true) );
///   let count_product = q.count(conn).await?;
///   let first_product = q.limit(1).run(conn).await?;
///   Ok(())
/// }
/// ```
///
#[derive(Debug, sqlx::FromRow, AokModel)]
#[aok_path(crate)]
#[aok(db(Postgres, Mssql, Mysql, Sqlite))]
#[aok(schema = "examples", table = "orders")]
#[aok(HasMany(product_orders, super::example_objects::ProductOrders, "order_id"))]
pub struct Order {
    #[aok(primary_key)]
    #[sqlx(rename = "ID")]
    pub id: i32,
    pub customer: String,
    pub sell_price: i32,
}

/// The ProductOrder Join Table.
///
/// Start off a query on this table using. [all()](#method.all) or [where_col()](#method.where_col).
///
#[derive(Debug, sqlx::FromRow, AokModel)]
#[aok_path(crate)]
#[aok(db(Postgres, Mssql, Mysql, Sqlite))]
#[aok(schema = "examples", table = "products_orders")]
#[aok(BelongsTo(order, super::example_objects::Order, "order_id"))]
#[aok(BelongsTo(product, super::example_objects::Product, "product_id"))]
pub struct ProductOrders {
    #[aok(primary_key)]
    id: i32,
    order_id: i32,
    product_id: i32,
}
