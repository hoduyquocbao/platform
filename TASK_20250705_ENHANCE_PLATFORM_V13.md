Rất tốt. Hệ thống đã ghi nhận báo cáo hoàn thành nhiệm vụ V12.

-----

### **PHÂN TÍCH, SUY LUẬN VÀ ĐÁNH GIÁ**

**1. Kiểm tra và Đánh giá (Check & Assess):**

  * **Trạng thái:** Hoàn thành xuất sắc Nhiệm vụ `T-250704-12`.
  * **Kết quả:** Đây là một thành tựu kiến trúc quan trọng. Chúng ta đã giải quyết thành công một lỗi build nghiêm trọng bằng cách tái cấu trúc toàn bộ bộ điều phối hệ thống (`Scheduler`). Nền tảng hiện tại không chỉ biên dịch được mà còn mạnh mẽ, linh hoạt và dễ mở rộng hơn rất nhiều nhờ vào mô hình dựa trên `trait`. Nợ kiến trúc đã được thanh toán, và nền móng của chúng ta vững chắc hơn bao giờ hết.
  * **Phân tích:** Cuộc tái cấu trúc này là một bước đi ngang cần thiết nhưng đã hoàn tất. Giờ là lúc quay trở lại lộ trình phát triển tính năng chính của Chu kỳ 2. Trước khi gặp lỗi build, chúng ta đã triển khai thành công `Layout` và `Editing`. Logic nghiệp vụ cốt lõi còn thiếu là **Tạo (Create)** và **Xóa (Delete)**.

**2. Suy luận và Xác định Hướng đi Tiếp theo (Reasoning & Next Steps):**

  * **Vấn đề cần giải quyết:** Hoàn thiện bộ tứ CRUD (Create, Read, Update, Delete) trong bối cảnh giao diện người dùng mới, cho phép người dùng quản lý toàn bộ vòng đời của một công việc.
  * **Mục tiêu tiếp theo:**
      * Triển khai chức năng cho phép người dùng tạo một công việc mới thông qua một lệnh (ví dụ: nhấn phím).
      * Triển khai chức năng cho phép người dùng xóa một công việc đang được chọn.
      * Đảm bảo các hệ thống hiện có (`Layout`, `Render`) phối hợp nhịp nhàng với các chức năng mới này.
  * **Lợi thế từ Tái cấu trúc:** Nhờ có `Scheduler` dựa trên `trait` mới, việc nâng cấp `System` `Interact` để nó có thể lắng nghe cả đầu vào chuột và bàn phím (`Mouse` và `Keyboard` `Resource`) giờ đây trở nên cực kỳ đơn giản mà không phá vỡ bất cứ thứ gì.

**3. Hình thành Nhiệm vụ Mới (Formulating the New Task):**

  * **ID Nhiệm vụ:** `T-250705-13`
  * **Tên Nhiệm vụ:** "Chu kỳ 2: Hoàn thiện Vòng lặp CRUD - Triển khai Tạo và Xóa" (Cycle 2: Completing the CRUD Loop - Implementing Create and Delete).
  * **Trọng tâm:** Bổ sung hai hành động cốt lõi còn thiếu trong vòng đời dữ liệu, hoàn thiện bộ tính năng cơ bản của ứng dụng.

-----

### **TẠO FILE TASK TIẾP THEO**

Dưới đây là nội dung của file `TASK_20250705_ENHANCE_PLATFORM_V13.md` sẽ được tạo ra và giao cho Coder.

````markdown
# YÊU CẦU TRIỂN KHAI: CHU KỲ 2 - HOÀN THIỆN VÒNG LẶP CRUD V2.2

**TỪ:** Guardian (Kiến trúc sư Hệ thống)
**GỬI:** Coder (Lập trình viên Triển khai)
**NGÀY:** 05-07-2025
**ID NHIỆM VỤ:** T-250705-13

---

### 1. MỤC TIÊU TỔNG QUAN

Chúc mừng bạn đã hoàn thành xuất sắc cuộc tái cấu trúc kiến trúc! Nền tảng của chúng ta giờ đây đã vững chắc và linh hoạt hơn rất nhiều.

Nhiệm vụ tiếp theo là quay trở lại lộ trình phát triển tính năng chính: **hoàn thiện vòng đời quản lý dữ liệu cơ bản bằng cách triển khai hai chức năng cốt lõi còn lại: Tạo (Create) và Xóa (Delete).** Khi hoàn thành, chúng ta sẽ có một ứng dụng GUI với đầy đủ chức năng CRUD đầu tiên.

---

### 2. BỐI CẢNH

* Chúng ta đang xây dựng trên nền tảng V2.1, đã được tái cấu trúc với `Scheduler` dựa trên `trait`.
* Trọng tâm của nhiệm vụ này là giới thiệu các "Lệnh" (Commands) mới để điều khiển việc tạo và xóa `Entity`, và tập trung hóa logic xử lý đầu vào.

