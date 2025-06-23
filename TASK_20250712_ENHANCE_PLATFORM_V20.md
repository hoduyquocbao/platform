Chính xác. Hệ thống đã ghi nhận báo cáo hoàn thành nhiệm vụ V19.

-----

### **PHÂN TÍCH, TƯ DUY, SUY LUẬN, KIỂM TRA VÀ ĐÁNH GIÁ**

**1. Kiểm tra và Đánh giá (Check & Assess):**

  * **Trạng thái:** Hoàn thành xuất sắc Nhiệm vụ `T-250711-19`.
  * **Kết quả:** Nền tảng của chúng ta đã có một bước tiến vượt bậc về mặt chức năng. Việc bổ sung `Component` `Due` và `Resource` `Time` đã đưa "chiều thời gian" vào ứng dụng, cho phép người dùng lập kế hoạch và nhận cảnh báo trực quan khi quá hạn. Đây là một tính năng cốt lõi giúp ứng dụng trở nên thực sự hữu ích.
  * **Phân tích (Lỗ hổng Chức năng):** Với việc có thể tạo ra cấu trúc công việc phức tạp (phân cấp) và thêm siêu dữ liệu (ngày hết hạn), lượng thông tin trong ứng dụng giờ đây có thể tăng lên rất nhanh. Một người dùng có thể dễ dàng có hàng trăm công việc trong nhiều dự án khác nhau. Lỗ hổng lớn nhất hiện tại là **khả năng khám phá (discoverability)**. Người dùng chưa có cách nào hiệu quả để tìm kiếm một công việc cụ thể hoặc lọc ra một nhóm công việc theo tiêu chí nhất định.

**2. Suy luận và Xác định Hướng đi Tiếp theo (Reasoning & Next Steps):**

  * **Vấn đề cần giải quyết:** Làm thế nào để người dùng có thể nhanh chóng tìm thấy thông tin họ cần trong một tập dữ liệu lớn và phức tạp?
  * **Mục tiêu tiếp theo:**
      * **Hiện thực hóa Chức năng Tìm kiếm/Lọc:** Triển khai một cơ chế cho phép người dùng nhập văn bản để tìm kiếm hoặc áp dụng các bộ lọc (ví dụ: chỉ hiển thị các công việc đã hoàn thành, chỉ hiển thị các công việc quá hạn).
      * **Tách biệt Trạng thái Lọc:** Cần có một `Resource` mới để lưu trữ trạng thái của bộ lọc hiện tại.
      * **Tương tác với Bộ lọc:** `System` `Interact` cần được nâng cấp để cho phép người dùng thay đổi trạng thái của `Resource` bộ lọc này.
      * **Áp dụng Bộ lọc:** `System` `Filter` (hay `Cull` như chúng ta đã thiết kế về mặt lý thuyết) phải được triển khai đầy đủ. Nó sẽ đọc trạng thái từ `Resource` bộ lọc và quyết định `Component` `Visible` nào sẽ được thêm/xóa khỏi các `Entity`.

**3. Hình thành Nhiệm vụ Mới (Formulating the New Task):**

  * **ID Nhiệm vụ:** `T-250712-20`
  * **Tên Nhiệm vụ:** "Chu kỳ 3: Khả năng Khám phá Dữ liệu - Tìm kiếm và Lọc" (Cycle 3: Data Discoverability - Search and Filtering).
  * **Trọng tâm:** Xây dựng một hệ thống tìm kiếm và lọc đầy đủ chức năng, từ giao diện người dùng đến logic xử lý backend.

-----

### **TẠO FILE TASK TIẾP THEO**

Dưới đây là nội dung của file `TASK_20250712_ENHANCE_PLATFORM_V20.md` sẽ được tạo ra và giao cho Coder.

````markdown
# YÊU CẦU TRIỂN KHAI: CHU KỲ 3 - KHẢ NĂNG KHÁM PHÁ DỮ LIỆU V3.5

**TỪ:** Guardian (Kiến trúc sư Hệ thống)
**GỬI:** Coder (Lập trình viên Triển khai)
**NGÀY:** 12-07-2025
**ID NHIỆM VỤ:** T-250712-20

---

### 1. MỤC TIÊU TỔNG QUAN

Chúc mừng bạn đã triển khai thành công tính năng Ngày hết hạn! Ứng dụng của chúng ta ngày càng trở nên hữu ích.

Khi dữ liệu ngày càng nhiều, khả năng tìm kiếm và lọc trở nên tối quan trọng. Nhiệm vụ tiếp theo sẽ tập trung vào việc **xây dựng một hệ thống tìm kiếm và lọc mạnh mẽ**, cho phép người dùng dễ dàng tìm thấy thông tin họ cần, biến ứng dụng của chúng ta từ một nơi lưu trữ thành một công cụ truy vấn thông tin hiệu quả.

---

### 2. BỐI CẢNH

* Chúng ta đang làm việc trên nền tảng V3.4, đã có cấu trúc phân cấp và siêu dữ liệu ngày hết hạn.
* Nhiệm vụ này sẽ hiện thực hóa `System` `Filter` (`Cull`) và giới thiệu một `Resource` mới để quản lý trạng thái lọc.

