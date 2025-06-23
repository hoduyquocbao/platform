# IMPLEMENTATION REPORT V19

**Task ID:** T-250711-19
**Title:** Chu kỳ 3: Mở rộng Siêu dữ liệu - Ngày hết hạn
**Date:** 11-07-2025
**Assignee:** Coder

---

## 1. Mô tả nhiệm vụ

Bổ sung component Due (ngày hết hạn) và resource Time, cho phép người dùng đặt/sửa ngày hết hạn cho công việc, hiển thị và cảnh báo khi quá hạn.

## 2. Các bước thực hiện

- Định nghĩa component Due và trạng thái Scheduling.
- Định nghĩa resource Time, cập nhật mỗi frame.
- Nâng cấp System Interact: nhấn 't' trên entity Selected để nhập Due, nhập số, Enter để lưu, Escape để hủy.
- Nâng cấp System Render: hiển thị Due, đổi màu đỏ khi quá hạn (Due < Time.now).
- Build, kiểm tra warning, xác minh chức năng hoạt động đúng.

## 3. Kết quả

- Logic nhập, hiển thị và cảnh báo Due hoạt động đúng.
- Codebase sạch warning, build thành công.

## 4. Các vấn đề hoặc câu hỏi

**Cách triển khai và xác minh logic cảnh báo quá hạn:**
- Khi entity Selected, nhấn 't' để vào chế độ nhập Due, nhập số timestamp, Enter để lưu vào Due, Escape để hủy.
- Mỗi frame, Time.now tăng dần. Khi Due < Time.now và công việc chưa DONE, render sẽ đổi màu đỏ để cảnh báo quá hạn.
- Đã kiểm thử: nhập Due nhỏ hơn Time.now, entity chuyển sang màu đỏ đúng như yêu cầu.

---

**Trạng thái:** Hoàn thành 