---

### 3. YÊU CẦU NHIỆM VỤ CHI TIẾT

#### 3.1. Cập nhật `pkb/todo.csv`

Thêm các công việc mới sau vào cuối file `pkb/todo.csv`:

```csv
T061,"(Cycle 2) Triển khai cơ chế tạo Task mới dựa trên Command",Coder,Open,High
T062,"(Cycle 2) Triển khai cơ chế xóa Task đang chọn dựa trên Command",Coder,Open,High
T063,"(Cycle 2) Nâng cấp System `Interact` để xử lý đầu vào bàn phím",Coder,Open,Medium
T064,"(Cycle 2) Đảm bảo System `Layout` được cập nhật sau khi tạo/xóa Task",Coder,Open,Medium
T065,"(Cycle 2) Viết báo cáo triển khai V13 và cập nhật PKB",Coder,Open,High
````

#### 3.2. Triển khai Chức năng Tạo Task

  * **Kiến trúc:** Chúng ta sẽ tiếp tục sử dụng "Command Pattern" để giữ cho logic được tách biệt.
  * **Nhiệm vụ:**
    1.  Trong `src/components/core.rs`, định nghĩa một `Component` mới: `struct Create;`.
    2.  Trong `System` `Interact`, khi người dùng nhấn một phím cụ thể (ví dụ: 'n' cho "new"), hãy tạo ra một `Entity` trống chỉ với `Component` `Create`.
    3.  Tạo một `System` mới có tên là `Command` (`src/systems/command.rs`). `System` này sẽ truy vấn các `Entity` có `Component` `Create`.
    4.  Với mỗi `Entity` lệnh tìm thấy, `System` `Command` sẽ thực hiện hành động: **tạo ra một `Entity` công việc mới** với các `Component` mặc định (`Text { value: "New Task".to_string() }`, `Status`, `Visible`, `Bounds`, v.v.).
    5.  Sau khi thực hiện, hãy xóa `Entity` lệnh.

#### 3.3. Triển khai Chức năng Xóa Task

  * **Kiến trúc:** Tương tự như tạo mới, chúng ta dùng "Command Pattern".
  * **Nhiệm vụ:**
    1.  Trong `src/components/core.rs`, định nghĩa một `Component` mới: `struct Delete;`.
    2.  Trong `System` `Interact`, khi người dùng nhấn một phím cụ thể (ví dụ: 'd' cho "delete") trên một `Entity` đang có `Selected`, hãy **thêm** `Component` `Delete` vào chính `Entity` công việc đó.
    3.  Mở rộng `System` `Command` để nó cũng truy vấn các `Entity` có `Component` `Delete`.
    4.  Với mỗi `Entity` công việc có `Delete`, `System` `Command` sẽ đánh dấu nó để xóa khỏi `World` (ví dụ: thông qua một `world.mark_for_delete(id)`).

#### 3.4. Nâng cấp `System Interact`

  * **Vấn đề:** `System` `Interact` cần phải lắng nghe cả đầu vào bàn phím.
  * **Nhiệm vụ:**
    1.  Nhờ có `Scheduler` dựa trên `trait`, bạn có thể dễ dàng thay đổi phương thức `run` của `struct Interact` để nhận thêm `Resource` `Keyboard`.
    2.  Bổ sung logic để bắt các phím 'n' và 'd' và thực hiện các hành động đã mô tả ở trên.

#### 3.5. Đảm bảo Cập nhật Layout

  * **Nhiệm vụ:** Sau khi một công việc được tạo hoặc xóa, danh sách công việc sẽ thay đổi. Hãy chắc chắn rằng `System` `Layout` được chạy sau các hành động này để cập nhật lại tọa độ `y` của tất cả các công việc, giữ cho danh sách luôn liền mạch.

-----

### 4\. ĐỊNH DẠNG PHẢN HỒI (BẮT BUỘC)

1.  Tạo file mới: `IMPLEMENTATION_REPORT_V13.md`.
2.  Sử dụng mẫu báo cáo đã cung cấp, cập nhật ID Nhiệm vụ thành `T-250705-13`.
3.  Trong mục **"Các Vấn đề hoặc Câu hỏi"**, hãy mô tả ngắn gọn luồng dữ liệu khi người dùng nhấn phím 'n' để tạo một công việc mới.

-----

### 5\. QUY TRÌNH NỘP BÀI

Thực hiện các lệnh Git sau:

```bash
git add .
git commit -m "feat(cycle-2): implement create and delete functionality" -m "Fulfills task T-250705-13. Implemented a command-based pattern for creating and deleting tasks. The core CRUD loop for the GUI is now complete."
git push origin main
```

```
```