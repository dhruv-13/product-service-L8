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
        },
        Product {
            id: 5,
            name: "LG OLED C2 55-inch 4K Smart TV".to_string(),
            price: 1499.99,
            description: "Dive into the world of perfect blacks and infinite contrast with the LG OLED C2. Self-lit pixels deliver stunning picture quality for the ultimate home entertainment experience.".to_string(),
            image: "/lg-oled-tv.jpg".to_string()
        },
        Product {
            id: 6,
            name: "Xbox Series X Gaming Console".to_string(),
            price: 499.99,
            description: "Next-generation gaming power meets lightning-fast load times. The Xbox Series X delivers true 4K gaming, ray tracing, and access to hundreds of incredible games.".to_string(),
            image: "/xbox-series-x.jpg".to_string()
        },
        Product {
            id: 7,
            name: "iPad Pro 12.9-inch Wi-Fi + Cellular".to_string(),
            price: 1599.99,
            description: "Powerful, versatile, and portable. The iPad Pro features the M2 chip, stunning Liquid Retina XDR display, and supports Apple Pencil and Magic Keyboard.".to_string(),
            image: "/ipad-pro.jpg".to_string()
        },
        Product {
            id: 8,
            name: "Bose QuietComfort Ultra Earbuds".to_string(),
            price: 299.99,
            description: "Immerse yourself in world-class noise cancellation and premium sound. Bose QuietComfort Ultra Earbuds deliver exceptional audio quality and comfort.".to_string(),
            image: "/bose-earbuds.jpg".to_string()
        },
        Product {
            id: 9,
            name: "GoPro HERO11 Black".to_string(),
            price: 399.99,
            description: "Capture life's most exciting moments in stunning 5.3K video and 27MP photos. The GoPro HERO11 Black offers advanced stabilization and rugged waterproof design.".to_string(),
            image: "/gopro-hero11.jpg".to_string()
        },
        Product {
            id: 10,
            name: "Samsung Galaxy S23 Ultra Smartphone".to_string(),
            price: 1199.99,
            description: "Revolutionize your mobile experience with the Samsung Galaxy S23 Ultra. Featuring a 200MP camera, S Pen integration, and powerful performance in a sleek design.".to_string(),
            image: "/samsung-s23-ultra.jpg".to_string()
        }
    ]
}