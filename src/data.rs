use crate::model::Product;
use crate::configuration::Settings;

pub fn fetch_products(_settings: &Settings) -> Vec<Product> {
    vec![
        Product {
            id: 1,
            name: "Smart 4K UHD TV".to_string(),
            price: 499.99,
            description: "Experience stunning clarity with the Best Buy Smart 4K UHD TV, featuring HDR support and built-in streaming apps.".to_string(),
            image: "/smart_tv.jpg".to_string()
        },
        Product {
            id: 2,
            name: "Wireless Noise-Cancelling Headphones".to_string(),
            price: 299.99,
            description: "Immerse yourself in pure sound with these wireless noise-cancelling headphones, designed for long-lasting comfort and superior audio quality.".to_string(),
            image: "/headphones.jpg".to_string()
        },
        Product {
            id: 3,
            name: "Gaming Laptop".to_string(),
            price: 1199.99,
            description: "Dominate your gaming sessions with the high-performance Best Buy Gaming Laptop, equipped with a powerful GPU and fast refresh rate.".to_string(),
            image: "/gaming_laptop.jpg".to_string()
        },
        Product {
            id: 4,
            name: "Smart Home Speaker".to_string(),
            price: 79.99,
            description: "Control your smart home with ease using the Best Buy Smart Home Speaker, featuring voice assistant integration and rich sound.".to_string(),
            image: "/smart_speaker.jpg".to_string()
        }
        // Add more products or connect to a database here
    ]
}
