Chính xác. Hệ thống đã ghi nhận báo cáo hoàn thành nhiệm vụ V18.

-----

### **PHÂN TÍCH, TƯ DUY, SUY LUẬN, KIỂM TRA VÀ ĐÁNH GIÁ**

**1. Kiểm tra và Đánh giá (Check & Assess):**

  * **Trạng thái:** Hoàn thành xuất sắc Nhiệm vụ `T-250710-18`.
  * **Kết quả:** Nền tảng của chúng ta giờ đây không chỉ có các tính năng phong phú mà còn cực kỳ **bền vững (robust)**. Logic xóa theo tầng đã đảm bảo toàn vẹn dữ liệu cho mô hình phân cấp, loại bỏ một trong những loại lỗi nghiêm trọng và khó gỡ nhất trong các ứng dụng có cấu trúc cây.
  * **Phân tích (Lỗ hổng Chức năng):** Chúng ta đã xây dựng một cấu trúc phân cấp mạnh mẽ (`Parent`/`Children`), các tương tác linh hoạt (`Expand`/`Collapse`), và các hành động an toàn (`Cascading Deletes`). Tuy nhiên, bản thân các "công việc" (tasks) vẫn còn rất thô sơ. Chúng chỉ là một đoạn văn bản với trạng thái hoàn thành. Để trở thành một công cụ quản lý thực thụ, chúng cần có thêm **siêu dữ liệu (metadata)** và nhận thức được bối cảnh quan trọng nhất: **thời gian**.

**2. Suy luận và Xác định Hướng đi Tiếp theo (Reasoning & Next Steps):**

  * **Vấn đề cần giải quyết:** Làm thế nào để người dùng có thể lập kế hoạch và theo dõi công việc theo thời gian? Làm thế nào để hệ thống có thể cung cấp các cảnh báo và phản hồi trực quan về thời hạn?
  * **Mục tiêu tiếp theo:**
      * **Hiện thực hóa Khái niệm Thời gian:** Giới thiệu khái niệm "Ngày hết hạn" (Due Date) vào mô hình dữ liệu.
      * **Tương tác với Thời gian:** Cung cấp cho người dùng khả năng thiết lập hoặc thay đổi ngày hết hạn cho một công việc.
      * **Phản hồi Trực quan theo Thời gian:** Giao diện phải hiển thị ngày hết hạn và thay đổi hiển thị (ví dụ: đổi màu) khi một công việc bị quá hạn.
  * **Hành động cụ thể:**
    1.  Định nghĩa một `Component` mới `Due` để lưu trữ thông tin thời hạn.
    2.  Định nghĩa một `Resource` `Time` để hệ thống nhận biết được thời gian hiện tại.
    3.  Nâng cấp các `System` `Interact` và `Render` để cho phép người dùng đặt và xem thông tin thời hạn.

**3. Hình thành Nhiệm vụ Mới (Formulating the New Task):**

  * **ID Nhiệm vụ:** `T-250711-19`
  * **Tên Nhiệm vụ:** "Chu kỳ 3: Mở rộng Siêu dữ liệu - Ngày hết hạn" (Cycle 3: Expanding Metadata - Due Dates).
  * **Trọng tâm:** Bổ sung một chiều dữ liệu hoàn toàn mới vào ứng dụng, làm cho nó trở nên hữu ích hơn rất nhiều cho việc lập kế hoạch và quản lý.

-----

### **TẠO FILE TASK TIẾP THEO**

Dưới đây là nội dung của file `TASK_20250711_ENHANCE_PLATFORM_V19.md` sẽ được tạo ra và giao cho Coder.

````markdown
# YÊU CẦU TRIỂN KHAI: CHU KỲ 3 - MỞ RỘNG SIÊU DỮ LIỆU NGÀY HẾT HẠN V3.4

**TỪ:** Guardian (Kiến trúc sư Hệ thống)
**GỬI:** Coder (Lập trình viên Triển khai)
**NGÀY:** 11-07-2025
**ID NHIỆM VỤ:** T-250711-19

---

### 1. MỤC TIÊU TỔNG QUAN

Chúc mừng bạn đã hoàn thành xuất sắc việc đảm bảo toàn vẹn dữ liệu cho hệ thống. Nền tảng của chúng ta giờ đây đã rất ổn định.

Nhiệm vụ tiếp theo là bổ sung một trong những loại siêu dữ liệu quan trọng nhất cho việc quản lý công việc: **Ngày hết hạn (Due Dates)**. Chúng ta sẽ cho phép người dùng đặt thời hạn cho công việc và hệ thống sẽ cung cấp phản hồi trực quan khi công việc đó bị quá hạn.

---

### 2. BỐI CẢNH

