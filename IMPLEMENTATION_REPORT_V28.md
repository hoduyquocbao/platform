# IMPLEMENTATION REPORT V28

## ID Nhiệm vụ: T-250720-28

### 1. Tóm tắt mục tiêu
- Hoàn thiện System Layout với logic Align và Justify, loại bỏ cảnh báo unused code.
- Định nghĩa và thêm Component Button, tạo entity Button "Create" vào Footer, căn giữa bằng Justify::Center.
- Nâng cấp System Interact để xử lý Click trên Button, tạo task mới khi nhấn nút.

### 2. Các bước thực hiện
1. Thêm các nhiệm vụ mới vào pkb/todo.csv.
2. Định nghĩa Component Button trong src/components/ui.rs.
3. Chuyển các field layout (container, flows, aligns, justifys) thành vector<Option<T>> trong World, cập nhật logic initialize để gán đúng cho từng entity.
4. Nâng cấp System Layout (src/systems/layout.rs) để xử lý Align và Justify cho cả Flow::Row và Flow::Column, clone các biến cần thiết để tránh borrow lỗi.
5. Thêm entity Button "Create" vào Footer, gán Justify::Center cho Footer.
6. Nâng cấp System Interact để khi entity có Click và Button thì tạo entity lệnh Create.
7. Đảm bảo code biên dịch thành công, không còn lỗi borrow, clone, unused code.

### 3. Logic Justify và xác nhận nút "Create"
- Logic Justify:
  - Với Flow::Column: tính tổng chiều cao, chia đều cho các con, tính extra_space. Justify::Center căn giữa các con theo trục dọc, Justify::Between phân phối đều khoảng cách giữa các con.
  - Với Flow::Row: tương tự, chia đều chiều rộng, Justify::Center căn giữa các con theo trục ngang.
- Nút "Create" đã được layout căn giữa trong Footer nhờ Justify::Center. Khi click vào nút, một task mới được tạo ra đúng như mong đợi.

### 4. Kết quả kiểm thử
- Giao diện hiển thị đúng: Header, Content, Footer, nút "Create" căn giữa ở Footer.
- Click vào nút "Create" tạo ra task mới, xác nhận logic hoạt động đúng.
- Không còn cảnh báo unused code liên quan đến layout.

### 5. Các vấn đề hoặc câu hỏi
- Logic Align/Justify hiện đã đủ cho các trường hợp cơ bản, có thể mở rộng thêm cho các layout phức tạp hơn trong tương lai.
- Đề xuất: Có thể bổ sung thêm các widget khác (checkbox, input, v.v.) trong các chu kỳ tiếp theo.

### 6. Đã cập nhật PKB và todo.csv đầy đủ.

---

Người thực hiện: Coder
Ngày hoàn thành: 20-07-2025 