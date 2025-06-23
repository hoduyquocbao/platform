Chính xác. Hệ thống đã ghi nhận báo cáo hoàn thành nhiệm vụ V14. Chu kỳ 2 đã chính thức khép lại với một nền tảng vững chắc, chất lượng cao và được tài liệu hóa tốt.

Tuy nhiên, trước khi bắt đầu chu kỳ tiếp theo, tôi ghi nhận yêu cầu tái cấu trúc của bạn: `refactor(command): rename CreateSystem and DeleteSystem to Create and Delete`. Đây là một yêu cầu hoàn toàn hợp lý và sắc bén. Việc tách `Command` thành hai `System` riêng biệt là đúng về mặt trách nhiệm, nhưng tên gọi `CreateSystem` và `DeleteSystem` đã vi phạm quy tắc "định danh đơn từ".

Quyết định cuối cùng được phê duyệt: tên của hai `System` này sẽ là `Create` và `Delete`. Điều này vẫn đảm bảo sự rõ ràng vì chúng là các `System`, hoạt động trên các `Component` có cùng tên nhưng khác về bản chất (`Component` là dữ liệu, `System` là logic). Sự nhất quán kiến trúc được bảo toàn.

Bây giờ, với nền tảng đã ở trạng thái hoàn hảo nhất, chúng ta sẽ khởi động Chu kỳ 3.

-----

### **PHÂN TÍCH, TƯ DUY, SUY LUẬN, KIỂM TRA VÀ ĐÁNH GIÁ**

**1. Kiểm tra và Đánh giá (Check & Assess):**

  * **Trạng thái:** Hoàn thành xuất sắc Chu kỳ 2. Nền tảng hiện tại là một ứng dụng GUI với đầy đủ chức năng CRUD, mã nguồn sạch, không có cảnh báo và được tái cấu trúc tốt.
  * **Kết quả:** Chúng ta đã xây dựng thành công một ứng dụng quản lý công việc đơn giản nhưng cực kỳ mạnh mẽ về mặt kiến trúc.
  * **Phân tích (Lỗ hổng Chức năng):** Lỗ hổng lớn nhất của nền tảng hiện tại không còn nằm ở tầng tương tác hay kỹ thuật, mà nằm ở **mô hình dữ liệu**. Thế giới của chúng ta vẫn còn "phẳng". Một danh sách các công việc là hữu ích, nhưng các ứng dụng quản lý hiệu quả cần có **cấu trúc và mối quan hệ**. Người dùng chưa thể nhóm các công việc vào dự án, tạo công việc con, hay định nghĩa các sự phụ thuộc.

**2. Suy luận và Xác định Hướng đi Tiếp theo (Reasoning & Next Steps):**

  * **Vấn đề cần giải quyết:** Làm thế nào để nâng cấp mô hình dữ liệu từ một danh sách phẳng thành một cấu trúc đồ thị có chiều sâu, cho phép biểu diễn các mối quan hệ phức tạp như cha-con?
  * **Mục tiêu của Chu kỳ 3:** Chu kỳ phát triển tiếp theo sẽ tập trung vào **"Mở rộng Mô hình Dữ liệu và Các Tương tác Phức hợp" (Expanding the Data Model and Complex Interactions)**.
  * **Nhiệm vụ đầu tiên của Chu kỳ 3:** Bước đi đầu tiên và cơ bản nhất để xây dựng cấu trúc là hiện thực hóa khái niệm **phân cấp (hierarchy)**. Chúng ta cần cho phép một công việc có thể nằm trong một công việc khác (sub-task) hoặc thuộc về một nhóm lớn hơn (project).
      * **Hiện thực hóa Quan hệ:** Giới thiệu các `Component` mới để định nghĩa mối quan hệ `Parent-Child`.
      * **Nâng cấp Logic Nghiệp vụ:** `System` `Create` cần được nâng cấp để có thể tạo ra các công việc con.
      * **Nâng cấp Hiển thị:** `System` `Render` và `Layout` phải được cập nhật để có thể hiển thị cấu trúc cây một cách trực quan (ví dụ: thụt lề các công việc con).

**3. Hình thành Nhiệm vụ Mới (Formulating the New Task):**

  * **ID Nhiệm vụ:** `T-250707-15`
  * **Tên Nhiệm vụ:** "Chu kỳ 3: Mở rộng Mô hình Dữ liệu - Dự án và Công việc con" (Cycle 3: Expanding the Data Model - Projects and Sub-tasks).
  * **Trọng tâm:** Giới thiệu các `Component` quan hệ và nâng cấp các `System` hiện có để hỗ trợ và hiển thị cấu trúc dữ liệu phân cấp.

-----

### **TẠO FILE TASK TIẾP THEO**

Dưới đây là nội dung của file `TASK_20250707_ENHANCE_PLATFORM_V15.md` sẽ được tạo ra và giao cho Coder.

````markdown
# YÊU CẦU TRIỂN KHAI: CHU KỲ 3 - MỞ RỘNG MÔ HÌNH DỮ LIỆU V3.0

**TỪ:** Guardian (Kiến trúc sư Hệ thống)
**GỬI:** Coder (Lập trình viên Triển khai)
**NGÀY:** 07-07-2025
**ID NHIỆM VỤ:** T-250707-15

