Chính xác. Hệ thống đã ghi nhận báo cáo hoàn thành nhiệm vụ V17.

-----

### **PHÂN TÍCH, TƯ DUY, SUY LUẬN, KIỂM TRA VÀ ĐÁNH GIÁ**

**1. Kiểm tra và Đánh giá (Check & Assess):**

  * **Trạng thái:** Hoàn thành xuất sắc Nhiệm vụ `T-250709-17`.
  * **Kết quả:** Nền tảng của chúng ta giờ đây đã có một giao diện phân cấp thực sự có thể tương tác được. Người dùng có thể quản lý sự phức tạp của giao diện bằng cách mở và đóng các node cha. Đây là một tính năng cốt lõi cho bất kỳ ứng dụng quản lý công việc nào và nó đã được triển khai thành công trên kiến trúc `trait-based` vững chắc của chúng ta.
  * **Phân tích (Lỗ hổng Toàn vẹn Dữ liệu):** Chúng ta đã thêm các mối quan hệ cha-con (`Parent`, `Children`) và các tương tác trên đó. Tuy nhiên, chúng ta chưa định nghĩa hành vi của hệ thống trong một trường hợp cực kỳ quan trọng: **điều gì xảy ra khi một `Entity` cha bị xóa?** `System` `Delete` hiện tại có khả năng chỉ xóa `Entity` cha, để lại toàn bộ các `Entity` con của nó trở thành "mồ côi" (orphaned entities) trong `World`. Đây là một lỗi nghiêm trọng về toàn vẹn dữ liệu, có thể gây ra memory leak và hành vi không thể đoán trước.

**2. Suy luận và Xác định Hướng đi Tiếp theo (Reasoning & Next Steps):**

  * **Vấn đề cần giải quyết:** Phải đảm bảo rằng khi một node cha bị xóa, toàn bộ cây con của nó cũng phải được xóa một cách sạch sẽ và có trật tự.
  * **Mục tiêu tiếp theo:**
      * **Hiện thực hóa Chức năng Xóa theo Tầng (Cascading Deletes):** Nâng cấp logic xóa để nó có khả năng duyệt qua toàn bộ cây con của một `Entity` và đánh dấu tất cả chúng để xóa.
      * **Đảm bảo Sự Bền vững của Kiến trúc:** Việc giải quyết các vấn đề về toàn vẹn dữ liệu ngay sau khi thêm một tính năng mới là một thực hành kiến trúc tốt nhất. Nó ngăn chặn các lỗi phức tạp phát sinh sau này.
  * **Hành động cụ thể:**
    1.  Tái cấu trúc `System` `Delete` (hoặc tạo một `System` `Cascade` mới nếu cần).
    2.  Khi xử lý một `Entity` cần xóa, `System` phải kiểm tra xem `Entity` đó có `Component` `Children` không.
    3.  Nếu có, nó phải thực hiện một cuộc duyệt đệ quy (recursive traversal) qua tất cả các `Entity` con và cháu, đánh dấu tất cả chúng để xóa.
    4.  Chỉ sau khi toàn bộ cây con đã được xử lý, `Entity` cha ban đầu mới được xóa.

**3. Hình thành Nhiệm vụ Mới (Formulating the New Task):**

  * **ID Nhiệm vụ:** `T-250710-18`
  * **Tên Nhiệm vụ:** "Chu kỳ 3: Đảm bảo Toàn vẹn Dữ liệu - Xóa theo Tầng" (Cycle 3: Ensuring Data Integrity - Cascading Deletes).
  * **Trọng tâm:** Thêm một lớp logic quan trọng vào hệ thống để đảm bảo tính toàn vẹn của cấu trúc dữ liệu phân cấp khi thực hiện thao tác xóa.

-----

### **TẠO FILE TASK TIẾP THEO**

Dưới đây là nội dung của file `TASK_20250710_ENHANCE_PLATFORM_V18.md` sẽ được tạo ra và giao cho Coder.

````markdown
# YÊU CẦU TRIỂN KHAI: CHU KỲ 3 - ĐẢM BẢO TOÀN VẸN DỮ LIỆU V3.3

**TỪ:** Guardian (Kiến trúc sư Hệ thống)
**GỬI:** Coder (Lập trình viên Triển khai)
**NGÀY:** 10-07-2025
**ID NHIỆM VỤ:** T-250710-18

---

### 1. MỤC TIÊU TỔNG QUAN

Chúc mừng bạn đã triển khai thành công tính năng tương tác phân cấp! Nền tảng của chúng ta ngày càng mạnh mẽ.

