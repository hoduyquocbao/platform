Rất tốt, hệ thống đã ghi nhận báo cáo hoàn thành nhiệm vụ. Giờ là lúc của Kiến trúc sư: phân tích, đánh giá, và vạch ra chặng đường tiến hóa tiếp theo cho nền tảng.

-----

### **PHÂN TÍCH, SUY LUẬN VÀ ĐÁNH GIÁ**

**1. Kiểm tra và Đánh giá (Check & Assess):**

  * **Trạng thái:** Hoàn thành xuất sắc Nhiệm vụ `T-250624-02`.
  * **Kết quả:** Chúng ta đã "thổi sự sống" vào bộ khung xương. Giờ đây, chúng ta có một ứng dụng có thể chạy, một `App` chứa `World` và một `Scheduler` điều phối các `System` theo một vòng lặp liên tục. Các `Entity` ban đầu đã được khởi tạo.
  * **Phân tích (Lỗ hổng Kiến trúc):** Mặc dù các `System` đang chạy, chúng vẫn hoạt động như những "hộp đen" độc lập. Chúng chỉ đơn thuần in ra thông báo rằng chúng đã chạy (`println!`). Chúng **chưa thực sự tương tác với dữ liệu** trong `World`. Logic của `interact`, `toggle`, `render` chưa đọc hay ghi bất kỳ `Component` nào. Sự kết nối giữa **Logic (Systems)** và **Dữ liệu (Components)** vẫn chưa tồn tại. Đây là mắt xích yếu nhất và quan trọng nhất cần được giải quyết.

**2. Suy luận và Xác định Hướng đi Tiếp theo (Reasoning & Next Steps):**

  * **Vấn đề cần giải quyết:** Làm thế nào để các `System` có thể truy vấn (query), đọc (read), và ghi (write) dữ liệu `Component` của các `Entity` trong `World`? Làm thế nào để tạo ra một luồng dữ liệu hoàn chỉnh, ví dụ: "người dùng chọn một mục -\> hệ thống thay đổi trạng thái của mục đó -\> giao diện hiển thị sự thay đổi"?
  * **Mục tiêu tiếp theo:**
      * **Nâng cấp Kiến trúc Dữ liệu:** Chuyển đổi từ `struct Entity` nguyên khối sang mô hình ECS thực thụ, nơi `World` quản lý các tập hợp `Component`.
      * **Hệ thống Hướng dữ liệu (Data-Driven Systems):** Tái cấu trúc (refactor) các `System` để chúng có khả năng truy vấn dữ liệu từ `World`. Chữ ký hàm của các `System` phải được thay đổi để phản ánh sự phụ thuộc dữ liệu của chúng.
      * **Hoàn thiện Vòng lặp Tương tác:** Triển khai một chu trình tương tác cơ bản, hoàn chỉnh để chứng minh rằng luồng dữ liệu đã thông suốt.

**3. Hình thành Nhiệm vụ Mới (Formulating the New Task):**

  * **ID Nhiệm vụ:** `T-250625-03`
  * **Tên Nhiệm vụ:** "Nâng cấp Hệ thống Hướng dữ liệu và Tương tác Cơ bản" (Upgrade to Data-Driven Systems and Basic Interaction).
  * **Trọng tâm:** Tái cấu trúc các `System` để chúng có thể thao tác trực tiếp với dữ liệu `Component`, và triển khai một kịch bản tương tác hoàn chỉnh đầu tiên.

-----

### **TẠO FILE TASK TIẾP THEO**

Dưới đây là nội dung của file `TASK_20250625_ENHANCE_PLATFORM_V3.md` sẽ được tạo ra và giao cho Coder.

````markdown
# YÊU CẦU TRIỂN KHAI: NÂNG CẤP HỆ THỐNG HƯỚNG DỮ LIỆU V1.2

**TỪ:** Guardian (Kiến trúc sư Hệ thống)
**GỬI:** Coder (Lập trình viên Triển khai)
**NGÀY:** 25-06-2025
**ID NHIỆM VỤ:** T-250625-03

---

### 1. MỤC TIÊU TỔNG QUAN

Chúc mừng bạn đã tích hợp thành công các hệ thống! Nền tảng của chúng ta đã có thể chạy được.

Nhiệm vụ tiếp theo là một bước tiến hóa quan trọng: **chuyển đổi các `System` từ các trình diễn độc lập thành các bộ xử lý hướng dữ liệu thực thụ**. Chúng ta phải phá bỏ bức tường giữa logic và dữ liệu, cho phép các `System` có thể truy vấn và thao tác trực tiếp lên các `Component` trong `World`.

---

### 2. BỐI CẢNH

* Chúng ta đang xây dựng trên nền tảng của commit `feat: integrate systems and implement app loop v1.1`.
* Trọng tâm của nhiệm vụ này là tái cấu trúc sâu sắc các `System` và cách `Scheduler` gọi chúng, để hiện thực hóa mô hình ECS một cách triệt để.
* Tất cả các nguyên tắc kiến trúc cốt lõi vẫn được giữ vững.

