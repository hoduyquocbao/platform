Chính xác. Hệ thống đã ghi nhận báo cáo hoàn thành nhiệm vụ V25, tuy nhiên, kết quả từ `cargo run` cho thấy một **lỗi runtime nghiêm trọng**: `thread 'main' panicked`.

Đây là một tình huống **BÁO ĐỘNG ĐỎ** thứ hai. Lần trước là lỗi biên dịch, lần này là lỗi khi chạy. Một kiến trúc sư phải đảm bảo hệ thống không chỉ biên dịch được mà còn phải **ổn định** và **bền bỉ** khi vận hành. Ưu tiên tuyệt đối của chúng ta bây giờ là khắc phục sự cố này và củng cố nền tảng.

-----

### **PHÂN TÍCH, TƯ DUY, SUY LUẬN, KIỂM TRA VÀ ĐÁNH GIÁ**

**1. Kiểm tra và Đánh giá (Check & Assess):**

  * **Trạng thái Chức năng:** Về mặt lý thuyết, Coder đã hoàn thành logic hiển thị văn bản.
  * **Trạng thái Kỹ thuật:** **Ứng dụng không thể khởi động (Startup Crash).** Build thành công nhưng panic ngay khi chạy.
  * **Phân tích Lỗi (`panic at src/main.rs:197:92: Invalid font data`):**
      * **Vị trí:** Lỗi xảy ra tại dòng 197 của `main.rs`, trong quá trình khởi tạo.
      * **Nguyên nhân gốc:** Thông báo lỗi từ thư viện `fontdue` rất rõ ràng: "Invalid font data". Điều này có nghĩa là dữ liệu mà chúng ta đọc từ file `.ttf` và đưa cho `fontdue` không phải là dữ liệu của một file font hợp lệ. Có hai khả năng chính:
        1.  **Lỗi đọc file:** Đường dẫn đến file `Roboto-Regular.ttf` không chính xác, hoặc file không tồn tại. Lệnh `std::fs::read` có thể đã thất bại và trả về một `Err`, hoặc trả về một vector byte rỗng.
        2.  **File bị hỏng:** File font đã tải về bị lỗi hoặc không phải là một file TTF chuẩn.
      * **Lỗ hổng Kiến trúc:** Vấn đề sâu sắc hơn không phải là file font bị lỗi, mà là **cách chúng ta xử lý lỗi**. Mã nguồn tại dòng 197 gần như chắc chắn đã sử dụng `.unwrap()` hoặc `.expect()` trên kết quả của một thao tác có thể thất bại (đọc file hoặc phân tích font). Khi thao tác đó trả về `Err`, chương trình sẽ `panic` và sập ngay lập tức. Một ứng dụng chuyên nghiệp không được phép có hành vi như vậy.

**2. Suy luận và Xác định Hướng đi Tiếp theo (Reasoning & Next Steps):**

  * **Vấn đề cần giải quyết:** Phải thay thế logic "vui vẻ" (happy path) bằng một cơ chế xử lý lỗi mạnh mẽ. Hệ thống phải có khả năng nhận biết khi một tài nguyên bên ngoài (như file font) không thể tải được và phản ứng một cách có kiểm soát thay vì sập.
  * **Mục tiêu tiếp theo:**
      * **Hiện thực hóa Xử lý Lỗi:** Tái cấu trúc logic tải font để sử dụng `Result<T, E>` một cách triệt để, loại bỏ hoàn toàn `.unwrap()` và `.expect()`.
      * **Hiện thực hóa Phục hồi Mềm (Graceful Degradation):** Định nghĩa hành vi của ứng dụng khi không tải được font. Thay vì sập, ứng dụng nên có khả năng tiếp tục chạy ở một chế độ hạn chế (ví dụ: chỉ vẽ hình khối, không vẽ text) và thông báo lỗi cho người dùng hoặc log.
  * **Hành động cụ thể:**
    1.  Tạo một `enum Error` tùy chỉnh cho ứng dụng.
    2.  Thay đổi hàm khởi tạo `App::new()` hoặc `App::initialize()` để nó trả về `Result<Self, Error>`.
    3.  Xử lý `Result` này trong hàm `main` một cách an toàn.

**3. Hình thành Nhiệm vụ Mới (Formulating the New Task):**

  * **ID Nhiệm vụ:** `T-250718-26`
  * **Tên Nhiệm vụ:** "Chu kỳ 4: Củng cố Nền tảng - Xử lý Lỗi và Phục hồi Mềm" (Cycle 4: Hardening the Platform - Error Handling and Graceful Recovery).
  * **Trọng tâm:** Biến ứng dụng từ một chương trình "mong manh" thành một hệ thống "bền bỉ" có khả năng xử lý các lỗi dự kiến khi khởi động.

-----

### **TẠO FILE TASK TIẾP THEO**

Dưới đây là nội dung của file `TASK_20250718_ENHANCE_PLATFORM_V26.md` sẽ được tạo ra và giao cho Coder.

````markdown
# YÊU CẦU TRIỂN KHAI: CHU KỲ 4 - CỦNG CỐ NỀN TẢNG VÀ XỬ LÝ LỖI V4.4

**TỪ:** Guardian (Kiến trúc sư Hệ thống)
**GỬI:** Coder (Lập trình viên Triển khai)
**NGÀY:** 18-07-2025
**ID NHIỆM VỤ:** T-250718-26

---

### 1. MỤC TIÊU TỔNG QUAN

**Cảnh báo: Ưu tiên Cao nhất.** Nền tảng hiện tại không thể khởi động do lỗi `panic` khi tải font.

