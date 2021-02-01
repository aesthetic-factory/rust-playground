use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
pub async fn print_tokio() {
    let (v1, v2, v3) = tokio::join!(
        async {
            sleep(Duration::from_millis(1500)).await;
            println!("Value 1 ready");
            "Value 1"
        },
        async {
            sleep(Duration::from_millis(2800)).await;
            println!("Value 2 ready");
            "Value 2"
        },
        async {
            sleep(Duration::from_millis(600)).await;
            println!("Value 3 ready");
            "Value 3"
        },
    );

    assert_eq!(v1, "Value 1");
    assert_eq!(v2, "Value 2");
    assert_eq!(v3, "Value 3");
}