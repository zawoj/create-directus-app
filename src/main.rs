use clap::Parser;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

/// Struct for CLI arguments parsing using clap
#[derive(Parser)]
#[clap(name = "create-directus-app")]
#[clap(version = "1.0")]
#[clap(about = "CLI tool to create Directus app structure and configuration")]
struct Cli {
    #[clap(value_parser)]
    project_name: String,
}

fn main() {
    let args = Cli::parse();

    // Create the project directories
    create_project_directories(&args.project_name);

    // Generate .env and .env.example files
    create_env_files(&args.project_name);

    // Generate docker-compose.yml file
    create_docker_compose_file(&args.project_name);

}

fn create_project_directories(project_name: &str) {
    let base_path = Path::new(project_name);

    // List of directories to create
    let directories = vec![
        base_path.join("database"),
        base_path.join("uploads"),
        base_path.join("extensions"),
        base_path.join("data"),
    ];

    // Create each directory
    for dir in directories {
        if let Err(e) = fs::create_dir_all(&dir) {
            eprintln!("Failed to create directory {}: {}", dir.display(), e);
        } else {
            println!("Created directory: {}", dir.display());
        }
    }
}

fn create_env_files(project_name: &str) {
    let base_path = Path::new(project_name);
    let env_path = base_path.join(".env");
    let env_example_path = base_path.join(".env.example");

    // Content for .env and .env.example
    let env_content = r#"
ADMIN_USERNAME=test@gmail.com
ADMIN_PASSWORD=test_admin
ADMIN_EMAIL=test@gmail.com
SOFTWARE_PASSWORD=sfgsfhg
SOFTWARE_VERSION_TAG=latest
DOMAIN=your.domain.com
EMAIL_FROM=sender@email.com
TEST_URL=http://172.17.0.1:8055/admin/login
EXTENSIONS_AUTO_RELOAD=true
CORS_ENABLED=true
CORS_ORIGIN=true
"#;

    // Write .env file
    write_to_file(&env_path, env_content);
    println!("Created file: {}", env_path.display());

    // Write .env.example file
    write_to_file(&env_example_path, env_content);
    println!("Created file: {}", env_example_path.display());
}

fn write_to_file(path: &Path, content: &str) {
    let mut file = match File::create(path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Failed to create file {}: {}", path.display(), e);
            return;
        },
    };

    if let Err(e) = file.write_all(content.as_bytes()) {
        eprintln!("Failed to write to file {}: {}", path.display(), e);
    }
}

fn create_docker_compose_file(project_name: &str) {
    let base_path = Path::new(project_name);
    let compose_path = base_path.join("docker-compose.yml");

    let compose_content = r#"
version: "3.3"
services:
  database:
    image: elestio/postgres:15
    restart: always
    volumes:
      - ./data/database:/var/lib/postgresql15/data
    networks:
      - directus
    environment:
      POSTGRES_USER: "directus"
      POSTGRES_PASSWORD: ${SOFTWARE_PASSWORD}
      POSTGRES_DB: "directus"
      PGDATA: /var/lib/postgresql15/data/db_data
  cache:
    image: redis:6
    restart: always
    networks:
      - directus
  directus:
    restart: always
    image: elestio/directus:${SOFTWARE_VERSION_TAG}
    ports:
      - 8055:8055
    user: 0:0
    volumes:
      - ./uploads:/directus/uploads
      - ./extensions:/directus/extensions
    networks:
      - directus
    depends_on:
      - cache
      - database
    environment:
      KEY: ${SOFTWARE_PASSWORD}
      SECRET: ${SOFTWARE_PASSWORD}
      PUBLIC_URL: https://${DOMAIN}
      DB_CLIENT: "pg"
      DB_HOST: "database"
      DB_PORT: "5432"
      DB_DATABASE: "directus"
      DB_USER: "directus"
      DB_PASSWORD: ${SOFTWARE_PASSWORD}
      EMAIL_FROM: ${EMAIL_FROM}
      EMAIL_TRANSPORT: "smtp"
      EMAIL_SMTP_HOST: "172.17.0.1"
      EMAIL_SMTP_PORT: 25
      EMAIL_SMTP_SECURE: "false"
      EMAIL_SMTP_IGNORE_TLS: "true"
      CACHE_ENABLED: "true"
      CACHE_STORE: "redis"
      CACHE_REDIS: "redis://cache:6379"
      REDIS: "redis://cache:6379"
      ADMIN_EMAIL: ${ADMIN_EMAIL}
      ADMIN_PASSWORD: ${ADMIN_PASSWORD}
      CACHE_AUTO_PURGE: "true"
      NODE_TLS_REJECT_UNAUTHORIZED: 0
      EXTENSIONS_AUTO_RELOAD: "true"
      CHOKIDAR_USEPOLLING: "true"
      CORS_ENABLED: "true"
      CORS_ORIGIN: "true"
networks:
  directus:
.
"#;

    write_to_file(&compose_path, compose_content);
    println!("Created file: {}", compose_path.display());
}
