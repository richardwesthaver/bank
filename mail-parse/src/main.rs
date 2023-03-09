// from: no-reply@cofcu.com

fn main() {
  let data = std::fs::read("/Users/ellis/mail/ellis/all/new/1678233418_4.32477.jekyll,U=64999,FMD5=a181a603769c1f98ad927e7367c7aa51:2,").unwrap();
  let r = mailparse::parse_mail(&data).unwrap();
  let headers = r.get_headers();
  let body = scraper::Html::parse_fragment(&r.get_body().unwrap());
  println!("{headers:?}");
  let sel = scraper::Selector::parse("table.transactions").unwrap();
  let sel1 = scraper::Selector::parse("tr.transaction-row").unwrap();
  let sel2 = scraper::Selector::parse("tr.transaction-row td table").unwrap();
  let sel3 = scraper::Selector::parse("td").unwrap();
  let mut tbl = body.select(&sel).into_iter();
  while let Some(tbl) = tbl.next() {
    let mut row = tbl.select(&sel1).into_iter();
    while let Some(row) = row.next() {
      let mut cell = row.select(&sel2).into_iter();
      while let Some(cell) = cell.next() {
	let mut dat = cell.select(&sel3).into_iter();
	while let Some(dat) = dat.next() {
	  println!("{0:?}", dat.inner_html());
	}
      }
    }
  }
}
