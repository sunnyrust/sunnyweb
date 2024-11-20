//!
//! <div align="center">
//!  
//!   <h3>An async ORM written in rust using the sqlx framework.</h3>
//! </div>
//!
//! # What is aok
//! aok is an ORM for the Rust programming language.
//! The primary way to interact with aok is by adding the "aok" and "sqlx" macros to your structs.
//! This adds to your structs aok functions and allows for great autocompletion and very intuitive code.
//!
//! # Macros - Setup your structs
//!
//! To add aok to your struct you will need to derive `sqlx::FromRow` and `aok::aokModel`
//! ```rust,ignore
//! #[derive(sqlx::FromRow, aok::AokModel)]
//! ```
//!
//! ## Databases
//! You will need to tell aok what Databases you want to support.
//! ```rust,ignore
//! #[aok(db(Postgres))]
//! ```
//! You can list them individually or all together
//! ```rust,ignore
//! #[aok(db(Postgres, Mssql, Mysql, Sqlite))]
//! ```
//!
//! ## Connect to table / view
//! let aok know what DB table/schema it will be reading/writing data from
//! ```rust,ignore
//! #[aok(schema = "public", table = "products")]
//! ```
//! if you don't want to allow writing to the table you can add readonly. (useful for views)
//! ```rust,ignore
//! #[aok(readonly)]
//! ```
//!
//! ## Build Relations for Joining
//! You can write queries that join across tables if you Wireup aok with relationships
//!
//! aok Supports:
//! - BelongsTo
//! - HasMany
//!
//! They are both in the format:
//! ```rust,ignore
//! [field, rust_path_to_other_object, "str_of_foreign_key_column"]
//! ```
//! NOTE: Both `BelongsTo` and `HasMany` need to know the foreign_key column
//!
//! If you are working on a struct that has a foreign_key to another table use a `BelongsTo`
//!
//! ```rust,ignore
//! #[aok(BelongsTo(product, super::product::product_id, "product_id"))]
//! struct Order {
//!     product_id: ...,
//!     ...,
//! }
//! ```
//!
//! If you are working on a struct that has a foreign_key pointed at it, use a `HasMany`
//! ```rust,ignore
//! #[aok(HasMany(orders, super::order::Order, "product_id"))]
//! struct Product {
//!     ...,
//! }
//! ```
//!
//! If working with a many-to-many relationship, add a `HasMany` on both structs, and add two `BelongsTo`
//! on the join table struct
//! ```rust,ignore
//! #[aok(BelongsTo(order, super::order::Order, "order_id"))]
//! #[aok(BelongsTo(product, super::product::Product, "product_id"))]
//! struct ProductOrders {
//!   #[aok(primary_key)]
//!   id: i32,
//!   order_id: i32,
//!   product_id: i32,
//! }
//! ```
//!
//! ## Fields Level Attributes
//! There are a couple attributes you can add to your fields to help control how aok functions
//! - `#[aok(primary_key)]` Important! Add this to the primary key of your table.
//! - `#[sqlx(rename = "xyz")]` let aok know the underlying column has a different name than the field
//! - `#[aok(ignore)]` Tell aok this fields it not in the database.
//!
//!
//! ## Putting it all together
//! Here is a working example of what a fully setup struct might look like
//! ```rust,ignore
//! #[derive(Debug, sqlx::FromRow, aok::AokModel)]
//! #[aok(db(Postgres))]
//! #[aok(schema = "public", table = "products")]
//! #[aok(HasMany(order, super::order::Order, "product_id"))]
//! pub struct Product {
//!     #[aok(primary_key)]
//!     #[sqlx(rename = "ID")]
//!     pub id: i32,
//!     pub active: Option<bool>,
//!     pub description: Option<String>,
//!     pub name: String,
//! }
//! ```
//!
//! # Using aok
//!
//! When AokModel is added to your struct the following is added directly to your model
//!
//! - [`Model::new()`]( ./state/struct.DbState.html#method.new_uncreated ) make a new model ready to be saved to the database
//! - `Model::find_by_id()`
//! - [`Model::all()`](./query/builder/struct.QueryBuilder.html) start a query for a Model
//! - [`Model::where_col()`](./query/builder/struct.QueryBuilder.html) start a query for a Model
//! - `Model::from_raw_sql()` finds Model using raw custom SQL str
//!
//! Instances of your model are wrapped in a [aok::state::DbState](./state/struct.DbState.html).
//! From your instance you can update, create, and delete.
//!
//! Start a query from the struct that derived AokModel
//! ```rust,ignore
//! let conn = aok::connection::connect_postgres(&url).await.unwrap();
//! let sellers = Product::all()
//!       .where_col(|product| product.price.equal(3.50))
//!       .map_query(|product| product.seller )
//!       .where_col(|seller| seller.name.ilike("%Nessie%") )
//!       .order_by_desc(|seller| seller.id )
//!       .limit( 10 )
//!       .run(&conn).await?;
//!
//! ```
//!
//! For more examples on how to use aok check out the 
//!
//!

pub(crate) mod alias;
pub mod connection;
pub mod errors;
pub mod query;
pub mod relations;
pub mod state;
pub mod table;
pub mod writers;

#[cfg(feature = "detect")]
/// query the database and return schema and table information
pub mod detect;

#[cfg(feature = "check")]
/// Get differences between your rust aok structs and what is in the database.
/// Useful to detect when the two get out of sync
pub mod check;

#[cfg(feature = "example_objects")]
/// This is a set of example Structs that derive AokModel.
///
/// They are here so you can see what aok will create when you derive AokModel.
/// In addition to the original structs, several helper structs are created. They are used for
/// generating queries
///
/// For this set of example objects, We are wiring up Three database tables. The `Product` and `Order`
/// tables have a many-to-many relationship with a join table `products_orders` in the middle
///
/// Viewing the source for these example objects is very useful to help understand aok.
pub mod example_objects;

// Re-exports
pub use sqlx;
pub use aok_macros::AokModel;
