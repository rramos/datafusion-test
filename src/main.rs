use datafusion::prelude::*;

#[tokio::main]
async fn main() -> datafusion::error::Result<()> {
  // create the dataframe
  let ctx = SessionContext::new();
  let df = ctx.read_csv("tests/data/example.csv", CsvReadOptions::new()).await?;

  let df = df.filter(col("a").lt_eq(col("b")))?
           .aggregate(vec![col("a")], vec![min(col("b"))])?
           .limit(0, Some(100))?;

  // execute and print results
  df.show().await?;
  Ok(())
}