* Chúng ta đang làm việc trên nền tảng V3.3, đã có logic xóa theo tầng an toàn.
* Nhiệm vụ này sẽ giới thiệu một `Component` và một `Resource` mới, đồng thời nâng cấp các `System` tương tác và hiển thị.

---

### 3. YÊU CẦU NHIỆM VỤ CHI TIẾT

#### 3.1. Cập nhật `pkb/todo.csv`

Thêm các công việc mới sau vào cuối file `pkb/todo.csv`:

```csv
T090,"(Cycle 3) Định nghĩa Component `Due` và Resource `Time`",Coder,Open,High
T091,"(Cycle 3) Nâng cấp System `Interact` để hỗ trợ đặt ngày hết hạn",Coder,Open,High
T092,"(Cycle 3) Nâng cấp System `Render` để hiển thị và cảnh báo ngày hết hạn",Coder,Open,Medium
T093,"(Cycle 3) Viết kịch bản kiểm thử để xác minh logic quá hạn",Coder,Open,High
T094,"(Cycle 3) Viết báo cáo triển khai V19 và cập nhật PKB",Coder,Open,High
````

#### 3.2. Định nghĩa `Component` và `Resource` Thời gian

  * **File:** `src/components/core.rs` và `src/resources/mod.rs`
  * **Nhiệm vụ:**
    1.  Định nghĩa một `Component` mới để lưu trữ ngày hết hạn dưới dạng một con dấu thời gian (timestamp): `pub struct Due(pub u64);`.
    2.  Định nghĩa một `Resource` mới để cung cấp thời gian hiện tại cho toàn hệ thống: `pub struct Time { pub now: u64 };`. Trong `main.rs`, hãy cập nhật `Resource` này mỗi frame.

#### 3.3. Nâng cấp `System Interact`

  * **File:** `src/systems/interaction.rs`
  * **Nhiệm vụ:**
    1.  Bổ sung một cơ chế để người dùng có thể đặt hoặc thay đổi `Due` date.
    2.  **Gợi ý triển khai:** Khi một `Entity` đang `Selected` và người dùng nhấn một phím (ví dụ: 't'), thêm một `Component` trạng thái mới là `Scheduling`.
    3.  Khi một `Entity` có `Scheduling`, người dùng có thể dùng các phím số để nhập một timestamp mới.
    4.  Khi nhấn 'Enter', `System` sẽ cập nhật hoặc thêm `Component` `Due` với giá trị vừa nhập, xóa `Component` `Scheduling`, và thêm `Component` `Dirty` để lưu lại thay đổi.

#### 3.4. Nâng cấp `System Render`

  * **File:** `src/systems/render.rs`
  * **Nhiệm vụ:**
    1.  Khi vẽ một `Entity`, `System` `Render` cần truy vấn xem nó có `Component` `Due` hay không. Nếu có, hãy hiển thị nó bên cạnh công việc.
    2.  `System` `Render` cũng phải truy vấn `Resource` `Time`.
    3.  Đối với mỗi `Entity` có `Due`, hãy so sánh `Due.0` với `Time.now`.
    4.  Nếu `Due.0 < Time.now` (và công việc chưa hoàn thành), hãy hiển thị `Entity` đó với một màu khác (ví dụ: màu đỏ) để cảnh báo nó đã bị quá hạn.

#### 3.5. Xác minh Chức năng

  * **Nhiệm vụ:**
    1.  Thực hiện hành động đặt ngày hết hạn cho một công việc.
    2.  Xác minh rằng ngày hết hạn được hiển thị chính xác.
    3.  Trong vòng lặp `main`, hãy cho giá trị của `Time.now` tăng dần. Xác minh rằng khi `Time.now` vượt qua `Due.0`, công việc đó sẽ đổi màu.

-----

### 4\. ĐỊNH DẠNG PHẢN HỒI (BẮT BUỘC)

1.  Tạo file mới: `IMPLEMENTATION_REPORT_V19.md`.
2.  Sử dụng mẫu báo cáo đã cung cấp, cập nhật ID Nhiệm vụ thành `T-250711-19`.
3.  Trong mục **"Các Vấn đề hoặc Câu hỏi"**, hãy mô tả chi tiết cách bạn đã triển khai và xác minh logic cảnh báo công việc quá hạn.

-----

### 5\. QUY TRÌNH NỘP BÀI

Thực hiện các lệnh Git sau:

```bash
git add .
git commit -m "feat(cycle-3): implement due date metadata and interaction" -m "Fulfills task T-250711-19. Added Due component and Time resource. Interact and Render systems are enhanced to allow setting and displaying due dates, including visual warnings for overdue tasks."
git push origin main
```

```
```