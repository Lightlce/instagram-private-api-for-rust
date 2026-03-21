use instagram_private_api_rust::client::IgApiClient;

fn main() {
    let client = IgApiClient::from_seed("example_user");

    println!("login endpoint: {}", client.repositories.account.login_endpoint());
    println!(
        "publish plan: {:?}",
        client.services.publish.create_photo_publish_plan()
    );
    println!(
        "live create endpoint: {}",
        client.repositories.advanced.live.create_broadcast_endpoint()
    );
}
