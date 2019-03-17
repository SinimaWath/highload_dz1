use super::config;

struct TestCase {
    path: String,
    expected: Option<config::Config>,
    err: Option<String>,
}

#[test]
fn test_valid_ok() {
    let test = TestCase{
        path: String::from("test/test.txt"),
        expected: None,
        err: None,
    };

    let result = config::Config::read(&&test.path[..]);
    assert_eq!(result.is_ok(), true);
}

#[test]
fn test_valid_config() {
    let test = TestCase{
        path: String::from("test/test.txt"),
        expected: Some(config::Config {
            dir_root: String::from("test/test.txt"),
            thread_count: 0,
        }),
        err: None,
    };

    match config::Config::read(&&test.path[..]) {
        Ok(cfg) => {
            let expected = test.expected.unwrap();
            assert_eq!(cfg.dir_root, expected.dir_root);
            assert_eq!(cfg.thread_count, expected.thread_count);
        },
        Err(err) => panic!("Unexcpected error {}", err),
    }
}

#[test]
fn test_not_contains_document_root() {
    let test = TestCase{
        path: String::from("test/test_no_document_root.txt"),
        expected: None,
        err: Some(config::DOCUMENT_ROOT_ERROR.to_owned()),
    };

    match config::Config::read(&&test.path[..]) {
        Ok(_) => panic!("Unexcpected OK"),
        Err(err) => assert_eq!(err, test.err.unwrap()),
    }    
}

#[test]
fn test_invalid_format_document_root() {
    let test = TestCase{
        path: String::from("test/test_document_root_invalid.txt"),
        expected: None,
        err: Some(config::DOCUMENT_ROOT_INVALID_FORMAT.to_owned()),
    };

    match config::Config::read(&&test.path[..]) {
        Ok(_) => panic!("Unexcpected OK"),
        Err(err) => assert_eq!(err, test.err.unwrap()),
    }    
}

#[test]
fn test_not_contains_thread_limit() {
    let test = TestCase{
        path: String::from("test/test_no_thread_limit.txt"),
        expected: None,
        err: Some(config::THREAD_LIMIT_ERROR.to_owned()),
    };

    match config::Config::read(&&test.path[..]) {
        Ok(_) => panic!("Unexcpected OK"),
        Err(err) => assert_eq!(err, test.err.unwrap()),
    }   
}

fn test_invalid_format_thread_limit() {
    let test = TestCase{
        path: String::from("test/test_thread_limit_invalid.txt"),
        expected: None,
        err: Some(config::THREAD_LIMIT_INVALID_FORMAT.to_owned()),
    };

    match config::Config::read(&&test.path[..]) {
        Ok(_) => panic!("Unexcpected OK"),
        Err(err) => assert_eq!(err, test.err.unwrap()),
    }    
}
