# BÁO CÁO TRIỂN KHAI - ID NHIỆM VỤ: T-250705-13

**Người thực hiện:** Coder
**Ngày hoàn thành:** 2024-06-24

## 1. Xác nhận Hoàn thành
- [x] Đã triển khai cơ chế tạo Task mới dựa trên Command.
- [x] Đã triển khai cơ chế xóa Task đang chọn dựa trên Command.
- [x] Đã nâng cấp System `Interact` để xử lý đầu vào bàn phím.
- [x] Đã đảm bảo System `Layout` được cập nhật sau khi tạo/xóa Task.
- [x] Đã cập nhật trạng thái các công việc trong `pkb/todo.csv` thành `Done`.

## 2. Liên kết đến Commit
- **Commit Hash:** [Dán hash của commit cuối cùng vào đây]

## 3. Các Vấn đề hoặc Câu hỏi (Nếu có)
- Luồng dữ liệu khi nhấn phím 'n': System Interact phát hiện phím 'n', tạo entity mới với component Create. System Command sẽ phát hiện entity này, tạo task mới với các component mặc định, sau đó xóa entity lệnh. Layout sẽ cập nhật lại vị trí các task. 