pub mod client;
pub mod request;

#[cfg(test)]
mod tests {
    use crate::client::Client;
    use crate::client::Request;
    use crate::request::category::FindCateByPidRequest;
    #[test]
    fn it_works() {
        let clt = Client::new("Replace with your App Key", "Replace with your App Secret");
        let req = FindCateByPidRequest::new(652, "");
        match req.run(&clt, "Replace with your access token") {
            Ok(r) => println!("==============ret: {:#?}", r),
            Err(e) => println!("==============err: {}", e),
        };
        assert_eq!(2 + 2, 4);
        println!("test end");
    }
}
