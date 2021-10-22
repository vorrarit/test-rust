
mod bitkub;

use bitkub::bitkub::{Ticker, exchange_get_tickers};
use sqlx::{PgConnection, Connection, FromRow, Type};
use futures::TryStreamExt;
use actix_web::{HttpServer, App, Responder, get, HttpResponse};

#[derive(Debug, FromRow, Type)]
struct Position {
    id: i64,
    symbol: String,
    qty: sqlx::types::BigDecimal,
    purchase_token_price: sqlx::types::BigDecimal,
    sell_token_price: sqlx::types::BigDecimal,
    status: String,
    created_date: chrono::DateTime<chrono::Utc>
}

// #[tokio::main]
// async fn main1() -> Result<(), Box<dyn std::error::Error>> {
//     let tickers = exchange_get_tickers().await?;

//     let btc = tickers.get("THB_BTC").unwrap();
//     println!("{}", btc.last);


//     let mut con = PgConnection::connect("postgres://postgres:password@localhost:5432/tradingbot").await?;
//     let mut rows = sqlx::query_as::<_, Position>("select * from positions").fetch(&mut con);
//     while let Some(position) = rows.try_next().await? {
//         println!("{} {} {}", position.symbol, position.qty, position.created_date);
//     }


//     Ok(())
// }

#[get("/THB_BTC")]
async fn get_btc() -> impl Responder {
    let tickers_response = exchange_get_tickers().await;
    let btc_price = match tickers_response {
        Ok(tickers) => {
            let btc = tickers.get("THB_BTC").unwrap();
            println!("{}", btc.last);
            btc.last
        },
        Err(err) => {
            println!("{}", err);
            0.0
        }
    };



    // let tickers = match exchange_get_tickers().await {
    //     Ok(tickers) => tickers,
    //     Err(error) => return HttpResponse:
    // }

    // let btc = tickers.get("THB_BTC").unwrap();
//     println!("{}", btc.last);

    // HttpResponse::Ok().body("hello")
    format!("{}", btc_price)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .service(get_btc)
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}