Tuy nhiên, có một lỗ hổng kiến trúc nghiêm trọng cần được giải quyết ngay lập tức. Nhiệm vụ này tập trung vào việc **đảm bảo toàn vẹn dữ liệu bằng cách triển khai chức năng "Xóa theo Tầng" (Cascading Deletes)**. Chúng ta phải chắc chắn rằng khi một công việc cha bị xóa, tất cả các công việc con của nó cũng được dọn dẹp một cách an toàn.

---

### 2. BỐI CẢNH

* Chúng ta đang làm việc trên nền tảng V3.2, đã có cấu trúc phân cấp tương tác.
* `System` `Delete` hiện tại có thể đang để lại các `Entity` con mồ côi khi xóa cha. Đây là một lỗi cần được ưu tiên khắc phục.
* Nhiệm vụ này tập trung vào logic backend và sự ổn định của hệ thống, không có thay đổi lớn về giao diện người dùng.

---

### 3. YÊU CẦU NHIỆM VỤ CHI TIẾT

#### 3.1. Cập nhật `pkb/todo.csv`

Thêm các công việc mới sau vào cuối file `pkb/todo.csv`:

```csv
T086,"(Cycle 3) Phân tích và thiết kế logic xóa theo tầng",Coder,Open,Critical
T087,"(Cycle 3) Nâng cấp System `Delete` để thực hiện xóa theo tầng",Coder,Open,Critical
T088,"(Cycle 3) Viết kịch bản kiểm thử để xác minh logic xóa",Coder,Open,High
T089,"(Cycle 3) Viết báo cáo triển khai V18 và cập nhật PKB",Coder,Open,High
````

#### 3.2. Nâng cấp `System Delete`

  * **File:** `src/systems/command.rs` (hoặc nơi chứa `System Delete`)

  * **Nhiệm vụ:**

    1.  Tái cấu trúc logic xử lý `Component` `Delete`.
    2.  Khi `System` xử lý một `Entity` `E` có `Component` `Delete`, nó phải thực hiện các bước sau **trước khi** thực sự xóa `E`:
    3.  **Kiểm tra `Children`:** Kiểm tra xem `E` có `Component` `Children` hay không.
    4.  **Đệ quy Đánh dấu:** Nếu có, hãy viết một hàm (hoặc sử dụng một vòng lặp với stack) để duyệt qua tất cả các `Entity` con trong `Children`. Với mỗi `Entity` con, hãy thêm `Component` `Delete` vào nó. Quá trình này phải lặp lại cho các cháu, chắt... cho đến khi toàn bộ cây con đã được đánh dấu.
    5.  **Thứ tự Xóa:** Đảm bảo rằng logic xóa cuối cùng (ví dụ `world.sweep()`) sẽ dọn dẹp tất cả các `Entity` được đánh dấu một cách chính xác.

  * **Gợi ý Kiến trúc:**
    Bạn có thể chia quá trình này thành hai `System` hoặc hai giai đoạn để đảm bảo sự rõ ràng:

      * **`System MarkCascade`**: Chạy trước, chỉ làm một việc: tìm các `Entity` có `Delete` và `Children`, sau đó duyệt cây con để thêm `Delete` vào tất cả các node con.
      * **`System Sweep`**: Chạy sau, chỉ làm một việc: xóa tất cả các `Entity` có `Component` `Delete` khỏi `World`.

#### 3.3. Xác minh Chức năng

  * **Nhiệm vụ:**
    1.  Khởi tạo một cây công việc có cấu trúc (ví dụ: 1 Project -\> 2 Task con -\> mỗi Task con có 2 Sub-task con nữa).
    2.  Thực hiện hành động xóa Project cha.
    3.  Quan sát output của `System` `Render`. Toàn bộ cây công việc phải biến mất khỏi màn hình.
    4.  (Nâng cao) Nếu có thể, hãy kiểm tra trạng thái của `World` để xác nhận rằng tất cả các `Entity` liên quan đã thực sự bị xóa khỏi các vector `Component`.

-----

### 4\. ĐỊNH DẠNG PHẢN HỒI (BẮT BUỘC)

1.  Tạo file mới: `IMPLEMENTATION_REPORT_V18.md`.
2.  Sử dụng mẫu báo cáo đã cung cấp, cập nhật ID Nhiệm vụ thành `T-250710-18`.
3.  Trong mục **"Các Vấn đề hoặc Câu hỏi"**, hãy mô tả chi tiết cách bạn đã triển khai và xác minh logic xóa theo tầng để đảm bảo không còn `Entity` mồ côi.

-----

### 5\. QUY TRÌNH NỘP BÀI

Thực hiện các lệnh Git sau:

```bash
git add .
git commit -m "fix(data): implement cascading deletes for hierarchy" -m "Fulfills task T-250710-18. Implemented a cascading delete mechanism to prevent orphaned entities when deleting a parent node. This ensures data integrity for the hierarchical model."
git push origin main
```

```
```