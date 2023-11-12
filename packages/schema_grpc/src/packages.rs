pub mod todo_package {
  tonic::include_proto!("todo_package");

  pub const FILE_DESCRIPTOR_SET: &[u8] =
    tonic::include_file_descriptor_set!("todo_package_descriptor");
}