---

### 3. YÊU CẦU NHIỆM VỤ CHI TIẾT

#### 3.1. Cập nhật `pkb/todo.csv`

Thêm các công việc mới sau vào cuối file `pkb/todo.csv`:

```csv
T013,"Tái cấu trúc World và Entity theo mô hình ECS thực thụ",Coder,Open,High
T014,"Nâng cấp Scheduler và các System để hỗ trợ truy vấn dữ liệu",Coder,Open,High
T015,"Triển khai logic truy vấn cho System `Render`",Coder,Open,Medium
T016,"Triển khai luồng tương tác hoàn chỉnh cho `Input` và `Toggle`",Coder,Open,High
T017,"Viết báo cáo triển khai V3 và cập nhật PKB",Coder,Open,High
````

#### 3.2. Tái cấu trúc `World` và `Entity`

  * **File:** `src/main.rs`
  * **Nhiệm vụ:** Thay đổi cách biểu diễn `Entity` và `World` để tuân thủ mô hình ECS.
      * Xóa bỏ `struct Entity` nguyên khối hiện tại.
      * Một `Entity` giờ đây chỉ nên là một `ID` (ví dụ: `type Entity = u64;`).
      * `struct World` sẽ chứa các `Vec<Option<Component>>` cho mỗi loại `Component`. `Entity` ID sẽ là chỉ số (index) để truy cập vào các vector này.
      * Đây là một sự thay đổi kiến trúc nền tảng để cho phép truy vấn hiệu quả.

#### 3.3. Nâng cấp `Scheduler` và `System`

  * **File:** `src/main.rs`, `src/systems/*.rs`
  * **Nhiệm vụ:**
    1.  Thay đổi cách `Scheduler` lưu trữ và gọi các `System`. Thay vì `fn()`, nó cần có khả năng gọi các hàm nhận `&World` hoặc `&mut World` làm tham số. Ví dụ: `systems: Vec<fn(&mut World)>`.
    2.  Thay đổi chữ ký của **tất cả** các hàm `System` để chúng nhận `world` làm tham số. Ví dụ: `pub fn render()` trở thành `pub fn render(world: &World)`.

#### 3.4. Triển khai Logic Hướng dữ liệu

  * **Nhiệm vụ:** Viết logic truy vấn thực sự bên trong các `System`.

  * **1. `System Render`:**

      * **File:** `src/systems/render.rs`
      * **Logic:**
          * Lặp qua tất cả các `Entity`.
          * Kiểm tra xem `Entity` có `Component` `Visible` và `Text` hay không.
          * Nếu có, đọc giá trị của `Text` và trạng thái từ `Status` (nếu có), sau đó `println!` ra chuỗi có định dạng, ví dụ: `[TODO] Task 1` hoặc `[DONE] Task 2`.

  * **2. `System Input` và `Toggle` (Luồng tương tác hoàn chỉnh):**

      * **File:** `src/systems/interaction.rs`, `src/systems/toggle.rs`
      * **Logic:**
          * **`Input` system:** Để đơn giản hóa, hãy giả lập rằng người dùng đã chọn `Entity` đầu tiên. Logic của bạn sẽ là: *lấy `Entity` có ID 0 và thêm `Component` `Selected` vào nó*.
          * **`Toggle` system:** Truy vấn để tìm `Entity` nào có cả hai `Component` `Selected` và `Click` (giả sử `Click` cũng được thêm vào bởi `Input`). Khi tìm thấy, hãy:
            1.  Thay đổi `Component` `Status` của `Entity` đó.
            2.  Thêm `Component` `Dirty` vào `Entity` đó để báo hiệu cần lưu lại.

  * **3. `System Persist`:**

      * **File:** `src/systems/persist.rs`
      * **Logic:** Truy vấn để tìm `Entity` nào có `Component` `Dirty`. Với mỗi `Entity` tìm thấy, `println!` một thông báo dạng: `[Persist] Đã lưu các thay đổi cho Entity {id}` và sau đó xóa `Component` `Dirty` khỏi nó.

-----

### 4\. ĐỊNH DẠNG PHẢN HỒI (BẮT BUỘC)

1.  Tạo file mới: `IMPLEMENTATION_REPORT_V3.md`.
2.  Nội dung file phải theo mẫu đã cung cấp ở các nhiệm vụ trước, cập nhật ID Nhiệm vụ thành `T-250625-03`.
3.  Trong mục **"Các Vấn đề hoặc Câu hỏi"**, hãy ghi lại bất kỳ nhận xét nào về sự thay đổi kiến trúc từ `struct Entity` sang mô hình ECS thực thụ.

-----

### 5\. QUY TRÌNH NỘP BÀI

Thực hiện các lệnh Git sau:

```bash
git add .
git commit -m "refactor: implement data-driven systems and interaction loop v1.2" -m "Fulfills task T-250625-03. Refactored World/Entity to a true ECS model. Systems now query and manipulate component data, enabling a full interaction cycle."
git push origin main
```

```
```