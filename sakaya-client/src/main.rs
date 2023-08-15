fn main() -> Result<(), minreq::Error> {
   let o = minreq::get("http://127.0.0.1:7878").send()?;
   let s = o.as_str()?;
   print!("{}", s);
   Ok(())
}