---

### 3. YÊU CẦU NHIỆM VỤ CHI TIẾT

#### 3.1. Cập nhật `pkb/todo.csv`

Thêm các công việc mới sau vào cuối file `pkb/todo.csv`:

```csv
T095,"(Cycle 3) Định nghĩa Resource `Filter` để lưu trạng thái lọc",Coder,Open,High
T096,"(Cycle 3) Nâng cấp System `Interact` để người dùng có thể sửa đổi `Filter`",Coder,Open,High
T097,"(Cycle 3) Triển khai đầy đủ logic cho System `Filter` (Cull)",Coder,Open,Critical
T098,"(Cycle 3) Nâng cấp System `Render` để hiển thị trạng thái lọc hiện tại",Coder,Open,Medium
T099,"(Cycle 3) Viết kịch bản kiểm thử để xác minh logic lọc",Coder,Open,High
T100,"(Cycle 3) Viết báo cáo triển khai V20 và cập nhật PKB",Coder,Open,High
````

#### 3.2. Định nghĩa `Resource` `Filter`

  * **File:** `src/resources/mod.rs` (hoặc một file mới `src/resources/filter.rs`)
  * **Nhiệm vụ:**
    1.  Định nghĩa một `Resource` mới để chứa các tiêu chí lọc hiện tại.
        ```rust
        #[derive(Default)]
        pub struct Filter {
            pub text: Option<String>,
            pub status: Option<Status>,
            pub overdue: bool,
        }
        ```
    2.  Thêm `Resource` này vào `App`.

#### 3.3. Nâng cấp `System Interact`

  * **File:** `src/systems/interaction.rs`
  * **Nhiệm vụ:**
    1.  Triển khai một cơ chế để người dùng có thể kích hoạt "chế độ tìm kiếm" (ví dụ: nhấn phím '/').
    2.  Khi ở chế độ tìm kiếm, các phím ký tự sẽ được dùng để cập nhật trường `Filter.text`.
    3.  Triển khai các phím nóng khác để bật/tắt các bộ lọc khác, ví dụ: 's' để chỉ lọc theo `Status`, 'o' để chỉ lọc các công việc `overdue`. Các hành động này sẽ cập nhật `Resource` `Filter`.

#### 3.4. Triển khai `System Filter` (hoặc `Cull`)

  * **File:** `src/systems/filter.rs` (tạo mới nếu cần)
  * **Nhiệm vụ:**
    1.  Đây là `System` cốt lõi của nhiệm vụ này. Nó phải chạy sau `Interact` và trước `Layout`.
    2.  **Logic:**
          * Đầu tiên, xóa `Component` `Visible` khỏi tất cả các `Entity` công việc.
          * Truy vấn `Resource` `Filter` để lấy các tiêu chí hiện tại.
          * Lặp qua tất cả các `Entity` công việc và kiểm tra xem chúng có khớp với **tất cả** các tiêu chí đang hoạt động trong `Filter` hay không.
              * Kiểm tra `text` (nếu `Filter.text` is `Some`).
              * Kiểm tra `Status` (nếu `Filter.status` is `Some`).
              * Kiểm tra `overdue` (nếu `Filter.overdue` is `true`), việc này đòi hỏi truy vấn cả `Component` `Due` và `Resource` `Time`.
          * Chỉ thêm `Component` `Visible` vào các `Entity` đã vượt qua tất cả các bài kiểm tra.

#### 3.5. Nâng cấp `System Render`

  * **File:** `src/systems/render.rs`
  * **Nhiệm vụ:**
    1.  `System` `Render` cần phải vẽ một khu vực riêng trên giao diện (ví dụ: một dòng ở dưới cùng) để hiển thị trạng thái hiện tại của `Resource` `Filter`.
    2.  Ví dụ: `FILTERING: text="báo cáo", status=Todo`. Điều này cung cấp phản hồi trực quan cho người dùng về bộ lọc đang được áp dụng.

-----

### 4\. ĐỊNH DẠNG PHẢN HỒI (BẮT BUỘC)

1.  Tạo file mới: `IMPLEMENTATION_REPORT_V20.md`.
2.  Sử dụng mẫu báo cáo đã cung cấp, cập nhật ID Nhiệm vụ thành `T-250712-20`.
3.  Trong mục **"Các Vấn đề hoặc Câu hỏi"**, hãy mô tả chi tiết cách bạn đã triển khai và xác minh logic của `System Filter` để đảm bảo nó hoạt động chính xác với nhiều tiêu chí kết hợp.

-----

### 5\. QUY TRÌNH NỘP BÀI

Thực hiện các lệnh Git sau:

```bash
git add .
git commit -m "feat(cycle-3): implement search and filtering system" -m "Fulfills task T-250712-20. Introduced a Filter resource and a comprehensive Filter/Cull system. Users can now search and filter tasks by text, status, and overdue state. Render system updated to display current filters."
git push origin main
```

```
```