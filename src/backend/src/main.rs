mod shared_types;

fn main() {
    println!("RHR backend scaffold");
}

#[cfg(test)]
mod tests {
    use super::shared_types::ServerStatus;
    use ts_rs::TS;

    #[test]
    fn export_bindings() {
        ServerStatus::export().expect("failed to export TypeScript bindings");
    }
}
