/// Component lưu thông tin người dùng.
pub struct User {
    pub name: String,
}

/// Component thể hiện quyền sở hữu, trỏ đến Entity của người dùng.
pub struct Owner(pub usize); 