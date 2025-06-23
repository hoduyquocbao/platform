Chính xác. Hệ thống đã ghi nhận báo cáo hoàn thành nhiệm vụ V21. Chu kỳ 3 đã chính thức khép lại.

-----

### **PHÂN TÍCH, TƯ DUY, SUY LUẬN, KIỂM TRA VÀ ĐÁNH GIÁ**

**1. Kiểm tra và Đánh giá (Check & Assess):**

  * **Trạng thái:** Hoàn thành xuất sắc Nhiệm vụ `T-250713-21` và toàn bộ Chu kỳ 3.
  * **Kết quả:** Nền tảng của chúng ta hiện đang ở trạng thái hoàn hảo nhất về mặt kiến trúc và chất lượng mã nguồn. Chúng ta đã xây dựng được một ứng dụng quản lý công việc cực kỳ mạnh mẽ với mô hình dữ liệu phức hợp (phân cấp), siêu dữ liệu (ngày hết hạn), và khả năng khám phá (lọc/tìm kiếm). Toàn bộ hệ thống được xây dựng trên một nền tảng `trait-based` vững chắc, đã được tái cấu trúc và dọn dẹp cẩn thận, không còn nợ kỹ thuật.
  * **Phân tích (Lỗ hổng Trải nghiệm Lớn nhất):** Mặc dù cỗ máy bên trong là một kiệt tác kỹ thuật, "bộ mặt" của nó vẫn còn là một giao diện dòng lệnh (`println!`). Toàn bộ sức mạnh và sự phức tạp của hệ thống vẫn chưa được thể hiện một cách trực quan cho người dùng cuối. Đây là rào cản lớn nhất ngăn cản nền tảng trở thành một sản phẩm thực thụ.

**2. Suy luận và Xác định Hướng đi Tiếp theo (Reasoning & Next Steps):**

  * **Vấn đề cần giải quyết:** Làm thế nào để chuyển đổi toàn bộ trải nghiệm người dùng từ văn bản tĩnh sang một giao diện đồ họa (GUI) thực thụ, nơi các công việc được "vẽ" ra như những thực thể hình ảnh và tương tác mang lại cảm giác tức thì?
  * **Mục tiêu của Chu kỳ 4:** Chu kỳ phát triển tiếp theo sẽ là một bước nhảy vọt, tập trung vào **"Hiện thực hóa Trực quan: Tích hợp Backend Đồ họa và Giao diện Người dùng Đồ họa" (Visual Realization: Graphics Backend Integration and Graphical User Interface)**.
  * **Nhiệm vụ đầu tiên của Chu kỳ 4:** Bước đi đầu tiên và cơ bản nhất là thay thế hoàn toàn `System` `Render` dựa trên `println!` bằng một hệ thống có khả năng mở một cửa sổ ứng dụng và vẽ các hình khối cơ bản.
      * **Tích hợp Thư viện:** Cần chọn và tích hợp một thư viện đồ họa 2D tối giản vào dự án.
      * **Tạo Cửa sổ:** Vòng lặp chính của ứng dụng giờ đây sẽ được điều khiển bởi vòng lặp sự kiện của cửa sổ.
      * **Nâng cấp Hiển thị:** `System` `Render` phải được tái cấu trúc để không in ra text nữa, mà thay vào đó là thực hiện các lệnh vẽ hình chữ nhật màu sắc lên một bộ đệm khung hình (framebuffer).
      * **Nâng cấp Input:** `System` `Interact` phải lấy đầu vào từ hệ thống sự kiện của cửa sổ đồ họa thay vì giả lập.

**3. Hình thành Nhiệm vụ Mới (Formulating the New Task):**

  * **ID Nhiệm vụ:** `T-250714-22`
  * **Tên Nhiệm vụ:** "Chu kỳ 4: Tích hợp Backend Đồ họa - Cửa sổ và Hình khối Cơ bản" (Cycle 4: Graphics Backend Integration - Window and Basic Shapes).
  * **Trọng tâm:** Thay thế hoàn toàn giao diện dòng lệnh bằng một cửa sổ đồ họa, đặt nền móng cho việc xây dựng một GUI thực thụ.

-----

### **TẠO FILE TASK TIẾP THEO**

Dưới đây là nội dung của file `TASK_20250714_ENHANCE_PLATFORM_V22.md` sẽ được tạo ra và giao cho Coder.

````markdown
# YÊU CẦU TRIỂN KHAI: CHU KỲ 4 - TÍCH HỢP BACKEND ĐỒ HỌA V4.0

**TỪ:** Guardian (Kiến trúc sư Hệ thống)
**GỬI:** Coder (Lập trình viên Triển khai)
**NGÀY:** 14-07-2025
**ID NHIỆM VỤ:** T-250714-22

---

### 1. MỤC TIÊU TỔNG QUAN

Chúc mừng bạn đã hoàn thành xuất sắc Chu kỳ 3! Nền tảng của chúng ta hiện đang ở trạng thái mạnh mẽ và trong sạch nhất có thể.