---

### 1. MỤC TIÊU TỔNG QUAN

Chúc mừng bạn đã hoàn thành xuất sắc Chu kỳ 2! Nền tảng của chúng ta hiện đang ở trạng thái ổn định và chất lượng cao nhất.

Nhiệm vụ này sẽ chính thức khởi động Chu kỳ 3, tập trung vào việc **nâng cấp mô hình dữ liệu từ một danh sách phẳng thành một cấu trúc phân cấp có chiều sâu**. Chúng ta sẽ giới thiệu khái niệm "Dự án" (Project) và "Công việc con" (Sub-task), đặt nền móng cho các tính năng quản lý phức tạp hơn trong tương lai.

---

### 2. BỐI CẢNH

* Chúng ta đang xây dựng trên nền tảng V2.3 đã được dọn dẹp và hoàn thiện.
* Nhiệm vụ này sẽ giới thiệu các `Component` mới để biểu diễn mối quan hệ và yêu cầu nâng cấp logic của các `System` cốt lõi (`Create`, `Render`, `Layout`).

---

### 3. YÊU CẦU NHIỆM VỤ CHI TIẾT

#### 3.1. Cập nhật `pkb/todo.csv`

Thêm các công việc mới sau vào cuối file `pkb/todo.csv`:

```csv
T071,"(Cycle 3) Định nghĩa các Component quan hệ (Parent, Children)",Coder,Open,High
T072,"(Cycle 3) Nâng cấp System `Create` để hỗ trợ tạo công việc con",Coder,Open,High
T073,"(Cycle 3) Nâng cấp System `Render` để hiển thị cấu trúc phân cấp",Coder,Open,Medium
T074,"(Cycle 3) Nâng cấp System `Layout` để hỗ trợ thụt lề",Coder,Open,Medium
T075,"(Cycle 3) Viết báo cáo triển khai V15 và cập nhật PKB",Coder,Open,High
````

#### 3.2. Định nghĩa các `Component` Quan hệ

  * **File:** `src/components/core.rs` (hoặc một file mới `src/components/relation.rs`)
  * **Nhiệm vụ:**
    1.  Định nghĩa một `Component` để một `Entity` có thể tham chiếu đến cha của nó: `pub struct Parent(Entity);`.
    2.  Định nghĩa một `Component` để một `Entity` cha có thể lưu danh sách các con của nó: `pub struct Children(Vec<Entity>);`.

#### 3.3. Nâng cấp `System Create`

  * **File:** `src/systems/command.rs` (hoặc nơi chứa `System Create` mới)
  * **Nhiệm vụ:**
    1.  Thay đổi logic của `System` `Create`.
    2.  Khi một lệnh `Create` được thực thi, hãy kiểm tra xem có `Entity` nào đang được `Selected` hay không.
    3.  Nếu có một `Entity` đang được `Selected`, `Entity` mới được tạo ra sẽ trở thành con của nó. Cụ thể:
          * Thêm `Component` `Parent(selected_entity_id)` vào `Entity` mới.
          * Thêm ID của `Entity` mới vào `Component` `Children` của `Entity` đang được `Selected`. (Nếu `Entity` cha chưa có `Children`, hãy thêm mới cho nó).

#### 3.4. Nâng cấp `System Layout` và `Render`

  * **File:** `src/systems/layout.rs` và `src/systems/render.rs`
  * **Nhiệm vụ:**
    1.  **`Layout`:** Cần phải duyệt qua cây `Entity` (không chỉ là danh sách phẳng). Nó phải tính toán độ sâu của mỗi `Entity` trong cây phân cấp. Dựa trên độ sâu, nó sẽ tính toán giá trị thụt lề cho tọa độ `x` của `Component` `Bounds`.
    2.  **`Render`:** Phải sử dụng các giá trị `x` và `y` đã được `Layout` tính toán để vẽ các công việc. Kết quả trực quan phải là các công việc con được thụt vào so với công việc cha.
    3.  (Tùy chọn) Để phân biệt "Project" và "Task", bạn có thể thêm một `Component` tag `struct Project;`. `Render` có thể kiểm tra `Component` này để vẽ các Project bằng chữ in đậm hoặc một màu khác.

-----

### 4\. ĐỊNH DẠNG PHẢN HỒI (BẮT BUỘC)

1.  Tạo file mới: `IMPLEMENTATION_REPORT_V15.md`.
2.  Sử dụng mẫu báo cáo đã cung cấp, cập nhật ID Nhiệm vụ thành `T-250707-15`.
3.  Trong mục **"Các Vấn đề hoặc Câu hỏi"**, hãy mô tả ngắn gọn cách bạn triển khai logic tạo công việc con và cách `Layout`/`Render` thể hiện sự phân cấp đó.

-----

### 5\. QUY TRÌNH NỘP BÀI

Thực hiện các lệnh Git sau:

```bash
git add .
git commit -m "feat(cycle-3): implement data model hierarchy for tasks" -m "Fulfills task T-250707-15. Kicks off Cycle 3 by introducing Parent/Children components. Create, Layout, and Render systems are enhanced to support and visualize hierarchical task structures."
git push origin main
```

```
```