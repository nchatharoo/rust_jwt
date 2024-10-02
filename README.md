cargo build --release

sudo cp target/release/rust_jwt /usr/local/bin/rust_jwt

rust_jwt encode subject "MyName" my_secret_key HS256

rust_jwt decode <your_jwt_here> my_secret_key HS256
