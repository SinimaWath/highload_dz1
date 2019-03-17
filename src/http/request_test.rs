// use dz1::::request;
use super::request;

struct TestCase {
    raw_http: &'static [u8],
    expected: request::HTTPRequest,
}

#[test]
fn get_method_parse() {
    let testCase = TestCase{
        raw_http: "GET /foo/bar/ HTTP/1.1".as_bytes(),
        expected: request::HTTPRequest{method: String::from("GET"), path: String::from("")},
    };

    let result = match request::HTTPRequest::parse(testCase.raw_http) {
        Ok(req) => assert_eq!(req.method, testCase.expected.method),
        Err(()) => panic!("Unsxpected panic"),
    };
}

#[test]
fn head_method_parse() {
    let testCase = TestCase{
        raw_http: "HEAD /foo/bar/ HTTP/1.1".as_bytes(),
        expected: request::HTTPRequest{method: String::from("HEAD"), path: String::from("")},
    };

    let result = match request::HTTPRequest::parse(testCase.raw_http) {
        Ok(req) => assert_eq!(req.method, testCase.expected.method),
        Err(()) => panic!("Unexpected panic"),
    };
}

#[test]
fn not_valid_method_parse() {
    let testCase = TestCase{
        raw_http: "POST /foo/bar/ HTTP/1.1".as_bytes(),
        expected: request::HTTPRequest::new(),
    };

    let result = match request::HTTPRequest::parse(testCase.raw_http) {
        Ok(req) => panic!("Unexpected header"),
        Err(()) => 1,
    };
}

#[test]
fn valid_path_parse() {
    let testCase = TestCase{
        raw_http: "HEAD /foo/bar HTTP/1.1".as_bytes(),
        expected: request::HTTPRequest{method: String::from("HEAD"), path: String::from("/foo/bar")},
    };

    let result = match request::HTTPRequest::parse(testCase.raw_http) {
        Ok(req) => assert_eq!(req.path, testCase.expected.path),
        Err(()) => panic!("Unexpected panic"),
    };
}

#[test]
fn index_valid_path_parse() {
    let testCase = TestCase{
        raw_http: "HEAD /foo/bar/ HTTP/1.1".as_bytes(),
        expected: request::HTTPRequest{method: String::from("HEAD"), path: String::from("/foo/bar/index.html")},
    };

    let result = match request::HTTPRequest::parse(testCase.raw_http) {
        Ok(req) => assert_eq!(req.path, testCase.expected.path),
        Err(()) => panic!("Unexpected panic"),
    };
}

#[test]
fn with_query_valid_path_parse() {
    let testCase = TestCase{
        raw_http: "HEAD /foo/bar/kek.html?asdsa HTTP/1.1".as_bytes(),
        expected: request::HTTPRequest{method: String::from("HEAD"), path: String::from("/foo/bar/kek.html")},
    };

    let result = match request::HTTPRequest::parse(testCase.raw_http) {
        Ok(req) => assert_eq!(req.path, testCase.expected.path),
        Err(()) => panic!("Unexpected panic"),
    };
}


#[test]
fn with_space_valid_path_parse() {
    let testCase = TestCase{
        raw_http: "HEAD /foo/bar/space%20in%20name.html HTTP/1.1".as_bytes(),
        expected: request::HTTPRequest{method: String::from("HEAD"), path: String::from("/foo/bar/space in name.html")},
    };

    let result = match request::HTTPRequest::parse(testCase.raw_http) {
        Ok(req) => assert_eq!(req.path, testCase.expected.path),
        Err(()) => panic!("Unexpected panic"),
    };
}

#[test]
fn with_space_query_valid_path_parse() {
    let testCase = TestCase{
        raw_http: "HEAD /foo/bar/space%20in%20name.html?l&=1 HTTP/1.1".as_bytes(),
        expected: request::HTTPRequest{method: String::from("HEAD"), path: String::from("/foo/bar/space in name.html")},
    };

    let result = match request::HTTPRequest::parse(testCase.raw_http) {
        Ok(req) => assert_eq!(req.path, testCase.expected.path),
        Err(()) => panic!("Unexpected panic"),
    };
}


#[test]
fn url_encode_valid_path_parse() {
    let testCase = TestCase{
        raw_http: "HEAD /foo/bar/%70%61%67%65%2e%68%74%6d%6c HTTP/1.1".as_bytes(),
        expected: request::HTTPRequest{method: String::from("HEAD"), path: String::from("/foo/bar/page.html")},
    };

    let result = match request::HTTPRequest::parse(testCase.raw_http) {
        Ok(req) => assert_eq!(req.path, testCase.expected.path),
        Err(()) => panic!("Unexpected panic"),
    };
}

#[test]
fn url_encode_query_valid_path_parse() {
    let testCase = TestCase{
        raw_http: "HEAD /foo/bar/%70%61%67%65%2e%68%74%6d%6c?asd=1&asd HTTP/1.1".as_bytes(),
        expected: request::HTTPRequest{method: String::from("HEAD"), path: String::from("/foo/bar/page.html")},
    };

    let result = match request::HTTPRequest::parse(testCase.raw_http) {
        Ok(req) => assert_eq!(req.path, testCase.expected.path),
        Err(()) => panic!("Unexpected panic"),
    };
}

#[test]
fn dot_escape_not_valid_path_parse() {
    let testCase = TestCase{
        raw_http: "HEAD /foo/bar/../../ HTTP/1.1".as_bytes(),
        expected: request::HTTPRequest{method: String::from(""), path: String::from("")},
    };

    let result = match request::HTTPRequest::parse(testCase.raw_http) {
        Ok(req) => panic!("Unexpected OK"),
        Err(()) => 1,
    };
}

#[test]
fn file_with_dot_valid_path_parse() {
    let testCase = TestCase{
        raw_http: "HEAD /foo/bar/index..html HTTP/1.1".as_bytes(),
        expected: request::HTTPRequest{method: String::from("HEAD"), path: String::from("/foo/bar/index..html")},
    };

    let result = match request::HTTPRequest::parse(testCase.raw_http) {
        Ok(req) => assert_eq!(req.path, testCase.expected.path),
        Err(()) => panic!("Unexpected Err"),
    };
}