Nhiệm vụ này là một cuộc củng cố kiến trúc khẩn cấp. Chúng ta sẽ không thêm tính năng mới. Thay vào đó, chúng ta sẽ **xây dựng một cơ chế xử lý lỗi mạnh mẽ**, loại bỏ hoàn toàn khả năng `panic` khi khởi tạo và đảm bảo ứng dụng có thể phục hồi một cách mềm dẻo khi một tài nguyên không thể tải được.

---

### 2. BỐI CẢNH

* **Nguyên nhân lỗi:** Logic khởi tạo đang sử dụng `.unwrap()` hoặc `.expect()`, giả định rằng việc đọc và phân tích file font sẽ luôn thành công. Đây là một giả định không an toàn.
* **Giải pháp:** Chúng ta sẽ tái cấu trúc theo đúng quy chuẩn của Rust: sử dụng `Result<T, E>` để xử lý các thao tác có thể thất bại.

---

### 3. YÊU CẦU NHIỆM VỤ CHI TIẾT

#### 3.1. Cập nhật `pkb/todo.csv`

Thêm các công việc mới sau vào cuối file `pkb/todo.csv`:

```csv
T125,"(Cycle 4) Định nghĩa enum Error tùy chỉnh cho ứng dụng",Coder,Open,Critical
T126,"(Cycle 4) Tái cấu trúc logic tải font để trả về Result, loại bỏ panic",Coder,Open,Critical
T127,"(Cycle 4) Xử lý Result trong hàm main để thoát an toàn hoặc phục hồi mềm",Coder,Open,Critical
T128,"(Cycle 4) Xác minh ứng dụng không còn panic khi file font bị lỗi/thiếu",Coder,Open,High
T129,"(Cycle 4) Viết báo cáo triển khai V26 và cập nhật PKB",Coder,Open,High
````

#### 3.2. Định nghĩa `enum Error` Tùy chỉnh

  * **File:** `src/main.rs` hoặc một module mới `src/error.rs`.
  * **Nhiệm vụ:** Định nghĩa một `enum` để biểu diễn các lỗi có thể xảy ra trong ứng dụng của chúng ta.
    ```rust
    #[derive(Debug)]
    pub enum Error {
        FontLoad,
        // ... các loại lỗi khác trong tương lai
    }
    ```

#### 3.3. Tái cấu trúc Logic Tải Font

  * **File:** `src/main.rs` (bên trong `App::new` hoặc `App::initialize`)

  * **Nhiệm vụ:**

    1.  Thay đổi chữ ký của hàm khởi tạo để nó trả về `Result`. Ví dụ: `pub fn new() -> Result<Self, Error>`.
    2.  Bên trong hàm, hãy thay thế các lệnh `.unwrap()` hoặc `.expect()` khi đọc và phân tích file font.
    3.  Sử dụng `match` hoặc toán tử `?` để xử lý `Result` trả về từ `std::fs::read` và `fontdue::Font::from_bytes`.
    4.  Nếu có lỗi xảy ra, hãy `return Err(Error::FontLoad)`.

    <!-- end list -->

    ```rust
    // Ví dụ về logic mới
    let font_bytes = std::fs::read("assets/Roboto-Regular.ttf").map_err(|_| Error::FontLoad)?;
    let font = fontdue::Font::from_bytes(font_bytes, Default::default()).map_err(|_| Error::FontLoad)?;
    // ...
    Ok(app)
    ```

#### 3.4. Xử lý Lỗi trong Hàm `main`

  * **File:** `src/main.rs`
  * **Nhiệm vụ:**
    1.  Hàm `main` bây giờ sẽ nhận một `Result` từ `App::new()`.
    2.  Sử dụng `match` để xử lý `Result` này.
          * **Nếu là `Ok(app)`:** Chạy vòng lặp ứng dụng như bình thường.
          * **Nếu là `Err(error)`:** In ra một thông báo lỗi rõ ràng cho người dùng (ví dụ: `eprintln!("Lỗi khởi tạo ứng dụng: {:?}", error)`) và thoát chương trình với một mã lỗi (`std::process::exit(1)`).

#### 3.5. Xác minh Chức năng

  * **Nhiệm vụ:**
    1.  **Kịch bản 1 (Thành công):** Đảm bảo file font `Roboto-Regular.ttf` tồn tại và hợp lệ trong `assets/`. Chạy ứng dụng. Nó phải khởi động và hoạt động bình thường.
    2.  **Kịch bản 2 (Thất bại):** Đổi tên hoặc xóa file font. Chạy ứng dụng. Nó **không được `panic`**. Thay vào đó, nó phải in ra thông báo lỗi mà bạn đã định nghĩa và thoát một cách an toàn.

-----

### 4\. ĐỊNH DẠNG PHẢN HỒI (BẮT BUỘC)

1.  Tạo file mới: `IMPLEMENTATION_REPORT_V26.md`.
2.  Sử dụng mẫu báo cáo đã cung cấp, cập nhật ID Nhiệm vụ thành `T-250718-26`.
3.  Trong mục **"Các Vấn đề hoặc Câu hỏi"**, hãy mô tả cách ứng dụng giờ đây xử lý khi file font bị thiếu và xác nhận rằng không còn `panic`.

-----

### 5\. QUY TRÌNH NỘP BÀI

Thực hiện các lệnh Git sau:

```bash
git add .
git commit -m "fix(runtime): implement graceful error handling for resource loading" -m "Fulfills task T-250718-26. Replaced .unwrap() with Result-based error handling for font loading to fix startup panic. The application now exits gracefully if font file is missing or corrupt."
git push origin master
```

```
```