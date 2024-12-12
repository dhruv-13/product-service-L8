use crate::model::Product;
use crate::configuration::Settings;

pub fn fetch_products(_settings: &Settings) -> Vec<Product> {
    vec![
        Product {
            id: 1,
            name: "Samsung 65-inch QLED 4K Smart TV".to_string(),
            price: 1299.99,
            description: "Immerse yourself in breathtaking clarity with the Samsung QLED 4K Smart TV. Quantum Dot technology delivers vibrant colors and stunning contrast for an unparalleled viewing experience.".to_string(),
            image: "/samsung-tv.jpg".to_string()
        },
        Product {
            id: 2,
            name: "Apple MacBook Pro 14-inch".to_string(),
            price: 1999.99,
            description: "Unleash your creativity and productivity with the powerful Apple MacBook Pro. Featuring the M2 Pro chip, stunning Liquid Retina XDR display, and all-day battery life.".to_string(),
            image: "/macbook-pro.jpg".to_string()
        },
        Product {
            id: 3,
            name: "Sony WH-1000XM5 Noise Canceling Headphones".to_string(),
            price: 399.99,
            description: "Experience audio perfection with Sony's premium noise-canceling headphones. Advanced sound technology, exceptional comfort, and industry-leading noise cancellation.".to_string(),
            image: "/sony-headphones.jpg".to_string()
        },
        Product {
            id: 4,
            name: "DJI Mavic 3 Drone".to_string(),
            price: 2049.99,
            description: "Capture breathtaking aerial photography with the DJI Mavic 3. Professional-grade camera, extended flight time, and intelligent tracking features for content creators.".to_string(),
            image: "/dji-drone.jpg".to_string()
        }
    ]
}