Nhiệm vụ này sẽ chính thức khởi động Chu kỳ 4, một chu kỳ mang tính cách mạng: **chuyển đổi ứng dụng từ giao diện dòng lệnh sang giao diện đồ họa (GUI) thực thụ.** Mục tiêu đầu tiên của chúng ta là tích hợp một backend đồ họa, mở một cửa sổ ứng dụng, và vẽ các công việc của chúng ta dưới dạng các hình khối màu sắc.

---

### 2. BỐI CẢNH

* Chúng ta đang xây dựng trên nền tảng V3.6 đã được hoàn thiện và không còn cảnh báo.
* Nhiệm vụ này sẽ thay đổi căn bản cách ứng dụng của chúng ta trình bày thông tin và nhận đầu vào, đòi hỏi tái cấu trúc các `System` `Render` và `Interact`.

---

### 3. YÊU CẦU NHIỆM VỤ CHI TIẾT

#### 3.1. Cập nhật `pkb/todo.csv`

Thêm các công việc mới sau vào cuối file `pkb/todo.csv`:

```csv
T106,"(Cycle 4) Lựa chọn và tích hợp thư viện đồ họa 2D tối giản",Coder,Open,Critical
T107,"(Cycle 4) Cập nhật vòng lặp chính để quản lý cửa sổ đồ họa",Coder,Open,Critical
T108,"(Cycle 4) Nâng cấp System `Interact` để nhận sự kiện từ cửa sổ",Coder,Open,High
T109,"(Cycle 4) Tái cấu trúc System `Render` để vẽ hình khối cơ bản",Coder,Open,High
T110,"(Cycle 4) Viết báo cáo triển khai V22 và cập nhật PKB",Coder,Open,High
````

#### 3.2. Lựa chọn và Tích hợp Thư viện Đồ họa

  * **Nhiệm vụ:**
    1.  Chọn một thư viện đồ họa 2D tối giản của Rust. Các gợi ý tốt là **`minifb`** (cực kỳ đơn giản cho việc vẽ framebuffer) hoặc **`macroquad`** (cung cấp API vẽ 2D ở mức cao hơn một chút).
    2.  Thêm thư viện đã chọn làm dependency trong `Cargo.toml`.

#### 3.3. Cập nhật Vòng lặp Chính và Cửa sổ

  * **File:** `src/main.rs`
  * **Nhiệm vụ:**
    1.  Thay đổi hàm `main` để khởi tạo một cửa sổ ứng dụng bằng thư viện đã chọn.
    2.  Vòng lặp `loop` vô hạn trước đây sẽ được thay thế bằng vòng lặp của cửa sổ (ví dụ: `while window.is_open() && !window.is_key_down(Key::Escape)`).
    3.  `App::run()` sẽ được gọi bên trong vòng lặp này.

#### 3.4. Nâng cấp `System Interact`

  * **File:** `src/systems/interaction.rs`
  * **Nhiệm vụ:**
    1.  `Resource` `Mouse` bây giờ sẽ không được giả lập nữa.
    2.  `System Interact` phải lấy vị trí con trỏ và trạng thái nút chuột trực tiếp từ API của thư viện đồ họa đã chọn (ví dụ: `window.get_mouse_pos()`, `window.get_mouse_down()`).
    3.  Tương tự với `Resource` `Keyboard`.

#### 3.5. Tái cấu trúc `System Render`

  * **File:** `src/systems/render.rs`
  * **Nhiệm vụ:**
    1.  Đây là phần tái cấu trúc lớn nhất. Toàn bộ logic `println!` sẽ bị xóa bỏ.
    2.  `System Render` bây giờ sẽ nhận một tham chiếu có thể thay đổi đến bộ đệm khung hình (framebuffer) của cửa sổ như một `Resource`.
    3.  Nó sẽ truy vấn tất cả các `Entity` có `Visible`, `Bounds`, và `Style`.
    4.  Với mỗi `Entity`, nó sẽ sử dụng API của thư viện đồ họa để **vẽ một hình chữ nhật** tại tọa độ `Bounds.x`, `Bounds.y` với kích thước `Bounds.width`, `Bounds.height`.
    5.  Màu của hình chữ nhật sẽ được xác định bởi `Style.color` và trạng thái `Hover`/`Selected`.

-----

### 4\. ĐỊNH DẠNG PHẢN HỒI (BẮT BUỘC)

1.  Tạo file mới: `IMPLEMENTATION_REPORT_V22.md`.
2.  Sử dụng mẫu báo cáo đã cung cấp, cập nhật ID Nhiệm vụ thành `T-250714-22`.
3.  Trong mục **"Các Vấn đề hoặc Câu hỏi"**, hãy ghi lại thư viện đồ họa bạn đã chọn và mô tả ngắn gọn cách bạn đã tái cấu trúc `System Render` để vẽ các hình chữ nhật thay vì in ra văn bản.

-----

### 5\. QUY TRÌNH NỘP BÀI

Thực hiện các lệnh Git sau:

```bash
git add .
git commit -m "feat(cycle-4): integrate graphics backend and render basic shapes" -m "Fulfills task T-250714-22. Kicks off Cycle 4 by replacing the console output with a graphical window using a 2D library. The Render system now draws tasks as colored rectangles."
git push origin main
```

```
```