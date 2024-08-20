use bluer::rfcomm::Stream;
use bluer::Address;

async fn set_powered_adapter(
    session: bluer::Session,
) -> Result<bluer::Adapter, Box<dyn std::error::Error>> {
    let adapter = session.default_adapter().await?;
    adapter.set_powered(true).await?;
    Ok(adapter)
}

async fn find_address(
    adapter: bluer::Adapter,
    address: [u8; 3],
) -> Result<Address, Box<dyn std::error::Error>> {
    let device_addresses = adapter.device_addresses().await?;

    let ear_address = device_addresses
        .iter()
        .find(|&addr| match addr.0 {
            [a, b, c, _, _, _] => a == address[0] && b == address[1] && c == address[2],
        })
        .ok_or_else(|| {
            "Couldn't find any Ear devices connected. Make sure you're paired with your Ear."
        })?;

    Ok(*ear_address)
}

pub async fn connect(address: [u8; 3], channel: u8) -> Result<Stream, Box<dyn std::error::Error>> {
    let session = bluer::Session::new().await?;
    let adapter = set_powered_adapter(session).await?;
    let ear_address = find_address(adapter, address).await?;

    println!("Connecting to Ear at {:?}", ear_address);

    let stream = Stream::connect(bluer::rfcomm::SocketAddr {
        addr: ear_address,
        channel,
    })
    .await?;
    Ok(stream)
}
